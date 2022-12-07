// Generated on 2022-12-07 by fhirbolt-codegen v0.1.0
#[doc = "A series of links that provide context to this bundle."]
#[derive(Default, Debug, Clone)]
pub struct BundleLink {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A name which details the functional use for this link - see [<http://www.iana.org/assignments/link>-relations/link-relations.xhtml#link-relations-1](<http://www.iana.org/assignments/link>-relations/link-relations.xhtml#link-relations-1)."]
    pub r#relation: super::super::types::String,
    #[doc = "The reference details for the link."]
    pub r#url: super::super::types::Uri,
}
impl crate::AnyResource for BundleLink {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
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
            let some = Ok(some)?;
            state.serialize_entry("relation", &some)?;
        }
        if self.r#relation.id.is_some() || !self.r#relation.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#relation.id,
                extension: &self.r#relation.extension,
            };
            state.serialize_entry("_relation", &primitive_element)?;
        }
        if let Some(some) = self.r#url.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("url", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Relation => {
                                let some = r#relation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relation"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "relation", "url"],
                                ));
                            },
                        }
                    }
                    Ok(BundleLink {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#relation: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#relation.unwrap_or(Default::default())
                        } else {
                            r#relation.ok_or(serde::de::Error::missing_field("relation"))?
                        },
                        r#url: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Information about the search process that lead to the creation of this entry."]
#[derive(Default, Debug, Clone)]
pub struct BundleEntrySearch {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Why this entry is in the result set - whether it's included as a match or because of an _include requirement, or to convey information or warning information about the search process."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "When searching, the server's search ranking score for the entry."]
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
                let some = Ok(some)?;
                state.serialize_entry("mode", &some)?;
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
                let some = some
                    .parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                state.serialize_entry("score", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Mode => {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "mode", "score"],
                                ));
                            },
                        }
                    }
                    Ok(BundleEntrySearch {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#mode,
                        r#score,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Additional information about how this entry should be processed as part of a transaction or batch.  For history, it shows how the entry was processed to create the version contained in the entry."]
#[derive(Default, Debug, Clone)]
pub struct BundleEntryRequest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "In a transaction or batch, this is the HTTP action to be executed for this entry. In a history bundle, this indicates the HTTP action that occurred."]
    pub r#method: super::super::types::Code,
    #[doc = "The URL for this entry, relative to the root (the address to which the request is posted)."]
    pub r#url: super::super::types::Uri,
    #[doc = "If the ETag values match, return a 304 Not Modified status. See the API documentation for [\"Conditional Read\"](<http.html>#cread)."]
    pub r#if_none_match: Option<super::super::types::String>,
    #[doc = "Only perform the operation if the last updated date matches. See the API documentation for [\"Conditional Read\"](<http.html>#cread)."]
    pub r#if_modified_since: Option<super::super::types::Instant>,
    #[doc = "Only perform the operation if the Etag value matches. For more information, see the API section [\"Managing Resource Contention\"](<http.html>#concurrency)."]
    pub r#if_match: Option<super::super::types::String>,
    #[doc = "Instruct the server not to perform the create if a specified resource already exists. For further information, see the API documentation for [\"Conditional Create\"](<http.html>#ccreate). This is just the query portion of the URL - what follows the \"?\" (not including the \"?\")."]
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
            let some = Ok(some)?;
            state.serialize_entry("method", &some)?;
        }
        if self.r#method.id.is_some() || !self.r#method.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#method.id,
                extension: &self.r#method.extension,
            };
            state.serialize_entry("_method", &primitive_element)?;
        }
        if let Some(some) = self.r#url.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("url", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("ifNoneMatch", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("ifModifiedSince", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("ifMatch", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("ifNoneExist", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Method => {
                                let some = r#method.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("method"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "ifModifiedSince",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::IfModifiedSincePrimitiveElement => {
                                let some = r#if_modified_since.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_ifModifiedSince",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "method",
                                        "url",
                                        "ifNoneMatch",
                                        "ifModifiedSince",
                                        "ifMatch",
                                        "ifNoneExist",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BundleEntryRequest {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#method: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#method.unwrap_or(Default::default())
                        } else {
                            r#method.ok_or(serde::de::Error::missing_field("method"))?
                        },
                        r#url: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#url.unwrap_or(Default::default())
                        } else {
                            r#url.ok_or(serde::de::Error::missing_field("url"))?
                        },
                        r#if_none_match,
                        r#if_modified_since,
                        r#if_match,
                        r#if_none_exist,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Indicates the results of processing the corresponding 'request' entry in the batch or transaction being responded to or what the results of an operation where when returning history."]
