extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

pub fn collapse_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let original = input.value();

    let collapsed = original
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    TokenStream::from(quote! {
        #collapsed
    })
}
