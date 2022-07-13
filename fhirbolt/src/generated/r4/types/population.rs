// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PopulationAge {
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
impl Default for PopulationAge {
    fn default() -> PopulationAge {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct Population {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#age: Option<PopulationAge>,
    pub r#gender: Option<Box<super::super::types::CodeableConcept>>,
    pub r#race: Option<Box<super::super::types::CodeableConcept>>,
    pub r#physiological_condition: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for Population {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#age.as_ref() {
            match some {
                PopulationAge::Range(ref value) => {
                    state.serialize_entry("ageRange", value)?;
                }
                PopulationAge::CodeableConcept(ref value) => {
                    state.serialize_entry("ageCodeableConcept", value)?;
                }
            }
        }
        if let Some(some) = self.r#gender.as_ref() {
            state.serialize_entry("gender", some)?;
        }
        if let Some(some) = self.r#race.as_ref() {
            state.serialize_entry("race", some)?;
        }
        if let Some(some) = self.r#physiological_condition.as_ref() {
            state.serialize_entry("physiologicalCondition", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Population {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Population;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Population")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Population, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#age: Option<PopulationAge> = None;
                let mut r#gender: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#race: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#physiological_condition: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "ageRange" => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field("ageRange"));
                            }
                            r#age = Some(PopulationAge::Range(map_access.next_value()?));
                        }
                        "ageCodeableConcept" => {
                            if r#age.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "ageCodeableConcept",
                                ));
                            }
                            r#age = Some(PopulationAge::CodeableConcept(map_access.next_value()?));
                        }
                        "gender" => {
                            if r#gender.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            r#gender = Some(map_access.next_value()?);
                        }
                        "race" => {
                            if r#race.is_some() {
                                return Err(serde::de::Error::duplicate_field("race"));
                            }
                            r#race = Some(map_access.next_value()?);
                        }
                        "physiologicalCondition" => {
                            if r#physiological_condition.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "physiologicalCondition",
                                ));
                            }
                            r#physiological_condition = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "age",
                                    "gender",
                                    "race",
                                    "physiological_condition",
                                ],
                            ))
                        }
                    }
                }
                Ok(Population {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#age,
                    r#gender,
                    r#race,
                    r#physiological_condition,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
