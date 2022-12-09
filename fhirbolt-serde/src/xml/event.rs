#[derive(Debug)]
pub enum Event {
    ElementStart(Element),
    ElementEnd,
    EmptyElement(Element),
    Eof,
}

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub value: Option<String>,
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.to_owned(),
            id: None,
            url: None,
            value: None,
        }
    }
}