#[derive(Default, Debug, Clone)]
pub struct BundleEntryResponse {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The status code returned by processing this entry. The status SHALL start with a 3 digit HTTP code (e.g. 404) and may contain the standard HTTP description associated with the status code."]
    pub r#status: super::super::types::String,
    #[doc = "The location header created by processing this operation, populated if the operation returns a location."]
    pub r#location: Option<super::super::types::Uri>,
    #[doc = "The Etag for the resource, if the operation for the entry produced a versioned resource (see [Resource Metadata and Versioning](<http.html>#versioning) and [Managing Resource Contention](<http.html>#concurrency))."]
    pub r#etag: Option<super::super::types::String>,
    #[doc = "The date/time that the resource was modified on the server."]
    pub r#last_modified: Option<super::super::types::Instant>,
    #[doc = "An OperationOutcome containing hints and warnings produced as part of processing this entry in a batch or transaction."]
    pub r#outcome: Option<Box<super::super::Resource>>,
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
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("location", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("etag", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("lastModified", &some)?;
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
            Unknown(String),
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
                let mut r#outcome: Option<Box<super::super::Resource>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "status",
                                        "location",
                                        "etag",
                                        "lastModified",
                                        "outcome",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(BundleEntryResponse {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#location,
                        r#etag,
                        r#last_modified,
                        r#outcome,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An entry in a bundle resource - will either contain a resource or information about a resource (transactions and history only)."]
#[derive(Default, Debug, Clone)]
pub struct BundleEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A series of links that provide context to this entry."]
    pub r#link: Vec<BundleLink>,
    #[doc = "The Absolute URL for the resource.  The fullUrl SHALL NOT disagree with the id in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall be version-independent URL consistent with the Resource.id. The fullUrl is a version independent reference to the resource. The fullUrl element SHALL have a value except that: \n* fullUrl can be empty on a POST (although it does not need to when specifying a temporary id for reference in the bundle)\n* Results from operations might involve resources that are not identified."]
    pub r#full_url: Option<super::super::types::Uri>,
    #[doc = "The Resource for the entry. The purpose/meaning of the resource is determined by the Bundle.type."]
    pub r#resource: Option<Box<super::super::Resource>>,
    #[doc = "Information about the search process that lead to the creation of this entry."]
    pub r#search: Option<BundleEntrySearch>,
    #[doc = "Additional information about how this entry should be processed as part of a transaction or batch.  For history, it shows how the entry was processed to create the version contained in the entry."]
    pub r#request: Option<BundleEntryRequest>,
    #[doc = "Indicates the results of processing the corresponding 'request' entry in the batch or transaction being responded to or what the results of an operation where when returning history."]
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
                let some = Ok(some)?;
                state.serialize_entry("fullUrl", &some)?;
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
            Unknown(String),
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
                let mut r#resource: Option<Box<super::super::Resource>> = None;
                let mut r#search: Option<BundleEntrySearch> = None;
                let mut r#request: Option<BundleEntryRequest> = None;
                let mut r#response: Option<BundleEntryResponse> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "link",
                                        "fullUrl",
                                        "resource",
                                        "search",
                                        "request",
                                        "response",
                                    ],
                                ));
                            },
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
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A container for a collection of resources."]
#[derive(Default, Debug, Clone)]
pub struct Bundle {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A persistent identifier for the bundle that won't change as a bundle is copied from server to server."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the purpose of this bundle - how it is intended to be used."]
    pub r#type: super::super::types::Code,
    #[doc = "The date/time that the bundle was assembled - i.e. when the resources were placed in the bundle."]
    pub r#timestamp: Option<super::super::types::Instant>,
    #[doc = "If a set of search matches, this is the total number of entries of type 'match' across all pages in the search.  It does not include search.mode = 'include' or 'outcome' entries and it does not provide a count of the number of entries in the Bundle."]
    pub r#total: Option<super::super::types::UnsignedInt>,
    #[doc = "A series of links that provide context to this bundle."]
    pub r#link: Vec<BundleLink>,
    #[doc = "An entry in a bundle resource - will either contain a resource or information about a resource (transactions and history only)."]
    pub r#entry: Vec<BundleEntry>,
    #[doc = "Digital Signature - base64 encoded. XML-DSig or a JWT."]
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
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
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
            let some = Ok(some)?;
            state.serialize_entry("type", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("timestamp", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("total", &some)?;
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
            Unknown(String),
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
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
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
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "identifier",
                                        "type",
                                        "timestamp",
                                        "total",
                                        "link",
                                        "entry",
                                        "signature",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Bundle {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#identifier,
                        r#type: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#timestamp,
                        r#total,
                        r#link: r#link.unwrap_or(vec![]),
                        r#entry: r#entry.unwrap_or(vec![]),
                        r#signature,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
