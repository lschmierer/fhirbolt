mod de;

pub use de::DeserializationContext;

pub type Element = indexmap::IndexMap<String, Value>;

#[derive(Debug)]
pub enum Value {
    Element(Element),
    Sequence(Vec<Element>),
    PrimitiveValue(PrimitiveValue),
}

#[derive(Debug)]
pub enum PrimitiveValue {
    Bool(bool),
    Integer(i64),
    Decimal(String),
    String(String),
}
