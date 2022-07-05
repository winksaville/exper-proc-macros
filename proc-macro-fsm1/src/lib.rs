#![feature(core_intrinsics)]
use std::collections::HashMap;
use std::sync::Mutex;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, ItemFn, Macro, File};
use syn::visit::{self, Visit};

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

            pub fn dispatch(&mut self) {
                if self.sm.current_state_changed {
                    // Handle changing state such as executing "enter code" for
                    // the current_state state
                    self.sm.current_state_changed = false;
                }

                //let body = self.sm.state_fns[self.sm.current_state_fns_idx].body;
                //(body)(self);
                match (self.sm.current_state_fn)(self) {
                    StateResult::NotHandled => {
                        // TBD, execute entry fn of current_state
                    }
                    StateResult::Handled => {
                        // Nothing to do
                    }
                    StateResult::TransitionTo(next_state) => {
                        self.sm.previous_state_fn = self.sm.current_state_fn;
                        self.sm.current_state_fn = next_state;
                        self.sm.current_state_changed = true;
                    }
                }

                if self.sm.current_state_changed {
                    // Handle changing state such as executing exit "code" for
                    // the previous state, do not change current_state_changed
                    // so we execute "enter_code" on next dispatch.
                }
            }
        }

        type StateFn = fn(&mut #fsm_name, /* &Protocol1 */) -> StateResult;

        enum StateResult {
            NotHandled,
            Handled,
            TransitionTo(StateFn),
        }

        struct StateFns {
            parent: Option<StateFn>,
            entry: Option<StateFn>,
            body: StateFn,
            exit: Option<StateFn>,
        }

        //#[derive(Debug)]
        struct SM {
            current_state_fn: StateFn,
            previous_state_fn: StateFn,
            current_state_changed: bool,
        }

        impl Default for SM {
            fn default() -> Self {
                Self::new()
            }
        }

        impl SM {
            fn new() -> Self {
                let initial_state = #fsm_name::initial;
                Self {
                    current_state_fn: initial_state,
                    previous_state_fn: initial_state,
                    current_state_changed: true,
                }
            }
        }
    );
    //println!("fsm1:- output={:#?}", output);

    output.into()
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
