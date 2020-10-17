#![allow(dead_code)] // TODO: remove

extern crate proc_macro;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::*;
use syn::{self, parse_macro_input};
use syn::{ExprCall, ExprMethodCall, ExprAwait, ExprTry};

#[derive(Debug, Clone)]
enum Callable {
    Function(ExprCall),
    Method(ExprMethodCall), // we should go to the lowest method or function
    Await(ExprAwait),
    Try(ExprTry),
}

#[derive(Debug, Clone)]
struct PipeInput {
    expr: syn::Expr,
    ops: syn::punctuated::Punctuated<syn::ExprCall, syn::token::Comma>
}

impl syn::parse::Parse for PipeInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        println!("{:#?}", input);
        // get the inital expression
        let expr: syn::Expr = input.parse()?;
        input.parse::<syn::token::Comma>()?;
            
        
        Ok(Self {
            expr,
            ops: input.parse_terminated(syn::ExprCall::parse)?, // parses the rest of the callables
        })
    }
}


#[proc_macro]
pub fn pipe(input: TokenStream) -> TokenStream {
    let mut input: PipeInput = parse_macro_input!(input);
    let arg0: syn::Expr = syn::ExprPath{
        attrs: vec![],
        qself: None,
        path: syn::Ident::new("result", Span::call_site()).into(),
    }.into();

    for op in input.ops.iter_mut() {
        op.args.insert(0, arg0.clone());
    }        

    let init = input.expr;
    let ops = input.ops.iter();
    quote!({
        let result = #init;
        #(let result = #ops;)*
        result
    }).into()
}
