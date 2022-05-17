use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, ExprBinary};
use quote::quote;

// Show dbg! doesn't work if it's enabled
#[proc_macro]
pub fn expr_binary_dbg_not_working(input: TokenStream) -> TokenStream {
    let _expr: ExprBinary = parse_macro_input!(input as ExprBinary);
    //dbg!(_expr); // This won't compile as ExprBinary doesn't impl Debug
    // but may have in the past as I see it in syn section
    // [here](https://ferrous-systems.com/blog/testing-proc-macros/#syn)

    // Just return an empty TokenStream as this is just test and does nothing, yet.
    println!("expr_binary: does nothing, but fails if dbg! is used");
    TokenStream::default()
}

// A valid use of ExprBinary [from](https://ferrous-systems.com/blog/testing-proc-macros/#quote).
#[proc_macro]
pub fn expr_binary_swap_and_subtract(input: TokenStream) -> TokenStream {
    let expr: ExprBinary = parse_macro_input!(input as ExprBinary);
    let left = expr.left;
    let right = expr.right;

    // Requires use of .into() as it produces a proc_macro2::TokenStream,
    // [see](https://docs.rs/quote/1.0.18/quote/macro.quote.html).
    quote!(#right - #left).into()
}
