use derive_syn_parse::Parse;
use syn::{punctuated::Punctuated, token::Brace, Attribute, Expr, Ident, Token, Visibility};

#[derive(Parse)]
pub(crate) struct Enum {
    #[call(Attribute::parse_outer)]
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) visibility: Visibility,
    _struct_token: Token![enum],
    pub(crate) ident: Ident,
    #[brace]
    _open_brace: Brace,
    #[parse_terminated(Variant::parse)]
    pub(crate) variants: Punctuated<Variant, Token![,]>,
}

#[derive(Parse)]
pub(crate) struct Variant {
    pub(crate) ident: Ident,
    _arrow: Token![=],
    pub(crate) char: Expr,
}
