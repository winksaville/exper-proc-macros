use proc_macro::{self, TokenStream};
use proc_macro2::{Ident, Span};
use quote::quote;

// A marco that will be invoked by `outer`
#[proc_macro]
pub fn inner_using_outer_declarations(_input: TokenStream) -> TokenStream {
    quote!(
        println!("inner_using_outer_declarations: a={}, b={}, {}+{}={}", a, b, a, b, a + b);
    )
    .into()
}

#[proc_macro]
pub fn inner_creating_own_scope_using_no_outer_declarations(_input: TokenStream) -> TokenStream {
    quote!({
        let a = 1;
        let b = 2;

        // This is hygienic because `fn add` is in it's own scope
        fn add(l: i32, r: i32) -> i32 {
            l + r
        }
        println!(
            "inner_creating_own_scope_using_no_outer_declarations: a={}, b={}, {}+{}={}",
            a,
            b,
            a,
            b,
            add(a, b)
        );
    })
    .into()
}

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

#[proc_macro]
pub fn inner_replacing_outer_scope_declaractions_using_braces(_input: TokenStream) -> TokenStream {
    let add_ident = Ident::new("add", Span::def_site());
    quote!{
        let a = 5;
        let b = 6;

        fn #add_ident(l: i32, r: i32) -> i32 {
            l + r
        }
        println!("inner_replacing_outer_scope_declaractions_using_braces: a={}, b={}, {}+{}={}", a, b, a, b, #add_ident(a, b));
    }.into()
}

#[proc_macro]
pub fn inner_replacing_outer_scope_declaractions_using_square_brackets(
    _input: TokenStream,
) -> TokenStream {
    let add_ident = Ident::new("add", Span::def_site());
    quote![
        let a = 7;
        let b = 8;

        fn #add_ident(l: i32, r: i32) -> i32 {
            l + r
        }
        println!("inner_replacing_outer_scope_declaractions_using_square_brackets: a={}, b={}, {}+{}={}", a, b, a, b, #add_ident(a, b));
    ].into()
}

// A macro that invokes `inner` showing we can nest macros but we
// need to be careful about hygiene, https://veykril.github.io/tlborm/proc-macros/hygiene.html.
// Solution, use proc_macro2::Ident to generate the name in the
// above macros.
#[proc_macro]
pub fn outer(_input: TokenStream) -> TokenStream {
    let q = quote! {
        let a = 10;
        let b = 20;
        println!("outer: a={}, b={}, {}*{}={}", a, b, a, b, a * b);
        inner_using_outer_declarations!();
        inner_creating_own_scope_using_no_outer_declarations!();
        inner_using_outer_declarations!();
        inner_replacing_outer_scope_declaractions_using_parens!();
        inner_using_outer_declarations!();
        inner_replacing_outer_scope_declaractions_using_braces!();
        inner_using_outer_declarations!();
        inner_replacing_outer_scope_declaractions_using_square_brackets!();
        inner_using_outer_declarations!();
    };

    q.into()
}
