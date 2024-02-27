use derive_syn_parse::Parse;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
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
    pub(crate) fields: Variants,
}

pub(crate) struct Variants {
    pub(crate) variants: Punctuated<Variant, Token![,]>,
}

impl Parse for Variants {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Variants {
            variants: input.parse_terminated(Variant::parse, Token![,])?,
        })
    }
}

#[derive(Parse)]
pub(crate) struct Variant {
    pub(crate) ident: Ident,
    _arrow: Token![=],
    pub(crate) char: Expr,
}
