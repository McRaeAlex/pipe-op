#![allow(dead_code)] // TODO: remove

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::*;
use syn::{self, parse_macro_input};
use syn::{Expr, ExprCall, ExprMethodCall};

#[derive(Debug)]
enum Callable<'a> {
    Function(&'a mut ExprCall),
    Method(&'a mut ExprMethodCall),
}

#[derive(Debug, Clone)]
struct PipeInput {
    expr: syn::Expr,
    ops: syn::punctuated::Punctuated<Expr, syn::token::Comma>,
}

impl syn::parse::Parse for PipeInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        // get the inital expression
        let expr: syn::Expr = input.parse()?;
        input.parse::<syn::token::Comma>()?;

        Ok(Self {
            expr,
            ops: input.parse_terminated(syn::Expr::parse)?, // parses the rest of the callables
        })
    }
}

impl PipeInput {
    fn pipe_output(mut self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        // create the argument to inject
        let arg0: syn::Expr = syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: syn::Ident::new("result", Span::call_site()).into(),
        }
        .into();

        for op in self.ops.iter_mut() {
            // Here we move right to left in the chaining calls.
            // So func().method().field.call() we are going from right to left
            // and keeping track of the leftmost value we can inject the arg
            // into. When we are all the way to the right we do it.
            let insert_into = match get_rightmost_callable(op) {
                Some(val) => val,
                None => return Err("Nothing to inject arg into".into()),
            };
            match insert_into {
                Callable::Function(f) => {
                    f.args.insert(0, arg0.clone());
                }
                Callable::Method(m) => {
                    m.args.insert(0, arg0.clone());
                }
            }
        }

        // formatting the output
        let init = self.expr;
        let ops = self.ops.iter();
        Ok(quote!({
            let result = #init;
            #(let result = #ops;)*
            result
        })
        .into())
    }
}

// goes threw an expression tree and gets the furthest left callable method
// or function.
fn get_rightmost_callable(e: &mut Expr) -> Option<Callable> {
    match e {
        Expr::Call(f) => {
            // return this function
            return Some(Callable::Function(f));
        }
        Expr::MethodCall(m) => {
            // it is possible that we can go further so we don't break
            // this is some vodoo magic passed down to me by Mutabah on discord
            // Its safe because m is never actually mutated untill after this
            // function call
            let something = unsafe { &mut *(m.receiver.as_mut() as *mut _) };
            match get_rightmost_callable(something) {
                Some(val) => Some(val),
                None => {
                    Some(Callable::Method(m))
                }
            }
        }
        Expr::Try(t) => get_rightmost_callable(t.expr.as_mut()),
        Expr::Await(a) => get_rightmost_callable(a.base.as_mut()),
        Expr::Field(f) => get_rightmost_callable(f.base.as_mut()),
        Expr::Path(_) | Expr::Lit(_) | Expr::Block(_) => None,
        _ => unimplemented!(),
    }
}

#[proc_macro]
pub fn pipe(input: TokenStream) -> TokenStream {
    let input: PipeInput = parse_macro_input!(input);
    input.pipe_output().expect("Unacceptable input")
}
