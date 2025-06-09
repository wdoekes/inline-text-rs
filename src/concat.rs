extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree};
use quote::quote;
use syn::Error;

pub fn concat_impl(input: TokenStream) -> TokenStream {
    let token_stream = proc_macro2::TokenStream::from(input);
    let mut output = String::new();

    for token in token_stream {
        match token {
            TokenTree::Literal(lit) => match syn::parse_str::<syn::LitStr>(&lit.to_string()) {
                Ok(parsed) => output.push_str(&parsed.value()),
                Err(_) => {
                    return Error::new_spanned(lit, "Expected a valid string literal")
                        .to_compile_error()
                        .into();
                }
            },
            TokenTree::Punct(p) if p.as_char() == ',' => continue,
            other => {
                return Error::new_spanned(other, "Expected string literal")
                    .to_compile_error()
                    .into();
            }
        }
    }

    let out = Literal::string(&output);
    quote!(#out).into()
}

/*
fn parse_string_literal(lit: &Literal) -> String {
    let s = lit.to_string();
    let unquoted = s.strip_prefix('"').and_then(|s| s.strip_suffix('"'));
    match unquoted {
        Some(inner) => inner.replace("\\n", "\n").replace("\\t", "\t"), // handle escaped newlines/tabs
        None => panic!("Expected a string literal"),
    }
}
*/
