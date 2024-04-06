use deluxe::{extract_attributes, ExtractAttributes};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Data::Struct, DeriveInput, Expr, Field,
    Fields::Named, FieldsNamed, GenericParam, Generics, Meta, TypeParam, WhereClause,
};

#[derive(ExtractAttributes)]
#[deluxe(attributes(separator))]
struct Separator(Expr);

pub(crate) fn from_str_derive_internal(item: TokenStream) -> syn::Result<TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;
    let separator: Result<Separator, _> = extract_attributes(&mut ast);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let where_clause = add_where_clause(where_clause, &ast.generics);
    if let Struct(data_struct) = &ast.data {
        if let Named(FieldsNamed { named: fields, .. }) = &data_struct.fields {
            let ident = &ast.ident;
            let split = if let Ok(Separator(sep)) = separator {
                quote!(value.split(#sep))
            } else {
                quote!(value.split_whitespace())
            };
            let parse_fields = parse_fields(fields);
            let fields = fields.iter().map(|f| &f.ident);
            return Ok(quote!(
                impl #impl_generics ::std::str::FromStr for #ident #ty_generics #where_clause {
                    type Err = ::utils::errors::ParseError;

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
    Err(syn::Error::new(Span::call_site(), ""))
}

fn parse_fields(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let parse = if f.attrs.iter().any(|a| match &a.meta {
                Meta::Path(path) if path.get_ident().is_some() => {
                    path.get_ident().unwrap() == "into"
                }
                _ => false,
            }) {
                quote!(part.into())
            } else {
                quote!(match part.trim().parse() {
                    Ok(parsed) => parsed,
                    Err(error) =>
                        return Err(::utils::errors::ParseError::ParseError(
                            ::std::stringify!(#ident),
                            ::std::format!("{}", error),
                        )),
                })
            };
            quote!(
                let #ident = if let Some(part) = parts.next() {
                    #parse
                } else {
                    return Err(::utils::errors::ParseError::EndOfInput(::std::stringify!(#ident)));
                };
            )
        })
        .collect()
}

fn add_where_clause(original: Option<&WhereClause>, generics: &Generics) -> WhereClause {
    let mut new = original.cloned().unwrap_or_else(|| WhereClause {
        where_token: Default::default(),
        predicates: Default::default(),
    });
    for param in &generics.params {
        if let GenericParam::Type(TypeParam { ident, .. }) = param {
            new.predicates
                .push(parse_quote!(#ident: ::std::str::FromStr));
            new.predicates
                .push(parse_quote!(<#ident as ::std::str::FromStr>::Err: ::std::fmt::Display));
        };
    }
    new
}
