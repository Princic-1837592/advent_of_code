use proc_macro::TokenStream;

mod enums;
mod structs;

#[proc_macro_derive(FromStr, attributes(separator, into))]
pub fn from_str_derive_macro(item: TokenStream) -> TokenStream {
    match structs::from_str_derive_internal(item.into()) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
    .into()
}

#[proc_macro_attribute]
pub fn from_char(attr: TokenStream, item: TokenStream) -> TokenStream {
    match enums::from_char_internal(attr, item) {
        Ok(result) => result,
        Err(error) => error.to_compile_error(),
    }
    .into()
}
