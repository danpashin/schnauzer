pub trait AutoEnumFields {
    fn all_fields(&self) -> Vec<Field>;
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: String,
}

impl Field {
    #[must_use]
    pub fn new(name: String, value: String) -> Self {
        Field { name, value }
    }
}
