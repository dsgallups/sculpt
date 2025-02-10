use sculpt::XmlComp;

// #[derive(XmlComp)]
// struct Test {
//     #[xmlcomp(attribute)]
//     foo: i32,
// }
//

#[derive(XmlComp)]
struct Test {}

#[derive(XmlComp)]
#[xml(f = 2, g = 3)]
struct Test2<T>(T);
