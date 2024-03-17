use proc_macro::TokenStream;

use crate::enums::from_char_internal;
mod enums;
mod structs;

#[proc_macro_derive(FromStr, attributes(separator))]
pub fn from_line_derive_macro(item: TokenStream) -> TokenStream {
    structs::from_line_derive_internal(item.into())
        .unwrap()
        .into()
}

#[proc_macro_attribute]
pub fn from_char(attr: TokenStream, item: TokenStream) -> TokenStream {
    from_char_internal(attr, item)
}
