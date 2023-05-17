pub mod de;
pub mod ser;

#[derive(Copy, Clone, PartialEq)]
pub enum Format {
    Json,
    Xml,
    InternalElement,
    External,
}
