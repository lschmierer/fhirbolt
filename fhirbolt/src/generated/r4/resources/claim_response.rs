// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for ClaimResponseAddItemServiced {
    fn default() -> ClaimResponseAddItemServiced {
        ClaimResponseAddItemServiced::Invalid
    }
}
#[derive(Debug, Clone)]
pub enum ClaimResponseAddItemLocation {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Address(Box<super::super::types::Address>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ClaimResponseAddItemLocation {
    fn default() -> ClaimResponseAddItemLocation {
        ClaimResponseAddItemLocation::Invalid
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseItemAdjudication {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Money>>,
    pub r#value: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for ClaimResponseItemAdjudication {
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
        state.serialize_entry("category", &self.r#category)?;
        if let Some(some) = self.r#reason.as_ref() {
            state.serialize_entry("reason", some)?;
        }
        if let Some(some) = self.r#amount.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if let Some(some) = self.r#value.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("value", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_value", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseItemAdjudication {
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
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "reason")]
            Reason,
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "_value")]
            ValuePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseItemAdjudication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemAdjudication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseItemAdjudication, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                let mut r#value: Option<super::super::types::Decimal> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Reason => {
                            if r#reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            r#reason = Some(map_access.next_value()?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        Field::Value => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ValuePrimitiveElement => {
                            let some = r#value.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_value"));
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
                Ok(ClaimResponseItemAdjudication {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#reason,
                    r#amount,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseItemDetailSubDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sub_detail_sequence: super::super::types::PositiveInt,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
}
impl serde::ser::Serialize for ClaimResponseItemDetailSubDetail {
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
        if let Some(some) = self.r#sub_detail_sequence.value.as_ref() {
            state.serialize_entry("subDetailSequence", some)?;
        }
        if self.r#sub_detail_sequence.id.is_some()
            || !self.r#sub_detail_sequence.extension.is_empty()
        {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sub_detail_sequence.id,
                extension: &self.r#sub_detail_sequence.extension,
            };
            state.serialize_entry("_subDetailSequence", &primitive_element)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseItemDetailSubDetail {
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
            #[serde(rename = "subDetailSequence")]
            SubDetailSequence,
            #[serde(rename = "_subDetailSequence")]
            SubDetailSequencePrimitiveElement,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseItemDetailSubDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sub_detail_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
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
                        Field::SubDetailSequence => {
                            let some = r#sub_detail_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetailSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SubDetailSequencePrimitiveElement => {
                            let some = r#sub_detail_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_subDetailSequence",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sub_detail_sequence: r#sub_detail_sequence
                        .ok_or(serde::de::Error::missing_field("sub_detail_sequence"))?,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseItemDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail_sequence: super::super::types::PositiveInt,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#sub_detail: Vec<ClaimResponseItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimResponseItemDetail {
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
        if let Some(some) = self.r#detail_sequence.value.as_ref() {
            state.serialize_entry("detailSequence", some)?;
        }
        if self.r#detail_sequence.id.is_some() || !self.r#detail_sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#detail_sequence.id,
                extension: &self.r#detail_sequence.extension,
            };
            state.serialize_entry("_detailSequence", &primitive_element)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseItemDetail {
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
            #[serde(rename = "detailSequence")]
            DetailSequence,
            #[serde(rename = "_detailSequence")]
            DetailSequencePrimitiveElement,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "subDetail")]
            SubDetail,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseItemDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#detail_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
                let mut r#sub_detail: Option<Vec<ClaimResponseItemDetailSubDetail>> = None;
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
                        Field::DetailSequence => {
                            let some = r#detail_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DetailSequencePrimitiveElement => {
                            let some = r#detail_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_detailSequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        Field::SubDetail => {
                            if r#sub_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetail"));
                            }
                            r#sub_detail = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#detail_sequence: r#detail_sequence
                        .ok_or(serde::de::Error::missing_field("detail_sequence"))?,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: super::super::types::PositiveInt,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#detail: Vec<ClaimResponseItemDetail>,
}
impl serde::ser::Serialize for ClaimResponseItem {
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
        if let Some(some) = self.r#item_sequence.value.as_ref() {
            state.serialize_entry("itemSequence", some)?;
        }
        if self.r#item_sequence.id.is_some() || !self.r#item_sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#item_sequence.id,
                extension: &self.r#item_sequence.extension,
            };
            state.serialize_entry("_itemSequence", &primitive_element)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseItem {
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
            #[serde(rename = "itemSequence")]
            ItemSequence,
            #[serde(rename = "_itemSequence")]
            ItemSequencePrimitiveElement,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "detail")]
            Detail,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
                let mut r#detail: Option<Vec<ClaimResponseItemDetail>> = None;
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
                        Field::ItemSequence => {
                            let some = r#item_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ItemSequencePrimitiveElement => {
                            let some = r#item_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_itemSequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        Field::Detail => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: r#item_sequence
                        .ok_or(serde::de::Error::missing_field("item_sequence"))?,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseAddItemDetailSubDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
}
impl serde::ser::Serialize for ClaimResponseAddItemDetailSubDetail {
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
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
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
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseAddItemDetailSubDetail {
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
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseAddItemDetailSubDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItemDetailSubDetail")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ClaimResponseAddItemDetailSubDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
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
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        Field::Modifier => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        Field::Factor => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FactorPrimitiveElement => {
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
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseAddItemDetailSubDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseAddItemDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#sub_detail: Vec<ClaimResponseAddItemDetailSubDetail>,
}
impl serde::ser::Serialize for ClaimResponseAddItemDetail {
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
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
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
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#sub_detail.is_empty() {
            state.serialize_entry("subDetail", &self.r#sub_detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseAddItemDetail {
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
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "subDetail")]
            SubDetail,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseAddItemDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItemDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseAddItemDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
                let mut r#sub_detail: Option<Vec<ClaimResponseAddItemDetailSubDetail>> = None;
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
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        Field::Modifier => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        Field::Factor => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FactorPrimitiveElement => {
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
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        Field::SubDetail => {
                            if r#sub_detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetail"));
                            }
                            r#sub_detail = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseAddItemDetail {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#sub_detail: r#sub_detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseAddItem {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: Vec<super::super::types::PositiveInt>,
    pub r#detail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#subdetail_sequence: Vec<super::super::types::PositiveInt>,
    pub r#provider: Vec<Box<super::super::types::Reference>>,
    pub r#product_or_service: Box<super::super::types::CodeableConcept>,
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#program_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#serviced: Option<ClaimResponseAddItemServiced>,
    pub r#location: Option<ClaimResponseAddItemLocation>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#unit_price: Option<Box<super::super::types::Money>>,
    pub r#factor: Option<super::super::types::Decimal>,
    pub r#net: Option<Box<super::super::types::Money>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#sub_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note_number: Vec<super::super::types::PositiveInt>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#detail: Vec<ClaimResponseAddItemDetail>,
}
impl serde::ser::Serialize for ClaimResponseAddItem {
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
        if !self.r#item_sequence.is_empty() {
            let values: Vec<_> = self.r#item_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("itemSequence", &values)?;
            }
            let requires_elements = self
                .r#item_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#item_sequence
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
                state.serialize_entry("_itemSequence", &primitive_elements)?;
            }
        }
        if !self.r#detail_sequence.is_empty() {
            let values: Vec<_> = self.r#detail_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("detailSequence", &values)?;
            }
            let requires_elements = self
                .r#detail_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#detail_sequence
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
                state.serialize_entry("_detailSequence", &primitive_elements)?;
            }
        }
        if !self.r#subdetail_sequence.is_empty() {
            let values: Vec<_> = self.r#subdetail_sequence.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("subdetailSequence", &values)?;
            }
            let requires_elements = self
                .r#subdetail_sequence
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#subdetail_sequence
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
                state.serialize_entry("_subdetailSequence", &primitive_elements)?;
            }
        }
        if !self.r#provider.is_empty() {
            state.serialize_entry("provider", &self.r#provider)?;
        }
        state.serialize_entry("productOrService", &self.r#product_or_service)?;
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if !self.r#program_code.is_empty() {
            state.serialize_entry("programCode", &self.r#program_code)?;
        }
        if let Some(some) = self.r#serviced.as_ref() {
            match some {
                ClaimResponseAddItemServiced::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("servicedDate", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_servicedDate", &primitive_element)?;
                    }
                }
                ClaimResponseAddItemServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
                ClaimResponseAddItemServiced::Invalid => {
                    return Err(serde::ser::Error::custom("serviced is invalid"))
                }
            }
        }
        if let Some(some) = self.r#location.as_ref() {
            match some {
                ClaimResponseAddItemLocation::CodeableConcept(ref value) => {
                    state.serialize_entry("locationCodeableConcept", value)?;
                }
                ClaimResponseAddItemLocation::Address(ref value) => {
                    state.serialize_entry("locationAddress", value)?;
                }
                ClaimResponseAddItemLocation::Reference(ref value) => {
                    state.serialize_entry("locationReference", value)?;
                }
                ClaimResponseAddItemLocation::Invalid => {
                    return Err(serde::ser::Error::custom("location is invalid"))
                }
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
        if let Some(some) = self.r#net.as_ref() {
            state.serialize_entry("net", some)?;
        }
        if let Some(some) = self.r#body_site.as_ref() {
            state.serialize_entry("bodySite", some)?;
        }
        if !self.r#sub_site.is_empty() {
            state.serialize_entry("subSite", &self.r#sub_site)?;
        }
        if !self.r#note_number.is_empty() {
            let values: Vec<_> = self.r#note_number.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("noteNumber", &values)?;
            }
            let requires_elements = self
                .r#note_number
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#note_number
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
                state.serialize_entry("_noteNumber", &primitive_elements)?;
            }
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#detail.is_empty() {
            state.serialize_entry("detail", &self.r#detail)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseAddItem {
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
            #[serde(rename = "itemSequence")]
            ItemSequence,
            #[serde(rename = "_itemSequence")]
            ItemSequencePrimitiveElement,
            #[serde(rename = "detailSequence")]
            DetailSequence,
            #[serde(rename = "_detailSequence")]
            DetailSequencePrimitiveElement,
            #[serde(rename = "subdetailSequence")]
            SubdetailSequence,
            #[serde(rename = "_subdetailSequence")]
            SubdetailSequencePrimitiveElement,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "programCode")]
            ProgramCode,
            #[serde(rename = "servicedDate")]
            ServicedDate,
            #[serde(rename = "_servicedDate")]
            ServicedDatePrimitiveElement,
            #[serde(rename = "servicedPeriod")]
            ServicedPeriod,
            #[serde(rename = "locationCodeableConcept")]
            LocationCodeableConcept,
            #[serde(rename = "locationAddress")]
            LocationAddress,
            #[serde(rename = "locationReference")]
            LocationReference,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "unitPrice")]
            UnitPrice,
            #[serde(rename = "factor")]
            Factor,
            #[serde(rename = "_factor")]
            FactorPrimitiveElement,
            #[serde(rename = "net")]
            Net,
            #[serde(rename = "bodySite")]
            BodySite,
            #[serde(rename = "subSite")]
            SubSite,
            #[serde(rename = "noteNumber")]
            NoteNumber,
            #[serde(rename = "_noteNumber")]
            NoteNumberPrimitiveElement,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "detail")]
            Detail,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseAddItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseAddItem")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseAddItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#detail_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#subdetail_sequence: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#provider: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#program_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#serviced: Option<ClaimResponseAddItemServiced> = None;
                let mut r#location: Option<ClaimResponseAddItemLocation> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<super::super::types::Money>> = None;
                let mut r#factor: Option<super::super::types::Decimal> = None;
                let mut r#net: Option<Box<super::super::types::Money>> = None;
                let mut r#body_site: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_site: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#note_number: Option<Vec<super::super::types::PositiveInt>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
                let mut r#detail: Option<Vec<ClaimResponseAddItemDetail>> = None;
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
                        Field::ItemSequence => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#item_sequence.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("itemSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::ItemSequencePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#item_sequence.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_itemSequence"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::DetailSequence => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec =
                                r#detail_sequence.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("detailSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::DetailSequencePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#detail_sequence.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_detailSequence"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::SubdetailSequence => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#subdetail_sequence
                                .get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("subdetailSequence"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SubdetailSequencePrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#subdetail_sequence
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
                                    "_subdetailSequence",
                                ));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Provider => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(map_access.next_value()?);
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some(map_access.next_value()?);
                        }
                        Field::Modifier => {
                            if r#modifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifier"));
                            }
                            r#modifier = Some(map_access.next_value()?);
                        }
                        Field::ProgramCode => {
                            if r#program_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("programCode"));
                            }
                            r#program_code = Some(map_access.next_value()?);
                        }
                        Field::ServicedDate => {
                            let r#enum = r#serviced.get_or_insert(
                                ClaimResponseAddItemServiced::Date(Default::default()),
                            );
                            if let ClaimResponseAddItemServiced::Date(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("serviced[x]"));
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            let r#enum = r#serviced.get_or_insert(
                                ClaimResponseAddItemServiced::Date(Default::default()),
                            );
                            if let ClaimResponseAddItemServiced::Date(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_servicedDate"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                            }
                        }
                        Field::ServicedPeriod => {
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            r#serviced = Some(ClaimResponseAddItemServiced::Period(
                                map_access.next_value()?,
                            ));
                        }
                        Field::LocationCodeableConcept => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "locationCodeableConcept",
                                ));
                            }
                            r#location = Some(ClaimResponseAddItemLocation::CodeableConcept(
                                map_access.next_value()?,
                            ));
                        }
                        Field::LocationAddress => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationAddress"));
                            }
                            r#location = Some(ClaimResponseAddItemLocation::Address(
                                map_access.next_value()?,
                            ));
                        }
                        Field::LocationReference => {
                            if r#location.is_some() {
                                return Err(serde::de::Error::duplicate_field("locationReference"));
                            }
                            r#location = Some(ClaimResponseAddItemLocation::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value()?);
                        }
                        Field::Factor => {
                            let some = r#factor.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("factor"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FactorPrimitiveElement => {
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
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value()?);
                        }
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some(map_access.next_value()?);
                        }
                        Field::SubSite => {
                            if r#sub_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("subSite"));
                            }
                            r#sub_site = Some(map_access.next_value()?);
                        }
                        Field::NoteNumber => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#note_number.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("noteNumber"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::NoteNumberPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#note_number.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_noteNumber"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        Field::Detail => {
                            if r#detail.is_some() {
                                return Err(serde::de::Error::duplicate_field("detail"));
                            }
                            r#detail = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseAddItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence: r#item_sequence.unwrap_or(vec![]),
                    r#detail_sequence: r#detail_sequence.unwrap_or(vec![]),
                    r#subdetail_sequence: r#subdetail_sequence.unwrap_or(vec![]),
                    r#provider: r#provider.unwrap_or(vec![]),
                    r#product_or_service: r#product_or_service
                        .ok_or(serde::de::Error::missing_field("product_or_service"))?,
                    r#modifier: r#modifier.unwrap_or(vec![]),
                    r#program_code: r#program_code.unwrap_or(vec![]),
                    r#serviced,
                    r#location,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#net,
                    r#body_site,
                    r#sub_site: r#sub_site.unwrap_or(vec![]),
                    r#note_number: r#note_number.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#detail: r#detail.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseTotal {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Box<super::super::types::CodeableConcept>,
    pub r#amount: Box<super::super::types::Money>,
}
impl serde::ser::Serialize for ClaimResponseTotal {
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
        state.serialize_entry("category", &self.r#category)?;
        state.serialize_entry("amount", &self.r#amount)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseTotal {
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
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "amount")]
            Amount,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseTotal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseTotal")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseTotal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some(map_access.next_value()?);
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseTotal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#category: r#category.ok_or(serde::de::Error::missing_field("category"))?,
                    r#amount: r#amount.ok_or(serde::de::Error::missing_field("amount"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponsePayment {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#adjustment: Option<Box<super::super::types::Money>>,
    pub r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::Date>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
impl serde::ser::Serialize for ClaimResponsePayment {
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
        if let Some(some) = self.r#adjustment.as_ref() {
            state.serialize_entry("adjustment", some)?;
        }
        if let Some(some) = self.r#adjustment_reason.as_ref() {
            state.serialize_entry("adjustmentReason", some)?;
        }
        if let Some(some) = self.r#date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("date", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_date", &primitive_element)?;
            }
        }
        state.serialize_entry("amount", &self.r#amount)?;
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponsePayment {
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
            #[serde(rename = "adjustment")]
            Adjustment,
            #[serde(rename = "adjustmentReason")]
            AdjustmentReason,
            #[serde(rename = "date")]
            Date,
            #[serde(rename = "_date")]
            DatePrimitiveElement,
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "identifier")]
            Identifier,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponsePayment;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponsePayment")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponsePayment, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#adjustment: Option<Box<super::super::types::Money>> = None;
                let mut r#adjustment_reason: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#date: Option<super::super::types::Date> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
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
                        Field::Adjustment => {
                            if r#adjustment.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustment"));
                            }
                            r#adjustment = Some(map_access.next_value()?);
                        }
                        Field::AdjustmentReason => {
                            if r#adjustment_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjustmentReason"));
                            }
                            r#adjustment_reason = Some(map_access.next_value()?);
                        }
                        Field::Date => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DatePrimitiveElement => {
                            let some = r#date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_date"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponsePayment {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#adjustment,
                    r#adjustment_reason,
                    r#date,
                    r#amount: r#amount.ok_or(serde::de::Error::missing_field("amount"))?,
                    r#identifier,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseProcessNote {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number: Option<super::super::types::PositiveInt>,
    pub r#type: Option<super::super::types::Code>,
    pub r#text: super::super::types::String,
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ClaimResponseProcessNote {
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
        if let Some(some) = self.r#number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("number", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_number", &primitive_element)?;
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
        if let Some(some) = self.r#text.value.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if self.r#text.id.is_some() || !self.r#text.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#text.id,
                extension: &self.r#text.extension,
            };
            state.serialize_entry("_text", &primitive_element)?;
        }
        if let Some(some) = self.r#language.as_ref() {
            state.serialize_entry("language", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseProcessNote {
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
            #[serde(rename = "number")]
            Number,
            #[serde(rename = "_number")]
            NumberPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "language")]
            Language,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseProcessNote;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseProcessNote")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseProcessNote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#number: Option<super::super::types::PositiveInt> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#language: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::Number => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("number"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NumberPrimitiveElement => {
                            let some = r#number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_number"));
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
                        Field::Text => {
                            let some = r#text.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TextPrimitiveElement => {
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
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseProcessNote {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number,
                    r#type,
                    r#text: r#text.ok_or(serde::de::Error::missing_field("text"))?,
                    r#language,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseInsurance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence: super::super::types::PositiveInt,
    pub r#focal: super::super::types::Boolean,
    pub r#coverage: Box<super::super::types::Reference>,
    pub r#business_arrangement: Option<super::super::types::String>,
    pub r#claim_response: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for ClaimResponseInsurance {
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
        if let Some(some) = self.r#sequence.value.as_ref() {
            state.serialize_entry("sequence", some)?;
        }
        if self.r#sequence.id.is_some() || !self.r#sequence.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#sequence.id,
                extension: &self.r#sequence.extension,
            };
            state.serialize_entry("_sequence", &primitive_element)?;
        }
        if let Some(some) = self.r#focal.value.as_ref() {
            state.serialize_entry("focal", some)?;
        }
        if self.r#focal.id.is_some() || !self.r#focal.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#focal.id,
                extension: &self.r#focal.extension,
            };
            state.serialize_entry("_focal", &primitive_element)?;
        }
        state.serialize_entry("coverage", &self.r#coverage)?;
        if let Some(some) = self.r#business_arrangement.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("businessArrangement", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_businessArrangement", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#claim_response.as_ref() {
            state.serialize_entry("claimResponse", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseInsurance {
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
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "focal")]
            Focal,
            #[serde(rename = "_focal")]
            FocalPrimitiveElement,
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "businessArrangement")]
            BusinessArrangement,
            #[serde(rename = "_businessArrangement")]
            BusinessArrangementPrimitiveElement,
            #[serde(rename = "claimResponse")]
            ClaimResponse,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseInsurance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseInsurance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#focal: Option<super::super::types::Boolean> = None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#business_arrangement: Option<super::super::types::String> = None;
                let mut r#claim_response: Option<Box<super::super::types::Reference>> = None;
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
                        Field::Sequence => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SequencePrimitiveElement => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Focal => {
                            let some = r#focal.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("focal"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FocalPrimitiveElement => {
                            let some = r#focal.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_focal"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Coverage => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            r#coverage = Some(map_access.next_value()?);
                        }
                        Field::BusinessArrangement => {
                            let some = r#business_arrangement.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "businessArrangement",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::BusinessArrangementPrimitiveElement => {
                            let some = r#business_arrangement.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_businessArrangement",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ClaimResponse => {
                            if r#claim_response.is_some() {
                                return Err(serde::de::Error::duplicate_field("claimResponse"));
                            }
                            r#claim_response = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseInsurance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence: r#sequence.ok_or(serde::de::Error::missing_field("sequence"))?,
                    r#focal: r#focal.ok_or(serde::de::Error::missing_field("focal"))?,
                    r#coverage: r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?,
                    r#business_arrangement,
                    r#claim_response,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponseError {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item_sequence: Option<super::super::types::PositiveInt>,
    pub r#detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#sub_detail_sequence: Option<super::super::types::PositiveInt>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for ClaimResponseError {
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
        if let Some(some) = self.r#item_sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("itemSequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_itemSequence", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#detail_sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("detailSequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_detailSequence", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#sub_detail_sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subDetailSequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subDetailSequence", &primitive_element)?;
            }
        }
        state.serialize_entry("code", &self.r#code)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponseError {
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
            #[serde(rename = "itemSequence")]
            ItemSequence,
            #[serde(rename = "_itemSequence")]
            ItemSequencePrimitiveElement,
            #[serde(rename = "detailSequence")]
            DetailSequence,
            #[serde(rename = "_detailSequence")]
            DetailSequencePrimitiveElement,
            #[serde(rename = "subDetailSequence")]
            SubDetailSequence,
            #[serde(rename = "_subDetailSequence")]
            SubDetailSequencePrimitiveElement,
            #[serde(rename = "code")]
            Code,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponseError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponseError")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponseError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#item_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#detail_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#sub_detail_sequence: Option<super::super::types::PositiveInt> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
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
                        Field::ItemSequence => {
                            let some = r#item_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ItemSequencePrimitiveElement => {
                            let some = r#item_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_itemSequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::DetailSequence => {
                            let some = r#detail_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("detailSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DetailSequencePrimitiveElement => {
                            let some = r#detail_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_detailSequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::SubDetailSequence => {
                            let some = r#sub_detail_sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subDetailSequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SubDetailSequencePrimitiveElement => {
                            let some = r#sub_detail_sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_subDetailSequence",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponseError {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#item_sequence,
                    r#detail_sequence,
                    r#sub_detail_sequence,
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct ClaimResponse {
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
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#sub_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#use: super::super::types::Code,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#created: super::super::types::DateTime,
    pub r#insurer: Box<super::super::types::Reference>,
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#outcome: super::super::types::Code,
    pub r#disposition: Option<super::super::types::String>,
    pub r#pre_auth_ref: Option<super::super::types::String>,
    pub r#pre_auth_period: Option<Box<super::super::types::Period>>,
    pub r#payee_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#item: Vec<ClaimResponseItem>,
    pub r#add_item: Vec<ClaimResponseAddItem>,
    pub r#adjudication: Vec<ClaimResponseItemAdjudication>,
    pub r#total: Vec<ClaimResponseTotal>,
    pub r#payment: Option<ClaimResponsePayment>,
    pub r#funds_reserve: Option<Box<super::super::types::CodeableConcept>>,
    pub r#form_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#form: Option<Box<super::super::types::Attachment>>,
    pub r#process_note: Vec<ClaimResponseProcessNote>,
    pub r#communication_request: Vec<Box<super::super::types::Reference>>,
    pub r#insurance: Vec<ClaimResponseInsurance>,
    pub r#error: Vec<ClaimResponseError>,
}
impl serde::ser::Serialize for ClaimResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "ClaimResponse")?;
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
        if let Some(some) = self.r#status.value.as_ref() {
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#sub_type.as_ref() {
            state.serialize_entry("subType", some)?;
        }
        if let Some(some) = self.r#use.value.as_ref() {
            state.serialize_entry("use", some)?;
        }
        if self.r#use.id.is_some() || !self.r#use.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#use.id,
                extension: &self.r#use.extension,
            };
            state.serialize_entry("_use", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#created.value.as_ref() {
            state.serialize_entry("created", some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        state.serialize_entry("insurer", &self.r#insurer)?;
        if let Some(some) = self.r#requestor.as_ref() {
            state.serialize_entry("requestor", some)?;
        }
        if let Some(some) = self.r#request.as_ref() {
            state.serialize_entry("request", some)?;
        }
        if let Some(some) = self.r#outcome.value.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        if self.r#outcome.id.is_some() || !self.r#outcome.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#outcome.id,
                extension: &self.r#outcome.extension,
            };
            state.serialize_entry("_outcome", &primitive_element)?;
        }
        if let Some(some) = self.r#disposition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("disposition", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_disposition", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#pre_auth_ref.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("preAuthRef", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preAuthRef", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#pre_auth_period.as_ref() {
            state.serialize_entry("preAuthPeriod", some)?;
        }
        if let Some(some) = self.r#payee_type.as_ref() {
            state.serialize_entry("payeeType", some)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        if !self.r#add_item.is_empty() {
            state.serialize_entry("addItem", &self.r#add_item)?;
        }
        if !self.r#adjudication.is_empty() {
            state.serialize_entry("adjudication", &self.r#adjudication)?;
        }
        if !self.r#total.is_empty() {
            state.serialize_entry("total", &self.r#total)?;
        }
        if let Some(some) = self.r#payment.as_ref() {
            state.serialize_entry("payment", some)?;
        }
        if let Some(some) = self.r#funds_reserve.as_ref() {
            state.serialize_entry("fundsReserve", some)?;
        }
        if let Some(some) = self.r#form_code.as_ref() {
            state.serialize_entry("formCode", some)?;
        }
        if let Some(some) = self.r#form.as_ref() {
            state.serialize_entry("form", some)?;
        }
        if !self.r#process_note.is_empty() {
            state.serialize_entry("processNote", &self.r#process_note)?;
        }
        if !self.r#communication_request.is_empty() {
            state.serialize_entry("communicationRequest", &self.r#communication_request)?;
        }
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if !self.r#error.is_empty() {
            state.serialize_entry("error", &self.r#error)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ClaimResponse {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "subType")]
            SubType,
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "insurer")]
            Insurer,
            #[serde(rename = "requestor")]
            Requestor,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "disposition")]
            Disposition,
            #[serde(rename = "_disposition")]
            DispositionPrimitiveElement,
            #[serde(rename = "preAuthRef")]
            PreAuthRef,
            #[serde(rename = "_preAuthRef")]
            PreAuthRefPrimitiveElement,
            #[serde(rename = "preAuthPeriod")]
            PreAuthPeriod,
            #[serde(rename = "payeeType")]
            PayeeType,
            #[serde(rename = "item")]
            Item,
            #[serde(rename = "addItem")]
            AddItem,
            #[serde(rename = "adjudication")]
            Adjudication,
            #[serde(rename = "total")]
            Total,
            #[serde(rename = "payment")]
            Payment,
            #[serde(rename = "fundsReserve")]
            FundsReserve,
            #[serde(rename = "formCode")]
            FormCode,
            #[serde(rename = "form")]
            Form,
            #[serde(rename = "processNote")]
            ProcessNote,
            #[serde(rename = "communicationRequest")]
            CommunicationRequest,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "error")]
            Error,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ClaimResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ClaimResponse")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ClaimResponse, V::Error>
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
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#insurer: Option<Box<super::super::types::Reference>> = None;
                let mut r#requestor: Option<Box<super::super::types::Reference>> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#disposition: Option<super::super::types::String> = None;
                let mut r#pre_auth_ref: Option<super::super::types::String> = None;
                let mut r#pre_auth_period: Option<Box<super::super::types::Period>> = None;
                let mut r#payee_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#item: Option<Vec<ClaimResponseItem>> = None;
                let mut r#add_item: Option<Vec<ClaimResponseAddItem>> = None;
                let mut r#adjudication: Option<Vec<ClaimResponseItemAdjudication>> = None;
                let mut r#total: Option<Vec<ClaimResponseTotal>> = None;
                let mut r#payment: Option<ClaimResponsePayment> = None;
                let mut r#funds_reserve: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#form_code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#form: Option<Box<super::super::types::Attachment>> = None;
                let mut r#process_note: Option<Vec<ClaimResponseProcessNote>> = None;
                let mut r#communication_request: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#insurance: Option<Vec<ClaimResponseInsurance>> = None;
                let mut r#error: Option<Vec<ClaimResponseError>> = None;
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
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusPrimitiveElement => {
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::SubType => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some(map_access.next_value()?);
                        }
                        Field::Use => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("use"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UsePrimitiveElement => {
                            let some = r#use.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_use"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        Field::Created => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CreatedPrimitiveElement => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_created"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Insurer => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            r#insurer = Some(map_access.next_value()?);
                        }
                        Field::Requestor => {
                            if r#requestor.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            r#requestor = Some(map_access.next_value()?);
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(map_access.next_value()?);
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
                        Field::Disposition => {
                            let some = r#disposition.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("disposition"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DispositionPrimitiveElement => {
                            let some = r#disposition.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_disposition"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PreAuthRef => {
                            let some = r#pre_auth_ref.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("preAuthRef"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            let some = r#pre_auth_ref.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::PreAuthPeriod => {
                            if r#pre_auth_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("preAuthPeriod"));
                            }
                            r#pre_auth_period = Some(map_access.next_value()?);
                        }
                        Field::PayeeType => {
                            if r#payee_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("payeeType"));
                            }
                            r#payee_type = Some(map_access.next_value()?);
                        }
                        Field::Item => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("item"));
                            }
                            r#item = Some(map_access.next_value()?);
                        }
                        Field::AddItem => {
                            if r#add_item.is_some() {
                                return Err(serde::de::Error::duplicate_field("addItem"));
                            }
                            r#add_item = Some(map_access.next_value()?);
                        }
                        Field::Adjudication => {
                            if r#adjudication.is_some() {
                                return Err(serde::de::Error::duplicate_field("adjudication"));
                            }
                            r#adjudication = Some(map_access.next_value()?);
                        }
                        Field::Total => {
                            if r#total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            r#total = Some(map_access.next_value()?);
                        }
                        Field::Payment => {
                            if r#payment.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            r#payment = Some(map_access.next_value()?);
                        }
                        Field::FundsReserve => {
                            if r#funds_reserve.is_some() {
                                return Err(serde::de::Error::duplicate_field("fundsReserve"));
                            }
                            r#funds_reserve = Some(map_access.next_value()?);
                        }
                        Field::FormCode => {
                            if r#form_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("formCode"));
                            }
                            r#form_code = Some(map_access.next_value()?);
                        }
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            r#form = Some(map_access.next_value()?);
                        }
                        Field::ProcessNote => {
                            if r#process_note.is_some() {
                                return Err(serde::de::Error::duplicate_field("processNote"));
                            }
                            r#process_note = Some(map_access.next_value()?);
                        }
                        Field::CommunicationRequest => {
                            if r#communication_request.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "communicationRequest",
                                ));
                            }
                            r#communication_request = Some(map_access.next_value()?);
                        }
                        Field::Insurance => {
                            if r#insurance.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurance"));
                            }
                            r#insurance = Some(map_access.next_value()?);
                        }
                        Field::Error => {
                            if r#error.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            r#error = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(ClaimResponse {
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
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#sub_type,
                    r#use: r#use.ok_or(serde::de::Error::missing_field("use"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#created: r#created.ok_or(serde::de::Error::missing_field("created"))?,
                    r#insurer: r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?,
                    r#requestor,
                    r#request,
                    r#outcome: r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?,
                    r#disposition,
                    r#pre_auth_ref,
                    r#pre_auth_period,
                    r#payee_type,
                    r#item: r#item.unwrap_or(vec![]),
                    r#add_item: r#add_item.unwrap_or(vec![]),
                    r#adjudication: r#adjudication.unwrap_or(vec![]),
                    r#total: r#total.unwrap_or(vec![]),
                    r#payment,
                    r#funds_reserve,
                    r#form_code,
                    r#form,
                    r#process_note: r#process_note.unwrap_or(vec![]),
                    r#communication_request: r#communication_request.unwrap_or(vec![]),
                    r#insurance: r#insurance.unwrap_or(vec![]),
                    r#error: r#error.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
