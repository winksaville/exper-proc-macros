#![feature(core_intrinsics)]
use std::collections::HashMap;
use std::sync::Mutex;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, ItemFn, Macro, File};
use syn::visit::{self, Visit};

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

// From https://stackoverflow.com/questions/34832583/global-mutable-hashmap-in-a-library
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, usize>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

#[derive(Debug)]
struct EnterFn {
    #[allow(unused)]
    for_state_fn: syn::Ident,
}

impl Parse for EnterFn {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("parse for EnterFn: input={:#?}", input);
        let ident = input.parse::<syn::Ident>()?;
        Ok(EnterFn {
            for_state_fn: ident
        })
    }
}

#[proc_macro_attribute]
pub fn fsm1_state_entry_for(attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("proc_macro_attribute fsm1_state_entry_for:\nattr={:#?}\nitem={:#?}\n", attr, item);

    let r = parse_macro_input!(attr as EnterFn);
    //println!("proc_macro_attribute fsm1_state_entry_for: r={:#?}", r);

    let name = r.for_state_fn.to_string();
    let mut map = HASHMAP.lock().unwrap();
    map.insert(name, 123usize);
    //println!("proc_macro_attribute fsm1_state_entry_for: map={:#?}", map);

    item
}

#[proc_macro_attribute]
pub fn fsm1_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("proc_macro_attribute fsm1_state: item={:#?}", item);
    item
}

#[derive(Debug)]
struct Fsm1 {
    fsm_name: syn::Ident,
    fsm_fields: Vec<syn::Field>,
    fsm_fns: Vec<syn::ItemFn>,
    #[allow(unused)]
    fsm_fn_map: HashMap<String, usize>,
    //fsm_state_fn_idxs: Vec<usize>,
    fsm_state_fns_names: Vec<StateFnsNames>
}

#[derive(Debug)]
struct StateFnsNames {
    parent_fn_name: Option<String>,
    entry_fn_name: Option<String>,
    body_fn_name: String,
    exit_fn_name: Option<String>,
}

impl Parse for Fsm1 {
    fn parse(input: ParseStream) -> Result<Self> {
        println!("Fsm1::parse:+");
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
        let mut fn_map = HashMap::<String, usize>::new();
        while let Ok(a_fn) = input.parse::<syn::ItemFn>() {
            //println!("Fsm1::parse: tol ItemFn a_fn={:#?}", a_fn);

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

            // Add a_fn to fn_map and fns
            fn_map.insert(a_fn.sig.ident.to_string(), fns.len());
            fns.push(a_fn.clone());
        }


        let mut state_fns_names = Vec::<StateFnsNames>::new();
        for body_fn_idx in state_fn_idxs.clone() {
            let item_fn = &fns[body_fn_idx];
            let body_fn_name = item_fn.sig.ident.to_string();
            let entry_fn_name = body_fn_name.clone() + "_enter";
            let exit_fn_name = body_fn_name.clone() + "_exit";

            let entry_fn_name_opt = if fn_map.get(entry_fn_name.as_str()).is_some() {
                Some(entry_fn_name)
            } else {
                None
            };

            let exit_fn_name_opt = if fn_map.get(exit_fn_name.as_str()).is_some() {
                Some(exit_fn_name)
            } else {
                None
            };

            state_fns_names.push(StateFnsNames {
                parent_fn_name: None,
                entry_fn_name: entry_fn_name_opt,
                body_fn_name,
                exit_fn_name: exit_fn_name_opt,
            });
        }

        println!("Fsm1::parse:-");
        Ok(Fsm1 {
            fsm_name: item_struct.ident.clone(),
            fsm_fields: fields,
            fsm_fns: fns,
            fsm_fn_map: fn_map,
            //fsm_state_fn_idxs: state_fn_idxs,
            fsm_state_fns_names: state_fns_names,
        })
    }
}

