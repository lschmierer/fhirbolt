// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum AuditEventEntityDetailValue {
    String(Box<super::super::types::String>),
    Base64Binary(Box<super::super::types::Base64Binary>),
    Invalid,
}
impl Default for AuditEventEntityDetailValue {
    fn default() -> AuditEventEntityDetailValue {
        AuditEventEntityDetailValue::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEventAgentNetwork {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#address: Option<super::super::types::String>,
    pub r#type: Option<super::super::types::Code>,
}
impl serde::ser::Serialize for AuditEventAgentNetwork {
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
        if let Some(some) = self.r#address.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("address", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_address", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventAgentNetwork {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "_address")]
            AddressPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventAgentNetwork;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgentNetwork")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgentNetwork, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#address: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Address => {
                            let some = r#address.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AddressPrimitiveElement => {
                            let some = r#address.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_address"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TypePrimitiveElement => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(AuditEventAgentNetwork {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#address,
                    r#type,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEventAgent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#alt_id: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#requestor: super::super::types::Boolean,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#policy: Vec<super::super::types::Uri>,
    pub r#media: Option<Box<super::super::types::Coding>>,
    pub r#network: Option<AuditEventAgentNetwork>,
    pub r#purpose_of_use: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for AuditEventAgent {
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
            state.serialize_entry("type", some)?;
        }
        if !self.r#role.is_empty() {
            state.serialize_entry("role", &self.r#role)?;
        }
        if let Some(some) = self.r#who.as_ref() {
            state.serialize_entry("who", some)?;
        }
        if let Some(some) = self.r#alt_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("altId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_altId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#requestor.value.as_ref() {
            state.serialize_entry("requestor", some)?;
        }
        if self.r#requestor.id.is_some() || !self.r#requestor.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#requestor.id,
                extension: &self.r#requestor.extension,
            };
            state.serialize_entry("_requestor", &primitive_element)?;
        }
        if let Some(some) = self.r#location.as_ref() {
            state.serialize_entry("location", some)?;
        }
        if !self.r#policy.is_empty() {
            let values: Vec<_> = self.r#policy.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("policy", &values)?;
            }
            let requires_elements = self
                .r#policy
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#policy
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_policy", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#media.as_ref() {
            state.serialize_entry("media", some)?;
        }
        if let Some(some) = self.r#network.as_ref() {
            state.serialize_entry("network", some)?;
        }
        if !self.r#purpose_of_use.is_empty() {
            state.serialize_entry("purposeOfUse", &self.r#purpose_of_use)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventAgent {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "who")]
            Who,
            #[serde(rename = "altId")]
            AltId,
            #[serde(rename = "_altId")]
            AltIdPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "requestor")]
            Requestor,
            #[serde(rename = "_requestor")]
            RequestorPrimitiveElement,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "policy")]
            Policy,
            #[serde(rename = "_policy")]
            PolicyPrimitiveElement,
            #[serde(rename = "media")]
            Media,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "purposeOfUse")]
            PurposeOfUse,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventAgent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventAgent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventAgent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#role: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#who: Option<Box<super::super::types::Reference>> = None;
                let mut r#alt_id: Option<super::super::types::String> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#requestor: Option<super::super::types::Boolean> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#policy: Option<Vec<super::super::types::Uri>> = None;
                let mut r#media: Option<Box<super::super::types::Coding>> = None;
                let mut r#network: Option<AuditEventAgentNetwork> = None;
                let mut r#purpose_of_use: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        Field::Who => {
                            if r#who.is_some() {
                                return Err(serde::de::Error::duplicate_field("who"));
                            }
                            r#who = Some(map_access.next_value()?);
                        }
                        Field::AltId => {
                            let some = r#alt_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("altId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AltIdPrimitiveElement => {
                            let some = r#alt_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_altId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Requestor => {
                            let some = r#requestor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RequestorPrimitiveElement => {
                            let some = r#requestor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_requestor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Location => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            r#location = Some(map_access.next_value()?);
                        }
                        Field::Policy => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#policy.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("policy"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::PolicyPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#policy.get_or_insert(Vec::with_capacity(elements.len()));
                            if vec.len() != elements.len() {
                                return Err(serde::de::Error::invalid_length(
                                    elements.len(),
                                    &"primitive values length",
                                ));
                            }
                            if vec
                                .iter()
                                .any(|e| e.id.is_some() || !e.extension.is_empty())
                            {
                                return Err(serde::de::Error::duplicate_field("_policy"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Media => {
                            if r#media.is_some() {
                                return Err(serde::de::Error::duplicate_field("media"));
                            }
                            r#media = Some(map_access.next_value()?);
                        }
                        Field::Network => {
                            if r#network.is_some() {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            r#network = Some(map_access.next_value()?);
                        }
                        Field::PurposeOfUse => {
                            if r#purpose_of_use.is_some() {
                                return Err(serde::de::Error::duplicate_field("purposeOfUse"));
                            }
                            r#purpose_of_use = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AuditEventAgent {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#role: r#role.unwrap_or(vec![]),
                    r#who,
                    r#alt_id,
                    r#name,
                    r#requestor: r#requestor.ok_or(serde::de::Error::missing_field("requestor"))?,
                    r#location,
                    r#policy: r#policy.unwrap_or(vec![]),
                    r#media,
                    r#network,
                    r#purpose_of_use: r#purpose_of_use.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEventSource {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#site: Option<super::super::types::String>,
    pub r#observer: Box<super::super::types::Reference>,
    pub r#type: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for AuditEventSource {
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
        if let Some(some) = self.r#site.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("site", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_site", &primitive_element)?;
            }
        }
        state.serialize_entry("observer", &self.r#observer)?;
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventSource {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "site")]
            Site,
            #[serde(rename = "_site")]
            SitePrimitiveElement,
            #[serde(rename = "observer")]
            Observer,
            #[serde(rename = "type")]
            Type,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventSource;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventSource")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventSource, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#site: Option<super::super::types::String> = None;
                let mut r#observer: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::Coding>>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Site => {
                            let some = r#site.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SitePrimitiveElement => {
                            let some = r#site.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_site"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Observer => {
                            if r#observer.is_some() {
                                return Err(serde::de::Error::duplicate_field("observer"));
                            }
                            r#observer = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AuditEventSource {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#site,
                    r#observer: r#observer.ok_or(serde::de::Error::missing_field("observer"))?,
                    r#type: r#type.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEventEntityDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::String,
    pub r#value: AuditEventEntityDetailValue,
}
impl serde::ser::Serialize for AuditEventEntityDetail {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        match self.r#value {
            AuditEventEntityDetailValue::String(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueString", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueString", &primitive_element)?;
                }
            }
            AuditEventEntityDetailValue::Base64Binary(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueBase64Binary", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBase64Binary", &primitive_element)?;
                }
            }
            AuditEventEntityDetailValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventEntityDetail {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "valueString")]
            ValueString,
            #[serde(rename = "_valueString")]
            ValueStringPrimitiveElement,
            #[serde(rename = "valueBase64Binary")]
            ValueBase64Binary,
            #[serde(rename = "_valueBase64Binary")]
            ValueBase64BinaryPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventEntityDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntityDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntityDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::String> = None;
                let mut r#value: Option<AuditEventEntityDetailValue> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TypePrimitiveElement => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ValueString => {
                            let r#enum = r#value.get_or_insert(
                                AuditEventEntityDetailValue::String(Default::default()),
                            );
                            if let AuditEventEntityDetailValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                AuditEventEntityDetailValue::String(Default::default()),
                            );
                            if let AuditEventEntityDetailValue::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueString"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                        Field::ValueBase64Binary => {
                            let r#enum = r#value.get_or_insert(
                                AuditEventEntityDetailValue::Base64Binary(Default::default()),
                            );
                            if let AuditEventEntityDetailValue::Base64Binary(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            let r#enum = r#value.get_or_insert(
                                AuditEventEntityDetailValue::Base64Binary(Default::default()),
                            );
                            if let AuditEventEntityDetailValue::Base64Binary(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueBase64Binary",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_value[x]"));
                            }
                        }
                    }
                }
                Ok(AuditEventEntityDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#value: r#value.ok_or(serde::de::Error::missing_field("value"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEventEntity {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#what: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::Coding>>,
    pub r#role: Option<Box<super::super::types::Coding>>,
    pub r#lifecycle: Option<Box<super::super::types::Coding>>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#query: Option<super::super::types::Base64Binary>,
    pub r#detail: Vec<AuditEventEntityDetail>,
}
impl serde::ser::Serialize for AuditEventEntity {
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
        if let Some(some) = self.r#what.as_ref() {
            state.serialize_entry("what", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        if let Some(some) = self.r#lifecycle.as_ref() {
            state.serialize_entry("lifecycle", some)?;
        }
        if !self.r#security_label.is_empty() {
            state.serialize_entry("securityLabel", &self.r#security_label)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#query.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("query", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_query", &primitive_element)?;
            }
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEventEntity {
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
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "what")]
            What,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "lifecycle")]
            Lifecycle,
            #[serde(rename = "securityLabel")]
            SecurityLabel,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "query")]
            Query,
            #[serde(rename = "_query")]
            QueryPrimitiveElement,
            #[serde(rename = "detail")]
            Detail,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEventEntity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEventEntity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEventEntity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#what: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#role: Option<Box<super::super::types::Coding>> = None;
                let mut r#lifecycle: Option<Box<super::super::types::Coding>> = None;
                let mut r#security_label: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#query: Option<super::super::types::Base64Binary> = None;
                let mut r#detail: Option<Vec<AuditEventEntityDetail>> = None;
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
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::What => {
                            if r#what.is_some() {
                                return Err(serde::de::Error::duplicate_field("what"));
                            }
                            r#what = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        Field::Lifecycle => {
                            if r#lifecycle.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycle"));
                            }
                            r#lifecycle = Some(map_access.next_value()?);
                        }
                        Field::SecurityLabel => {
                            if r#security_label.is_some() {
                                return Err(serde::de::Error::duplicate_field("securityLabel"));
                            }
                            r#security_label = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Query => {
                            let some = r#query.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::QueryPrimitiveElement => {
                            let some = r#query.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_query"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Detail => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AuditEventEntity {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#what,
                    r#type,
                    r#role,
                    r#lifecycle,
                    r#security_label: r#security_label.unwrap_or(vec![]),
                    r#name,
                    r#description,
                    r#query,
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct AuditEvent {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::Coding>,
    pub r#subtype: Vec<Box<super::super::types::Coding>>,
    pub r#action: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#recorded: super::super::types::Instant,
    pub r#outcome: Option<super::super::types::Code>,
    pub r#outcome_desc: Option<super::super::types::String>,
    pub r#purpose_of_event: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#agent: Vec<AuditEventAgent>,
    pub r#source: AuditEventSource,
    pub r#entity: Vec<AuditEventEntity>,
}
impl serde::ser::Serialize for AuditEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "AuditEvent")?;
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#subtype.is_empty() {
            state.serialize_entry("subtype", &self.r#subtype)?;
        }
        if let Some(some) = self.r#action.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("action", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_action", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        if let Some(some) = self.r#recorded.value.as_ref() {
            state.serialize_entry("recorded", some)?;
        }
        if self.r#recorded.id.is_some() || !self.r#recorded.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#recorded.id,
                extension: &self.r#recorded.extension,
            };
            state.serialize_entry("_recorded", &primitive_element)?;
        }
        if let Some(some) = self.r#outcome.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("outcome", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_outcome", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#outcome_desc.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("outcomeDesc", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_outcomeDesc", &primitive_element)?;
            }
        }
        if !self.r#purpose_of_event.is_empty() {
            state.serialize_entry("purposeOfEvent", &self.r#purpose_of_event)?;
        }
        if !self.r#agent.is_empty() {
            state.serialize_entry("agent", &self.r#agent)?;
        }
        state.serialize_entry("source", &self.r#source)?;
        if !self.r#entity.is_empty() {
            state.serialize_entry("entity", &self.r#entity)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for AuditEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subtype")]
            Subtype,
            #[serde(rename = "action")]
            Action,
            #[serde(rename = "_action")]
            ActionPrimitiveElement,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "recorded")]
            Recorded,
            #[serde(rename = "_recorded")]
            RecordedPrimitiveElement,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "outcomeDesc")]
            OutcomeDesc,
            #[serde(rename = "_outcomeDesc")]
            OutcomeDescPrimitiveElement,
            #[serde(rename = "purposeOfEvent")]
            PurposeOfEvent,
            #[serde(rename = "agent")]
            Agent,
            #[serde(rename = "source")]
            Source,
            #[serde(rename = "entity")]
            Entity,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AuditEvent;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AuditEvent")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<AuditEvent, V::Error>
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
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#subtype: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#action: Option<super::super::types::Code> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#recorded: Option<super::super::types::Instant> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#outcome_desc: Option<super::super::types::String> = None;
                let mut r#purpose_of_event: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#agent: Option<Vec<AuditEventAgent>> = None;
                let mut r#source: Option<AuditEventSource> = None;
                let mut r#entity: Option<Vec<AuditEventEntity>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRules => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        Field::Contained => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Subtype => {
                            if r#subtype.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtype"));
                            }
                            r#subtype = Some(map_access.next_value()?);
                        }
                        Field::Action => {
                            let some = r#action.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ActionPrimitiveElement => {
                            let some = r#action.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_action"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        Field::Recorded => {
                            let some = r#recorded.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorded"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RecordedPrimitiveElement => {
                            let some = r#recorded.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_recorded"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Outcome => {
                            let some = r#outcome.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OutcomePrimitiveElement => {
                            let some = r#outcome.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_outcome"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::OutcomeDesc => {
                            let some = r#outcome_desc.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcomeDesc"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OutcomeDescPrimitiveElement => {
                            let some = r#outcome_desc.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_outcomeDesc"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PurposeOfEvent => {
                            if r#purpose_of_event.is_some() {
                                return Err(serde::de::Error::duplicate_field("purposeOfEvent"));
                            }
                            r#purpose_of_event = Some(map_access.next_value()?);
                        }
                        Field::Agent => {
                            if r#agent.is_some() {
                                return Err(serde::de::Error::duplicate_field("agent"));
                            }
                            r#agent = Some(map_access.next_value()?);
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(map_access.next_value()?);
                        }
                        Field::Entity => {
                            if r#entity.is_some() {
                                return Err(serde::de::Error::duplicate_field("entity"));
                            }
                            r#entity = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(AuditEvent {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#subtype: r#subtype.unwrap_or(vec![]),
                    r#action,
                    r#period,
                    r#recorded: r#recorded.ok_or(serde::de::Error::missing_field("recorded"))?,
                    r#outcome,
                    r#outcome_desc,
                    r#purpose_of_event: r#purpose_of_event.unwrap_or(vec![]),
                    r#agent: r#agent.unwrap_or(vec![]),
                    r#source: r#source.ok_or(serde::de::Error::missing_field("source"))?,
                    r#entity: r#entity.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
