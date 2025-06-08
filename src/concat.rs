extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit, Token, parse_macro_input, punctuated::Punctuated};

pub fn concat_impl(input: TokenStream) -> TokenStream {
    // Parse input as comma-separated expressions
    let args = parse_macro_input!(input with Punctuated::<Expr, Token![,]>::parse_terminated);

    let mut pieces = Vec::new();

    for expr in args.iter() {
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => {
                pieces.push(s.value());
            }
            _ => {
                return syn::Error::new_spanned(expr, "Expected a string literal")
                    .to_compile_error()
                    .into();
            }
        }
    }

    let joined = pieces.concat();

    TokenStream::from(quote! {
        #joined
    })
}
