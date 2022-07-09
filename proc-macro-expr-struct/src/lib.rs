use proc_macro::{self, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Result, parse::{Parse, ParseStream}};

#[derive(Debug)]
struct Init {
    struct_name: syn::Ident,
    struct_fields: Vec<syn::Field>,
    struct_fn: syn::ItemFn,
}

impl Parse for Init {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("Fsm1::parse:+");
        //println!("Init::parse: input={:#?}", input);

        let item_struct = input.parse::<syn::ItemStruct>()?;
        //println!("Init::parse: item_struct={:#?}", item_struct);

        // Get the data fields
        let fields: Vec<syn::Field> = match item_struct.fields {
            syn::Fields::Named(fields_named) => {
                fields_named.named.iter().cloned().collect()
            }
            _ => {
                let err = syn::Error::new_spanned(item_struct, "Init::parse: expecting fsm struct");
                return Err(err);
            }
        };
        //println!("Init::parse: fields={:#?}", fields);

        let the_fn = input.parse::<syn::ItemFn>()?;
        //println!("Init::parse: the_fn={:#?}", the_fn);

        println!("Fsm1::parse:-");
        Ok(Init {
            struct_name: item_struct.ident.clone(),
            struct_fields: fields,
            struct_fn: the_fn,
        })
    }
}

#[proc_macro]
pub fn init(input: TokenStream) -> TokenStream {
    println!("init:+ input={:#?}", &input);

    let init = parse_macro_input!(input as Init);
    println!("init: init={:#?}", init);

    let struct_name = init.struct_name;
    println!("init: struct_name={:#?}", struct_name);

    let struct_fields = init.struct_fields;
    println!("init: struct_fields={:#?}", struct_fields);

    let struct_fn = init.struct_fn;
    println!("init: struct_fn={:#?}", struct_fn);


    let fn_name = struct_fn.sig.ident.to_string();
    println!("init: fn_name={}", fn_name);

    let ts: TokenStream2 = quote!(initial);
    println!("fsm1: 2-1 quote!(initial)             ts={:?}", ts); // init: 2-1 quote!(initial)             ts=TokenStream [Ident { ident: "initial", span: #5 bytes(64..988) }]
    let ts: TokenStream2 = quote!(#fn_name);
    println!("fsm1: 2-0 quote!(#fn_name)            ts={:?}", ts); // init: 2-0 quote!(#process_fn_name)    ts=TokenStream [Literal { kind: Str, symbol: "initial", suffix: None, span: #5 bytes(64..988) }] 
    let ts: TokenStream2 = quote!(
        StateFns {
            do_process: Self::#fn_name,
        }
    );
    println!("fsm1: 2.1 Self::#process_fn_name      ts={:?}", ts);

    let ts: TokenStream2 = quote!(
        StructFns {
            do_process: Self::#fn_name, // Error
            //do_process: Self::initial, // Ok
            //do_process: #struct_name::initial, // Ok
            //do_process: #struct_fn_name, // Ok
            //do_process: 2, // Ok
        }
    );
    println!("init: 2.2 #fsm_name::initial          ts={:?}", ts);

    let es = syn::parse2::<syn::ExprStruct>(ts);
     println!("init: 2 sf_es={:?}", &es);

    let output = quote!(
        type ProcessFn = fn(&mut X);

        #[derive(Default)]
        struct #struct_name {
            do_process: ProcessFn,
            #(
               #struct_fields,
            )*,
        }

        impl #struct_name {
            fn new() -> Self {
                Self {
                    do_process: Self::#fn_name, // Fails
                    do_process: Self::process, // Ok
                    ..Default::default(),
                }
            }

            fn dispatch(&mut self) {
                (self.do_process)(self)
            }

            #[allow(unused)]
            #struct_fn
        }
    );
    //println!("output={:#?}", output);

    output.into()
}

