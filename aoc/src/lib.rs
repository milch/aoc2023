use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{bracketed, parse_macro_input, punctuated::Punctuated, token::Bracket, Expr, Lit, Token};

enum RangeOrArray {
    ItemArray(Punctuated<Lit, Token![,]>),
    Range(u8, u8),
}

struct Input {
    match_expr: Expr,
    call_expr: syn::ExprCall,
    items: RangeOrArray,
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let match_expr = input.parse()?;
        let _separator = input.parse::<Token![=>]>()?;
        let call_expr = input.parse()?;
        let _separator = input.parse::<Token![,]>()?;

        let la = input.lookahead1();
        if la.peek(syn::Lit) {
            let start = input.parse::<syn::LitInt>()?;
            let _delim = input.parse::<Token![..]>()?;
            let end = input.parse::<syn::LitInt>()?;
            Ok(Input {
                match_expr,
                call_expr,
                items: RangeOrArray::Range(start.base10_parse()?, end.base10_parse()?),
            })
        } else if la.peek(Bracket) {
            // Explicit array
            let items;
            bracketed!(items in input);
            let item_array = items.parse_terminated(Lit::parse, Token![,])?;

            Ok(Input {
                match_expr,
                call_expr,
                items: RangeOrArray::ItemArray(item_array),
            })
        } else {
            Err(la.error())
        }
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

            let expr = quote! {
                #match_expr => #call, #expanded
            };

            return aoc(expr.into());
        }
        RangeOrArray::ItemArray(item_array) => item_array
            .iter()
            .map(|elem| {
                let (elem_str, ident) = match elem {
                    Lit::Int(num) => (
                        syn::LitInt::new(num.base10_digits(), num.span()),
                        proc_macro2::Ident::new(
                            &format!("day_{:02}", num.base10_parse::<usize>().unwrap()),
                            num.span(),
                        ),
                    ),
                    _ => unimplemented!(),
                };
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
