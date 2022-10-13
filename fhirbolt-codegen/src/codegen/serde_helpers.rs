use quote::quote;

use crate::codegen::SourceFile;

pub fn generate_serde_helpers() -> SourceFile {
    SourceFile {
        name: "serde_helpers".into(),
        source: quote! {
            #[derive(serde::Serialize)]
            pub struct PrimitiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                pub id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                pub extension: &'a [Box<super::types::Extension>],
            }

            #[derive(serde::Deserialize)]
            pub struct PrimitiveElementOwned {
                pub id: Option<std::string::String>,
                pub extension: Vec<Box<super::types::Extension>>,
            }
        },
    }
}
