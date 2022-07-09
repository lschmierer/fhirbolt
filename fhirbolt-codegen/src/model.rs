use serde::Deserialize;
use serde_json::value::RawValue;

#[derive(Deserialize, Debug)]
pub struct Bundle {
    pub entry: Vec<BundleEntry>,
}

impl Bundle {
    pub fn from_json_str(json: &str) -> serde_json::Result<Bundle> {
        serde_json::from_str(json)
    }
}

#[derive(Deserialize, Debug)]
pub struct BundleEntry {
    pub resource: Resource,
}

#[derive(Deserialize, Debug)]
#[serde(from = "&RawValue")]
pub enum Resource {
    StructureDefinition(StructureDefinition),
    Other,
}

#[derive(Deserialize, Debug)]
struct TmpResource {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

impl From<&RawValue> for Resource {
    fn from(raw: &RawValue) -> Resource {
        match serde_json::from_str::<TmpResource>(raw.get())
            .unwrap()
            .resource_type
            .as_ref()
        {
            "StructureDefinition" => {
                Resource::StructureDefinition(serde_json::from_str(raw.get()).unwrap())
            }
            _ => Resource::Other,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct StructureDefinition {
    pub url: String,

    pub kind: String,
    pub r#type: String,
    pub name: String,
    pub snapshot: StructureDefinitionSnapshot,
}

#[derive(Deserialize, Debug)]
pub struct StructureDefinitionSnapshot {
    pub element: Vec<ElementDefinition>,
}

#[derive(Deserialize, Debug)]
pub struct ElementDefinition {
    pub id: String,
    pub path: String,
    pub min: Option<u32>,
    pub max: Option<String>,
    pub r#type: Option<Vec<ElementDefinitionType>>,
    #[serde(rename = "contentReference")]
    pub content_reference: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ElementDefinitionType {
    pub code: String,
}
