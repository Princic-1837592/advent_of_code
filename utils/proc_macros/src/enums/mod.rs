use parser::Enum;
use proc_macro2::TokenStream;
use quote::quote;

mod parser;

pub(crate) fn from_char_internal(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> syn::Result<TokenStream> {
    let Enum {
        attrs,
        visibility,
        ident,
        variants,
        ..
    } = syn::parse(item)?;
    let chars: Vec<_> = variants.iter().map(|v| v.char.clone()).collect();
    let idents: Vec<_> = variants.iter().map(|v| v.ident.clone()).collect();

    Ok(quote!(
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
    ))
}
