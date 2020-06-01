# rayon-macro

[![rayon-macro crate](https://img.shields.io/crates/v/rayon-macro.svg)](https://crates.io/crates/rayon-macro)
[![rayon-macro documentation](https://docs.rs/rayon-macro/badge.svg)](https://docs.rs/rayon-macro)
![minimum rustc 1.31](https://img.shields.io/badge/rustc-1.31+-red.svg)
![build status](https://github.com/rayon-rs/rayon-macro/workflows/master/badge.svg)

The `rayon-macro` crate provides procedural macros to make it easier to
transform serial code into `rayon`-enabled parallel code. For example,
the `parallel!` macro can be used like this:

```rust
use rayon_macro::parallel;

parallel!(for i in 0..10 {
    println!("iteration {}", i);
});
```

It will be expanded to something like this:

```rust
(0..10).into_par_iter().for_each(|i| {
    println!("iteration {}", i);
});
```

Control-flow expressions in the body will also be transformed as needed.

This crate currently requires `rustc 1.31.0` or greater.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
