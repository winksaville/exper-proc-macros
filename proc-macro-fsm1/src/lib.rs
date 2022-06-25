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
                fields_named.named.iter().cloned().collect()
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
        while let Ok(a_fn) = input.parse::<syn::ItemFn>() {
            //println!("Fsm1::parse: state {:#?}", a_fn);

            // Look at the attributes and check for "fsm1_state"
            for a in a_fn.attrs.iter() {
                if let Some(ident) = a.path.get_ident() {
                    if ident == "fsm1_state" {
                        // Save the index of this function in state_fn_idxs
                        state_fn_idxs.push(fns.len());
                        //println!("Fsm1::parse: {} has a fsm1_state attribute, idx={}", a_fn.sig.ident.to_string(), state_fn_idxs.last().unwrap());
                    }
                }
            }

            // Add a_fn to fns
            fns.push(a_fn.clone());
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

#[derive(Debug)]
struct TransitionToId {
    id: syn::Path
}

impl Parse for TransitionToId {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("TransitionToId::parse:+");
        //println!("TransitionToId::parse: input={:#?}", input);

        let tt_id = input.parse::<syn::Path>()?;

        //println!("TransitionToId::parse:- tt_id={:#?}", tt_id);
        Ok(TransitionToId {
            id: tt_id,
        })
    }
}

#[proc_macro]
pub fn transition_to(input: TokenStream) -> TokenStream {
    let tt_id = parse_macro_input!(input as TransitionToId);

    let id = tt_id.id;
    //println!("transition_to: id={:#?}", id);

    quote!(
        // Allows:
        //   transition_to!(Self::done)
        // or
        //   transition_to!(MyFsm::done)
        //self.transition_to(#id);

        // Allows:
        //   transition_to!(done)
        self.transition_to(Self::#id);
    ).into()
}

#[proc_macro]
pub fn fsm1(input: TokenStream) -> TokenStream {
    //println!("fsm1:+ input={:#?}", &input);
    let in_ts = input;

    let fsm = parse_macro_input!(in_ts as Fsm1);
    //println!("fsm1: fsm={:#?}", fsm);

    let fsm_name = fsm.fsm_name;
    //println!("fsm1: fsm_name={:#?}", fsm_name);

    let fsm_fields = fsm.fsm_fields; 
    //println!("fsm1: fsm_fields={:#?}", fsm_fields);

    let fsm_fns = fsm.fsm_fns; 
    //println!("fsm1: fsm_states={:#?}", fsm_states);

    let output = quote!(
        //#[derive(Debug)]
        #[derive(Default)]
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

            pub fn transition_to(&mut self, next_state: StateFn) {
                // DOES NOT WORK if multiple invocations of this in one StateFn!!
                // How can we reliably detect this at compile time?
                if self.sm.current_state_changed {
                    panic!("Only one transition_to maybe executed in a StateFn")
                }
                self.sm.previous_state = self.sm.current_state;
                self.sm.current_state = next_state;
                self.sm.current_state_changed = true;
            }

            pub fn dispatch(&mut self) {
                if self.sm.current_state_changed {
                    // Handle changing state such as executing "enter code" for
                    // the current_state state
                    self.sm.current_state_changed = false;
                }

                (self.sm.current_state)(self);

                if self.sm.current_state_changed {
                    // Handle changing state such as executing exit "code" for
                    // the previous state, do not change current_state_changed
                    // so we execut "enter_code" on next dispatch.
                }
            }
        }

        //This 'static allows derive(Debug) to work but then dispatch
        // doesn't, so remove Debug for now :(

        //type StateFn = fn(&'static mut #fsm_name, /* &Protocol1 */) -> bool;
        type StateFn = fn(&mut #fsm_name, /* &Protocol1 */) -> bool;

        //#[derive(Debug)]
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
