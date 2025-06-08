extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitInt, LitStr, Token, Result,
};


struct DedentArgs {
    string: LitStr,
    keep_ws: usize,
}

impl Parse for DedentArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        let (string, keep_ws) = if lookahead.peek(LitStr) {
            // Format: dedent!("...", keep_ws = N)
            let string: LitStr = input.parse()?;

            let mut keep_ws = 0;
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                let ident: Ident = input.parse()?;
                if ident != "keep_ws" {
                    return Err(syn::Error::new(ident.span(), "expected `keep_ws`"));
                }
                input.parse::<Token![=]>()?;
                let litint: LitInt = input.parse()?;
                keep_ws = litint.base10_parse::<usize>()?;
            }

            (string, keep_ws)
        } else {
            // Format: dedent!(keep_ws = N, "...")
            let ident: Ident = input.parse()?;
            if ident != "keep_ws" {
                return Err(syn::Error::new(ident.span(), "expected `keep_ws`"));
            }
            input.parse::<Token![=]>()?;
            let litint: LitInt = input.parse()?;
            let keep_ws = litint.base10_parse::<usize>()?;
            input.parse::<Token![,]>()?;
            let string: LitStr = input.parse()?;

            (string, keep_ws)
        };

        Ok(DedentArgs {
            string,
            keep_ws,
        })
    }
}

pub fn dedent_impl(input: TokenStream) -> TokenStream {
    let DedentArgs {
        string,
        keep_ws,
    } = parse_macro_input!(input as DedentArgs);

    let original = string.value();
    let mut lines: Vec<&str> = original.lines().collect();
    if original.ends_with('\n') {
        lines.push("");
    }

    // Find minimum indent (tabs or spaces, but not mixed)
    let mut min_indent = usize::MAX;
    let mut indent_style: Option<char> = None; // Either Some(' ') or Some('\t')

    // If the user uses:
    // let s = "\
    //     SELECT *
    //     FROM abc;
    // ";
    // Then the rust removes not just the LF from line 1, but also the
    // leading spaces.
    let mut first_line_needs_indent = false;

    for (i, line) in lines.iter().enumerate() {
        // Skip first line if it starts with non-whitespace
        if i == 0 {
            match line.chars().next() {
                Some(' ') | Some('\t') => {} // keep processing
                _ => { first_line_needs_indent = true; continue }, // skip this line from indent analysis
            }
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut chars = line.chars().peekable();
        let mut indent_len = 0;
        let mut current_style: Option<char> = None;

        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\t' => {
                    if current_style.is_none() {
                        current_style = Some(c);
                    } else if current_style != Some(c) {
                        // Mixed indentation on a single line
                        let msg = format!("Mixed spaces and tabs on same line: {:?}", line);
                        return syn::Error::new_spanned(&string, msg).to_compile_error().into();
                    }
                    indent_len += 1;
                    chars.next();
                }
                _ => break,
            }
        }

        if let Some(style) = current_style {
            match indent_style {
                None => indent_style = Some(style),
                Some(prev) if prev != style => {
                    let msg = format!("Mixed indentation across lines: found both '{}' and '{}'", prev, style);
                    return syn::Error::new_spanned(&string, msg).to_compile_error().into();
                }
                _ => {}
            }
            min_indent = min_indent.min(indent_len);
        }
    }

    let indent_prefix = indent_style
        .map(|c| c.to_string().repeat(keep_ws))
        .unwrap_or_default();

    let actual_indent = min_indent.saturating_sub(keep_ws);

    let dedented = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            let stripped = if line.trim().is_empty() {
                ""
            } else if i == 0 && first_line_needs_indent {
                line
            } else if line.len() >= actual_indent {
                &line[actual_indent..]
            } else {
                line
            };

            if i == 0 && first_line_needs_indent && !stripped.is_empty() {
                format!("{}{}", indent_prefix, stripped)
            } else {
                stripped.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    TokenStream::from(quote! {
        #dedented
    })
}
