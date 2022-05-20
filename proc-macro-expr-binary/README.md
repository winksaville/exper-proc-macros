# dbg-expr-binary

Because ExprBinary doesn't `impl Debug` this won't compile:

```
wink@3900x 22-05-17T17:37:03.620Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ cargo check
   Compiling expr-binary v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/expr-binary)
error[E0277]: `ExprBinary` doesn't implement `Debug`
 --> expr-binary/src/lib.rs:8:5
  |
8 |     dbg!(_expr); // This won't compile as ExprBinary doesn't impl Debug
  |     ^^^^^^^^^^^ `ExprBinary` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |
  = help: the trait `Debug` is not implemented for `ExprBinary`
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0277`.
error: could not compile `expr-binary` due to previous error
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

