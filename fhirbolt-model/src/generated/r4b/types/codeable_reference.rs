// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "Base StructureDefinition for CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).\n\nThis is a common pattern in record keeping - a reference may be made to a specific condition, observation, plan, or definition, or a reference may be made to a general concept defined in a knowledge base somewhere."]
#[derive(Default, Debug, Clone)]
pub struct CodeableReference {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A reference to a concept - e.g. the information is identified by its general class to the degree of precision found in the terminology."]
    pub r#concept: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to a resource the provides exact details about the information being referenced."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for CodeableReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if let Some(some) = self.r#concept.as_ref() {
                state.serialize_entry("concept", some)?;
            }
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CodeableReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "concept")]
            Concept,
            #[serde(rename = "reference")]
            Reference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CodeableReference;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CodeableReference")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CodeableReference, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#concept: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::Concept => {
                                if r#concept.is_some() {
                                    return Err(serde::de::Error::duplicate_field("concept"));
                                }
                                r#concept = Some(map_access.next_value()?);
                            }
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "concept", "reference"],
                                ));
                            },
                        }
                    }
                    Ok(CodeableReference {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#concept,
                        r#reference,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
