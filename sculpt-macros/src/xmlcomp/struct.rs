use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Data, Ident, LitStr, Token};

pub struct StructImpl {
    pub tag: LitStr,
}

impl StructImpl {
    pub fn new(ident: &Ident) -> Self {
        let val = ident.to_string();
        Self {
            tag: LitStr::new(&val, ident.span()),
        }
    }

    pub fn to_tokens(&self, writer_ident: &Ident, data: &Data) -> TokenStream {
        let tag = &self.tag;
        quote! {
            let el_writer = #writer_ident
                .create_element(tag);


            Ok(())
        }
    }
}

impl Parse for StructImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            let is = &*ident.to_string();

            match is {
                "tag" => {
                    return Ok(Self {
                        tag: input.parse()?,
                    })
                }
                _ => return Err(syn::Error::new(ident.span(), "Invalid attribute")),
            }
        }

        Err(syn::Error::new(input.span(), "No tag"))
    }
}
