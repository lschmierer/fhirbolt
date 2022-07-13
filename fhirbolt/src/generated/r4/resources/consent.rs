// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ConsentSource {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ConsentSource {
    fn default() -> ConsentSource {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConsentPolicy {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authority: Option<super::super::types::Uri>,
    pub r#uri: Option<super::super::types::Uri>,
}
impl serde::ser::Serialize for ConsentPolicy {
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
        if let Some(some) = self.r#authority.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("authority", some)?;
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
                state.serialize_entry("_authority", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#uri.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("uri", some)?;
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
                state.serialize_entry("_uri", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ConsentPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsentPolicy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConsentPolicy")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConsentPolicy, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#authority: Option<super::super::types::Uri> = None;
                let mut r#uri: Option<super::super::types::Uri> = None;
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
                        "authority" => {
                            let some = r#authority.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_authority" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#authority.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_authority"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "uri" => {
                            let some = r#uri.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("uri"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_uri" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#uri.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_uri"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "authority", "uri"],
                            ))
                        }
                    }
                }
                Ok(ConsentPolicy {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#authority,
                    r#uri,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConsentVerification {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#verified: super::super::types::Boolean,
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    pub r#verification_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for ConsentVerification {
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
        if let Some(some) = self.r#verified.value.as_ref() {
            state.serialize_entry("verified", some)?;
        }
        if self.r#verified.id.is_some() || !self.r#verified.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#verified.id,
                extension: &self.r#verified.extension,
            };
            state.serialize_entry("_verified", &primitive_element)?;
        }
        if let Some(some) = self.r#verified_with.as_ref() {
            state.serialize_entry("verifiedWith", some)?;
        }
        if let Some(some) = self.r#verification_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("verificationDate", some)?;
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
                state.serialize_entry("_verificationDate", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ConsentVerification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsentVerification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConsentVerification")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConsentVerification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#verified: Option<super::super::types::Boolean> = None;
                let mut r#verified_with: Option<Box<super::super::types::Reference>> = None;
                let mut r#verification_date: Option<super::super::types::DateTime> = None;
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
                        "verified" => {
                            let some = r#verified.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_verified" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#verified.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_verified"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "verifiedWith" => {
                            if r#verified_with.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifiedWith"));
                            }
                            r#verified_with = Some(map_access.next_value()?);
                        }
                        "verificationDate" => {
                            let some = r#verification_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("verificationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_verificationDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#verification_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_verificationDate"));
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
                                    "verified",
                                    "verified_with",
                                    "verification_date",
                                ],
                            ))
                        }
                    }
                }
                Ok(ConsentVerification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#verified: r#verified.ok_or(serde::de::Error::missing_field("verified"))?,
                    r#verified_with,
                    r#verification_date,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConsentProvisionActor {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Box<super::super::types::CodeableConcept>,
    pub r#reference: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ConsentProvisionActor {
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
        state.serialize_entry("role", &self.r#role)?;
        state.serialize_entry("reference", &self.r#reference)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ConsentProvisionActor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsentProvisionActor;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConsentProvisionActor")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConsentProvisionActor, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
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
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "role", "reference"],
                            ))
                        }
                    }
                }
                Ok(ConsentProvisionActor {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#role: r#role.ok_or(serde::de::Error::missing_field("role"))?,
                    r#reference: r#reference.ok_or(serde::de::Error::missing_field("reference"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConsentProvisionData {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meaning: super::super::types::Code,
    pub r#reference: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ConsentProvisionData {
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
        if let Some(some) = self.r#meaning.value.as_ref() {
            state.serialize_entry("meaning", some)?;
        }
        if self.r#meaning.id.is_some() || !self.r#meaning.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#meaning.id,
                extension: &self.r#meaning.extension,
            };
            state.serialize_entry("_meaning", &primitive_element)?;
        }
        state.serialize_entry("reference", &self.r#reference)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ConsentProvisionData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsentProvisionData;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConsentProvisionData")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConsentProvisionData, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#meaning: Option<super::super::types::Code> = None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
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
                        "meaning" => {
                            let some = r#meaning.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("meaning"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_meaning" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#meaning.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_meaning"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "meaning",
                                    "reference",
                                ],
                            ))
                        }
                    }
                }
                Ok(ConsentProvisionData {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#meaning: r#meaning.ok_or(serde::de::Error::missing_field("meaning"))?,
                    r#reference: r#reference.ok_or(serde::de::Error::missing_field("reference"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ConsentProvision {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#actor: Vec<ConsentProvisionActor>,
    pub r#action: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    pub r#purpose: Vec<Box<super::super::types::Coding>>,
    pub r#class: Vec<Box<super::super::types::Coding>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#data_period: Option<Box<super::super::types::Period>>,
    pub r#data: Vec<ConsentProvisionData>,
    pub r#provision: Vec<ConsentProvision>,
}
impl serde::ser::Serialize for ConsentProvision {
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
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("type", some)?;
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
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if !self.r#actor.is_empty() {
            state.serialize_entry("actor", &self.r#actor)?;
        }
        if !self.r#action.is_empty() {
            state.serialize_entry("action", &self.r#action)?;
        }
        if !self.r#security_label.is_empty() {
            state.serialize_entry("securityLabel", &self.r#security_label)?;
        }
        if !self.r#purpose.is_empty() {
            state.serialize_entry("purpose", &self.r#purpose)?;
        }
        if !self.r#class.is_empty() {
            state.serialize_entry("class", &self.r#class)?;
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if let Some(some) = self.r#data_period.as_ref() {
            state.serialize_entry("dataPeriod", some)?;
        }
        if !self.r#data.is_empty() {
            state.serialize_entry("data", &self.r#data)?;
        }
        if !self.r#provision.is_empty() {
            state.serialize_entry("provision", &self.r#provision)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ConsentProvision {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ConsentProvision;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ConsentProvision")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ConsentProvision, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#actor: Option<Vec<ConsentProvisionActor>> = None;
                let mut r#action: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#security_label: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#purpose: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#class: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#data_period: Option<Box<super::super::types::Period>> = None;
                let mut r#data: Option<Vec<ConsentProvisionData>> = None;
                let mut r#provision: Option<Vec<ConsentProvision>> = None;
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
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        "actor" => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some(map_access.next_value()?);
                        }
                        "action" => {
                            if r#action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#action = Some(map_access.next_value()?);
                        }
                        "securityLabel" => {
                            if r#security_label.is_some() {
                                return Err(serde::de::Error::duplicate_field("securityLabel"));
                            }
                            r#security_label = Some(map_access.next_value()?);
                        }
                        "purpose" => {
                            if r#purpose.is_some() {
                                return Err(serde::de::Error::duplicate_field("purpose"));
                            }
                            r#purpose = Some(map_access.next_value()?);
                        }
                        "class" => {
                            if r#class.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            r#class = Some(map_access.next_value()?);
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "dataPeriod" => {
                            if r#data_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("dataPeriod"));
                            }
                            r#data_period = Some(map_access.next_value()?);
                        }
                        "data" => {
                            if r#data.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            r#data = Some(map_access.next_value()?);
                        }
                        "provision" => {
                            if r#provision.is_some() {
                                return Err(serde::de::Error::duplicate_field("provision"));
                            }
                            r#provision = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "period",
                                    "actor",
                                    "action",
                                    "security_label",
                                    "purpose",
                                    "class",
                                    "code",
                                    "data_period",
                                    "data",
                                    "provision",
                                ],
                            ))
                        }
                    }
                }
                Ok(ConsentProvision {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#period,
                    r#actor: r#actor.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                    r#security_label: r#security_label.unwrap_or(vec![]),
                    r#purpose: r#purpose.unwrap_or(vec![]),
                    r#class: r#class.unwrap_or(vec![]),
                    r#code: r#code.unwrap_or(vec![]),
                    r#data_period,
                    r#data: r#data.unwrap_or(vec![]),
                    r#provision: r#provision.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Consent {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#scope: Box<super::super::types::CodeableConcept>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#date_time: Option<super::super::types::DateTime>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#organization: Vec<Box<super::super::types::Reference>>,
    pub r#source: Option<ConsentSource>,
    pub r#policy: Vec<ConsentPolicy>,
    pub r#policy_rule: Option<Box<super::super::types::CodeableConcept>>,
    pub r#verification: Vec<ConsentVerification>,
    pub r#provision: Option<ConsentProvision>,
}
impl serde::ser::Serialize for Consent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Consent")?;
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
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("scope", &self.r#scope)?;
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if let Some(some) = self.r#patient.as_ref() {
            state.serialize_entry("patient", some)?;
        }
        if let Some(some) = self.r#date_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("dateTime", some)?;
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
                state.serialize_entry("_dateTime", &primitive_element)?;
            }
        }
        if !self.r#performer.is_empty() {
            state.serialize_entry("performer", &self.r#performer)?;
        }
        if !self.r#organization.is_empty() {
            state.serialize_entry("organization", &self.r#organization)?;
        }
        if let Some(some) = self.r#source.as_ref() {
            match some {
                ConsentSource::Attachment(ref value) => {
                    state.serialize_entry("sourceAttachment", value)?;
                }
                ConsentSource::Reference(ref value) => {
                    state.serialize_entry("sourceReference", value)?;
                }
            }
        }
        if !self.r#policy.is_empty() {
            state.serialize_entry("policy", &self.r#policy)?;
        }
        if let Some(some) = self.r#policy_rule.as_ref() {
            state.serialize_entry("policyRule", some)?;
        }
        if !self.r#verification.is_empty() {
            state.serialize_entry("verification", &self.r#verification)?;
        }
        if let Some(some) = self.r#provision.as_ref() {
            state.serialize_entry("provision", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Consent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Consent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Consent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Consent, V::Error>
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
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#scope: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#date_time: Option<super::super::types::DateTime> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#organization: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#source: Option<ConsentSource> = None;
                let mut r#policy: Option<Vec<ConsentPolicy>> = None;
                let mut r#policy_rule: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#verification: Option<Vec<ConsentVerification>> = None;
                let mut r#provision: Option<ConsentProvision> = None;
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
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "scope" => {
                            if r#scope.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            r#scope = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "dateTime" => {
                            let some = r#date_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_dateTime" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#date_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_dateTime"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "organization" => {
                            if r#organization.is_some() {
                                return Err(serde::de::Error::duplicate_field("organization"));
                            }
                            r#organization = Some(map_access.next_value()?);
                        }
                        "sourceAttachment" => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceAttachment"));
                            }
                            r#source = Some(ConsentSource::Attachment(map_access.next_value()?));
                        }
                        "sourceReference" => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceReference"));
                            }
                            r#source = Some(ConsentSource::Reference(map_access.next_value()?));
                        }
                        "policy" => {
                            if r#policy.is_some() {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            r#policy = Some(map_access.next_value()?);
                        }
                        "policyRule" => {
                            if r#policy_rule.is_some() {
                                return Err(serde::de::Error::duplicate_field("policyRule"));
                            }
                            r#policy_rule = Some(map_access.next_value()?);
                        }
                        "verification" => {
                            if r#verification.is_some() {
                                return Err(serde::de::Error::duplicate_field("verification"));
                            }
                            r#verification = Some(map_access.next_value()?);
                        }
                        "provision" => {
                            if r#provision.is_some() {
                                return Err(serde::de::Error::duplicate_field("provision"));
                            }
                            r#provision = Some(map_access.next_value()?);
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
                                    "status",
                                    "scope",
                                    "category",
                                    "patient",
                                    "date_time",
                                    "performer",
                                    "organization",
                                    "source",
                                    "policy",
                                    "policy_rule",
                                    "verification",
                                    "provision",
                                ],
                            ))
                        }
                    }
                }
                Ok(Consent {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#scope: r#scope.ok_or(serde::de::Error::missing_field("scope"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#patient,
                    r#date_time,
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#organization: r#organization.unwrap_or(vec![]),
                    r#source,
                    r#policy: r#policy.unwrap_or(vec![]),
                    r#policy_rule,
                    r#verification: r#verification.unwrap_or(vec![]),
                    r#provision,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
