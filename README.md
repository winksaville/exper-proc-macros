# expr-proc-macros

A workspace with experiments using rust proc-macros.

Searches:

https://www.google.com/search?q=rust+procedural+macros+tutorial&tbs=qdr:y

 * https://ferrous-systems.com/blog/testing-proc-macros/ Good starter
 * https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/
 * https://blog.turbo.fish/proc-macro-basics/

https://www.google.com/search?q=rust+%22procedural%22+macros+dsl+example

 * https://github.com/korken89/smlang-rs This is interesting as my goal is to create a sm-macro!

## Build run

See [proc-macro-nesting/README.md](proc-macro-nesting/README.md) for more info.

```
wink@3900x 22-06-03T21:30:27.447Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run -- --verbose
   Compiling proc-macro2 v1.0.39
   Compiling unicode-ident v1.0.0
   Compiling syn v1.0.95
   Compiling quote v1.0.18
   Compiling proc-macro-nesting v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/proc-macro-nesting)
   Compiling proc-macro-verbatim v0.2.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/proc-macro-verbatim)
   Compiling proc-macro-derive-using-parse v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/proc-macro-derive-using-parse)
   Compiling proc-macro-expr-binary v0.2.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/proc-macro-expr-binary)
   Compiling proc-macro-derive-describe v0.2.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros/proc-macro-derive-describe)
   Compiling expr-proc-macros v0.1.0 (/home/wink/prgs/rust/myrepos/expr-proc-macros)
    Finished dev [unoptimized + debuginfo] target(s) in 3.81s
     Running `target/debug/expr-proc-macros --verbose`
MyStruct is a struct with these named fields: my_string, my_enum, my_number.
MyTupleStruct is a struct with these 3 unamed fields.
MyEnum is an enum with these variants: VariantA, VariantB.
MyUnion is a union with these named fields: unsigned, signed.
hello, world
expr_binary_to_swap_and_substract: res=-1
outer: a=10, b=20, 10*20=200
inner_using_outer_declarations: a=10, b=20, 10+20=30
inner_creating_own_scope_using_no_outer_declarations: a=1, b=2, 1+2=3
inner_using_outer_declarations: a=10, b=20, 10+20=30
inner_replacing_outer_scope_declaractions_using_parens: a=3, b=4, 3+4=7
inner_using_outer_declarations: a=3, b=4, 3+4=7
inner_replacing_outer_scope_declaractions_using_braces: a=5, b=6, 5+6=11
inner_using_outer_declarations: a=5, b=6, 5+6=11
inner_replacing_outer_scope_declaractions_using_square_brackets: a=7, b=8, 7+8=15
inner_using_outer_declarations: a=7, b=8, 7+8=15
```

## Experimental Macros

* [proc-macro-derive-describe](./proc-macro-derive-describe/README.md) located at [/proc-macro-derive-describe](/proc-macro-derive-describe/)
* [proc-macro-verbatim](./proc-macro-verbatim/README.md) located at [/proc-macro-verbatim](/proc-macro-verbatim/)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
