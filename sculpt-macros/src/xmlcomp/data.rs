use syn::{parse::Parse, Data, Field, LitStr};

pub struct DataImpl {
    opts: Opts,
}

impl DataImpl {
    pub fn new(data: &Data) -> Vec<Self> {
        match data {
            Data::Struct(data) => {
                //todo

                data.fields.iter().map(Self::new_from_field).collect()
            }
            Data::Enum(data) => {
                //todo
                //todo
                data.variants
            }
            _ => {
                panic!("Does not support enums rn")
            }
        }
    }

    pub fn new_from_field(field: &Field) -> Self {
        todo!()
    }
}

enum Opts {
    Child,
    Attribute { name: LitStr },
}
