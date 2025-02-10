mod error;
use std::io::Write;

pub use error::*;

use quick_xml::Writer;

/// Identifies an element that can be converted into XML.
pub trait XmlComp<W: Write> {
    /// uses an xml writer to turn a type into an xml fragment
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError>;
}

impl<W: Write, T: XmlComp<W>> XmlComp<W> for [T] {
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError> {
        for item in self.iter() {
            item.to_xml(writer)?;
        }

        Ok(())
    }
}

impl<W: Write, T: XmlComp<W>> XmlComp<W> for Vec<T> {
    fn to_xml(&self, writer: &mut Writer<W>) -> Result<(), XmlWriteError> {
        for item in self.iter() {
            item.to_xml(writer)?;
        }

        Ok(())
    }
}
