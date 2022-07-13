// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct RelatedPersonCommunication {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#preferred: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for RelatedPersonCommunication {
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
        state.serialize_entry("language", &self.r#language)?;
        if let Some(some) = self.r#preferred.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preferred", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preferred", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for RelatedPersonCommunication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RelatedPersonCommunication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RelatedPersonCommunication")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RelatedPersonCommunication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preferred: Option<super::super::types::Boolean> = None;
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
                        "language" => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some(map_access.next_value()?);
                        }
                        "preferred" => {
                            let some = r#preferred.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("preferred"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_preferred" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#preferred.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_preferred"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "language",
                                    "preferred",
                                ],
                            ))
                        }
                    }
                }
                Ok(RelatedPersonCommunication {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#language: r#language.ok_or(serde::de::Error::missing_field("language"))?,
                    r#preferred,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct RelatedPerson {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#communication: Vec<RelatedPersonCommunication>,
}
impl serde::ser::Serialize for RelatedPerson {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "RelatedPerson")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#active.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("active", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_active", &primitive_element)?;
            }
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if !self.r#relationship.is_empty() {
            state.serialize_entry("relationship", &self.r#relationship)?;
        }
        if !self.r#name.is_empty() {
            state.serialize_entry("name", &self.r#name)?;
        }
        if !self.r#telecom.is_empty() {
            state.serialize_entry("telecom", &self.r#telecom)?;
        }
        if let Some(some) = self.r#gender.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("gender", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_gender", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#birth_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("birthDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_birthDate", &primitive_element)?;
            }
        }
        if !self.r#address.is_empty() {
            state.serialize_entry("address", &self.r#address)?;
        }
        if !self.r#photo.is_empty() {
            state.serialize_entry("photo", &self.r#photo)?;
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if !self.r#communication.is_empty() {
            state.serialize_entry("communication", &self.r#communication)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for RelatedPerson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = RelatedPerson;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("RelatedPerson")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<RelatedPerson, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#active: Option<super::super::types::Boolean> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#relationship: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#name: Option<Vec<Box<super::super::types::HumanName>>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#gender: Option<super::super::types::Code> = None;
                let mut r#birth_date: Option<super::super::types::Date> = None;
                let mut r#address: Option<Vec<Box<super::super::types::Address>>> = None;
                let mut r#photo: Option<Vec<Box<super::super::types::Attachment>>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#communication: Option<Vec<RelatedPersonCommunication>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "active" => {
                            let some = r#active.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_active" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#active.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_active"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "relationship" => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            r#relationship = Some(map_access.next_value()?);
                        }
                        "name" => {
                            if r#name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            r#name = Some(map_access.next_value()?);
                        }
                        "telecom" => {
                            if r#telecom.is_some() {
                                return Err(serde::de::Error::duplicate_field("telecom"));
                            }
                            r#telecom = Some(map_access.next_value()?);
                        }
                        "gender" => {
                            let some = r#gender.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("gender"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_gender" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#gender.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_gender"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "birthDate" => {
                            let some = r#birth_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("birthDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_birthDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#birth_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_birthDate"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "address" => {
                            if r#address.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            r#address = Some(map_access.next_value()?);
                        }
                        "photo" => {
                            if r#photo.is_some() {
                                return Err(serde::de::Error::duplicate_field("photo"));
                            }
                            r#photo = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        "communication" => {
                            if r#communication.is_some() {
                                return Err(serde::de::Error::duplicate_field("communication"));
                            }
                            r#communication = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "active",
                                    "patient",
                                    "relationship",
                                    "name",
                                    "telecom",
                                    "gender",
                                    "birth_date",
                                    "address",
                                    "photo",
                                    "period",
                                    "communication",
                                ],
                            ))
                        }
                    }
                }
                Ok(RelatedPerson {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#active,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#relationship: r#relationship.unwrap_or(vec![]),
                    r#name: r#name.unwrap_or(vec![]),
                    r#telecom: r#telecom.unwrap_or(vec![]),
                    r#gender,
                    r#birth_date,
                    r#address: r#address.unwrap_or(vec![]),
                    r#photo: r#photo.unwrap_or(vec![]),
                    r#period,
                    r#communication: r#communication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
