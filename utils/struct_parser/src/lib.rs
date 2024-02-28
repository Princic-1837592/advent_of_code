use deluxe::{parse_attributes, ParseAttributes};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Expr, Fields::Named};

#[proc_macro_derive(FromLine, attributes(separator, into))]
pub fn from_line_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    from_line_derive_macro2(item.into()).unwrap().into()
}

#[derive(ParseAttributes)]
#[deluxe(attributes(separator))]
struct Separator(Expr);

fn from_line_derive_macro2(item: TokenStream) -> deluxe::Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(item)?;
    let separator: Result<Separator, _> = parse_attributes(&ast);
    if let Struct(data_struct) = &ast.data {
        if let Named(fields) = &data_struct.fields {
            let ident = &ast.ident;
            let split = if let Ok(separator) = separator {
                let separator = separator.0;
                quote!(value.split(#separator))
            } else {
                quote!(value.split_whitespace())
            };
            let fields: Vec<_> = fields
                .named
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    if f.attrs.is_empty() {
                        quote!(#ident: parts.next().unwrap().trim().parse().unwrap())
                    } else {
                        quote!(#ident: parts.next().unwrap().trim().into())
                    }
                })
                .collect();
            let result = quote!(
                impl From<&str> for #ident{
                    fn from(value: &str) -> Self {
                        let mut parts = #split;
                        Self {
                            #(#fields),*
                        }
                    }
                }
            );
            println!("{}", result);
            return Ok(result);
        }
    }
    panic!("Something went wrong");
}
