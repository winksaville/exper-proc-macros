#![feature(core_intrinsics)]
use std::collections::HashMap;

//use std::str::FromStr;
use proc_macro2::TokenStream as TokenStream2;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result, ItemFn, Macro, File};
use syn::visit::{self, Visit};

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

    let _name = r.for_state_fn.to_string();
    //println!("fms1_state_entry_for: name={}", name);

    item
}

#[proc_macro_attribute]
pub fn fsm1_state(_attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("proc_macro_attribute fsm1_state: item={:#?}", item);
    item
}

#[derive(Debug)]
struct Fsm1 {
    fsm_ident: syn::Ident,
    fsm_fields: Vec<syn::Field>,
    fsm_fns: Vec<syn::ItemFn>,
    #[allow(unused)]
    fsm_fn_map: HashMap<String, usize>,
    //fsm_state_fn_idxs: Vec<usize>,
    fsm_state_fn_idents: Vec<StateFnIdents>
}

#[derive(Debug)]
struct StateFnIdents {
    parent_fn_ident: Option<syn::Ident>,
    entry_fn_ident: Option<syn::Ident>,
    process_fn_ident: syn::Ident,
    exit_fn_ident: Option<syn::Ident>,
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


        let mut state_fn_idents = Vec::<StateFnIdents>::new();
        for process_fn_idx in state_fn_idxs.clone() {
            let item_fn = &fns[process_fn_idx];
            let process_fn_ident = item_fn.sig.ident.clone();
            let new_ident = |ident: syn::Ident, suffix: &str| {
                syn::Ident::new((ident.to_string() + suffix.to_owned().as_str()).as_str(), ident.span())
            };
            let entry_fn_ident = new_ident(process_fn_ident.clone(), "_exit");
            let exit_fn_ident = new_ident(process_fn_ident.clone(), "_exit");

            let entry_fn_ident_opt = if fn_map.get(entry_fn_ident.to_string().as_str()).is_some() {
                Some(entry_fn_ident)
            } else {
                None
            };

            let exit_fn_ident_opt = if fn_map.get(exit_fn_ident.to_string().as_str()).is_some() {
                Some(exit_fn_ident)
            } else {
                None
            };

            state_fn_idents.push(StateFnIdents {
                parent_fn_ident: None,
                entry_fn_ident: entry_fn_ident_opt,
                process_fn_ident,
                exit_fn_ident: exit_fn_ident_opt,
            });
        }

        //println!("Fsm1::parse:-");
        Ok(Fsm1 {
            fsm_ident: item_struct.ident.clone(),
            fsm_fields: fields,
            fsm_fns: fns,
            fsm_fn_map: fn_map,
            //fsm_state_fn_idxs: state_fn_idxs,
            fsm_state_fn_idents: state_fn_idents,
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

    let fsm_ident = fsm.fsm_ident;
    //println!("fsm1: fsm_ident={:#?}", fsm_ident);

    let fsm_fields = fsm.fsm_fields; 
    //println!("fsm1: fsm_fields={:#?}", fsm_fields);

    let fsm_fns = fsm.fsm_fns; 
    //println!("fsm1: fsm_fns={:#?}", fsm_fns);

    let _fsm_fn_map = fsm.fsm_fn_map;
    //println!("fsm1: fsm_fn_map={:?}", fsm_fn_map);

    let mut fsm_state_fns = Vec::<syn::ExprStruct>::new();
    for sfn in fsm.fsm_state_fn_idents {
        //println!("fsm1: sf={:#?}", sfn);

        let process_fn_ident = sfn.process_fn_ident;
        //println!("fsm1: process_fn_ident={}", process_fn_ident);

        let opt_fn_ident = |ident: Option<syn::Ident>| {
            match ident {
                Some(ident) => quote!(Some(#fsm_ident::#ident)),
                None => quote!(None),
            }
        };
        let parent_fn = opt_fn_ident(sfn.parent_fn_ident);
        //println!("fsm1: parent_fn={}", parent_fn);
        let entry_fn = opt_fn_ident(sfn.entry_fn_ident);
        //println!("fsm1: entry_fn={}", entry_fn);
        let exit_fn = opt_fn_ident(sfn.exit_fn_ident);
        //println!("fsm1: exit_fn={}", exit_fn);

        let ts: TokenStream2 = quote!(
            StateFns {
                parent: #parent_fn,
                entry: #entry_fn,
                process: #fsm_ident::#process_fn_ident,
                exit: #exit_fn,
            }
        );
        let sf_es = syn::parse2::<syn::ExprStruct>(ts);
        if let Ok(es) = sf_es {
            fsm_state_fns.push(es);
        }
    }
    //println!("fsm1: fsm_state_fns:\n{:#?}", fsm_state_fns);

    let fsm_state_fns_len = fsm_state_fns.len();

    let output = quote!(
        //#[derive(Debug)]
        #[derive(Default)]
        struct #fsm_ident {
            sm: SM, // Why is this not seend by vscode code completion?

            #(
                #[allow(unused)]
                #fsm_fields
            ),*
        }

        impl #fsm_ident {
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

        type StateFn = fn(&mut #fsm_ident, /* &Protocol1 */) -> StateResult;

        enum StateResult {
            NotHandled,
            Handled,
            TransitionTo(StateFn),
        }

        struct StateFns {
            parent: Option<StateFn>,
            entry: Option<StateFn>,
            process: StateFn,
            exit: Option<StateFn>,
        }

        //#[derive(Debug)]
        struct SM {
            state_fns: [StateFns; #fsm_state_fns_len],
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
                let initial_state = #fsm_ident::initial;
                Self {
                    //state_fns: vec![],
                    state_fns: [
                        #(
                            #fsm_state_fns
                        ),*
                    ],
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
        //println!("Function node.sig.ident={:?}", node.sig.ident);

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        //println!("Macro: node={:?}", node);

        // Delegate to the default impl to visit any nested macros.
        visit::visit_macro(self, node);
    }
}
