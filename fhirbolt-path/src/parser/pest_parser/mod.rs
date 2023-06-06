use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/pest_parser/fhirpath.pest"]
pub struct PestParser;
