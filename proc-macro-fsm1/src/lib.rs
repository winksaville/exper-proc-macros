#![feature(core_intrinsics)]
use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::visit_mut::{self, VisitMut};
use syn::{parse_macro_input, Macro, Result};

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
    fsm_state_fn_ident_map: HashMap<String, usize>,
    fsm_state_fn_idents: Vec<StateFnIdents>,
}

#[derive(Debug)]
struct StateFnIdents {
    parent_fn_ident: Option<syn::Ident>,
    enter_fn_ident: Option<syn::Ident>,
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
            syn::Fields::Named(fields_named) => fields_named.named.iter().cloned().collect(),
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
                        break; // Never push more than one, although there should only be one
                    }
                }
            }

            // Add a_fn to fn_map and fns
            fn_map.insert(a_fn.sig.ident.to_string(), fns.len());
            fns.push(a_fn.clone());
        }

        let mut state_fn_idents_map = HashMap::<String, usize>::new();
        let mut state_fn_idents = Vec::<StateFnIdents>::new();
        for process_fn_idx in state_fn_idxs.clone() {
            let item_fn = &fns[process_fn_idx];
            let process_fn_ident = item_fn.sig.ident.clone();

            let new_ident = |ident: syn::Ident, suffix: &str| {
                syn::Ident::new(
                    (ident.to_string() + suffix.to_owned().as_str()).as_str(),
                    ident.span(),
                )
            };
            let enter_fn_ident = new_ident(process_fn_ident.clone(), "_enter");
            let exit_fn_ident = new_ident(process_fn_ident.clone(), "_exit");

            let enter_fn_ident_opt = if fn_map.get(enter_fn_ident.to_string().as_str()).is_some() {
                Some(enter_fn_ident)
            } else {
                None
            };

            let exit_fn_ident_opt = if fn_map.get(exit_fn_ident.to_string().as_str()).is_some() {
                Some(exit_fn_ident)
            } else {
                None
            };

            state_fn_idents_map.insert(process_fn_ident.to_string(), state_fn_idents.len());
            state_fn_idents.push(StateFnIdents {
                parent_fn_ident: None,
                enter_fn_ident: enter_fn_ident_opt,
                process_fn_ident,
                exit_fn_ident: exit_fn_ident_opt,
            });
        }

        //println!("Fsm1::parse:-");
        Ok(Fsm1 {
            fsm_ident: item_struct.ident.clone(),
            fsm_fields: fields,
            fsm_fns: fns,
            fsm_state_fn_ident_map: state_fn_idents_map,
            fsm_state_fn_idents: state_fn_idents,
        })
    }
}

