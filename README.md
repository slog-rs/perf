<p align="center">

  <a href="https://github.com/slog-rs/slog">
  <img src="https://cdn.rawgit.com/slog-rs/misc/master/media/slog.svg" alt="slog-rs logo">
  </a>
  <br>

  <a href="https://travis-ci.org/slog-rs/perf">
      <img src="https://img.shields.io/travis/slog-rs/perf/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://crates.io/crates/slog-perf">
      <img src="https://img.shields.io/crates/d/slog-perf.svg" alt="slog-perf on crates.io">
  </a>

  <a href="https://gitter.im/slog-rs/slog">
      <img src="https://img.shields.io/gitter/room/slog-rs/slog.svg" alt="slog-rs Gitter Chat">
  </a>
</p>

# slog-perf - Performance and time reporting for [slog-rs]

This crate provides very useful tools for reporting performance metrics
through `slog`.

Check documentation for details and [reverse dependencies] for examples.

[reverse dependencies]: https://crates.io/crates/slog-perf/reverse_dependencies

For more information about slog, help, to report issues etc. see [slog-rs][slog-rs].

[slog-rs]: //github.com/slog-rs/slog


## Sample output

The following is a sample output taken from [`rdedup`][rdedup]. Read more on
[Rust's fearless concurrency in rdedup](http://dpc.pw/blog/2017/04/rusts-fearless-concurrency-in-rdedup/)

[rdedup]: https://github.com/dpc/rdedup

```
Apr 18 21:07:01.604 INFO time report, name: input-reader, tx: 3.595258855, input: 0.246253282
Apr 18 21:07:01.626 INFO time report, name: chunker, rx-and-chunking: 3.224178159, tx: 0.622769659
Apr 18 21:07:03.109 INFO time report, name: chunker, rx-and-chunking: 4.012730305, tx: 0.000014623
Apr 18 21:07:03.109 INFO time report, name: chunk-processing, compress: 2.7148105080000002, rx: 1.300910598, processing: 1.140292378, encrypt: 0.172064646, tx-writer: 0.014618051, tx-digest: 0.003738682
Apr 18 21:07:03.109 INFO time report, name: chunk-processing, compress: 2.651608273, rx: 1.463426039, processing: 1.063661373, encrypt: 0.138678476, tx-writer: 0.026703753, tx-digest: 0.002391723
Apr 18 21:07:03.109 INFO time report, name: chunk-processing, compress: 2.836732468, processing: 1.252152728, rx: 1.060371588, encrypt: 0.166409699, tx-writer: 0.026879077, tx-digest: 0.00393774
Apr 18 21:07:03.109 INFO time report, name: chunk-processing, compress: 3.453143579, processing: 1.6281752109999998, encrypt: 0.17906647, rx: 0.044625981, tx-writer: 0.035811789, tx-digest: 0.005779669
Apr 18 21:07:03.109 INFO time report, name: chunk-processing, compress: 2.888089877, processing: 1.353910947, rx: 0.928534413, encrypt: 0.151586489, tx-writer: 0.021566908, tx-digest: 0.002685722
Apr 18 21:07:03.110 INFO time report, name: chunk-processing, compress: 2.546534754, rx: 1.5378131019999999, processing: 1.058227244, encrypt: 0.180615367, tx-writer: 0.01845777, tx-digest: 0.004919227
Apr 18 21:07:03.110 INFO time report, name: chunk-processing, compress: 2.9532250639999997, processing: 1.4180076609999999, rx: 0.781645676, encrypt: 0.181882181, tx-writer: 0.00843785, tx-digest: 0.003109534
Apr 18 21:07:03.110 INFO time report, name: chunk-processing, compress: 3.144602231, processing: 1.317602689, rx: 0.694346659, encrypt: 0.173031093, tx-writer: 0.014143753, tx-digest: 0.002759141
Apr 18 21:07:03.110 INFO time report, name: chunk-writer, rx: 5.298270641, fsync: 0.044138053, write: 0.00320627, processing: 0.000783067
```
