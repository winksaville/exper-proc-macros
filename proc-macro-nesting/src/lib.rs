use proc_macro::{self, TokenStream};
use quote::quote;

// A marco that will be invoked by `outer`
#[proc_macro]
pub fn inner(_input: TokenStream) -> TokenStream {
    let stuff = 0;
    quote!(#stuff).into()
}

// A macro that invokes `inner` showing we can nest macros,
// but I need need to `use proc_macro_nesting::inner`.
#[proc_macro]
pub fn outer(_input: TokenStream) -> TokenStream {
    let q = quote! {
        println!("outer: inner={}", inner!());
    };

    q.into()
}
