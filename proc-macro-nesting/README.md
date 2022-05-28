# nesting of proc macros

See if you can nest proc-macros

Answer yes, but I had to `use proc_macro_nesting::{outer, inner}` in
`main.rs` even though I didn't directly use it:
```
use proc_macro_nesting::{outer, inner};

fn main() {
    outer!();
}
```

Running this would yield:
```
outer: inner=0
```
## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

