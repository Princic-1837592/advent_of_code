use deluxe::{extract_attributes, ExtractAttributes};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data::Struct, DeriveInput, Expr, Fields::Named, FieldsNamed};

#[derive(ExtractAttributes)]
#[deluxe(attributes(separator))]
struct Separator(Expr);

pub(crate) fn from_line_derive_internal(item: TokenStream) -> deluxe::Result<TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;
    let separator: Result<Separator, _> = extract_attributes(&mut ast);
    if let Struct(data_struct) = &ast.data {
        if let Named(FieldsNamed { named: fields, .. }) = &data_struct.fields {
            let ident = &ast.ident;
            let split = if let Ok(separator) = separator {
                let separator = separator.0;
                quote!(value.split(#separator))
            } else {
                quote!(value.split_whitespace())
            };
            let parse_fields: Vec<_> = fields
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    quote!(
                        let #ident = if let Some(part) = parts.next() {
                            match part.trim().parse() {
                                Ok(parsed) => parsed,
                                Err(error) => return Err(format!("Error while parsing `{}`: {}", stringify!(#ident), error)),
                            }
                        } else {
                            return Err(format!("Unexpected end of input while parsing {}", stringify!(#ident)));
                        };
                    )
                })
                .collect();
            let fields = fields.iter().map(|f| &f.ident);
            return Ok(quote!(
                impl std::str::FromStr for #ident{
                    type Err = String;

                    fn from_str(value: &str) -> Result<Self, Self::Err> {
                        let mut parts = #split;
                        #(#parse_fields)*
                        Ok(Self {
                            #(#fields),*
                        })
                    }
                }
            ));
        }
    }
    panic!("Something went wrong");
}
