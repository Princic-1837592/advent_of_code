use derive_syn_parse::Parse;
use syn::{
    parse::{Parse, ParseStream},
    token::Brace,
    Attribute, Expr, Ident, Token, Visibility,
};

#[derive(Parse)]
pub(crate) struct Enum {
    #[call(Attribute::parse_outer)]
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) visibility: Visibility,
    _struct_token: Token![enum],
    pub(crate) ident: Ident,
    #[brace]
    _open_brace: Brace,
    #[inside(_open_brace)]
    #[call(parse_vector)]
    pub(crate) variants: Vec<Variant>,
}

#[derive(Parse)]
pub(crate) struct Variant {
    pub(crate) ident: Ident,
    _arrow: Token![=],
    pub(crate) char: Expr,
}

fn parse_vector<T: Parse>(input: ParseStream) -> syn::Result<Vec<T>> {
    Ok(input
        .parse_terminated(T::parse, Token![,])?
        .into_iter()
        .collect())
}
