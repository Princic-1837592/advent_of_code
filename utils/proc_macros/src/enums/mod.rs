use parser::Enum;
use proc_macro::TokenStream;
use quote::quote;

mod parser;

pub(crate) fn from_char_internal(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let Enum {
        attrs,
        visibility,
        ident,
        variants,
        ..
    } = syn::parse_macro_input!(item as Enum);
    let chars: Vec<_> = variants.iter().map(|v| v.char.clone()).collect();
    let idents: Vec<_> = variants.iter().map(|v| v.ident.clone()).collect();

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
