use sculpt::xml::XmlComp;

// #[derive(XmlComp)]
// struct Test {
//     #[xmlcomp(attribute)]
//     foo: i32,
// }
//

struct Test;

impl<__W> XmlComp<__W> for Test
where
    __W: std::io::Write,
{
    fn to_xml(
        &self,
        writer: &mut quick_xml::Writer<__W>,
    ) -> Result<(), sculpt::prelude::XmlWriteError> {
        Ok(())
    }
}

struct Test2<T>(T);

impl<__W, T> XmlComp<__W> for Test2<T>
where
    __W: std::io::Write,
    T: XmlComp<__W>,
{
    fn to_xml(
        &self,
        writer: &mut quick_xml::Writer<__W>,
    ) -> Result<(), sculpt::prelude::XmlWriteError> {
        self.0.to_xml(writer)
    }
}
