# flexible-string
 
A stack heap flexible string designed to improve performance.

`FlexibleString` was first implemented in [spdlog-rs] crate, which improved performance for [spdlog-rs] by about double (see [benchmarks of spdlog-rs]). Now it is extracted to a separate crate for use by other crates.

For more details, please read the crate [documentation].

## Benchmarks

Run `cargo +nightly bench` in the root directory of this repository for benchmarking.

The following results are generated with `Windows 10 64 bit` and `Intel i9-10900KF CPU @ 3.70GHz`.

 - `FlexibleString`

   ```
   test bench_clone    ... bench:           7 ns/iter (+/- 0)
   test bench_from_str ... bench:           6 ns/iter (+/- 0)
   test bench_push     ... bench:           0 ns/iter (+/- 0)
   test bench_push_str ... bench:           0 ns/iter (+/- 0)
   ```

 - `std::string::String`

   ```
   test bench_clone    ... bench:          46 ns/iter (+/- 0)
   test bench_from_str ... bench:          40 ns/iter (+/- 0)
   test bench_push     ... bench:          41 ns/iter (+/- 0)
   test bench_push_str ... bench:          39 ns/iter (+/- 0)
   ```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[spdlog-rs]: https://crates.io/crates/spdlog-rs
[benchmarks of spdlog-rs]: https://github.com/SpriteOvO/spdlog-rs#benchmarks
[documentation]: https://docs.rs/flexible-string