#[proc_macro]
pub fn fsm1(input: TokenStream) -> TokenStream {
    let syntax_tree: File = syn::parse2(input.clone().into()).unwrap();
    Visitor.visit_file(&syntax_tree);

    //println!("fsm1:+ input={:#?}", &input);
    let in_ts = input;

    let fsm = parse_macro_input!(in_ts as Fsm1);
    //println!("fsm1: fsm={:#?}", fsm);

    let fsm_name = fsm.fsm_name;
    //println!("fsm1: fsm_name={:#?}", fsm_name);

    let fsm_fields = fsm.fsm_fields; 
    //println!("fsm1: fsm_fields={:#?}", fsm_fields);

    let fsm_fns = fsm.fsm_fns; 
    //println!("fsm1: fsm_fns={:#?}", fsm_fns);

    let mut fsm_state_fns = Vec::<proc_macro::TokenStream>::new();
    //let mut fsm_state_fns = Vec::<syn::ItemStruct>::new();
    for sfn in fsm.fsm_state_fns_names {
        //println!("fsm1: sf={:#?}", sfn);

        let body_fn_name = sfn.body_fn_name;
        //println!("fsm1: body_fn_name={}", body_fn_name);

        let opt_fn_name = |name: Option<String>| {
            match name {
                Some(name) => quote!(Some(#fsm_name::#name)),
                None => quote!(None),
            }
        };
        let parent_fn = opt_fn_name(sfn.parent_fn_name);
        //println!("fsm1: parent_fn={}", parent_fn);
        let entry_fn = opt_fn_name(sfn.entry_fn_name);
        //println!("fsm1: entry_fn={}", entry_fn);
        let exit_fn = opt_fn_name(sfn.exit_fn_name);
        //println!("fsm1: exit_fn={}", exit_fn);

        let sf_ts: proc_macro::TokenStream = quote!(
            StateFns {
                parent: #parent_fn,
                entry: #entry_fn,
                body: #fsm_name::#body_fn_name,
                exit: #exit_fn,
            }
        ).into();
        //println!("fsm1: sf_ts={:#?}", sf_ts);
        fsm_state_fns.push(sf_ts);

        //let sf_item_struct = parse_macro_input!(sf_ts as syn::ItemStruct);
        //fsm_state_fns.push(sf_item_struct);
    }
    //println!("fsm1: fsm_state_fns:\n{:#?}", fsm_state_fns);

    let map = HASHMAP.lock().unwrap();
    println!("fsm1: lazy_static map={:#?}", map);

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

            pub fn transition_to(&mut self, _next_state: StateFn) {
                // DOES NOT WORK if multiple invocations of this in one StateFn!!
                // How can we reliably detect this at compile time?
                if self.sm.current_state_changed {
                    panic!("Only one transition_to maybe executed in a StateFn")
                }
                let next_state_fns_idx = 0; // TODO use _next_state
                self.sm.previous_state_fns_idx = self.sm.current_state_fns_idx;
                self.sm.current_state_fns_idx = next_state_fns_idx;
                self.sm.current_state_changed = true;
            }

            pub fn dispatch(&mut self) {
                if self.sm.current_state_changed {
                    // Handle changing state such as executing "enter code" for
                    // the current_state state
                    self.sm.current_state_changed = false;
                }

                let body = self.sm.state_fns[self.sm.current_state_fns_idx].body;
                (body)(self);

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

        struct StateFns {
            parent: Option<StateFn>,
            entry: Option<StateFn>,
            body: StateFn,
            exit: Option<StateFn>,
        }

        //#[derive(Debug)]
        struct SM {
            state_fns: Vec<StateFns>,
            current_state_fns_idx: usize,
            previous_state_fns_idx: usize,
            current_state_changed: bool,
        }

        impl Default for SM {
            fn default() -> Self {
                Self::new()
            }
        }

        impl SM {
            pub fn new() -> Self {
                //let initial_state = #fsm_name::initial;
                Self {
                    state_fns: vec![
                        //#(
                        //    #fsm_state_fns
                        //),*
                        StateFns {
                            parent: None,
                            entry: None,
                            body: #fsm_name::initial,
                            exit: None,
                        },
                        StateFns {
                            parent: None,
                            entry: None,
                            body: #fsm_name::do_work,
                            exit: None,
                        },
                        StateFns {
                            parent: None,
                            entry: None,
                            body: #fsm_name::done,
                            exit: None,
                        },
                    ],
                    current_state_fns_idx: 0, //initial_state,
                    previous_state_fns_idx: 0, //initial_state,
                    current_state_changed: true,
                }
            }
        }
    );
    //println!("fsm1:- output={:#?}", output);

    output.into()
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


    //let map = HASHMAP.lock().unwrap();
    let id = tt_id.id;
    //println!("transition_to: id={:#?}", id);
    //println!("transition_to: id={:#?} lazy_static map={:#?}", id, map);

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


struct Visitor;

impl<'ast> Visit<'ast> for Visitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function node.sig.ident={:?}", node.sig.ident);

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        println!("Macro: node={:?}", node);

        // Delegate to the default impl to visit any nested macros.
        visit::visit_macro(self, node);
    }
}
