#![feature(core_intrinsics)]
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result};

// Current thoughts on format of an FSM "DSL"
//  fsm1 calculator {
//      protocol Arithmetic {
//          add {
//              value: i32,
//          }
//          sub {
//              value: i32,
//          }
//      }
//      data {
//          i: i32;
//      }
//      state initial(data, msg: Arithmetic) {
//          println!("initial:+");
//      }
//      state work(data, msg: Arithmetic) {
//          println!("work:+");
//
//      }
//      state done(data, msg: Arithmetic) {
//          println!("initial:+");
//      }
//  }
//
// Currently implemented (parses isn't yet an FSM)
//  fsm1!(
//      MyFsm {
//              a_i32: i32,
//              a_u32: u32,
//      }
//      #[fsm1_state]
//      fn initial(&self) {
//          println!("MyFSM: self={:#?}", self);
//      }
//  );
//

// Figure out how to expose the name of the fn this
// attribute is on so it can be added to struct Fsm1 at
// compile time.
#[proc_macro_attribute]
pub fn fsm1_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("proc_macro_attribute fsm1_state: item={:#?}", item);
    item
}

#[derive(Debug)]
struct Fsm1 {
    fsm_name: syn::Ident,
    #[allow(unused)]
    fsm_fields: Vec<syn::Field>,
    #[allow(unused)]
    fsm_fns: Vec<syn::ItemFn>,
    #[allow(unused)]
    fsm_state_fn_idxs: Vec<usize>,
}

impl Parse for Fsm1 {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("Fsm1::parse:+");
        //println!("Fsm1::parse: input={:#?}", input);

        let item_struct = input.parse::<syn::ItemStruct>()?;
        //println!("Fsm1::parse: item_struct={:#?}", item_struct);

        // Parse all of the FSM1 data fields
        let fields: Vec<syn::Field> = match item_struct.fields {
            syn::Fields::Named(fields_named) => {
                fields_named.named.iter().map(|f| f.clone()).collect()
            }
            _ => {
                let err = syn::Error::new_spanned(item_struct, "Fsm1::parse: expecting fsm struct");
                return Err(err);
            }
        };
        //println!("Fsm1::parse: fields={:#?}", fields);

        // The only thing that should remain are functions
        let mut state_fn_idxs = Vec::<usize>::new();
        let mut fns = Vec::<syn::ItemFn>::new();
        loop {
            match input.parse::<syn::ItemFn>() {
                Ok(a_fn) => {
                    //println!("Fsm1::parse: state {:#?}", a_fn);

                    // Look at the attributes and check for "fsm1_state"
                    for a in a_fn.attrs.iter() {
                        if let Some(ident) = a.path.get_ident() {
                            if ident == "fsm1_state" {
                                // Save the index of state functions
                                state_fn_idxs.push(fns.len());
                                println!("Fsm1::parse: {} has a fsm1_state attribute, idx={}", a_fn.sig.ident.to_string(), state_fn_idxs[fns.len()-1]);
                            }
                        }
                    }

                    // Add a_fn to fns
                    fns.push(a_fn.clone());
                }
                Err(_e) => {
                    //fn type_of_v<T>(_: &T) -> &str {
                    //    std::intrinsics::type_name::<T>()
                    //}
                    //println!("Fsm1::parse: expecting a function found {:#?}", type_of_v(&_e));
                    break // out of loop
                }
            }
        }

        //println!("Fsm1::parse:-");
        Ok(Fsm1{
            fsm_name: item_struct.ident.clone(),
            fsm_fields: fields,
            fsm_fns: fns,
            fsm_state_fn_idxs: state_fn_idxs,
        })
    }
}

mod kw {
    syn::custom_keyword!(state);
}

#[proc_macro]
pub fn fsm1(input: TokenStream) -> TokenStream {
    //println!("fsm1:+ input={:#?}", input);
    let in_ts = input.clone();

    let fsm = parse_macro_input!(in_ts as Fsm1);
    //println!("fsm1: fsm={:#?}", fsm);

    let fsm_name = fsm.fsm_name;
    //println!("fsm1: fsm_name={:#?}", fsm_name);

    let fsm_fields = fsm.fsm_fields; 
    //println!("fsm1: fsm_fields={:#?}", fsm_fields);

    let fsm_fns = fsm.fsm_fns; 
    //println!("fsm1: fsm_states={:#?}", fsm_states);

    let output = quote!(
        #[derive(Debug, Default)]
        struct #fsm_name {
            sm: SM, // Why is this not seend by vscode code completion?

            #(
                #[allow(unused)]
                #fsm_fields
            ),*
        }

        impl #fsm_name {
            pub fn new() -> Self {
                Default::default()
            }

            #(
                #[allow(unused)]
                #fsm_fns
            )*
        }

        type StateFn = fn(&'static mut #fsm_name, /* &Protocol1 */) -> bool;

        #[derive(Debug)]
        struct SM {
            current_state: StateFn,
            previous_state: StateFn,
            current_state_changed: bool,
        }

        impl Default for SM {
            fn default() -> Self {
                Self::new()
            }
        }

        impl SM {
            pub fn new() -> Self {
                let initial_state = #fsm_name::initial;
                Self {
                    current_state: initial_state,
                    previous_state: initial_state,
                    current_state_changed: true,
                }
            }
        }
    );
    //println!("fsm1:- output={:#?}", output);

    output.into()
}
