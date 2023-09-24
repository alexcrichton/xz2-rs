# liblzma

[Documentation](https://docs.rs/liblzma)

Bindings to the liblzma implementation in Rust, also provides types to
read/write xz streams.

**This crate is forked from [xz2](https://crates.io/crates/xz2) and `liblzma = "0.1.7"` is fully compatible with `xz2 = "0.1.7"`,**
so you can migrate simply.

## Migrate from xz2

```toml
# Cargo.toml
[dependencies]
-xz2 = "0.1.7"
+liblzma = "0.1.7"
```

```rust
// *.rs
-use xz2;
+use liblzma;
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in liblzma by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
