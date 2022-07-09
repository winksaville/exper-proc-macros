# proc-macro-expr-struct

Trying to provide a "minimal" example of a proc-macro initializing
a struct that needs a to initailize a field with function pointer type
and I'm trying to initialize it in a quote! macro.

I'm hoping this will be used a simple example for discussing with the
rust community, or maybe allow me to come up with a solution.

# Run

```
wink@3900x 22-07-09T00:56:46.465Z:~/prgs/rust/myrepos/exper-proc-macros/proc-macro-expr-struct (Unable-init-initialize-sturct-with-path)
$ cargo run
   Compiling proc-macro-expr-struct v0.1.0 (/home/wink/prgs/rust/myrepos/exper-proc-macros/proc-macro-expr-struct)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/home/wink/prgs/rust/myrepos/exper-proc-macros/target/debug/proc-macro-expr-struct`
X: x.data=0
X: x.data=1
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

