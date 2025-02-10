mod error;
pub use error::*;

use quick_xml::Writer;

/// Identifies an element that can be converted into XML.
pub trait XmlComp<W> {
    /// uses an xml writer to turn a type into an xml fragment
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError>;
}

impl<W, T: XmlComp<W>> XmlComp<W> for [T] {
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError> {
        for item in self.iter() {
            item.to_xml(writer)?;
        }

        Ok(())
    }
}

impl<W, T: XmlComp<W>> XmlComp<W> for Vec<T> {
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError> {
        for item in self.iter() {
            item.to_xml(writer)?;
        }

        Ok(())
    }
}
