use proc_macro::{self, TokenStream};

// Verbatium simply returns the input tokens as output
#[proc_macro]
pub fn verbatium(input: TokenStream) -> TokenStream {
    input
}
