// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::types::DataRequirementCodeFilter;
impl serde::ser::Serialize for SerializationContext<&DataRequirementCodeFilter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "DataRequirement.codeFilter", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("path", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_path", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#path.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("path", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#search_param.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("searchParam", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_searchParam", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#search_param.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("searchParam", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#value_set.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("valueSet", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_valueSet", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#value_set.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("valueSet", ctx)));
        }
        if !self.value.r#code.is_empty() {
            tri!(self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DataRequirementCodeFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DataRequirementCodeFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<DataRequirementCodeFilter>
{
    type Value = DataRequirementCodeFilter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DataRequirementCodeFilter>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DataRequirementCodeFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementCodeFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementCodeFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "searchParam")]
                    SearchParam,
                    #[serde(rename = "_searchParam")]
                    SearchParamPrimitiveElement,
                    #[serde(rename = "valueSet")]
                    ValueSet,
                    #[serde(rename = "_valueSet")]
                    ValueSetPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "path", "searchParam", "valueSet", "code"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#search_param: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#value_set: Option<fhirbolt_model::r5::types::Canonical> = None;
                let mut r#code: Option<Vec<fhirbolt_model::r5::types::Coding>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::SearchParam => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#search_param.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#search_param =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SearchParamPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_searchParam"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("searchParam");
                            }
                        }
                        Field::ValueSet => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                r#value_set =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ValueSetPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueSet"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("valueSet");
                            }
                        }
                        Field::Code => {
                            if self.0.from == crate::context::Format::Json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Coding>,
                                > = self.0.transmute();
                                r#code = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Coding,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DataRequirementCodeFilter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path,
                    r#search_param,
                    r#value_set,
                    r#code: r#code.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<DataRequirementCodeFilter>>
{
    type Value = Box<DataRequirementCodeFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DataRequirementCodeFilter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<DataRequirementCodeFilter>>
{
    type Value = Vec<DataRequirementCodeFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DataRequirementCodeFilter>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DataRequirementCodeFilter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DataRequirementCodeFilter> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::types::DataRequirementDateFilter;
impl serde::ser::Serialize for SerializationContext<&DataRequirementDateFilter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "DataRequirement.dateFilter", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("path", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_path", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#path.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("path", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#search_param.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("searchParam", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_searchParam", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#search_param.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("searchParam", ctx)));
        }
        {
            use fhirbolt_model::r5::types::DataRequirementDateFilterValue as _Enum;
            if let Some(some) = self.value.r#value.as_ref() {
                match some {
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("valueDateTime", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_valueDateTime", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("valueDateTime", ctx)));
                        }
                    }
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx)));
                    }
                    _Enum::Duration(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("valueDuration", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("value is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DataRequirementDateFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DataRequirementDateFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<DataRequirementDateFilter>
{
    type Value = DataRequirementDateFilter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DataRequirementDateFilter>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DataRequirementDateFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementDateFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementDateFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "searchParam")]
                    SearchParam,
                    #[serde(rename = "_searchParam")]
                    SearchParamPrimitiveElement,
                    #[serde(rename = "valueDateTime")]
                    ValueDateTime,
                    #[serde(rename = "_valueDateTime")]
                    ValueDateTimePrimitiveElement,
                    #[serde(rename = "valuePeriod")]
                    ValuePeriod,
                    #[serde(rename = "valueDuration")]
                    ValueDuration,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "path",
                            "searchParam",
                            "valueDateTime",
                            "valuePeriod",
                            "valueDuration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#search_param: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#value: Option<fhirbolt_model::r5::types::DataRequirementDateFilterValue> =
                    None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::SearchParam => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#search_param.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#search_param =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SearchParamPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_searchParam"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("searchParam");
                            }
                        }
                        Field::ValueDateTime => {
                            use fhirbolt_model::r5::types::DataRequirementDateFilterValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#value = Some(_Enum::DateTime(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r5::types::DataRequirementDateFilterValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValuePeriod => {
                            use fhirbolt_model::r5::types::DataRequirementDateFilterValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::ValueDuration => {
                            use fhirbolt_model::r5::types::DataRequirementDateFilterValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Duration>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Duration(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DataRequirementDateFilter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path,
                    r#search_param,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<DataRequirementDateFilter>>
{
    type Value = Box<DataRequirementDateFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DataRequirementDateFilter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<DataRequirementDateFilter>>
{
    type Value = Vec<DataRequirementDateFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DataRequirementDateFilter>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DataRequirementDateFilter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DataRequirementDateFilter> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::types::DataRequirementValueFilter;
impl serde::ser::Serialize for SerializationContext<&DataRequirementValueFilter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "DataRequirement.valueFilter", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#path.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("path", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_path", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#path.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("path", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#search_param.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("searchParam", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_searchParam", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#search_param.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("searchParam", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#comparator.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("comparator", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_comparator", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#comparator.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("comparator", ctx)));
        }
        {
            use fhirbolt_model::r5::types::DataRequirementValueFilterValue as _Enum;
            if let Some(some) = self.value.r#value.as_ref() {
                match some {
                    _Enum::DateTime(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("valueDateTime", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_valueDateTime", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("valueDateTime", ctx)));
                        }
                    }
                    _Enum::Period(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx)));
                    }
                    _Enum::Duration(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("valueDuration", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("value is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DataRequirementValueFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DataRequirementValueFilter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<DataRequirementValueFilter>
{
    type Value = DataRequirementValueFilter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DataRequirementValueFilter>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DataRequirementValueFilter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementValueFilter")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementValueFilter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "searchParam")]
                    SearchParam,
                    #[serde(rename = "_searchParam")]
                    SearchParamPrimitiveElement,
                    #[serde(rename = "comparator")]
                    Comparator,
                    #[serde(rename = "_comparator")]
                    ComparatorPrimitiveElement,
                    #[serde(rename = "valueDateTime")]
                    ValueDateTime,
                    #[serde(rename = "_valueDateTime")]
                    ValueDateTimePrimitiveElement,
                    #[serde(rename = "valuePeriod")]
                    ValuePeriod,
                    #[serde(rename = "valueDuration")]
                    ValueDuration,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "path",
                            "searchParam",
                            "comparator",
                            "valueDateTime",
                            "valuePeriod",
                            "valueDuration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#search_param: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#comparator: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#value: Option<
                    fhirbolt_model::r5::types::DataRequirementValueFilterValue,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::SearchParam => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#search_param.is_some() {
                                    return Err(serde::de::Error::duplicate_field("searchParam"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#search_param =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SearchParamPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#search_param.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_searchParam"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("searchParam");
                            }
                        }
                        Field::Comparator => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#comparator.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comparator"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#comparator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comparator"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#comparator =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ComparatorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#comparator.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comparator"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comparator");
                            }
                        }
                        Field::ValueDateTime => {
                            use fhirbolt_model::r5::types::DataRequirementValueFilterValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#value = Some(_Enum::DateTime(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r5::types::DataRequirementValueFilterValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        tri!(map_access.next_value_seed(&mut *_context));
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValuePeriod => {
                            use fhirbolt_model::r5::types::DataRequirementValueFilterValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Period(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::ValueDuration => {
                            use fhirbolt_model::r5::types::DataRequirementValueFilterValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Duration>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Duration(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DataRequirementValueFilter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path,
                    r#search_param,
                    r#comparator,
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<DataRequirementValueFilter>>
{
    type Value = Box<DataRequirementValueFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DataRequirementValueFilter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<DataRequirementValueFilter>>
{
    type Value = Vec<DataRequirementValueFilter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DataRequirementValueFilter>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DataRequirementValueFilter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DataRequirementValueFilter> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::types::DataRequirementSort;
impl serde::ser::Serialize for SerializationContext<&DataRequirementSort> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "DataRequirement.sort", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("path", &some?));
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_path", ctx)));
            }
        } else if self.value.r#path.id.as_deref() == Some("$invalid") {
            return missing_field_error("path");
        } else {
            tri!(self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#direction.id.as_deref() == Some("$invalid") {
                return missing_field_error("direction");
            }
            if let Some(some) = self.value.r#direction.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("direction", &some?));
            }
            if self.value.r#direction.id.is_some() || !self.value.r#direction.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#direction.id.as_ref(),
                    extension: &self.value.r#direction.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_direction", ctx)));
            }
        } else if self.value.r#direction.id.as_deref() == Some("$invalid") {
            return missing_field_error("direction");
        } else {
            tri!(self.with_context(&self.value.r#direction, |ctx| state
                .serialize_entry("direction", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DataRequirementSort>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DataRequirementSort>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DataRequirementSort> {
    type Value = DataRequirementSort;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DataRequirementSort>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DataRequirementSort;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirementSort")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirementSort, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "direction")]
                    Direction,
                    #[serde(rename = "_direction")]
                    DirectionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "path", "direction"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#path: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#direction: Option<fhirbolt_model::r5::types::Code> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Path => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#path = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::Direction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#direction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("direction"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#direction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("direction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#direction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DirectionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#direction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_direction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("direction");
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DataRequirementSort {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        tri!(r#path.ok_or(serde::de::Error::missing_field("path")))
                    },
                    r#direction: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#direction.unwrap_or(Default::default())
                    } else {
                        tri!(r#direction.ok_or(serde::de::Error::missing_field("direction")))
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<DataRequirementSort>>
{
    type Value = Box<DataRequirementSort>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DataRequirementSort>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<DataRequirementSort>>
{
    type Value = Vec<DataRequirementSort>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DataRequirementSort>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DataRequirementSort>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DataRequirementSort> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::types::DataRequirement;
impl serde::ser::Serialize for SerializationContext<&DataRequirement> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "DataRequirement", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            if let Some(some) = self.value.r#type.value.as_ref().map(Ok) {
                tri!(state.serialize_entry("type", &some?));
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                tri!(self.with_context(&primitive_element, |ctx| state
                    .serialize_entry("_type", ctx)));
            }
        } else if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            tri!(self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#profile.is_empty() {
                let values = tri!(self
                    .value
                    .r#profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("profile", &values));
                }
                let requires_elements = self
                    .value
                    .r#profile
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#profile
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_profile", ctx)));
                }
            }
        } else if !self.value.r#profile.is_empty() {
            tri!(self.with_context(&self.value.r#profile, |ctx| state
                .serialize_entry("profile", ctx)));
        }
        {
            use fhirbolt_model::r5::types::DataRequirementSubject as _Enum;
            if let Some(some) = self.value.r#subject.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("subjectCodeableConcept", ctx)));
                    }
                    _Enum::Reference(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("subjectReference", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("subject is invalid")),
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#must_support.is_empty() {
                let values = tri!(self
                    .value
                    .r#must_support
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>());
                if values.iter().any(|v| v.is_some()) {
                    tri!(state.serialize_entry("mustSupport", &values));
                }
                let requires_elements = self
                    .value
                    .r#must_support
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#must_support
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    tri!(self.with_context(&primitive_elements, |ctx| state
                        .serialize_entry("_mustSupport", ctx)));
                }
            }
        } else if !self.value.r#must_support.is_empty() {
            tri!(self.with_context(&self.value.r#must_support, |ctx| state
                .serialize_entry("mustSupport", ctx)));
        }
        if !self.value.r#code_filter.is_empty() {
            tri!(self.with_context(&self.value.r#code_filter, |ctx| state
                .serialize_entry("codeFilter", ctx)));
        }
        if !self.value.r#date_filter.is_empty() {
            tri!(self.with_context(&self.value.r#date_filter, |ctx| state
                .serialize_entry("dateFilter", ctx)));
        }
        if !self.value.r#value_filter.is_empty() {
            tri!(self.with_context(&self.value.r#value_filter, |ctx| state
                .serialize_entry("valueFilter", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#limit.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("limit", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_limit", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#limit.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("limit", ctx)));
        }
        if !self.value.r#sort.is_empty() {
            tri!(self.with_context(&self.value.r#sort, |ctx| state.serialize_entry("sort", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DataRequirement>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DataRequirement>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DataRequirement> {
    type Value = DataRequirement;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DataRequirement>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DataRequirement;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DataRequirement")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DataRequirement, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "extension")]
                    Extension,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "_profile")]
                    ProfilePrimitiveElement,
                    #[serde(rename = "subjectCodeableConcept")]
                    SubjectCodeableConcept,
                    #[serde(rename = "subjectReference")]
                    SubjectReference,
                    #[serde(rename = "mustSupport")]
                    MustSupport,
                    #[serde(rename = "_mustSupport")]
                    MustSupportPrimitiveElement,
                    #[serde(rename = "codeFilter")]
                    CodeFilter,
                    #[serde(rename = "dateFilter")]
                    DateFilter,
                    #[serde(rename = "valueFilter")]
                    ValueFilter,
                    #[serde(rename = "limit")]
                    Limit,
                    #[serde(rename = "_limit")]
                    LimitPrimitiveElement,
                    #[serde(rename = "sort")]
                    Sort,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "type",
                            "profile",
                            "subjectCodeableConcept",
                            "subjectReference",
                            "mustSupport",
                            "codeFilter",
                            "dateFilter",
                            "valueFilter",
                            "limit",
                            "sort",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#type: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#profile: Option<Vec<fhirbolt_model::r5::types::Canonical>> = None;
                let mut r#subject: Option<fhirbolt_model::r5::types::DataRequirementSubject> = None;
                let mut r#must_support: Option<Vec<fhirbolt_model::r5::types::String>> = None;
                let mut r#code_filter: Option<
                    Vec<fhirbolt_model::r5::types::DataRequirementCodeFilter>,
                > = None;
                let mut r#date_filter: Option<
                    Vec<fhirbolt_model::r5::types::DataRequirementDateFilter>,
                > = None;
                let mut r#value_filter: Option<
                    Vec<fhirbolt_model::r5::types::DataRequirementValueFilter>,
                > = None;
                let mut r#limit: Option<fhirbolt_model::r5::types::PositiveInt> = None;
                let mut r#sort: Option<Vec<fhirbolt_model::r5::types::DataRequirementSort>> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Profile => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("profile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#profile.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Canonical,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_profile"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("profile");
                            }
                        }
                        Field::SubjectCodeableConcept => {
                            use fhirbolt_model::r5::types::DataRequirementSubject as _Enum;
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "subjectCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#subject = Some(_Enum::CodeableConcept(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::SubjectReference => {
                            use fhirbolt_model::r5::types::DataRequirementSubject as _Enum;
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(_Enum::Reference(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::MustSupport => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = tri!(map_access.next_value());
                                let vec = r#must_support.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#must_support.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MustSupportPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                let vec = r#must_support.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_mustSupport"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("mustSupport");
                            }
                        }
                        Field::CodeFilter => {
                            if self.0.from == crate::context::Format::Json {
                                if r#code_filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("codeFilter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::DataRequirementCodeFilter>,
                                > = self.0.transmute();
                                r#code_filter =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#code_filter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DataRequirementCodeFilter,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::DateFilter => {
                            if self.0.from == crate::context::Format::Json {
                                if r#date_filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateFilter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::DataRequirementDateFilter>,
                                > = self.0.transmute();
                                r#date_filter =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#date_filter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DataRequirementDateFilter,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ValueFilter => {
                            if self.0.from == crate::context::Format::Json {
                                if r#value_filter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueFilter"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::DataRequirementValueFilter>,
                                > = self.0.transmute();
                                r#value_filter =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#value_filter.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DataRequirementValueFilter,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Limit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#limit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("limit"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#limit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("limit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::PositiveInt,
                                > = self.0.transmute();
                                r#limit = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LimitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#limit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_limit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("limit");
                            }
                        }
                        Field::Sort => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sort.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sort"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::DataRequirementSort>,
                                > = self.0.transmute();
                                r#sort = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#sort.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DataRequirementSort,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(DataRequirement {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        tri!(r#type.ok_or(serde::de::Error::missing_field("type")))
                    },
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#subject,
                    r#must_support: r#must_support.unwrap_or(vec![]),
                    r#code_filter: r#code_filter.unwrap_or(vec![]),
                    r#date_filter: r#date_filter.unwrap_or(vec![]),
                    r#value_filter: r#value_filter.unwrap_or(vec![]),
                    r#limit,
                    r#sort: r#sort.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DataRequirement>> {
    type Value = Box<DataRequirement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DataRequirement>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DataRequirement>> {
    type Value = Vec<DataRequirement>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DataRequirement>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DataRequirement>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DataRequirement> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
