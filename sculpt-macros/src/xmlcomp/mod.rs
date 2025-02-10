use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, Data, DeriveInput, GenericParam,
    Generics, Ident, ImplGenerics, LitStr, Meta, TypeGenerics, TypeParam, WhereClause,
};

mod r#struct;
use r#struct::*;

mod data;
use data::*;

#[inline]
pub fn derive_xml_comp_inner(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let struct_opts = match input
        .attrs
        .into_iter()
        .find(|attr| attr.path().is_ident("openxml"))
        .map(|attr| attr.parse_args::<StructImpl>())
    {
        Some(attr) => attr.unwrap(),
        None => StructImpl::new(&name),
    };

    let (impl_generics, ty_generics, where_clause) = split_generics(input.generics);

    let element_comp = xmlcomp_data(&input.data);

    let expanded = quote! {
        impl #impl_generics sculpt::xml::XmlComp<__W> for #name #ty_generics #where_clause {
            fn to_xml(&self, __w: &mut quick_xml::Writer<__W>) -> Result<(), sculpt::xml::XmlWriteError> {
                Ok(())
            }
        }
    };
    expanded.into()
}

fn xmlcomp_data(data: &Data) -> TokenStream {
    TokenStream::default()
}

fn split_generics(mut generics: Generics) -> (TokenStream, TokenStream, TokenStream) {
    let struct_generics = generics.clone();
    let (_, ty_generics, _) = struct_generics.split_for_impl();

    let mut writer_bounds = Punctuated::new();
    writer_bounds.push(parse_quote!(std::io::Write));

    let writer_type = GenericParam::Type(TypeParam {
        attrs: vec![],
        ident: Ident::new("__W", Span::call_site()),
        colon_token: Some(Default::default()),
        bounds: writer_bounds,
        eq_token: None,
        default: None,
    });

    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param
                .bounds
                .push(parse_quote!(sculpt::xml::XmlComp<__W>));
        }
    }

    generics.params.push(writer_type);

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    (
        quote! {#impl_generics},
        quote! {#ty_generics},
        quote! {#where_clause},
    )
}
