#[derive(Debug, PartialEq)]
pub enum Event {
    ElementStart(Element),
    ElementEnd,
    EmptyElement(Element),
    Div(Element),
    Eof,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Element {
    pub name: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub value: Option<String>,
}

impl Element {
    pub fn new(name: String) -> Element {
        Element {
            name,
            id: None,
            url: None,
            value: None,
        }
    }

    pub fn is_resource(&self) -> bool {
        self.name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
    }

    pub fn resource_type(&self) -> Option<&str> {
        if self.is_resource() {
            Some(&self.name)
        } else {
            None
        }
    }
}
