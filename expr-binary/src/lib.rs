use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ExprBinary};

// Accept a binary expression
#[proc_macro]
pub fn expr_binary(input: TokenStream) -> TokenStream {
    let _expr: ExprBinary = parse_macro_input!(input as ExprBinary);
    //dbg!(_expr); // This won't compile as ExprBinary doesn't impl Debug
    // but may have in the past as I see it in syn section
    // [here](https://ferrous-systems.com/blog/testing-proc-macros/#syn)

    // Just return an empty TokenStream as this is just test and does nothing, yet.
    println!("expr_binary: does nothing, but fails if dbg! is used");
    TokenStream::default()
}
