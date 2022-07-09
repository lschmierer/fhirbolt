use convert_case::{Case, Casing};

pub trait RustCasing {
    fn to_rust_type_casing(&self) -> String;
    fn to_rust_mod_casing(&self) -> String;
    fn to_rust_identifier_casing(&self) -> String;
}

impl<S: AsRef<str>> RustCasing for S {
    fn to_rust_type_casing(&self) -> String {
        self.as_ref().to_case(Case::UpperCamel)
    }
    fn to_rust_mod_casing(&self) -> String {
        self.as_ref().to_case(Case::Snake)
    }
    fn to_rust_identifier_casing(&self) -> String {
        self.as_ref().to_case(Case::Snake)
    }
}
