use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Fields::Named, ItemStruct};

#[proc_macro_attribute]
pub fn from_line(attr: TokenStream, item: TokenStream) -> TokenStream {
    let separator = syn::parse_macro_input!(attr as Expr);
    let item = syn::parse_macro_input!(item as ItemStruct);
    let ItemStruct { ident, fields, .. } = item.clone();
    if let Named(fields) = fields {
        let fields = fields.named.iter().map(|f| f.ident.clone());
        quote!(
            #item

            impl From<&str> for #ident {
                fn from(value: &str) -> Self {
                    let mut parts = value.split(#separator);
                    Self {
                        #(#fields: parts.next().unwrap().trim().parse().unwrap()),*
                    }
                }
            }
        )
        .into()
    } else {
        unreachable!();
    }
}
