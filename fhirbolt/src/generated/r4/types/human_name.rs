// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct HumanName {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#use: Option<super::super::types::Code>,
    pub r#text: Option<super::super::types::String>,
    pub r#family: Option<super::super::types::String>,
    pub r#given: Vec<super::super::types::String>,
    pub r#prefix: Vec<super::super::types::String>,
    pub r#suffix: Vec<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
}
impl serde::ser::Serialize for HumanName {
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
        if let Some(some) = self.r#use.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("use", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_use", &primitive_element)?;
            }
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
        if let Some(some) = self.r#family.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("family", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_family", &primitive_element)?;
            }
        }
        if !self.r#given.is_empty() {
            let values: Vec<_> = self.r#given.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("given", &values)?;
            }
            let requires_elements = self
                .r#given
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#given
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
                state.serialize_entry("_given", &primitive_elements)?;
            }
        }
        if !self.r#prefix.is_empty() {
            let values: Vec<_> = self.r#prefix.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("prefix", &values)?;
            }
            let requires_elements = self
                .r#prefix
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#prefix
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
                state.serialize_entry("_prefix", &primitive_elements)?;
            }
        }
        if !self.r#suffix.is_empty() {
            let values: Vec<_> = self.r#suffix.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("suffix", &values)?;
            }
            let requires_elements = self
                .r#suffix
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#suffix
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
                state.serialize_entry("_suffix", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#period.as_ref() {
            state.serialize_entry("period", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for HumanName {
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
            #[serde(rename = "use")]
            Use,
            #[serde(rename = "_use")]
            UsePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "_text")]
            TextPrimitiveElement,
            #[serde(rename = "family")]
            Family,
            #[serde(rename = "_family")]
            FamilyPrimitiveElement,
            #[serde(rename = "given")]
            Given,
            #[serde(rename = "_given")]
            GivenPrimitiveElement,
            #[serde(rename = "prefix")]
            Prefix,
            #[serde(rename = "_prefix")]
            PrefixPrimitiveElement,
            #[serde(rename = "suffix")]
            Suffix,
            #[serde(rename = "_suffix")]
            SuffixPrimitiveElement,
            #[serde(rename = "period")]
            Period,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = HumanName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HumanName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<HumanName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#use: Option<super::super::types::Code> = None;
                let mut r#text: Option<super::super::types::String> = None;
                let mut r#family: Option<super::super::types::String> = None;
                let mut r#given: Option<Vec<super::super::types::String>> = None;
                let mut r#prefix: Option<Vec<super::super::types::String>> = None;
                let mut r#suffix: Option<Vec<super::super::types::String>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
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
                        Field::Family => {
                            let some = r#family.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("family"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FamilyPrimitiveElement => {
                            let some = r#family.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_family"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Given => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#given.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("given"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::GivenPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#given.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field("_given"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Prefix => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#prefix.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::PrefixPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#prefix.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field("_prefix"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Suffix => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#suffix.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(values.len())
                                    .collect::<Vec<_>>(),
                            );
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("suffix"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::SuffixPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#suffix.get_or_insert(
                                std::iter::repeat(Default::default())
                                    .take(elements.len())
                                    .collect::<Vec<_>>(),
                            );
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
                                return Err(serde::de::Error::duplicate_field("_suffix"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(HumanName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#use,
                    r#text,
                    r#family,
                    r#given: r#given.unwrap_or(vec![]),
                    r#prefix: r#prefix.unwrap_or(vec![]),
                    r#suffix: r#suffix.unwrap_or(vec![]),
                    r#period,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
