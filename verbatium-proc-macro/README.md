# verbatium-proc-macro

Shows the structure of how to create and use proc-macro. It is
the simplest proc-macro I could think of that does something
with it's input TokenStream :)


From [ferrous-systems testing-proc-macros](https://ferrous-systems.com/blog/testing-proc-macros/#tokenstream)
Adding a `dbg!` is useful for debuggin, so I did with `verbatium!`:
```
#[proc_macro]
pub fn verbatium(input: TokenStream) -> TokenStream {
    dbg!(input)
}
```
Yields the input stream in the compiler output, **NICE!**:
```
wink@3900x 22-05-15T17:38:50.224Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ cargo run --release
   Compiling verbatium-proc-macro v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/verbatium-proc-macro)
   Compiling expr-proc-macros v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros)
[verbatium-proc-macro/src/lib.rs:6] input = TokenStream [
    Ident {
        ident: "println",
        span: #0 bytes(556..563),
    },
    Punct {
        ch: '!',
        spacing: Alone,
        span: #0 bytes(563..564),
    },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [
            Literal {
                kind: Str,
                symbol: "hello, {}",
                suffix: None,
                span: #0 bytes(565..576),
            },
            Punct {
                ch: ',',
                spacing: Alone,
                span: #0 bytes(576..577),
            },
            Literal {
                kind: Str,
                symbol: "world",
                suffix: None,
                span: #0 bytes(578..585),
            },
        ],
        span: #0 bytes(564..586),
    },
]
    Finished release [optimized] target(s) in 0.37s
     Running `target/release/expr-proc-macros`
MyStruct is a struct with these named fields: my_string, my_enum, my_number.
MyTupleStruct is a struct with these 3 unamed fields.
MyEnum is an enum with these variants: VariantA, VariantB.
MyUnion is a union with these named fields: unsigned, signed.
hello, world
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

