mod xmlcomp;
use xmlcomp::*;

#[proc_macro_derive(XmlComp, attributes(xml))]
pub fn derive_xml_comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_xml_comp_inner(input)
}
