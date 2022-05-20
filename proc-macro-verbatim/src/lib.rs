use proc_macro::{self, TokenStream};

// Verbatim simply returns the input tokens as output
#[proc_macro]
pub fn verbatim(input: TokenStream) -> TokenStream {
    input
}
