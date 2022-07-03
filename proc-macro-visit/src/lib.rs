use proc_macro::{self, TokenStream};
use syn::visit::{self, Visit};
use syn::{self, ItemFn, Macro, File};

#[proc_macro]
pub fn valid_tokenstream(input: TokenStream) -> TokenStream {
    println!("valid_tokenstream: input={:#?}", input);

    // Return nothing
    TokenStream::new()
}

#[proc_macro]
pub fn visit1(input: TokenStream) -> TokenStream {
    println!("visit1: input={:#?}", input);
    let syntax_tree: File = syn::parse2(input.clone().into()).unwrap();
    println!("visit1: syntax_tree={:#?}", syntax_tree);
    Visitor::new().visit_file(&syntax_tree);
    println!("visit1: after visit_file");

    input
}


struct Visitor {
    data: u32,
}

impl Visitor {
    fn new() -> Self {
        Self { data: 0 }
    }
}

impl<'ast> Visit<'ast> for Visitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.data += 1;
        println!("visit_item_fn: data={} node.sig.ident={:?}", self.data, node.sig.ident);

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        self.data += 1;
        println!("visit_macro: data={} node={:?}", self.data, node);

        // Delegate to the default impl to visit any nested macros.
        visit::visit_macro(self, node);
    }
}

