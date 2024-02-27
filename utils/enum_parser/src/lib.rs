mod parser;

use proc_macro::TokenStream;
use quote::quote;

use crate::parser::Enum;

#[proc_macro_attribute]
pub fn from_char(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let Enum {
        attrs,
        visibility,
        ident,
        fields,
        ..
    } = syn::parse_macro_input!(item as Enum);
    let chars: Vec<_> = fields.variants.iter().map(|v| v.char.clone()).collect();
    let idents: Vec<_> = fields.variants.iter().map(|v| v.ident.clone()).collect();

    quote!(
        #(#attrs)*
        #visibility enum #ident{
            #(#idents),*
        }

        impl From<char> for #ident{
            fn from(value: char) -> Self {
                match value{
                    #(#chars => Self::#idents),*,
                    _ => unreachable!(),
                }
            }
        }
    )
    .into()
}
