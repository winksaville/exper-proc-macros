# Nesting of proc macros

See if you can nest proc-macros. The Answer yes, but I had to
`use proc_macro_nesting::{outer, inner}` in `main.rs` even
though I didn't directly use it:
```
use proc_macro_nesting::{outer, inner};

fn main() {
    outer!();
}
```

And you must be careful that the macros are hygienic.
    https://www.google.com/search?q=hygienic+proc+macros+rust
    https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.def_site

This means that identifiers are properly scoped and so you don't endup with multiply
defined identifiers. The way I got it to work was using proc_macro2 Ident and Span
used with:
```
#[proc_macro]
pub fn inner_replacing_outer_scope_declaractions_using_parens(_input: TokenStream) -> TokenStream {
    // Here we need to use Ident::new and def_site to make #add_ident hygienic
    let add_ident = Ident::new("add", Span::def_site());
    quote!(
        let a = 3;
        let b = 4;

        fn #add_ident(l: i32, r: i32) -> i32 {
            l + r
        }
        println!("inner_replacing_outer_scope_declaractions_using_parens: a={}, b={}, {}+{}={}", a, b, a, b, #add_ident(a, b));
    ).into()
}
```

Since I'm using [`SPAN::def_site`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.def_site)
this must be compiled with `RUSTFLAGS='--cfg procmacro2_semver_exempt'`:
```
wink@3900x 22-06-03T21:18:41.107Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ RUSTFLAGS='--cfg procmacro2_semver_exempt' cargo run -- --verbose
   Compiling proc-macro2 v1.0.39
   Compiling unicode-ident v1.0.0
   Compiling syn v1.0.95
   Compiling quote v1.0.18
...
outer: a=10, b=20, 10*20=200
inner_using_outer_declarations: a=10, b=20, 10+20=30
inner_creating_own_scope_using_no_outer_declarations: a=1, b=2, 1+2=3
inner_using_outer_declarations: a=10, b=20, 10+20=30
inner_replacing_outer_scope_declaractions_using_parens: a=3, b=4, 3+4=7
inner_using_outer_declarations: a=3, b=4, 3+4=7
inner_replacing_outer_scope_declaractions_using_braces: a=5, b=6, 5+6=11
inner_using_outer_declarations: a=5, b=6, 5+6=11
inner_replacing_outer_scope_declaractions_using_square_brackets: a=7, b=8, 7+8=15
inner_using_outer_declarations: a=7, b=8, 7+8=15outer: inner=0
```

> Note: I tried to use `.cargo/config.toml` with `build.rustflags = ["--cfg procmacro2_semver_exempt"]`
> but that fails with `error: Unrecognized option: 'cfg procmacro2_semver_exempt'` 
```
wink@3900x 22-06-03T21:30:20.810Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ cat .cargo/config.toml 
[build]
rustflags = ["--cfg procmacro2_semver_exempt", ]
wink@3900x 22-06-03T21:30:23.736Z:~/prgs/rust/myrepos/expr-proc-macros (main)
$ cargo run
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names '--cfg procmacro2_semver_exempt' --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: Unrecognized option: 'cfg procmacro2_semver_exempt'

wink@3900x 22-06-03T21:30:27.447Z:~/prgs/rust/myrepos/expr-proc-macros (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

