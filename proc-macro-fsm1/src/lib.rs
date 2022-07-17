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
    //println!("proc_macro_attribute fsm1_state: attr={:#?}", attr);
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
        struct StateFnInfo {
            idx: usize,
            parent_ident: Option<syn::Ident>,
        }
        let mut state_fn_info = Vec::<StateFnInfo>::new();
        let mut fns = Vec::<syn::ItemFn>::new();
        let mut fn_map = HashMap::<String, usize>::new();
        while let Ok(a_fn) = input.parse::<syn::ItemFn>() {
            //println!("Fsm1::parse: tol ItemFn a_fn={:#?}", a_fn);

            // Look at the attributes and check for "fsm1_state"
            for a in a_fn.attrs.iter() {
                //println!("Fsm1::parse: function attributes: {:#?}", a);

                if let Some(ident) = a.path.get_ident() {
                    if ident == "fsm1_state" {
                        // TODO: There is probably a better way to implement
                        // optional parameters to proc_macro_attribute. The problem
                        // is if there is no arguments "a.parse_args" returns err, but
                        // in Fsm1Args::parse I also handle the notion of no args in
                        // that it also returns None thus this feels over complicated.
                        #[derive(Debug)]
                        struct Fsm1Args {
                            #[allow(unused)]
                            arg_ident: Option<syn::Ident>,
                        }

                        impl Parse for Fsm1Args {
                            fn parse(input: ParseStream) -> Result<Self> {
                                // There should only be one ident
                                let name = if let Ok(id) = input.parse() {
                                    Some(id)
                                } else {
                                    None
                                };
                                Ok(Fsm1Args { arg_ident: name })
                            }
                        }

                        // Save the index of this function in state_fn_hdls
                        state_fn_info.push(StateFnInfo {
                            idx: fns.len(),
                            parent_ident: if let Ok(fa) = a.parse_args::<Fsm1Args>() {
                                fa.arg_ident
                            } else {
                                None
                            },
                        });
                        //println!("Fsm1::parse: {} has a fsm1_state attribute, hdl={}", a_fn.sig.ident.to_string(), state_fn_hdls.last().unwrap());
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
        for state_fn_info in state_fn_info {
            let item_fn = &fns[state_fn_info.idx];
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
                parent_fn_ident: state_fn_info.parent_ident,
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
    let mut fsm_initial_state_fns_hdl: Option<usize> = None;
    for sfn in &fsm_state_fn_idents {
        //println!("fsm1: sf={:#?}", sfn);

        let process_fn_ident = sfn.process_fn_ident.clone();
        //println!("fsm1: process_fn_ident={}", process_fn_ident);
        if process_fn_ident == "initial" {
            assert_eq!(fsm_initial_state_fns_hdl, None);
            fsm_initial_state_fns_hdl = Some(fsm_state_fns.len());
        }

        let opt_fn_ident = |ident: Option<syn::Ident>| match ident {
            Some(ident) => quote!(Some(#fsm_ident::#ident)),
            None => quote!(None),
        };
        let parent_hdl: TokenStream2 = if let Some(parent_ident) = &sfn.parent_fn_ident {
            let parent = parent_ident.to_string();
            if let Some(hdl) = fsm_state_fn_ident_map.get(&parent) {
                quote!(Some(#hdl))
            } else {
                // TODO: Improve error handling
                panic!(
                    "{}::{} is not defined and cannot be parent of {}",
                    parent, fsm_ident, process_fn_ident
                );
            }
        } else {
            quote!(None)
        };
        //println!("fsm1: parent_fn={}", parent_fn);
        let enter_fn = opt_fn_ident(sfn.enter_fn_ident.clone());
        //println!("fsm1: enter_fn={}", enter_fn);
        let exit_fn = opt_fn_ident(sfn.exit_fn_ident.clone());
        //println!("fsm1: exit_fn={}", exit_fn);

        let ts: TokenStream2 = quote!(
            StateFns {
                name: stringify!(#process_fn_ident).to_owned(),
                parent: #parent_hdl,
                enter: #enter_fn,
                process: #fsm_ident::#process_fn_ident,
                exit: #exit_fn,
                active: false,
            }
        );
        let sf_es = syn::parse2::<syn::ExprStruct>(ts);
        if let Ok(es) = sf_es {
            fsm_state_fns.push(es);
        }
    }
    //println!("fsm1: fsm_state_fns:\n{:#?}", fsm_state_fns);

    let fsm_state_fns_len = fsm_state_fns.len();
    let initial_state_hdl = if let Some(hdl) = fsm_initial_state_fns_hdl {
        hdl
    } else {
        // TODO: Better error handling
        panic!("No initial state");
    };
    //println!("fsm1: fsm_state_fns_len: {} initial_state_hdl={}", fsm_state_fns_len, initial_state_hdl);

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
        use std::collections::VecDeque;

        //#[derive(Debug)]
        #[derive(Default)] // TODO: This default should be private as new must be used
        struct #fsm_ident {
            sm: SM, // Why is this not seen by vscode code completion?

            #(
                #[allow(unused)]
                #fsm_fields
            ),*
        }

        impl #fsm_ident {
            pub fn new() -> Self {
                let mut sm: #fsm_ident = Default::default();

                sm.initial_enter_fns_hdls();

                sm
            }

            #(
                #[allow(unused)]
                #converted_fns
            )*

            // When the state machine starts there will be no fn's to
            // exit so we initialize only the enter_fns_hdls.
            fn initial_enter_fns_hdls(&mut self) {
                let mut enter_hdl = self.sm.current_state_fns_hdl;
                loop {
                    //println!("initial_enter_fns_hdls: push(enter_hdl={})", enter_hdl);
                    self.sm.enter_fns_hdls.push(enter_hdl);
                    enter_hdl = if let Some(hdl) = self.sm.state_fns[enter_hdl].parent {
                        hdl
                    } else {
                        break;
                    };
                }
            }

            // Starting at self.current_state_fns_hdl generate the
            // list of StateFns that we're going to exit. If exit_sentinel is None
            // then exit from current_state_fns_hdl and all of its parents.
            // If exit_sentinel is Some then exit from the current state_fns_hdl
            // up to but not including the exit_sentinel.
            fn setup_exit_fns_hdls(&mut self, exit_sentinel: Option<usize>) {

                let mut exit_hdl = self.sm.current_state_fns_hdl;
                loop {
                    //println!("setup_exit_fns_hdls: push_back(exit_hdl={})", exit_hdl);
                    self.sm.exit_fns_hdls.push_back(exit_hdl);

                    if Some(exit_hdl) == exit_sentinel {
                        // This handles the special case where we're transitioning to ourself
                        //println!("setup_exit_fns_hdls: reached sentinel, done");
                        return;
                    }

                    // Getting parents handle
                    exit_hdl = if let Some(hdl) = self.sm.state_fns[exit_hdl].parent {
                        hdl
                    } else {
                        // No parent we're done
                        //println!("setup_exit_fns_hdls: No more parents, done");
                        return;
                    };

                    if Some(exit_hdl) == exit_sentinel {
                        // Reached the exit sentinel so we're done
                        return;
                    }
                }
            }

            // Setup exit_fns_hdls and enter_fns_hdls.
            fn setup_exit_enter_fns_hdls(&mut self, next_state_hdl: usize) {
                let mut cur_hdl = next_state_hdl;

                // Setup the enter vector
                let exit_sentinel = loop {
                    //println!("setup_exit_enter_fns_hdls: push(cur_hdl={})", cur_hdl);
                    self.sm.enter_fns_hdls.push(cur_hdl);

                    cur_hdl = if let Some(hdl) = self.sm.state_fns[cur_hdl].parent {
                        //println!("setup_exit_enter_fns_hdls: cur_hdl={}", cur_hdl);
                        hdl
                    } else {
                        // Exit state_fns[self.current_state_fns_hdl] and all its parents
                        //println!("setup_exit_enter_fns_hdls: No more parents");
                        break None;
                    };

                    if self.sm.state_fns[cur_hdl].active {
                        // Exit state_fns[self.current_state_fns_hdl] and
                        // parents upto but excluding state_fns[cur_hdl]
                        //println!("setup_exit_enter_fns_hdls: set exit_sentinel={}", cur_hdl);
                        break Some(cur_hdl);
                    }
                };

                // Setup the exit vector
                self.setup_exit_fns_hdls(exit_sentinel);
            }

            // TODO: Not sure this is worth it, if it is consider adding fsm_name()
            fn state_name(&self) -> &str {
                &self.sm.state_fns[self.sm.current_state_fns_hdl].name
            }

            fn dispatch_hdl(&mut self, hdl: StateFnsHdl) {
                if self.sm.current_state_changed {
                    // Execute the enter functions
                    while let Some(enter_hdl) = self.sm.enter_fns_hdls.pop() {
                        if let Some(state_enter) = self.sm.state_fns[enter_hdl].enter {
                            //println!("enter while: enter_hdl={} call state_enter={}", enter_hdl, state_enter as usize);
                            (state_enter)(self);
                            self.sm.state_fns[enter_hdl].active = true;
                            //println!("enter while: retf state_enter={}", state_enter as usize);
                        } else {
                            //println!("enter while: enter_hdl={} NO ENTER FN", enter_hdl);
                        }
                    }

                    self.sm.current_state_changed = false;
                }

                let mut transition_dest_hdl = None;

                match (self.sm.state_fns[hdl].process)(self) {
                    StateResult::NotHandled => {
                        // This handles the special case where we're transitioning to ourself
                        if let Some(parent_hdl) = self.sm.state_fns[hdl].parent {
                            self.dispatch_hdl(parent_hdl);
                        } else {
                            // TODO: Consider calling a "default_handler" when NotHandled and no parent
                        }
                    }
                    StateResult::Handled => {
                        // Nothing to do
                    }
                    StateResult::TransitionTo(next_state_hdl) => {
                        self.setup_exit_enter_fns_hdls(next_state_hdl);
                        self.sm.current_state_changed = true;
                        transition_dest_hdl = Some(next_state_hdl);
                    }
                }

                if self.sm.current_state_changed {
                    while let Some(exit_hdl) = self.sm.exit_fns_hdls.pop_front() {
                        if let Some(state_exit) = self.sm.state_fns[exit_hdl].exit {
                            (state_exit)(self);
                        }
                    }
                }

                if let Some(hdl) = transition_dest_hdl {
                    // Change the previous and current state_fns_hdl after we've
                    // preformed the exit routines so state_name is correct.
                    self.sm.previous_state_fns_hdl = self.sm.current_state_fns_hdl;
                    self.sm.current_state_fns_hdl = hdl;
                }
            }

            pub fn dispatch(&mut self) {
                self.dispatch_hdl(self.sm.current_state_fns_hdl);
            }
        }

        type StateFn = fn(&mut #fsm_ident, /* &Protocol1 */) -> StateResult;
        type StateFnEnter = fn(&mut #fsm_ident, /* &Protocol1 */);
        type StateFnExit = fn(&mut #fsm_ident, /* &Protocol1 */);
        type StateFnsHdl = usize;

        enum StateResult {
            NotHandled,
            Handled,
            TransitionTo(StateFnsHdl),
        }

        struct StateFns {
            name: String, // TODO: Remove or add SM::name?
            parent: Option<StateFnsHdl>,
            enter: Option<StateFnEnter>,
            process: StateFn,
            exit: Option<StateFnExit>,
            active: bool,
        }

        //#[derive(Debug)]
        struct SM {
            //name: String, // TODO: dd SM::name
            state_fns: [StateFns; #fsm_state_fns_len],
            enter_fns_hdls: Vec<StateFnsHdl>,
            exit_fns_hdls: VecDeque<StateFnsHdl>,
            current_state_fns_hdl: StateFnsHdl,
            previous_state_fns_hdl: StateFnsHdl,
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
                    enter_fns_hdls: Vec::<StateFnsHdl>::with_capacity(#fsm_state_fns_len),
                    exit_fns_hdls: VecDeque::<StateFnsHdl>::with_capacity(#fsm_state_fns_len),
                    current_state_fns_hdl: #initial_state_hdl,
                    previous_state_fns_hdl: #initial_state_hdl,
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
                    if let Some(hdl) = self.fsm_state_fn_ident_map.get(&parameter) {
                        //println!("Visitor::visit_macro_mut: Found {} in {} with index {}", parameter, self.fsm_ident, hdl);
                        node.tokens = quote!(#hdl);
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
