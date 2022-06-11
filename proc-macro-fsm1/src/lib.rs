use proc_macro::{self, TokenStream};
//use proc_macro2::{Span};
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
//  fsm1!(name {
//          value: i32
//      }
//      state initial(self, msg: Xyz) {
//          // do something like:
//          // prinln!("self.value={}", self.value);
//      }
//  )
//

//#[derive(Debug)]
//enum Fsm1 {
//    FsmName(syn::Ident),
//    FsmStructData(syn::Block),
//}

#[derive(Debug)]
struct Fsm1 {
    fsm_name: syn::Ident,
    //fsm_data_struct: syn::ExprStruct,
    fsm_fields: Vec<syn::FieldValue>,
}

impl Parse for Fsm1 {
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("Fsm1::parse:+ input={:#?}", input);

        let lookahead = input.lookahead1();
        let expr_struct = if lookahead.peek(syn::Ident) {
            input.parse::<syn::ExprStruct>()?
        } else {
            let err = lookahead.error();
            println!("Fsm1::parse: expecting identifer, error={:?}", &err);
            return Err(err);
        };
        //println!("Fsm1::parse: expr_struct={:#?}", expr_struct);

        let name = if let Some(n) = expr_struct.path.get_ident() {
            n
        } else {
            let err = syn::Error::new_spanned(expr_struct.path, "Fsm1::parse: expecting identifier for fsm");
            return Err(err);
        };
        //println!("Fsm1::parse: name={:#?}", name);
        
        let fields: Vec<syn::FieldValue> = expr_struct.fields.iter().map(|f| f.clone()).collect();
        //println!("Fsm1::parse: fields={:#?}", fields);

        Ok(Fsm1{
            fsm_name: name.clone(),
            fsm_fields: fields,
        })
    }
}

mod kw {
    syn::custom_keyword!(state);
}

#[proc_macro]
pub fn fsm1(input: TokenStream) -> TokenStream {
    println!("fsm1:+ input={:#?}", input);
    let in_ts = input.clone();

    let fsm = parse_macro_input!(in_ts as Fsm1);
    println!("fsm1: fsm={:#?}", fsm);

    let fsm_name = fsm.fsm_name;
    println!("fsm1: fsm_name={:#?}", fsm_name);

    let fsm_fields = fsm.fsm_fields; 
    println!("fsm1: fsm_fields={:#?}", fsm_fields);

    let output = quote!(struct #fsm_name {
        #(#fsm_fields),*
    });
    println!("fsm1:- output={:#?}", output);

    output.into()
}
