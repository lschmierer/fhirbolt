// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct BundleLink {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#relation: super::super::types::String,
    pub r#url: super::super::types::Uri,
}
impl serde::ser::Serialize for BundleLink {
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
        if let Some(some) = self.r#relation.value.as_ref() {
            state.serialize_entry("relation", some)?;
        }
        if self.r#relation.id.is_some() || !self.r#relation.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#relation.id,
                extension: &self.r#relation.extension,
            };
            state.serialize_entry("_relation", &primitive_element)?;
        }
        if let Some(some) = self.r#url.value.as_ref() {
            state.serialize_entry("url", some)?;
        }
        if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#url.id,
                extension: &self.r#url.extension,
            };
            state.serialize_entry("_url", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BundleLink {
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
            #[serde(rename = "relation")]
            Relation,
            #[serde(rename = "_relation")]
            RelationPrimitiveElement,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BundleLink;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BundleLink")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BundleLink, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#relation: Option<super::super::types::String> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
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
                        Field::Relation => {
                            let some = r#relation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::RelationPrimitiveElement => {
                            let some = r#relation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_relation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Url => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UrlPrimitiveElement => {
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
                    }
                }
                Ok(BundleLink {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#relation: r#relation.ok_or(serde::de::Error::missing_field("relation"))?,
                    r#url: r#url.ok_or(serde::de::Error::missing_field("url"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BundleEntrySearch {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: Option<super::super::types::Code>,
    pub r#score: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for BundleEntrySearch {
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
        if let Some(some) = self.r#mode.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("mode", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_mode", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#score.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("score", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_score", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BundleEntrySearch {
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
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "score")]
            Score,
            #[serde(rename = "_score")]
            ScorePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BundleEntrySearch;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BundleEntrySearch")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BundleEntrySearch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#score: Option<super::super::types::Decimal> = None;
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
                        Field::Mode => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ModePrimitiveElement => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Score => {
                            let some = r#score.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ScorePrimitiveElement => {
                            let some = r#score.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_score"));
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
                Ok(BundleEntrySearch {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#mode,
                    r#score,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BundleEntryRequest {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#method: super::super::types::Code,
    pub r#url: super::super::types::Uri,
    pub r#if_none_match: Option<super::super::types::String>,
    pub r#if_modified_since: Option<super::super::types::Instant>,
    pub r#if_match: Option<super::super::types::String>,
    pub r#if_none_exist: Option<super::super::types::String>,
}
impl serde::ser::Serialize for BundleEntryRequest {
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
        if let Some(some) = self.r#method.value.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if self.r#method.id.is_some() || !self.r#method.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#method.id,
                extension: &self.r#method.extension,
            };
            state.serialize_entry("_method", &primitive_element)?;
        }
        if let Some(some) = self.r#url.value.as_ref() {
            state.serialize_entry("url", some)?;
        }
        if self.r#url.id.is_some() || !self.r#url.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#url.id,
                extension: &self.r#url.extension,
            };
            state.serialize_entry("_url", &primitive_element)?;
        }
        if let Some(some) = self.r#if_none_match.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("ifNoneMatch", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ifNoneMatch", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#if_modified_since.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("ifModifiedSince", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ifModifiedSince", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#if_match.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("ifMatch", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ifMatch", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#if_none_exist.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("ifNoneExist", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_ifNoneExist", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BundleEntryRequest {
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
            #[serde(rename = "method")]
            Method,
            #[serde(rename = "_method")]
            MethodPrimitiveElement,
            #[serde(rename = "url")]
            Url,
            #[serde(rename = "_url")]
            UrlPrimitiveElement,
            #[serde(rename = "ifNoneMatch")]
            IfNoneMatch,
            #[serde(rename = "_ifNoneMatch")]
            IfNoneMatchPrimitiveElement,
            #[serde(rename = "ifModifiedSince")]
            IfModifiedSince,
            #[serde(rename = "_ifModifiedSince")]
            IfModifiedSincePrimitiveElement,
            #[serde(rename = "ifMatch")]
            IfMatch,
            #[serde(rename = "_ifMatch")]
            IfMatchPrimitiveElement,
            #[serde(rename = "ifNoneExist")]
            IfNoneExist,
            #[serde(rename = "_ifNoneExist")]
            IfNoneExistPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BundleEntryRequest;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BundleEntryRequest")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BundleEntryRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#method: Option<super::super::types::Code> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#if_none_match: Option<super::super::types::String> = None;
                let mut r#if_modified_since: Option<super::super::types::Instant> = None;
                let mut r#if_match: Option<super::super::types::String> = None;
                let mut r#if_none_exist: Option<super::super::types::String> = None;
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
                        Field::Method => {
                            let some = r#method.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::MethodPrimitiveElement => {
                            let some = r#method.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_method"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Url => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::UrlPrimitiveElement => {
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
                        Field::IfNoneMatch => {
                            let some = r#if_none_match.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("ifNoneMatch"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IfNoneMatchPrimitiveElement => {
                            let some = r#if_none_match.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_ifNoneMatch"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IfModifiedSince => {
                            let some = r#if_modified_since.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("ifModifiedSince"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IfModifiedSincePrimitiveElement => {
                            let some = r#if_modified_since.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_ifModifiedSince"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IfMatch => {
                            let some = r#if_match.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("ifMatch"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IfMatchPrimitiveElement => {
                            let some = r#if_match.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_ifMatch"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::IfNoneExist => {
                            let some = r#if_none_exist.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("ifNoneExist"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::IfNoneExistPrimitiveElement => {
                            let some = r#if_none_exist.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_ifNoneExist"));
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
                Ok(BundleEntryRequest {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#method: r#method.ok_or(serde::de::Error::missing_field("method"))?,
                    r#url: r#url.ok_or(serde::de::Error::missing_field("url"))?,
                    r#if_none_match,
                    r#if_modified_since,
                    r#if_match,
                    r#if_none_exist,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BundleEntryResponse {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::String,
    pub r#location: Option<super::super::types::Uri>,
    pub r#etag: Option<super::super::types::String>,
    pub r#last_modified: Option<super::super::types::Instant>,
    pub r#outcome: Option<Box<super::Resource>>,
}
impl serde::ser::Serialize for BundleEntryResponse {
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
        if let Some(some) = self.r#location.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_location", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#etag.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("etag", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_etag", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#last_modified.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("lastModified", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_lastModified", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#outcome.as_ref() {
            state.serialize_entry("outcome", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BundleEntryResponse {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "_location")]
            LocationPrimitiveElement,
            #[serde(rename = "etag")]
            Etag,
            #[serde(rename = "_etag")]
            EtagPrimitiveElement,
            #[serde(rename = "lastModified")]
            LastModified,
            #[serde(rename = "_lastModified")]
            LastModifiedPrimitiveElement,
            #[serde(rename = "outcome")]
            Outcome,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BundleEntryResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BundleEntryResponse")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BundleEntryResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#status: Option<super::super::types::String> = None;
                let mut r#location: Option<super::super::types::Uri> = None;
                let mut r#etag: Option<super::super::types::String> = None;
                let mut r#last_modified: Option<super::super::types::Instant> = None;
                let mut r#outcome: Option<Box<super::Resource>> = None;
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
                        Field::Location => {
                            let some = r#location.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LocationPrimitiveElement => {
                            let some = r#location.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_location"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Etag => {
                            let some = r#etag.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("etag"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::EtagPrimitiveElement => {
                            let some = r#etag.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_etag"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::LastModified => {
                            let some = r#last_modified.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastModified"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LastModifiedPrimitiveElement => {
                            let some = r#last_modified.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_lastModified"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Outcome => {
                            if r#outcome.is_some() {
                                return Err(serde::de::Error::duplicate_field("outcome"));
                            }
                            r#outcome = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(BundleEntryResponse {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#location,
                    r#etag,
                    r#last_modified,
                    r#outcome,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct BundleEntry {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#link: Vec<BundleLink>,
    pub r#full_url: Option<super::super::types::Uri>,
    pub r#resource: Option<Box<super::Resource>>,
    pub r#search: Option<BundleEntrySearch>,
    pub r#request: Option<BundleEntryRequest>,
    pub r#response: Option<BundleEntryResponse>,
}
impl serde::ser::Serialize for BundleEntry {
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
        if !self.r#link.is_empty() {
            state.serialize_entry("link", &self.r#link)?;
        }
        if let Some(some) = self.r#full_url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("fullUrl", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_fullUrl", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#resource.as_ref() {
            state.serialize_entry("resource", some)?;
        }
        if let Some(some) = self.r#search.as_ref() {
            state.serialize_entry("search", some)?;
        }
        if let Some(some) = self.r#request.as_ref() {
            state.serialize_entry("request", some)?;
        }
        if let Some(some) = self.r#response.as_ref() {
            state.serialize_entry("response", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for BundleEntry {
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
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "fullUrl")]
            FullUrl,
            #[serde(rename = "_fullUrl")]
            FullUrlPrimitiveElement,
            #[serde(rename = "resource")]
            Resource,
            #[serde(rename = "search")]
            Search,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "response")]
            Response,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BundleEntry;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("BundleEntry")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<BundleEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#link: Option<Vec<BundleLink>> = None;
                let mut r#full_url: Option<super::super::types::Uri> = None;
                let mut r#resource: Option<Box<super::Resource>> = None;
                let mut r#search: Option<BundleEntrySearch> = None;
                let mut r#request: Option<BundleEntryRequest> = None;
                let mut r#response: Option<BundleEntryResponse> = None;
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
                        Field::Link => {
                            if r#link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            r#link = Some(map_access.next_value()?);
                        }
                        Field::FullUrl => {
                            let some = r#full_url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("fullUrl"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::FullUrlPrimitiveElement => {
                            let some = r#full_url.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_fullUrl"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Resource => {
                            if r#resource.is_some() {
                                return Err(serde::de::Error::duplicate_field("resource"));
                            }
                            r#resource = Some(map_access.next_value()?);
                        }
                        Field::Search => {
                            if r#search.is_some() {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            r#search = Some(map_access.next_value()?);
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(map_access.next_value()?);
                        }
                        Field::Response => {
                            if r#response.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            r#response = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(BundleEntry {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#link: r#link.unwrap_or(vec![]),
                    r#full_url,
                    r#resource,
                    r#search,
                    r#request,
                    r#response,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Bundle {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type: super::super::types::Code,
    pub r#timestamp: Option<super::super::types::Instant>,
    pub r#total: Option<super::super::types::UnsignedInt>,
    pub r#link: Vec<BundleLink>,
    pub r#entry: Vec<BundleEntry>,
    pub r#signature: Option<Box<super::super::types::Signature>>,
}
impl serde::ser::Serialize for Bundle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Bundle")?;
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
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
        if let Some(some) = self.r#timestamp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("timestamp", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_timestamp", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#total.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("total", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_total", &primitive_element)?;
            }
        }
        if !self.r#link.is_empty() {
            state.serialize_entry("link", &self.r#link)?;
        }
        if !self.r#entry.is_empty() {
            state.serialize_entry("entry", &self.r#entry)?;
        }
        if let Some(some) = self.r#signature.as_ref() {
            state.serialize_entry("signature", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Bundle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "_type")]
            TypePrimitiveElement,
            #[serde(rename = "timestamp")]
            Timestamp,
            #[serde(rename = "_timestamp")]
            TimestampPrimitiveElement,
            #[serde(rename = "total")]
            Total,
            #[serde(rename = "_total")]
            TotalPrimitiveElement,
            #[serde(rename = "link")]
            Link,
            #[serde(rename = "entry")]
            Entry,
            #[serde(rename = "signature")]
            Signature,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Bundle;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Bundle")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Bundle, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#timestamp: Option<super::super::types::Instant> = None;
                let mut r#total: Option<super::super::types::UnsignedInt> = None;
                let mut r#link: Option<Vec<BundleLink>> = None;
                let mut r#entry: Option<Vec<BundleEntry>> = None;
                let mut r#signature: Option<Box<super::super::types::Signature>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Bundle" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Bundle",
                                ));
                            }
                        }
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
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
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
                        Field::Timestamp => {
                            let some = r#timestamp.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TimestampPrimitiveElement => {
                            let some = r#timestamp.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_timestamp"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Total => {
                            let some = r#total.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::TotalPrimitiveElement => {
                            let some = r#total.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_total"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Link => {
                            if r#link.is_some() {
                                return Err(serde::de::Error::duplicate_field("link"));
                            }
                            r#link = Some(map_access.next_value()?);
                        }
                        Field::Entry => {
                            if r#entry.is_some() {
                                return Err(serde::de::Error::duplicate_field("entry"));
                            }
                            r#entry = Some(map_access.next_value()?);
                        }
                        Field::Signature => {
                            if r#signature.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            r#signature = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Bundle {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#identifier,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#timestamp,
                    r#total,
                    r#link: r#link.unwrap_or(vec![]),
                    r#entry: r#entry.unwrap_or(vec![]),
                    r#signature,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
