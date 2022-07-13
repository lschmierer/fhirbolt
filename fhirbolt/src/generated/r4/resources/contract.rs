// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ContractTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractTopic {
    fn default() -> ContractTopic {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractTermTopic {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractTermTopic {
    fn default() -> ContractTermTopic {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractTermOfferAnswerValue {
    Boolean(Box<super::super::types::Boolean>),
    Decimal(Box<super::super::types::Decimal>),
    Integer(Box<super::super::types::Integer>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
    Time(Box<super::super::types::Time>),
    String(Box<super::super::types::String>),
    Uri(Box<super::super::types::Uri>),
    Attachment(Box<super::super::types::Attachment>),
    Coding(Box<super::super::types::Coding>),
    Quantity(Box<super::super::types::Quantity>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractTermOfferAnswerValue {
    fn default() -> ContractTermOfferAnswerValue {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractTermAssetValuedItemEntity {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractTermAssetValuedItemEntity {
    fn default() -> ContractTermAssetValuedItemEntity {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractTermActionOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
impl Default for ContractTermActionOccurrence {
    fn default() -> ContractTermActionOccurrence {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractFriendlyContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractFriendlyContent {
    fn default() -> ContractFriendlyContent {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractLegalContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractLegalContent {
    fn default() -> ContractLegalContent {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractRuleContent {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractRuleContent {
    fn default() -> ContractRuleContent {
        unimplemented!()
    }
}
#[derive(Debug, Clone)]
pub enum ContractLegallyBinding {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for ContractLegallyBinding {
    fn default() -> ContractLegallyBinding {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractContentDefinition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    pub r#publication_date: Option<super::super::types::DateTime>,
    pub r#publication_status: super::super::types::Code,
    pub r#copyright: Option<super::super::types::Markdown>,
}
impl serde::ser::Serialize for ContractContentDefinition {
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
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#sub_type.as_ref() {
            state.serialize_entry("subType", some)?;
        }
        if let Some(some) = self.r#publisher.as_ref() {
            state.serialize_entry("publisher", some)?;
        }
        if let Some(some) = self.r#publication_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("publicationDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_publicationDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#publication_status.value.as_ref() {
            state.serialize_entry("publicationStatus", some)?;
        }
        if self.r#publication_status.id.is_some() || !self.r#publication_status.extension.is_empty()
        {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#publication_status.id,
                extension: &self.r#publication_status.extension,
            };
            state.serialize_entry("_publicationStatus", &primitive_element)?;
        }
        if let Some(some) = self.r#copyright.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("copyright", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_copyright", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractContentDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractContentDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractContentDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractContentDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#publisher: Option<Box<super::super::types::Reference>> = None;
                let mut r#publication_date: Option<super::super::types::DateTime> = None;
                let mut r#publication_status: Option<super::super::types::Code> = None;
                let mut r#copyright: Option<super::super::types::Markdown> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "subType" => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some(map_access.next_value()?);
                        }
                        "publisher" => {
                            if r#publisher.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            r#publisher = Some(map_access.next_value()?);
                        }
                        "publicationDate" => {
                            let some = r#publication_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicationDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_publicationDate" => {
                            let some = r#publication_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_publicationDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "publicationStatus" => {
                            let some = r#publication_status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicationStatus"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_publicationStatus" => {
                            let some = r#publication_status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_publicationStatus",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "copyright" => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("copyright"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_copyright" => {
                            let some = r#copyright.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_copyright"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
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
                                    "type",
                                    "sub_type",
                                    "publisher",
                                    "publication_date",
                                    "publication_status",
                                    "copyright",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractContentDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#sub_type,
                    r#publisher,
                    r#publication_date,
                    r#publication_status: r#publication_status
                        .ok_or(serde::de::Error::missing_field("publication_status"))?,
                    r#copyright,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermSecurityLabel {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number: Vec<super::super::types::UnsignedInt>,
    pub r#classification: Box<super::super::types::Coding>,
    pub r#category: Vec<Box<super::super::types::Coding>>,
    pub r#control: Vec<Box<super::super::types::Coding>>,
}
impl serde::ser::Serialize for ContractTermSecurityLabel {
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
        if !self.r#number.is_empty() {
            let values: Vec<_> = self.r#number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("number", &values)?;
            }
            let requires_elements = self
                .r#number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#number
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
                state.serialize_entry("_number", &primitive_elements)?;
            }
        }
        state.serialize_entry("classification", &self.r#classification)?;
        if !self.r#category.is_empty() {
            state.serialize_entry("category", &self.r#category)?;
        }
        if !self.r#control.is_empty() {
            state.serialize_entry("control", &self.r#control)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermSecurityLabel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermSecurityLabel;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermSecurityLabel")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermSecurityLabel, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#number: Option<Vec<super::super::types::UnsignedInt>> = None;
                let mut r#classification: Option<Box<super::super::types::Coding>> = None;
                let mut r#category: Option<Vec<Box<super::super::types::Coding>>> = None;
                let mut r#control: Option<Vec<Box<super::super::types::Coding>>> = None;
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
                        "number" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_number" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_number"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "classification" => {
                            if r#classification.is_some() {
                                return Err(serde::de::Error::duplicate_field("classification"));
                            }
                            r#classification = Some(map_access.next_value()?);
                        }
                        "category" => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        "control" => {
                            if r#control.is_some() {
                                return Err(serde::de::Error::duplicate_field("control"));
                            }
                            r#control = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "number",
                                    "classification",
                                    "category",
                                    "control",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermSecurityLabel {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number: r#number.unwrap_or(vec![]),
                    r#classification: r#classification
                        .ok_or(serde::de::Error::missing_field("classification"))?,
                    r#category: r#category.unwrap_or(vec![]),
                    r#control: r#control.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermOfferParty {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    pub r#role: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for ContractTermOfferParty {
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
        if !self.r#reference.is_empty() {
            state.serialize_entry("reference", &self.r#reference)?;
        }
        state.serialize_entry("role", &self.r#role)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOfferParty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOfferParty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferParty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOfferParty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "reference", "role"],
                            ))
                        }
                    }
                }
                Ok(ContractTermOfferParty {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.unwrap_or(vec![]),
                    r#role: r#role.ok_or(serde::de::Error::missing_field("role"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermOfferAnswer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: ContractTermOfferAnswerValue,
}
impl serde::ser::Serialize for ContractTermOfferAnswer {
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
        match self.r#value {
            ContractTermOfferAnswerValue::Boolean(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueBoolean", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueBoolean", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::Decimal(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDecimal", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDecimal", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::Integer(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueInteger", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueInteger", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::Date(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDate", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDate", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::DateTime(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueDateTime", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueDateTime", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::Time(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueTime", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueTime", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::String(ref value) => {
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
            ContractTermOfferAnswerValue::Uri(ref value) => {
                if let Some(some) = value.value.as_ref() {
                    state.serialize_entry("valueUri", some)?;
                }
                if value.id.is_some() || !value.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: &value.id,
                        extension: &value.extension,
                    };
                    state.serialize_entry("_valueUri", &primitive_element)?;
                }
            }
            ContractTermOfferAnswerValue::Attachment(ref value) => {
                state.serialize_entry("valueAttachment", value)?;
            }
            ContractTermOfferAnswerValue::Coding(ref value) => {
                state.serialize_entry("valueCoding", value)?;
            }
            ContractTermOfferAnswerValue::Quantity(ref value) => {
                state.serialize_entry("valueQuantity", value)?;
            }
            ContractTermOfferAnswerValue::Reference(ref value) => {
                state.serialize_entry("valueReference", value)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOfferAnswer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOfferAnswer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferAnswer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOfferAnswer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#value: Option<ContractTermOfferAnswerValue> = None;
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
                        "valueBoolean" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Boolean(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Boolean(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueBoolean" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Boolean(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Boolean(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueBoolean"));
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
                        "valueDecimal" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Decimal(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Decimal(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDecimal" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Decimal(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Decimal(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDecimal"));
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
                        "valueInteger" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Integer(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Integer(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueInteger" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::Integer(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::Integer(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueInteger"));
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
                        "valueDate" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Date(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDate" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Date(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueDate"));
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
                        "valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::DateTime(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueDateTime" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::DateTime(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_valueDateTime",
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
                        "valueTime" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Time(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Time(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueTime" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Time(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Time(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueTime"));
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
                        "valueString" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::String(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueString" => {
                            let r#enum = r#value.get_or_insert(
                                ContractTermOfferAnswerValue::String(Default::default()),
                            );
                            if let ContractTermOfferAnswerValue::String(variant) = r#enum {
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
                        "valueUri" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Uri(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Uri(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("value[x]"));
                            }
                        }
                        "_valueUri" => {
                            let r#enum = r#value.get_or_insert(ContractTermOfferAnswerValue::Uri(
                                Default::default(),
                            ));
                            if let ContractTermOfferAnswerValue::Uri(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueUri"));
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
                        "valueAttachment" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some(ContractTermOfferAnswerValue::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        "valueCoding" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(ContractTermOfferAnswerValue::Coding(
                                map_access.next_value()?,
                            ));
                        }
                        "valueQuantity" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some(ContractTermOfferAnswerValue::Quantity(
                                map_access.next_value()?,
                            ));
                        }
                        "valueReference" => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some(ContractTermOfferAnswerValue::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "value"],
                            ))
                        }
                    }
                }
                Ok(ContractTermOfferAnswer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#value: r#value.ok_or(serde::de::Error::missing_field("value"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermOffer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#party: Vec<ContractTermOfferParty>,
    pub r#topic: Option<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#decision: Option<Box<super::super::types::CodeableConcept>>,
    pub r#decision_mode: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#answer: Vec<ContractTermOfferAnswer>,
    pub r#text: Option<super::super::types::String>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermOffer {
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#party.is_empty() {
            state.serialize_entry("party", &self.r#party)?;
        }
        if let Some(some) = self.r#topic.as_ref() {
            state.serialize_entry("topic", some)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#decision.as_ref() {
            state.serialize_entry("decision", some)?;
        }
        if !self.r#decision_mode.is_empty() {
            state.serialize_entry("decisionMode", &self.r#decision_mode)?;
        }
        if !self.r#answer.is_empty() {
            state.serialize_entry("answer", &self.r#answer)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#link_id.is_empty() {
            let values: Vec<_> = self.r#link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("linkId", &values)?;
            }
            let requires_elements = self
                .r#link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#link_id
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
                state.serialize_entry("_linkId", &primitive_elements)?;
            }
        }
        if !self.r#security_label_number.is_empty() {
            let values: Vec<_> = self
                .r#security_label_number
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("securityLabelNumber", &values)?;
            }
            let requires_elements = self
                .r#security_label_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#security_label_number
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
                state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermOffer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermOffer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOffer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermOffer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#party: Option<Vec<ContractTermOfferParty>> = None;
                let mut r#topic: Option<Box<super::super::types::Reference>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#decision: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#decision_mode: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#answer: Option<Vec<ContractTermOfferAnswer>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
                    None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "party" => {
                            if r#party.is_some() {
                                return Err(serde::de::Error::duplicate_field("party"));
                            }
                            r#party = Some(map_access.next_value()?);
                        }
                        "topic" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic"));
                            }
                            r#topic = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "decision" => {
                            if r#decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            r#decision = Some(map_access.next_value()?);
                        }
                        "decisionMode" => {
                            if r#decision_mode.is_some() {
                                return Err(serde::de::Error::duplicate_field("decisionMode"));
                            }
                            r#decision_mode = Some(map_access.next_value()?);
                        }
                        "answer" => {
                            if r#answer.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            r#answer = Some(map_access.next_value()?);
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "linkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("linkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_linkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_linkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "securityLabelNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "securityLabelNumber",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_securityLabelNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_securityLabelNumber",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "party",
                                    "topic",
                                    "type",
                                    "decision",
                                    "decision_mode",
                                    "answer",
                                    "text",
                                    "link_id",
                                    "security_label_number",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermOffer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#party: r#party.unwrap_or(vec![]),
                    r#topic,
                    r#type,
                    r#decision,
                    r#decision_mode: r#decision_mode.unwrap_or(vec![]),
                    r#answer: r#answer.unwrap_or(vec![]),
                    r#text,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermAssetContext {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Option<Box<super::super::types::Reference>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<super::super::types::String>,
}
impl serde::ser::Serialize for ContractTermAssetContext {
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
        if let Some(some) = self.r#reference.as_ref() {
            state.serialize_entry("reference", some)?;
        }
        if !self.r#code.is_empty() {
            state.serialize_entry("code", &self.r#code)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAssetContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAssetContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetContext")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAssetContext, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#text: Option<super::super::types::String> = None;
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
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        "code" => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
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
                                    "reference",
                                    "code",
                                    "text",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermAssetContext {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference,
                    r#code: r#code.unwrap_or(vec![]),
                    r#text,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermAssetValuedItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#entity: Option<ContractTermAssetValuedItemEntity>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#effective_time: Option<super::super::types::DateTime>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#points: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#payment: Option<super::super::types::String>,
    pub r#payment_date: Option<super::super::types::DateTime>,
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    pub r#recipient: Option<Box<super::super::types::Reference>>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAssetValuedItem {
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
        if let Some(some) = self.r#entity.as_ref() {
            match some {
                ContractTermAssetValuedItemEntity::CodeableConcept(ref value) => {
                    state.serialize_entry("entityCodeableConcept", value)?;
                }
                ContractTermAssetValuedItemEntity::Reference(ref value) => {
                    state.serialize_entry("entityReference", value)?;
                }
            }
        }
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#effective_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("effectiveTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_effectiveTime", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#unit_price.as_ref() {
            state.serialize_entry("unitPrice", some)?;
        }
        if let Some(some) = self.r#factor.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("factor", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_factor", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#points.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("points", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_points", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if let Some(some) = self.r#payment.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("payment", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_payment", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#payment_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paymentDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_paymentDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#responsible.as_ref() {
            state.serialize_entry("responsible", some)?;
        }
        if let Some(some) = self.r#recipient.as_ref() {
            state.serialize_entry("recipient", some)?;
        }
        if !self.r#link_id.is_empty() {
            let values: Vec<_> = self.r#link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("linkId", &values)?;
            }
            let requires_elements = self
                .r#link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#link_id
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
                state.serialize_entry("_linkId", &primitive_elements)?;
            }
        }
        if !self.r#security_label_number.is_empty() {
            let values: Vec<_> = self
                .r#security_label_number
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("securityLabelNumber", &values)?;
            }
            let requires_elements = self
                .r#security_label_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#security_label_number
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
                state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAssetValuedItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAssetValuedItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetValuedItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ContractTermAssetValuedItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#entity: Option<ContractTermAssetValuedItemEntity> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#effective_time: Option<super::super::types::DateTime> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#points: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#payment: Option<super::super::types::String> = None;
                let mut r#payment_date: Option<super::super::types::DateTime> = None;
                let mut r#responsible: Option<Box<super::super::types::Reference>> = None;
                let mut r#recipient: Option<Box<super::super::types::Reference>> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
                    None;
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
                        "entityCodeableConcept" => {
                            if r#entity.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "entityCodeableConcept",
                                ));
                            }
                            r#entity = Some(ContractTermAssetValuedItemEntity::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        "entityReference" => {
                            if r#entity.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityReference"));
                            }
                            r#entity = Some(ContractTermAssetValuedItemEntity::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "effectiveTime" => {
                            let some = r#effective_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_effectiveTime" => {
                            let some = r#effective_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_effectiveTime"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "unitPrice" => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        "factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_factor" => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_factor"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "points" => {
                            let some = r#points.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("points"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_points" => {
                            let some = r#points.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_points"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "net" => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        "payment" => {
                            let some = r#payment.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_payment" => {
                            let some = r#payment.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_payment"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "paymentDate" => {
                            let some = r#payment_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_paymentDate" => {
                            let some = r#payment_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_paymentDate"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "responsible" => {
                            if r#responsible.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsible"));
                            }
                            r#responsible = Some(map_access.next_value()?);
                        }
                        "recipient" => {
                            if r#recipient.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            r#recipient = Some(map_access.next_value()?);
                        }
                        "linkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("linkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_linkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_linkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "securityLabelNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "securityLabelNumber",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_securityLabelNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_securityLabelNumber",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "entity",
                                    "identifier",
                                    "effective_time",
                                    "quantity",
                                    "unit_price",
                                    "factor",
                                    "points",
                                    "net",
                                    "payment",
                                    "payment_date",
                                    "responsible",
                                    "recipient",
                                    "link_id",
                                    "security_label_number",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermAssetValuedItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#entity,
                    r#identifier,
                    r#effective_time,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#points,
                    r#net,
                    r#payment,
                    r#payment_date,
                    r#responsible,
                    r#recipient,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermAsset {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type_reference: Vec<Box<super::super::types::Reference>>,
    pub r#subtype: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#relationship: Option<Box<super::super::types::Coding>>,
    pub r#context: Vec<ContractTermAssetContext>,
    pub r#condition: Option<super::super::types::String>,
    pub r#period_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Vec<Box<super::super::types::Period>>,
    pub r#use_period: Vec<Box<super::super::types::Period>>,
    pub r#text: Option<super::super::types::String>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#answer: Vec<ContractTermOfferAnswer>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
    pub r#valued_item: Vec<ContractTermAssetValuedItem>,
}
impl serde::ser::Serialize for ContractTermAsset {
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
        if let Some(some) = self.r#scope.as_ref() {
            state.serialize_entry("scope", some)?;
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if !self.r#type_reference.is_empty() {
            state.serialize_entry("typeReference", &self.r#type_reference)?;
        }
        if !self.r#subtype.is_empty() {
            state.serialize_entry("subtype", &self.r#subtype)?;
        }
        if let Some(some) = self.r#relationship.as_ref() {
            state.serialize_entry("relationship", some)?;
        }
        if !self.r#context.is_empty() {
            state.serialize_entry("context", &self.r#context)?;
        }
        if let Some(some) = self.r#condition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("condition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_condition", &primitive_element)?;
            }
        }
        if !self.r#period_type.is_empty() {
            state.serialize_entry("periodType", &self.r#period_type)?;
        }
        if !self.r#period.is_empty() {
            state.serialize_entry("period", &self.r#period)?;
        }
        if !self.r#use_period.is_empty() {
            state.serialize_entry("usePeriod", &self.r#use_period)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#link_id.is_empty() {
            let values: Vec<_> = self.r#link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("linkId", &values)?;
            }
            let requires_elements = self
                .r#link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#link_id
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
                state.serialize_entry("_linkId", &primitive_elements)?;
            }
        }
        if !self.r#answer.is_empty() {
            state.serialize_entry("answer", &self.r#answer)?;
        }
        if !self.r#security_label_number.is_empty() {
            let values: Vec<_> = self
                .r#security_label_number
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("securityLabelNumber", &values)?;
            }
            let requires_elements = self
                .r#security_label_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#security_label_number
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
                state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
            }
        }
        if !self.r#valued_item.is_empty() {
            state.serialize_entry("valuedItem", &self.r#valued_item)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAsset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAsset;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAsset")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAsset, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#scope: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#type_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#subtype: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#relationship: Option<Box<super::super::types::Coding>> = None;
                let mut r#context: Option<Vec<ContractTermAssetContext>> = None;
                let mut r#condition: Option<super::super::types::String> = None;
                let mut r#period_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#period: Option<Vec<Box<super::super::types::Period>>> = None;
                let mut r#use_period: Option<Vec<Box<super::super::types::Period>>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#answer: Option<Vec<ContractTermOfferAnswer>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
                    None;
                let mut r#valued_item: Option<Vec<ContractTermAssetValuedItem>> = None;
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
                        "scope" => {
                            if r#scope.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            r#scope = Some(map_access.next_value()?);
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "typeReference" => {
                            if r#type_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeReference"));
                            }
                            r#type_reference = Some(map_access.next_value()?);
                        }
                        "subtype" => {
                            if r#subtype.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtype"));
                            }
                            r#subtype = Some(map_access.next_value()?);
                        }
                        "relationship" => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            r#relationship = Some(map_access.next_value()?);
                        }
                        "context" => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        "condition" => {
                            let some = r#condition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("condition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_condition" => {
                            let some = r#condition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_condition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "periodType" => {
                            if r#period_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("periodType"));
                            }
                            r#period_type = Some(map_access.next_value()?);
                        }
                        "period" => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                        "usePeriod" => {
                            if r#use_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("usePeriod"));
                            }
                            r#use_period = Some(map_access.next_value()?);
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "linkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("linkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_linkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_linkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "answer" => {
                            if r#answer.is_some() {
                                return Err(serde::de::Error::duplicate_field("answer"));
                            }
                            r#answer = Some(map_access.next_value()?);
                        }
                        "securityLabelNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "securityLabelNumber",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_securityLabelNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_securityLabelNumber",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "valuedItem" => {
                            if r#valued_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuedItem"));
                            }
                            r#valued_item = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "scope",
                                    "type",
                                    "type_reference",
                                    "subtype",
                                    "relationship",
                                    "context",
                                    "condition",
                                    "period_type",
                                    "period",
                                    "use_period",
                                    "text",
                                    "link_id",
                                    "answer",
                                    "security_label_number",
                                    "valued_item",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermAsset {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#scope,
                    r#type: r#type.unwrap_or(vec![]),
                    r#type_reference: r#type_reference.unwrap_or(vec![]),
                    r#subtype: r#subtype.unwrap_or(vec![]),
                    r#relationship,
                    r#context: r#context.unwrap_or(vec![]),
                    r#condition,
                    r#period_type: r#period_type.unwrap_or(vec![]),
                    r#period: r#period.unwrap_or(vec![]),
                    r#use_period: r#use_period.unwrap_or(vec![]),
                    r#text,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#answer: r#answer.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                    r#valued_item: r#valued_item.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermActionSubject {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Vec<Box<super::super::types::Reference>>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ContractTermActionSubject {
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
        if !self.r#reference.is_empty() {
            state.serialize_entry("reference", &self.r#reference)?;
        }
        if let Some(some) = self.r#role.as_ref() {
            state.serialize_entry("role", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermActionSubject {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermActionSubject;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermActionSubject")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermActionSubject, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        "reference" => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(map_access.next_value()?);
                        }
                        "role" => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "reference", "role"],
                            ))
                        }
                    }
                }
                Ok(ContractTermActionSubject {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.unwrap_or(vec![]),
                    r#role,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTermAction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#subject: Vec<ContractTermActionSubject>,
    pub r#intent: Box<super::super::types::CodeableConcept>,
    pub r#link_id: Vec<super::super::types::String>,
    pub r#status: Box<super::super::types::CodeableConcept>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#context_link_id: Vec<super::super::types::String>,
    pub r#occurrence: Option<ContractTermActionOccurrence>,
    pub r#requester: Vec<Box<super::super::types::Reference>>,
    pub r#requester_link_id: Vec<super::super::types::String>,
    pub r#performer_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#performer_role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#performer_link_id: Vec<super::super::types::String>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#reason: Vec<super::super::types::String>,
    pub r#reason_link_id: Vec<super::super::types::String>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#security_label_number: Vec<super::super::types::UnsignedInt>,
}
impl serde::ser::Serialize for ContractTermAction {
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
        if let Some(some) = self.r#do_not_perform.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("doNotPerform", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_doNotPerform", &primitive_element)?;
            }
        }
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#subject.is_empty() {
            state.serialize_entry("subject", &self.r#subject)?;
        }
        state.serialize_entry("intent", &self.r#intent)?;
        if !self.r#link_id.is_empty() {
            let values: Vec<_> = self.r#link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("linkId", &values)?;
            }
            let requires_elements = self
                .r#link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#link_id
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
                state.serialize_entry("_linkId", &primitive_elements)?;
            }
        }
        state.serialize_entry("status", &self.r#status)?;
        if let Some(some) = self.r#context.as_ref() {
            state.serialize_entry("context", some)?;
        }
        if !self.r#context_link_id.is_empty() {
            let values: Vec<_> = self.r#context_link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("contextLinkId", &values)?;
            }
            let requires_elements = self
                .r#context_link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#context_link_id
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
                state.serialize_entry("_contextLinkId", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#occurrence.as_ref() {
            match some {
                ContractTermActionOccurrence::DateTime(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("occurrenceDateTime", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_occurrenceDateTime", &primitive_element)?;
                    }
                }
                ContractTermActionOccurrence::Period(ref value) => {
                    state.serialize_entry("occurrencePeriod", value)?;
                }
                ContractTermActionOccurrence::Timing(ref value) => {
                    state.serialize_entry("occurrenceTiming", value)?;
                }
            }
        }
        if !self.r#requester.is_empty() {
            state.serialize_entry("requester", &self.r#requester)?;
        }
        if !self.r#requester_link_id.is_empty() {
            let values: Vec<_> = self.r#requester_link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("requesterLinkId", &values)?;
            }
            let requires_elements = self
                .r#requester_link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#requester_link_id
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
                state.serialize_entry("_requesterLinkId", &primitive_elements)?;
            }
        }
        if !self.r#performer_type.is_empty() {
            state.serialize_entry("performerType", &self.r#performer_type)?;
        }
        if let Some(some) = self.r#performer_role.as_ref() {
            state.serialize_entry("performerRole", some)?;
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
        }
        if !self.r#performer_link_id.is_empty() {
            let values: Vec<_> = self.r#performer_link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("performerLinkId", &values)?;
            }
            let requires_elements = self
                .r#performer_link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#performer_link_id
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
                state.serialize_entry("_performerLinkId", &primitive_elements)?;
            }
        }
        if !self.r#reason_code.is_empty() {
            state.serialize_entry("reasonCode", &self.r#reason_code)?;
        }
        if !self.r#reason_reference.is_empty() {
            state.serialize_entry("reasonReference", &self.r#reason_reference)?;
        }
        if !self.r#reason.is_empty() {
            let values: Vec<_> = self.r#reason.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("reason", &values)?;
            }
            let requires_elements = self
                .r#reason
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#reason
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
                state.serialize_entry("_reason", &primitive_elements)?;
            }
        }
        if !self.r#reason_link_id.is_empty() {
            let values: Vec<_> = self.r#reason_link_id.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("reasonLinkId", &values)?;
            }
            let requires_elements = self
                .r#reason_link_id
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#reason_link_id
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
                state.serialize_entry("_reasonLinkId", &primitive_elements)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if !self.r#security_label_number.is_empty() {
            let values: Vec<_> = self
                .r#security_label_number
                .iter()
                .map(|v| &v.value)
                .collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("securityLabelNumber", &values)?;
            }
            let requires_elements = self
                .r#security_label_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#security_label_number
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
                state.serialize_entry("_securityLabelNumber", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTermAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTermAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAction")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTermAction, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Vec<ContractTermActionSubject>> = None;
                let mut r#intent: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#context: Option<Box<super::super::types::Reference>> = None;
                let mut r#context_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#occurrence: Option<ContractTermActionOccurrence> = None;
                let mut r#requester: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#requester_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#performer_type: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#performer_role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#reason: Option<Vec<super::super::types::String>> = None;
                let mut r#reason_link_id: Option<Vec<super::super::types::String>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#security_label_number: Option<Vec<super::super::types::UnsignedInt>> =
                    None;
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
                        "doNotPerform" => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("doNotPerform"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_doNotPerform" => {
                            let some = r#do_not_perform.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "intent" => {
                            if r#intent.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            r#intent = Some(map_access.next_value()?);
                        }
                        "linkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("linkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_linkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_linkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "status" => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some(map_access.next_value()?);
                        }
                        "context" => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(map_access.next_value()?);
                        }
                        "contextLinkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#context_link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("contextLinkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_contextLinkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#context_link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_contextLinkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "occurrenceDateTime" => {
                            let r#enum = r#occurrence.get_or_insert(
                                ContractTermActionOccurrence::DateTime(Default::default()),
                            );
                            if let ContractTermActionOccurrence::DateTime(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("occurrence[x]"));
                            }
                        }
                        "_occurrenceDateTime" => {
                            let r#enum = r#occurrence.get_or_insert(
                                ContractTermActionOccurrence::DateTime(Default::default()),
                            );
                            if let ContractTermActionOccurrence::DateTime(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_occurrenceDateTime",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_occurrence[x]"));
                            }
                        }
                        "occurrencePeriod" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence = Some(ContractTermActionOccurrence::Period(
                                map_access.next_value()?,
                            ));
                        }
                        "occurrenceTiming" => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence = Some(ContractTermActionOccurrence::Timing(
                                map_access.next_value()?,
                            ));
                        }
                        "requester" => {
                            if r#requester.is_some() {
                                return Err(serde::de::Error::duplicate_field("requester"));
                            }
                            r#requester = Some(map_access.next_value()?);
                        }
                        "requesterLinkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#requester_link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("requesterLinkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_requesterLinkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#requester_link_id
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_requesterLinkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "performerType" => {
                            if r#performer_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("performerType"));
                            }
                            r#performer_type = Some(map_access.next_value()?);
                        }
                        "performerRole" => {
                            if r#performer_role.is_some() {
                                return Err(serde::de::Error::duplicate_field("performerRole"));
                            }
                            r#performer_role = Some(map_access.next_value()?);
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "performerLinkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#performer_link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("performerLinkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_performerLinkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#performer_link_id
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_performerLinkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "reasonCode" => {
                            if r#reason_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonCode"));
                            }
                            r#reason_code = Some(map_access.next_value()?);
                        }
                        "reasonReference" => {
                            if r#reason_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reasonReference"));
                            }
                            r#reason_reference = Some(map_access.next_value()?);
                        }
                        "reason" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#reason.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_reason" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#reason.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_reason"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "reasonLinkId" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#reason_link_id.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("reasonLinkId"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_reasonLinkId" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#reason_link_id.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_reasonLinkId"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "securityLabelNumber" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field(
                                    "securityLabelNumber",
                                ));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_securityLabelNumber" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#security_label_number
                                .get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field(
                                    "_securityLabelNumber",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "do_not_perform",
                                    "type",
                                    "subject",
                                    "intent",
                                    "link_id",
                                    "status",
                                    "context",
                                    "context_link_id",
                                    "occurrence",
                                    "requester",
                                    "requester_link_id",
                                    "performer_type",
                                    "performer_role",
                                    "performer",
                                    "performer_link_id",
                                    "reason_code",
                                    "reason_reference",
                                    "reason",
                                    "reason_link_id",
                                    "note",
                                    "security_label_number",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTermAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#do_not_perform,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#intent: r#intent.ok_or(serde::de::Error::missing_field("intent"))?,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#context,
                    r#context_link_id: r#context_link_id.unwrap_or(vec![]),
                    r#occurrence,
                    r#requester: r#requester.unwrap_or(vec![]),
                    r#requester_link_id: r#requester_link_id.unwrap_or(vec![]),
                    r#performer_type: r#performer_type.unwrap_or(vec![]),
                    r#performer_role,
                    r#performer,
                    r#performer_link_id: r#performer_link_id.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#reason_link_id: r#reason_link_id.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractTerm {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#applies: Option<Box<super::super::types::Period>>,
    pub r#topic: Option<ContractTermTopic>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<super::super::types::String>,
    pub r#security_label: Vec<ContractTermSecurityLabel>,
    pub r#offer: ContractTermOffer,
    pub r#asset: Vec<ContractTermAsset>,
    pub r#action: Vec<ContractTermAction>,
    pub r#group: Vec<ContractTerm>,
}
impl serde::ser::Serialize for ContractTerm {
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        if let Some(some) = self.r#issued.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("issued", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_issued", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#applies.as_ref() {
            state.serialize_entry("applies", some)?;
        }
        if let Some(some) = self.r#topic.as_ref() {
            match some {
                ContractTermTopic::CodeableConcept(ref value) => {
                    state.serialize_entry("topicCodeableConcept", value)?;
                }
                ContractTermTopic::Reference(ref value) => {
                    state.serialize_entry("topicReference", value)?;
                }
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if let Some(some) = self.r#sub_type.as_ref() {
            state.serialize_entry("subType", some)?;
        }
        if let Some(some) = self.r#text.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("text", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_text", &primitive_element)?;
            }
        }
        if !self.r#security_label.is_empty() {
            state.serialize_entry("securityLabel", &self.r#security_label)?;
        }
        state.serialize_entry("offer", &self.r#offer)?;
        if !self.r#asset.is_empty() {
            state.serialize_entry("asset", &self.r#asset)?;
        }
        if !self.r#action.is_empty() {
            state.serialize_entry("action", &self.r#action)?;
        }
        if !self.r#group.is_empty() {
            state.serialize_entry("group", &self.r#group)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractTerm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractTerm;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTerm")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractTerm, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#issued: Option<super::super::types::DateTime> = None;
                let mut r#applies: Option<Box<super::super::types::Period>> = None;
                let mut r#topic: Option<ContractTermTopic> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#security_label: Option<Vec<ContractTermSecurityLabel>> = None;
                let mut r#offer: Option<ContractTermOffer> = None;
                let mut r#asset: Option<Vec<ContractTermAsset>> = None;
                let mut r#action: Option<Vec<ContractTermAction>> = None;
                let mut r#group: Option<Vec<ContractTerm>> = None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("issued"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_issued"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "applies" => {
                            if r#applies.is_some() {
                                return Err(serde::de::Error::duplicate_field("applies"));
                            }
                            r#applies = Some(map_access.next_value()?);
                        }
                        "topicCodeableConcept" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "topicCodeableConcept",
                                ));
                            }
                            r#topic =
                                Some(ContractTermTopic::CodeableConcept(map_access.next_value()?));
                        }
                        "topicReference" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topicReference"));
                            }
                            r#topic = Some(ContractTermTopic::Reference(map_access.next_value()?));
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "subType" => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some(map_access.next_value()?);
                        }
                        "text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_text" => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_text"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "securityLabel" => {
                            if r#security_label.is_some() {
                                return Err(serde::de::Error::duplicate_field("securityLabel"));
                            }
                            r#security_label = Some(map_access.next_value()?);
                        }
                        "offer" => {
                            if r#offer.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            r#offer = Some(map_access.next_value()?);
                        }
                        "asset" => {
                            if r#asset.is_some() {
                                return Err(serde::de::Error::duplicate_field("asset"));
                            }
                            r#asset = Some(map_access.next_value()?);
                        }
                        "action" => {
                            if r#action.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            r#action = Some(map_access.next_value()?);
                        }
                        "group" => {
                            if r#group.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            r#group = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "issued",
                                    "applies",
                                    "topic",
                                    "type",
                                    "sub_type",
                                    "text",
                                    "security_label",
                                    "offer",
                                    "asset",
                                    "action",
                                    "group",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractTerm {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#issued,
                    r#applies,
                    r#topic,
                    r#type,
                    r#sub_type,
                    r#text,
                    r#security_label: r#security_label.unwrap_or(vec![]),
                    r#offer: r#offer.ok_or(serde::de::Error::missing_field("offer"))?,
                    r#asset: r#asset.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                    r#group: r#group.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractSigner {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::Coding>,
    pub r#party: Box<super::super::types::Reference>,
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for ContractSigner {
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
        state.serialize_entry("type", &self.r#type)?;
        state.serialize_entry("party", &self.r#party)?;
        if !self.r#signature.is_empty() {
            state.serialize_entry("signature", &self.r#signature)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractSigner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractSigner;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractSigner")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractSigner, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::Coding>> = None;
                let mut r#party: Option<Box<super::super::types::Reference>> = None;
                let mut r#signature: Option<Vec<Box<super::super::types::Signature>>> = None;
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
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "party" => {
                            if r#party.is_some() {
                                return Err(serde::de::Error::duplicate_field("party"));
                            }
                            r#party = Some(map_access.next_value()?);
                        }
                        "signature" => {
                            if r#signature.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            r#signature = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "party",
                                    "signature",
                                ],
                            ))
                        }
                    }
                }
                Ok(ContractSigner {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#party: r#party.ok_or(serde::de::Error::missing_field("party"))?,
                    r#signature: r#signature.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractFriendly {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: ContractFriendlyContent,
}
impl serde::ser::Serialize for ContractFriendly {
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
        match self.r#content {
            ContractFriendlyContent::Attachment(ref value) => {
                state.serialize_entry("contentAttachment", value)?;
            }
            ContractFriendlyContent::Reference(ref value) => {
                state.serialize_entry("contentReference", value)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractFriendly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractFriendly;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractFriendly")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractFriendly, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractFriendlyContent> = None;
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
                        "contentAttachment" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content = Some(ContractFriendlyContent::Attachment(
                                map_access.next_value()?,
                            ));
                        }
                        "contentReference" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content =
                                Some(ContractFriendlyContent::Reference(map_access.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "content"],
                            ))
                        }
                    }
                }
                Ok(ContractFriendly {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: r#content.ok_or(serde::de::Error::missing_field("content"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractLegal {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: ContractLegalContent,
}
impl serde::ser::Serialize for ContractLegal {
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
        match self.r#content {
            ContractLegalContent::Attachment(ref value) => {
                state.serialize_entry("contentAttachment", value)?;
            }
            ContractLegalContent::Reference(ref value) => {
                state.serialize_entry("contentReference", value)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractLegal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractLegal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractLegal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractLegal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractLegalContent> = None;
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
                        "contentAttachment" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content =
                                Some(ContractLegalContent::Attachment(map_access.next_value()?));
                        }
                        "contentReference" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content =
                                Some(ContractLegalContent::Reference(map_access.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "content"],
                            ))
                        }
                    }
                }
                Ok(ContractLegal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: r#content.ok_or(serde::de::Error::missing_field("content"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ContractRule {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: ContractRuleContent,
}
impl serde::ser::Serialize for ContractRule {
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
        match self.r#content {
            ContractRuleContent::Attachment(ref value) => {
                state.serialize_entry("contentAttachment", value)?;
            }
            ContractRuleContent::Reference(ref value) => {
                state.serialize_entry("contentReference", value)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ContractRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContractRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractRule")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ContractRule, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#content: Option<ContractRuleContent> = None;
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
                        "contentAttachment" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content =
                                Some(ContractRuleContent::Attachment(map_access.next_value()?));
                        }
                        "contentReference" => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content =
                                Some(ContractRuleContent::Reference(map_access.next_value()?));
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "content"],
                            ))
                        }
                    }
                }
                Ok(ContractRule {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: r#content.ok_or(serde::de::Error::missing_field("content"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Contract {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#status: Option<super::super::types::Code>,
    pub r#legal_state: Option<Box<super::super::types::CodeableConcept>>,
    pub r#instantiates_canonical: Option<Box<super::super::types::Reference>>,
    pub r#instantiates_uri: Option<super::super::types::Uri>,
    pub r#content_derivative: Option<Box<super::super::types::CodeableConcept>>,
    pub r#issued: Option<super::super::types::DateTime>,
    pub r#applies: Option<Box<super::super::types::Period>>,
    pub r#expiration_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    pub r#authority: Vec<Box<super::super::types::Reference>>,
    pub r#domain: Vec<Box<super::super::types::Reference>>,
    pub r#site: Vec<Box<super::super::types::Reference>>,
    pub r#name: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#scope: Option<Box<super::super::types::CodeableConcept>>,
    pub r#topic: Option<ContractTopic>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#content_definition: Option<ContractContentDefinition>,
    pub r#term: Vec<ContractTerm>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    pub r#signer: Vec<ContractSigner>,
    pub r#friendly: Vec<ContractFriendly>,
    pub r#legal: Vec<ContractLegal>,
    pub r#rule: Vec<ContractRule>,
    pub r#legally_binding: Option<ContractLegallyBinding>,
}
impl serde::ser::Serialize for Contract {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Contract")?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("version", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#legal_state.as_ref() {
            state.serialize_entry("legalState", some)?;
        }
        if let Some(some) = self.r#instantiates_canonical.as_ref() {
            state.serialize_entry("instantiatesCanonical", some)?;
        }
        if let Some(some) = self.r#instantiates_uri.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("instantiatesUri", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_instantiatesUri", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#content_derivative.as_ref() {
            state.serialize_entry("contentDerivative", some)?;
        }
        if let Some(some) = self.r#issued.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("issued", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_issued", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#applies.as_ref() {
            state.serialize_entry("applies", some)?;
        }
        if let Some(some) = self.r#expiration_type.as_ref() {
            state.serialize_entry("expirationType", some)?;
        }
        if !self.r#subject.is_empty() {
            state.serialize_entry("subject", &self.r#subject)?;
        }
        if !self.r#authority.is_empty() {
            state.serialize_entry("authority", &self.r#authority)?;
        }
        if !self.r#domain.is_empty() {
            state.serialize_entry("domain", &self.r#domain)?;
        }
        if !self.r#site.is_empty() {
            state.serialize_entry("site", &self.r#site)?;
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
        if let Some(some) = self.r#title.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("title", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_title", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#subtitle.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subtitle", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subtitle", &primitive_element)?;
            }
        }
        if !self.r#alias.is_empty() {
            let values: Vec<_> = self.r#alias.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("alias", &values)?;
            }
            let requires_elements = self
                .r#alias
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#alias
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
                state.serialize_entry("_alias", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#author.as_ref() {
            state.serialize_entry("author", some)?;
        }
        if let Some(some) = self.r#scope.as_ref() {
            state.serialize_entry("scope", some)?;
        }
        if let Some(some) = self.r#topic.as_ref() {
            match some {
                ContractTopic::CodeableConcept(ref value) => {
                    state.serialize_entry("topicCodeableConcept", value)?;
                }
                ContractTopic::Reference(ref value) => {
                    state.serialize_entry("topicReference", value)?;
                }
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#sub_type.is_empty() {
            state.serialize_entry("subType", &self.r#sub_type)?;
        }
        if let Some(some) = self.r#content_definition.as_ref() {
            state.serialize_entry("contentDefinition", some)?;
        }
        if !self.r#term.is_empty() {
            state.serialize_entry("term", &self.r#term)?;
        }
        if !self.r#supporting_info.is_empty() {
            state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
        }
        if !self.r#relevant_history.is_empty() {
            state.serialize_entry("relevantHistory", &self.r#relevant_history)?;
        }
        if !self.r#signer.is_empty() {
            state.serialize_entry("signer", &self.r#signer)?;
        }
        if !self.r#friendly.is_empty() {
            state.serialize_entry("friendly", &self.r#friendly)?;
        }
        if !self.r#legal.is_empty() {
            state.serialize_entry("legal", &self.r#legal)?;
        }
        if !self.r#rule.is_empty() {
            state.serialize_entry("rule", &self.r#rule)?;
        }
        if let Some(some) = self.r#legally_binding.as_ref() {
            match some {
                ContractLegallyBinding::Attachment(ref value) => {
                    state.serialize_entry("legallyBindingAttachment", value)?;
                }
                ContractLegallyBinding::Reference(ref value) => {
                    state.serialize_entry("legallyBindingReference", value)?;
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Contract {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Contract;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Contract")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Contract, V::Error>
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
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#version: Option<super::super::types::String> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#legal_state: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#instantiates_canonical: Option<Box<super::super::types::Reference>> =
                    None;
                let mut r#instantiates_uri: Option<super::super::types::Uri> = None;
                let mut r#content_derivative: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#issued: Option<super::super::types::DateTime> = None;
                let mut r#applies: Option<Box<super::super::types::Period>> = None;
                let mut r#expiration_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#authority: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#domain: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#site: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#subtitle: Option<super::super::types::String> = None;
                let mut r#alias: Option<Vec<super::super::types::String>> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#scope: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#topic: Option<ContractTopic> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#content_definition: Option<ContractContentDefinition> = None;
                let mut r#term: Option<Vec<ContractTerm>> = None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#relevant_history: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#signer: Option<Vec<ContractSigner>> = None;
                let mut r#friendly: Option<Vec<ContractFriendly>> = None;
                let mut r#legal: Option<Vec<ContractLegal>> = None;
                let mut r#rule: Option<Vec<ContractRule>> = None;
                let mut r#legally_binding: Option<ContractLegallyBinding> = None;
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
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        "url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_url"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "legalState" => {
                            if r#legal_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("legalState"));
                            }
                            r#legal_state = Some(map_access.next_value()?);
                        }
                        "instantiatesCanonical" => {
                            if r#instantiates_canonical.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatesCanonical",
                                ));
                            }
                            r#instantiates_canonical = Some(map_access.next_value()?);
                        }
                        "instantiatesUri" => {
                            let some = r#instantiates_uri.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("instantiatesUri"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_instantiatesUri" => {
                            let some = r#instantiates_uri.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_instantiatesUri"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "contentDerivative" => {
                            if r#content_derivative.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentDerivative"));
                            }
                            r#content_derivative = Some(map_access.next_value()?);
                        }
                        "issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("issued"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_issued" => {
                            let some = r#issued.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_issued"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "applies" => {
                            if r#applies.is_some() {
                                return Err(serde::de::Error::duplicate_field("applies"));
                            }
                            r#applies = Some(map_access.next_value()?);
                        }
                        "expirationType" => {
                            if r#expiration_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationType"));
                            }
                            r#expiration_type = Some(map_access.next_value()?);
                        }
                        "subject" => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(map_access.next_value()?);
                        }
                        "authority" => {
                            if r#authority.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            r#authority = Some(map_access.next_value()?);
                        }
                        "domain" => {
                            if r#domain.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            r#domain = Some(map_access.next_value()?);
                        }
                        "site" => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some(map_access.next_value()?);
                        }
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
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
                        "title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_title" => {
                            let some = r#title.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_title"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subtitle"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_subtitle" => {
                            let some = r#subtitle.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_subtitle"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "alias" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#alias.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("alias"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_alias" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#alias.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_alias"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "author" => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(map_access.next_value()?);
                        }
                        "scope" => {
                            if r#scope.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            r#scope = Some(map_access.next_value()?);
                        }
                        "topicCodeableConcept" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "topicCodeableConcept",
                                ));
                            }
                            r#topic =
                                Some(ContractTopic::CodeableConcept(map_access.next_value()?));
                        }
                        "topicReference" => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topicReference"));
                            }
                            r#topic = Some(ContractTopic::Reference(map_access.next_value()?));
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "subType" => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some(map_access.next_value()?);
                        }
                        "contentDefinition" => {
                            if r#content_definition.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentDefinition"));
                            }
                            r#content_definition = Some(map_access.next_value()?);
                        }
                        "term" => {
                            if r#term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            r#term = Some(map_access.next_value()?);
                        }
                        "supportingInfo" => {
                            if r#supporting_info.is_some() {
                                return Err(serde::de::Error::duplicate_field("supportingInfo"));
                            }
                            r#supporting_info = Some(map_access.next_value()?);
                        }
                        "relevantHistory" => {
                            if r#relevant_history.is_some() {
                                return Err(serde::de::Error::duplicate_field("relevantHistory"));
                            }
                            r#relevant_history = Some(map_access.next_value()?);
                        }
                        "signer" => {
                            if r#signer.is_some() {
                                return Err(serde::de::Error::duplicate_field("signer"));
                            }
                            r#signer = Some(map_access.next_value()?);
                        }
                        "friendly" => {
                            if r#friendly.is_some() {
                                return Err(serde::de::Error::duplicate_field("friendly"));
                            }
                            r#friendly = Some(map_access.next_value()?);
                        }
                        "legal" => {
                            if r#legal.is_some() {
                                return Err(serde::de::Error::duplicate_field("legal"));
                            }
                            r#legal = Some(map_access.next_value()?);
                        }
                        "rule" => {
                            if r#rule.is_some() {
                                return Err(serde::de::Error::duplicate_field("rule"));
                            }
                            r#rule = Some(map_access.next_value()?);
                        }
                        "legallyBindingAttachment" => {
                            if r#legally_binding.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legallyBindingAttachment",
                                ));
                            }
                            r#legally_binding =
                                Some(ContractLegallyBinding::Attachment(map_access.next_value()?));
                        }
                        "legallyBindingReference" => {
                            if r#legally_binding.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legallyBindingReference",
                                ));
                            }
                            r#legally_binding =
                                Some(ContractLegallyBinding::Reference(map_access.next_value()?));
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
                                    "url",
                                    "version",
                                    "status",
                                    "legal_state",
                                    "instantiates_canonical",
                                    "instantiates_uri",
                                    "content_derivative",
                                    "issued",
                                    "applies",
                                    "expiration_type",
                                    "subject",
                                    "authority",
                                    "domain",
                                    "site",
                                    "name",
                                    "title",
                                    "subtitle",
                                    "alias",
                                    "author",
                                    "scope",
                                    "topic",
                                    "type",
                                    "sub_type",
                                    "content_definition",
                                    "term",
                                    "supporting_info",
                                    "relevant_history",
                                    "signer",
                                    "friendly",
                                    "legal",
                                    "rule",
                                    "legally_binding",
                                ],
                            ))
                        }
                    }
                }
                Ok(Contract {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#url,
                    r#version,
                    r#status,
                    r#legal_state,
                    r#instantiates_canonical,
                    r#instantiates_uri,
                    r#content_derivative,
                    r#issued,
                    r#applies,
                    r#expiration_type,
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#authority: r#authority.unwrap_or(vec![]),
                    r#domain: r#domain.unwrap_or(vec![]),
                    r#site: r#site.unwrap_or(vec![]),
                    r#name,
                    r#title,
                    r#subtitle,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#author,
                    r#scope,
                    r#topic,
                    r#type,
                    r#sub_type: r#sub_type.unwrap_or(vec![]),
                    r#content_definition,
                    r#term: r#term.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                    r#signer: r#signer.unwrap_or(vec![]),
                    r#friendly: r#friendly.unwrap_or(vec![]),
                    r#legal: r#legal.unwrap_or(vec![]),
                    r#rule: r#rule.unwrap_or(vec![]),
                    r#legally_binding,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
