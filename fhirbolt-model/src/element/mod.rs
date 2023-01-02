mod de;
mod ser;

pub use de::DeserializationContext;
pub use ser::SerializationContext;

pub type Element = indexmap::IndexMap<String, Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Element(Element),
    Sequence(Vec<Element>),
    Primitive(Primitive),
}

#[derive(Clone, Debug)]
pub enum Primitive {
    Bool(bool),
    Integer(i64),
    Decimal(String),
    String(String),
}
