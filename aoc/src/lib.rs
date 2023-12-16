use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{
    braced, bracketed, parse_macro_input, punctuated::Punctuated, token::Bracket, Expr, ExprCall,
    Ident, LitInt, Token,
};

enum RangeOrArray {
    ItemArray(Punctuated<LitInt, Token![,]>),
    Range(u8, u8),
}

struct Input {
    match_expr: Expr,
    call_expr: ExprCall,
    format_prefix: Ident,
    format_spec: usize,
    items: RangeOrArray,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let match_expr = input.parse()?;
        let _separator = input.parse::<Token![=>]>()?;
        let format_prefix = input.parse::<Ident>()?;

        let braced_content;
        braced!(braced_content in input);
        braced_content.parse::<Token![:]>()?;

        let format_spec = braced_content.parse::<LitInt>()?.base10_parse()?;
        input.parse::<Token![::]>()?;

        let call_expr = input.parse()?;
        let _separator = input.parse::<Token![,]>()?;

        let la = input.lookahead1();
        let items = if la.peek(syn::Lit) {
            let start = input.parse::<syn::LitInt>()?;
            let _delim = input.parse::<Token![..]>()?;
            let end = input.parse::<syn::LitInt>()?;
            RangeOrArray::Range(start.base10_parse()?, end.base10_parse()?)
        } else if la.peek(Bracket) {
            let items;
            bracketed!(items in input);
            let item_array = items.parse_terminated(LitInt::parse, Token![,])?;
            RangeOrArray::ItemArray(item_array)
        } else {
            return Err(la.error());
        };

        Ok(Input {
            match_expr,
            call_expr,
            format_prefix,
            format_spec,
            items,
        })
    }
}

#[proc_macro]
pub fn aoc(items: TokenStream) -> TokenStream {
    let input = parse_macro_input!(items as Input);

    let match_expr = input.match_expr;

    let call = input.call_expr;
    let arms = match input.items {
        RangeOrArray::Range(start, end) => {
            let elems = start..=end;
            let expanded = quote! {
                [ #( #elems ),* ]
            };

            let prefix = input.format_prefix;
            let spec = input.format_spec;
            let expr = quote! {
                #match_expr => #prefix{:#spec}::#call, #expanded
            };

            return aoc(expr.into());
        }
        RangeOrArray::ItemArray(item_array) => item_array
            .iter()
            .map(|elem| {
                let (elem_str, ident) = (
                    syn::LitInt::new(elem.base10_digits(), elem.span()),
                    proc_macro2::Ident::new(
                        &format!(
                            "{}{:0>width$}",
                            input.format_prefix,
                            elem.base10_parse::<usize>().unwrap(),
                            width = input.format_spec
                        ),
                        input.format_prefix.span(),
                    ),
                );
                quote! {
                    #elem_str => #ident::#call
                }
                .clone()
            })
            .collect::<Vec<_>>(),
    };

    let tokens = quote! {
        match(#match_expr) {
            #(#arms,)*
            _ => unimplemented!()
        }
    };

    TokenStream::from(tokens)
}