#[proc_macro]
pub fn fsm1(input: TokenStream) -> TokenStream {
    //println!("fsm1:+");

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

    let fsm_state_fn_ident_map = fsm.fsm_state_fn_ident_map;
    //println!("fsm1: fsm_state_fn_ident_map={:?}", _fsm_state_fn_ident_map);

    let fsm_state_fn_idents = fsm.fsm_state_fn_idents;
    let mut fsm_state_fns = Vec::<syn::ExprStruct>::new();
    let mut fsm_initial_state_fns_handle: Option<usize> = None;
    for sfn in &fsm_state_fn_idents {
        //println!("fsm1: sf={:#?}", sfn);

        let process_fn_ident = sfn.process_fn_ident.clone();
        //println!("fsm1: process_fn_ident={}", process_fn_ident);
        if process_fn_ident == "initial" {
            assert_eq!(fsm_initial_state_fns_handle, None);
            fsm_initial_state_fns_handle = Some(fsm_state_fns.len());
        }

        let opt_fn_ident = |ident: Option<syn::Ident>| match ident {
            Some(ident) => quote!(Some(#fsm_ident::#ident)),
            None => quote!(None),
        };
        let parent_fn = opt_fn_ident(sfn.parent_fn_ident.clone());
        //println!("fsm1: parent_fn={}", parent_fn);
        let enter_fn = opt_fn_ident(sfn.enter_fn_ident.clone());
        //println!("fsm1: enter_fn={}", enter_fn);
        let exit_fn = opt_fn_ident(sfn.exit_fn_ident.clone());
        //println!("fsm1: exit_fn={}", exit_fn);

        let ts: TokenStream2 = quote!(
            StateFns {
                parent: #parent_fn,
                enter: #enter_fn,
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
    let initial_state_handle = if let Some(handle) = fsm_initial_state_fns_handle {
        handle
    } else {
        // TODO: Better error handling
        panic!("No initial state");
    };
    //println!("fsm1: fsm_state_fns_len: {} initial_state_handle={}", fsm_state_fns_len, initial_state_handle);

    let mut visitor = Visitor {
        fsm_ident: fsm_ident.clone(),
        fsm_state_fn_ident_map,
    };

    let mut converted_fns = Vec::<syn::ItemFn>::new();
    for a_fn in fsm_fns.iter() {
        //println!("fsm1: visiting a_fn={:?}", a_fn.sig.ident);
        let mut mut_a_fn = a_fn.clone();
        visitor.visit_item_fn_mut(&mut mut_a_fn);
        converted_fns.push(mut_a_fn);
    }
    //println!("fsm1: converted_fns={:#?}", converted_fns);

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
                #converted_fns
            )*

            pub fn dispatch(&mut self) {
                if self.sm.current_state_changed {
                    if let Some(enter) = self.sm.state_fns[self.sm.current_state_fns_handle].enter {
                        enter(self);
                    }
                    self.sm.current_state_changed = false;
                }

                match (self.sm.state_fns[self.sm.current_state_fns_handle].process)(self) {
                    StateResult::NotHandled => {
                        // TODO; execute "parent.process".
                    }
                    StateResult::Handled => {
                        // Nothing to do
                    }
                    StateResult::TransitionTo(next_state) => {
                        self.sm.previous_state_fns_handle = self.sm.current_state_fns_handle;
                        self.sm.current_state_fns_handle = next_state;
                        self.sm.current_state_changed = true;
                    }
                }

                if self.sm.current_state_changed {
                    if let Some(exit) = self.sm.state_fns[self.sm.previous_state_fns_handle].exit {
                        exit(self);
                    }
                }
            }
        }

        type StateFn = fn(&mut #fsm_ident, /* &Protocol1 */) -> StateResult;
        type StateFnEnter = fn(&mut #fsm_ident, /* &Protocol1 */);
        type StateFnExit = fn(&mut #fsm_ident, /* &Protocol1 */);
        type StateFnsHandle = usize;

        enum StateResult {
            NotHandled,
            Handled,
            TransitionTo(usize),
        }

        struct StateFns {
            parent: Option<StateFn>,
            enter: Option<StateFnEnter>,
            process: StateFn,
            exit: Option<StateFnExit>,
        }

        //#[derive(Debug)]
        struct SM {
            state_fns: [StateFns; #fsm_state_fns_len],
            current_state_fns_handle: StateFnsHandle,
            previous_state_fns_handle: StateFnsHandle,
            current_state_changed: bool,
        }

        impl Default for SM {
            fn default() -> Self {
                Self::new()
            }
        }

        impl SM {
            fn new() -> Self {
                Self {
                    state_fns: [
                        #(
                            #fsm_state_fns
                        ),*
                    ],
                    current_state_fns_handle: #initial_state_handle,
                    previous_state_fns_handle: #initial_state_handle,
                    current_state_changed: true,
                }
            }
        }
    );
    //println!("fsm1: output={:#?}", output);

    //println!("fsm1:-");
    output.into()
}

#[proc_macro]
pub fn transition_to(item: TokenStream) -> TokenStream {
    let item_ts2: TokenStream2 = item.into();
    //println!("proc_macro transition_to!: item_ts2={:?}", item_ts2);

    quote!(StateResult::TransitionTo(#item_ts2)).into()
}

#[proc_macro]
pub fn handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro handled!: item={:?}", item);
    quote!(StateResult::Handled).into()
}

#[proc_macro]
pub fn not_handled(_item: TokenStream) -> TokenStream {
    //println!("proc_macro not_handled!: item={:?}", item);
    quote!(StateResult::NotHandled).into()
}

struct Visitor {
    fsm_ident: syn::Ident,
    fsm_state_fn_ident_map: HashMap<String, usize>,
}

impl VisitMut for Visitor {
    // Invoke visit_item_fn_mut which will invoke vist_macro_mut for
    // each macro in the funtion. The code here will convert each
    // transtion_to!(state_fn_name) to transition_to!(state_fn_index).
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        if let Some(ident_segment) = node.path.segments.last() {
            // The last segment is the name of the macro
            if ident_segment.ident == "transition_to" {
                // Found our macro, transition_to

                // Get the first token; aka: parameter to the function
                let mut iter = node.tokens.clone().into_iter();
                if let Some(token) = iter.next() {
                    if iter.next().is_some() {
                        // TODO: improve error handling
                        panic!("transition_to! may have only one parameter, the name of the state")
                    }
                    let parameter = token.to_string();
                    if let Some(idx) = self.fsm_state_fn_ident_map.get(&parameter) {
                        //println!("Visitor::visit_macro_mut: Found {} in {} with index {}", parameter, self.fsm_ident, idx);
                        node.tokens = quote!(#idx);
                        return;
                    } else {
                        panic!("No state named {} in {}", parameter, self.fsm_ident);
                    }
                } else {
                    // TODO: improve error handling
                    panic!("transition_to! must have one parameter, the name of the state")
                }
            }
        }

        // Delegate to the default impl to visit any nested macros.
        visit_mut::visit_macro_mut(self, node);

        //println!("Visitor::visit_macro_mut:- fsm_ident={} node={:?}",fsm_ident, node);
    }
}
