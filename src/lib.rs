//! This crate provides very useful tools for reporting performance metrics
//! through `slog`.
#[macro_use]
extern crate slog;

use slog::Logger;
use std::time;
use std::collections::HashMap;

/// Collect and report total time spent on set of activities
///
/// `TimeReporter` is useful for generating and reporting
/// time reports: how much time was spend on a given activity.
///
/// On `drop` or on call to `finish` it will report total times
/// gathered to `slog::Logger`.
pub struct TimeReporter {
    times: HashMap<&'static str, time::Duration>,
    cur_state_time: Option<(&'static str, time::Instant)>,
    name: String,
    log: Logger,
}

impl TimeReporter {
    /// Create new `TimeReporter`
    pub fn new<S : Into<String>>(name: S, log: Logger) -> TimeReporter {
        TimeReporter {
            times: HashMap::new(),
            name: name.into(),
            log: log,
            cur_state_time: None,
        }
    }

    /// Start counting time for a state named "key"
    ///
    /// If this the `TimeReporter` was already counting time
    /// for another state, it will end counting time for it
    /// before starting new one.
    pub fn start(&mut self, key: &'static str) {

        let now = time::Instant::now();

        self.save_current(now);

        self.cur_state_time = Some((key, now))
    }

    /// Start counting time and execute a function `f`
    ///
    /// This is handy syntax for `if let` or `while let` expressions
    /// where it would be inconvenient to add standalone `start` call.
    pub fn start_with<F, R>(&mut self, key :&'static str, f: F) -> R
        where F: FnOnce() -> R
    {
        self.start(key);

        f()
    }

    fn save_current(&mut self, now: time::Instant) {
        if let Some((key, prev)) = self.cur_state_time.take() {
            *self.times
                 .entry(key)
                 .or_insert(time::Duration::new(0, 0)) += now - prev;
        }
    }

    pub fn stop(&mut self) {
        let now = time::Instant::now();
        self.save_current(now);
    }

    /// Finish counting time and report results
    pub fn finish(self) {}
}

// Private struct implementing `slog::KV`
struct TimeReporterDroper<'a> {
    reporter: &'a TimeReporter,
}

impl<'a> slog::KV for TimeReporterDroper<'a> {
    fn serialize(&self,
                 _record: &slog::Record,
                 ser: &mut slog::Serializer)
                 -> slog::Result {

        let mut stats: Vec<(&'static str, time::Duration)> = self.reporter
            .times
            .iter()
            .map(|(&k, &v)| (k, v))
            .collect();

        // TODO: or by duration?
        stats.sort_by_key(|s| s.1);

        ser.emit_arguments(
            "name", &format_args!("{}", self.reporter.name)
            )?;

        for &(state, dur) in stats.iter().rev() {
            ser.emit_arguments(
                state,
                &format_args!("{}",
                            dur.as_secs() as f64
                            + dur.subsec_nanos() as f64 / 1000000000f64)
                )?;
        }


        Ok(())
    }
}
impl Drop for TimeReporter {
    fn drop(&mut self) {
        self.stop();

        debug_assert!(self.cur_state_time.is_none());

        let lrd = TimeReporterDroper { reporter: &self };

        info!(self.log, #"slog-perf", "time report"; lrd)
    }
}
