// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionSlicingDiscriminator> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.slicing.discriminator", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#type.id.as_deref() == Some("$invalid") {
                return missing_field_error("type");
            }
            if let Some(some) = self.value.r#type.value.as_ref().map(Ok) {
                state.serialize_entry("type", &some?)?;
            }
            if self.value.r#type.id.is_some() || !self.value.r#type.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#type.id.as_ref(),
                    extension: &self.value.r#type.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_type", ctx)
                })?;
            }
        } else if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if self.output_json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref().map(Ok) {
                state.serialize_entry("path", &some?)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else if self.value.r#path.id.as_deref() == Some("$invalid") {
            return missing_field_error("path");
        }
        self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionSlicingDiscriminator>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionSlicingDiscriminator>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionSlicingDiscriminator>
{
    type Value = ElementDefinitionSlicingDiscriminator;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionSlicingDiscriminator>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionSlicingDiscriminator;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicingDiscriminator")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ElementDefinitionSlicingDiscriminator, V::Error>
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
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "type", "path"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#type: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#path = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
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
                Ok(ElementDefinitionSlicingDiscriminator {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        r#path.ok_or(serde::de::Error::missing_field("path"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionSlicingDiscriminator>>
{
    type Value = Box<ElementDefinitionSlicingDiscriminator>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionSlicingDiscriminator>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionSlicingDiscriminator>>
{
    type Value = Vec<ElementDefinitionSlicingDiscriminator>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<ElementDefinitionSlicingDiscriminator>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionSlicingDiscriminator>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionSlicingDiscriminator> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionSlicing;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionSlicing> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.slicing", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#discriminator.is_empty() {
            self.with_context(&self.value.r#discriminator, |ctx| {
                state.serialize_entry("discriminator", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#ordered.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("ordered", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_ordered", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#ordered.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("ordered", ctx))?;
        }
        if self.output_json {
            if self.value.r#rules.id.as_deref() == Some("$invalid") {
                return missing_field_error("rules");
            }
            if let Some(some) = self.value.r#rules.value.as_ref().map(Ok) {
                state.serialize_entry("rules", &some?)?;
            }
            if self.value.r#rules.id.is_some() || !self.value.r#rules.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#rules.id.as_ref(),
                    extension: &self.value.r#rules.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_rules", ctx)
                })?;
            }
        } else if self.value.r#rules.id.as_deref() == Some("$invalid") {
            return missing_field_error("rules");
        }
        self.with_context(&self.value.r#rules, |ctx| {
            state.serialize_entry("rules", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionSlicing>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionSlicing>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionSlicing>
{
    type Value = ElementDefinitionSlicing;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionSlicing>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionSlicing;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionSlicing")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionSlicing, V::Error>
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
                    #[serde(rename = "discriminator")]
                    Discriminator,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "ordered")]
                    Ordered,
                    #[serde(rename = "_ordered")]
                    OrderedPrimitiveElement,
                    #[serde(rename = "rules")]
                    Rules,
                    #[serde(rename = "_rules")]
                    RulesPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "discriminator",
                            "description",
                            "ordered",
                            "rules",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#discriminator: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionSlicingDiscriminator>,
                > = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#ordered: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#rules: Option<fhirbolt_model::r4b::types::Code> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Discriminator => {
                            if self.0.from_json {
                                if r#discriminator.is_some() {
                                    return Err(serde::de::Error::duplicate_field("discriminator"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator >> = self . 0 . transmute () ;
                                r#discriminator = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#discriminator.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: types :: ElementDefinitionSlicingDiscriminator > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Ordered => {
                            if self.0.from_json {
                                let some = r#ordered.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ordered"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#ordered.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ordered"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#ordered = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OrderedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#ordered.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_ordered"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("ordered");
                            }
                        }
                        Field::Rules => {
                            if self.0.from_json {
                                let some = r#rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#rules = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_rules"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("rules");
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
                Ok(ElementDefinitionSlicing {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#discriminator: r#discriminator.unwrap_or(vec![]),
                    r#description,
                    r#ordered,
                    r#rules: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#rules.unwrap_or(Default::default())
                    } else {
                        r#rules.ok_or(serde::de::Error::missing_field("rules"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionSlicing>>
{
    type Value = Box<ElementDefinitionSlicing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionSlicing>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionSlicing>>
{
    type Value = Vec<ElementDefinitionSlicing>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionSlicing>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionSlicing>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionSlicing> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionBase;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionBase> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.base", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref().map(Ok) {
                state.serialize_entry("path", &some?)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else if self.value.r#path.id.as_deref() == Some("$invalid") {
            return missing_field_error("path");
        }
        self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        if self.output_json {
            if self.value.r#min.id.as_deref() == Some("$invalid") {
                return missing_field_error("min");
            }
            if let Some(some) = self.value.r#min.value.as_ref().map(Ok) {
                state.serialize_entry("min", &some?)?;
            }
            if self.value.r#min.id.is_some() || !self.value.r#min.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#min.id.as_ref(),
                    extension: &self.value.r#min.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_min", ctx))?;
            }
        } else if self.value.r#min.id.as_deref() == Some("$invalid") {
            return missing_field_error("min");
        }
        self.with_context(&self.value.r#min, |ctx| state.serialize_entry("min", ctx))?;
        if self.output_json {
            if self.value.r#max.id.as_deref() == Some("$invalid") {
                return missing_field_error("max");
            }
            if let Some(some) = self.value.r#max.value.as_ref().map(Ok) {
                state.serialize_entry("max", &some?)?;
            }
            if self.value.r#max.id.is_some() || !self.value.r#max.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#max.id.as_ref(),
                    extension: &self.value.r#max.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_max", ctx))?;
            }
        } else if self.value.r#max.id.as_deref() == Some("$invalid") {
            return missing_field_error("max");
        }
        self.with_context(&self.value.r#max, |ctx| state.serialize_entry("max", ctx))?;
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionBase>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionBase>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ElementDefinitionBase> {
    type Value = ElementDefinitionBase;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionBase>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionBase;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBase")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionBase, V::Error>
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
                    #[serde(rename = "min")]
                    Min,
                    #[serde(rename = "_min")]
                    MinPrimitiveElement,
                    #[serde(rename = "max")]
                    Max,
                    #[serde(rename = "_max")]
                    MaxPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "path", "min", "max"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#min: Option<fhirbolt_model::r4b::types::UnsignedInt> = None;
                let mut r#max: Option<fhirbolt_model::r4b::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#path = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::Min => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#min.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::UnsignedInt,
                                > = self.0.transmute();
                                r#min = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MinPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_min"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("min");
                            }
                        }
                        Field::Max => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#max = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_max"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("max");
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
                Ok(ElementDefinitionBase {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        r#path.ok_or(serde::de::Error::missing_field("path"))?
                    },
                    r#min: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#min.unwrap_or(Default::default())
                    } else {
                        r#min.ok_or(serde::de::Error::missing_field("min"))?
                    },
                    r#max: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#max.unwrap_or(Default::default())
                    } else {
                        r#max.ok_or(serde::de::Error::missing_field("max"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionBase>>
{
    type Value = Box<ElementDefinitionBase>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionBase>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionBase>>
{
    type Value = Vec<ElementDefinitionBase>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionBase>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionBase>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionBase> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionType;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionType> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.type", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#code.id.as_deref() == Some("$invalid") {
                return missing_field_error("code");
            }
            if let Some(some) = self.value.r#code.value.as_ref().map(Ok) {
                state.serialize_entry("code", &some?)?;
            }
            if self.value.r#code.id.is_some() || !self.value.r#code.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#code.id.as_ref(),
                    extension: &self.value.r#code.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_code", ctx)
                })?;
            }
        } else if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        }
        self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        if self.output_json {
            if !self.value.r#profile.is_empty() {
                let values = self
                    .value
                    .r#profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("profile", &values)?;
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_profile", ctx)
                    })?;
                }
            }
        } else if !self.value.r#profile.is_empty() {
            self.with_context(&self.value.r#profile, |ctx| {
                state.serialize_entry("profile", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#target_profile.is_empty() {
                let values = self
                    .value
                    .r#target_profile
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("targetProfile", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#target_profile
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#target_profile
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_targetProfile", ctx)
                    })?;
                }
            }
        } else if !self.value.r#target_profile.is_empty() {
            self.with_context(&self.value.r#target_profile, |ctx| {
                state.serialize_entry("targetProfile", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#aggregation.is_empty() {
                let values = self
                    .value
                    .r#aggregation
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("aggregation", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#aggregation
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#aggregation
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_aggregation", ctx)
                    })?;
                }
            }
        } else if !self.value.r#aggregation.is_empty() {
            self.with_context(&self.value.r#aggregation, |ctx| {
                state.serialize_entry("aggregation", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#versioning.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("versioning", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_versioning", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#versioning.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("versioning", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionType>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionType>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ElementDefinitionType> {
    type Value = ElementDefinitionType;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionType>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionType;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionType")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionType, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "_code")]
                    CodePrimitiveElement,
                    #[serde(rename = "profile")]
                    Profile,
                    #[serde(rename = "_profile")]
                    ProfilePrimitiveElement,
                    #[serde(rename = "targetProfile")]
                    TargetProfile,
                    #[serde(rename = "_targetProfile")]
                    TargetProfilePrimitiveElement,
                    #[serde(rename = "aggregation")]
                    Aggregation,
                    #[serde(rename = "_aggregation")]
                    AggregationPrimitiveElement,
                    #[serde(rename = "versioning")]
                    Versioning,
                    #[serde(rename = "_versioning")]
                    VersioningPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "code",
                            "profile",
                            "targetProfile",
                            "aggregation",
                            "versioning",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#code: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#profile: Option<Vec<fhirbolt_model::r4b::types::Canonical>> = None;
                let mut r#target_profile: Option<Vec<fhirbolt_model::r4b::types::Canonical>> = None;
                let mut r#aggregation: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#versioning: Option<fhirbolt_model::r4b::types::Code> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#code = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CodePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_code"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("code");
                            }
                        }
                        Field::Profile => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
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
                                    fhirbolt_model::r4b::types::Canonical,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProfilePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
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
                        Field::TargetProfile => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#target_profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("targetProfile"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#target_profile.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Canonical,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TargetProfilePrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#target_profile.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_targetProfile",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("targetProfile");
                            }
                        }
                        Field::Aggregation => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#aggregation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("aggregation"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#aggregation.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AggregationPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#aggregation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_aggregation"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("aggregation");
                            }
                        }
                        Field::Versioning => {
                            if self.0.from_json {
                                let some = r#versioning.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versioning"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#versioning.is_some() {
                                    return Err(serde::de::Error::duplicate_field("versioning"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#versioning = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VersioningPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#versioning.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_versioning"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("versioning");
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
                Ok(ElementDefinitionType {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#profile: r#profile.unwrap_or(vec![]),
                    r#target_profile: r#target_profile.unwrap_or(vec![]),
                    r#aggregation: r#aggregation.unwrap_or(vec![]),
                    r#versioning,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionType>>
{
    type Value = Box<ElementDefinitionType>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionType>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionType>>
{
    type Value = Vec<ElementDefinitionType>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionType>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionType>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionType> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionExample;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionExample> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.example", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#label.id.as_deref() == Some("$invalid") {
                return missing_field_error("label");
            }
            if let Some(some) = self.value.r#label.value.as_ref().map(Ok) {
                state.serialize_entry("label", &some?)?;
            }
            if self.value.r#label.id.is_some() || !self.value.r#label.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#label.id.as_ref(),
                    extension: &self.value.r#label.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_label", ctx)
                })?;
            }
        } else if self.value.r#label.id.as_deref() == Some("$invalid") {
            return missing_field_error("label");
        }
        self.with_context(&self.value.r#label, |ctx| {
            state.serialize_entry("label", ctx)
        })?;
        {
            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
            match self.value.r#value {
                _Enum::Base64Binary(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueBase64Binary", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBase64Binary", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueBase64Binary", ctx)
                        })?;
                    }
                }
                _Enum::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueBoolean", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                    }
                }
                _Enum::Canonical(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueCanonical", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueCanonical", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueCanonical", ctx)
                        })?;
                    }
                }
                _Enum::Code(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueCode", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueCode", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueCode", ctx))?;
                    }
                }
                _Enum::Date(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueDate", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDate", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueDate", ctx))?;
                    }
                }
                _Enum::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueDateTime", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueDateTime", ctx)
                        })?;
                    }
                }
                _Enum::Decimal(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(|v| {
                            v.parse::<serde_json::Number>()
                                .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                        }) {
                            state.serialize_entry("valueDecimal", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueDecimal", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueDecimal", ctx))?;
                    }
                }
                _Enum::Id(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueId", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueId", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueId", ctx))?;
                    }
                }
                _Enum::Instant(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueInstant", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueInstant", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueInstant", ctx))?;
                    }
                }
                _Enum::Integer(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueInteger", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueInteger", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueInteger", ctx))?;
                    }
                }
                _Enum::Markdown(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueMarkdown", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueMarkdown", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueMarkdown", ctx)
                        })?;
                    }
                }
                _Enum::Oid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueOid", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueOid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueOid", ctx))?;
                    }
                }
                _Enum::PositiveInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valuePositiveInt", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valuePositiveInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valuePositiveInt", ctx)
                        })?;
                    }
                }
                _Enum::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueString", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueString", ctx))?;
                    }
                }
                _Enum::Time(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueTime", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueTime", ctx))?;
                    }
                }
                _Enum::UnsignedInt(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueUnsignedInt", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUnsignedInt", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueUnsignedInt", ctx)
                        })?;
                    }
                }
                _Enum::Uri(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueUri", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUri", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUri", ctx))?;
                    }
                }
                _Enum::Url(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueUrl", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUrl", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUrl", ctx))?;
                    }
                }
                _Enum::Uuid(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("valueUuid", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_valueUuid", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("valueUuid", ctx))?;
                    }
                }
                _Enum::Address(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAddress", ctx))?;
                }
                _Enum::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAge", ctx))?;
                }
                _Enum::Annotation(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAnnotation", ctx))?;
                }
                _Enum::Attachment(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
                }
                _Enum::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableConcept", ctx)
                    })?;
                }
                _Enum::CodeableReference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueCodeableReference", ctx)
                    })?;
                }
                _Enum::Coding(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
                }
                _Enum::ContactPoint(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueContactPoint", ctx)
                    })?;
                }
                _Enum::Count(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueCount", ctx))?;
                }
                _Enum::Distance(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDistance", ctx))?;
                }
                _Enum::Duration(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDuration", ctx))?;
                }
                _Enum::HumanName(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueHumanName", ctx))?;
                }
                _Enum::Identifier(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueIdentifier", ctx))?;
                }
                _Enum::Money(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueMoney", ctx))?;
                }
                _Enum::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valuePeriod", ctx))?;
                }
                _Enum::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
                }
                _Enum::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRange", ctx))?;
                }
                _Enum::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRatio", ctx))?;
                }
                _Enum::RatioRange(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueRatioRange", ctx))?;
                }
                _Enum::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
                }
                _Enum::SampledData(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueSampledData", ctx))?;
                }
                _Enum::Signature(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueSignature", ctx))?;
                }
                _Enum::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueTiming", ctx))?;
                }
                _Enum::ContactDetail(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueContactDetail", ctx)
                    })?;
                }
                _Enum::Contributor(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueContributor", ctx))?;
                }
                _Enum::DataRequirement(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueDataRequirement", ctx)
                    })?;
                }
                _Enum::Expression(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueExpression", ctx))?;
                }
                _Enum::ParameterDefinition(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueParameterDefinition", ctx)
                    })?;
                }
                _Enum::RelatedArtifact(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueRelatedArtifact", ctx)
                    })?;
                }
                _Enum::TriggerDefinition(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueTriggerDefinition", ctx)
                    })?;
                }
                _Enum::UsageContext(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("valueUsageContext", ctx)
                    })?;
                }
                _Enum::Dosage(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("valueDosage", ctx))?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("value is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionExample>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionExample>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionExample>
{
    type Value = ElementDefinitionExample;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionExample>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionExample;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionExample")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionExample, V::Error>
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
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "valueBase64Binary")]
                    ValueBase64Binary,
                    #[serde(rename = "_valueBase64Binary")]
                    ValueBase64BinaryPrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueCanonical")]
                    ValueCanonical,
                    #[serde(rename = "_valueCanonical")]
                    ValueCanonicalPrimitiveElement,
                    #[serde(rename = "valueCode")]
                    ValueCode,
                    #[serde(rename = "_valueCode")]
                    ValueCodePrimitiveElement,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueDateTime")]
                    ValueDateTime,
                    #[serde(rename = "_valueDateTime")]
                    ValueDateTimePrimitiveElement,
                    #[serde(rename = "valueDecimal")]
                    ValueDecimal,
                    #[serde(rename = "_valueDecimal")]
                    ValueDecimalPrimitiveElement,
                    #[serde(rename = "valueId")]
                    ValueId,
                    #[serde(rename = "_valueId")]
                    ValueIdPrimitiveElement,
                    #[serde(rename = "valueInstant")]
                    ValueInstant,
                    #[serde(rename = "_valueInstant")]
                    ValueInstantPrimitiveElement,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueMarkdown")]
                    ValueMarkdown,
                    #[serde(rename = "_valueMarkdown")]
                    ValueMarkdownPrimitiveElement,
                    #[serde(rename = "valueOid")]
                    ValueOid,
                    #[serde(rename = "_valueOid")]
                    ValueOidPrimitiveElement,
                    #[serde(rename = "valuePositiveInt")]
                    ValuePositiveInt,
                    #[serde(rename = "_valuePositiveInt")]
                    ValuePositiveIntPrimitiveElement,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueTime")]
                    ValueTime,
                    #[serde(rename = "_valueTime")]
                    ValueTimePrimitiveElement,
                    #[serde(rename = "valueUnsignedInt")]
                    ValueUnsignedInt,
                    #[serde(rename = "_valueUnsignedInt")]
                    ValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "valueUri")]
                    ValueUri,
                    #[serde(rename = "_valueUri")]
                    ValueUriPrimitiveElement,
                    #[serde(rename = "valueUrl")]
                    ValueUrl,
                    #[serde(rename = "_valueUrl")]
                    ValueUrlPrimitiveElement,
                    #[serde(rename = "valueUuid")]
                    ValueUuid,
                    #[serde(rename = "_valueUuid")]
                    ValueUuidPrimitiveElement,
                    #[serde(rename = "valueAddress")]
                    ValueAddress,
                    #[serde(rename = "valueAge")]
                    ValueAge,
                    #[serde(rename = "valueAnnotation")]
                    ValueAnnotation,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueCodeableReference")]
                    ValueCodeableReference,
                    #[serde(rename = "valueCoding")]
                    ValueCoding,
                    #[serde(rename = "valueContactPoint")]
                    ValueContactPoint,
                    #[serde(rename = "valueCount")]
                    ValueCount,
                    #[serde(rename = "valueDistance")]
                    ValueDistance,
                    #[serde(rename = "valueDuration")]
                    ValueDuration,
                    #[serde(rename = "valueHumanName")]
                    ValueHumanName,
                    #[serde(rename = "valueIdentifier")]
                    ValueIdentifier,
                    #[serde(rename = "valueMoney")]
                    ValueMoney,
                    #[serde(rename = "valuePeriod")]
                    ValuePeriod,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "valueRatio")]
                    ValueRatio,
                    #[serde(rename = "valueRatioRange")]
                    ValueRatioRange,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    #[serde(rename = "valueSampledData")]
                    ValueSampledData,
                    #[serde(rename = "valueSignature")]
                    ValueSignature,
                    #[serde(rename = "valueTiming")]
                    ValueTiming,
                    #[serde(rename = "valueContactDetail")]
                    ValueContactDetail,
                    #[serde(rename = "valueContributor")]
                    ValueContributor,
                    #[serde(rename = "valueDataRequirement")]
                    ValueDataRequirement,
                    #[serde(rename = "valueExpression")]
                    ValueExpression,
                    #[serde(rename = "valueParameterDefinition")]
                    ValueParameterDefinition,
                    #[serde(rename = "valueRelatedArtifact")]
                    ValueRelatedArtifact,
                    #[serde(rename = "valueTriggerDefinition")]
                    ValueTriggerDefinition,
                    #[serde(rename = "valueUsageContext")]
                    ValueUsageContext,
                    #[serde(rename = "valueDosage")]
                    ValueDosage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "label",
                            "valueBase64Binary",
                            "valueBoolean",
                            "valueCanonical",
                            "valueCode",
                            "valueDate",
                            "valueDateTime",
                            "valueDecimal",
                            "valueId",
                            "valueInstant",
                            "valueInteger",
                            "valueMarkdown",
                            "valueOid",
                            "valuePositiveInt",
                            "valueString",
                            "valueTime",
                            "valueUnsignedInt",
                            "valueUri",
                            "valueUrl",
                            "valueUuid",
                            "valueAddress",
                            "valueAge",
                            "valueAnnotation",
                            "valueAttachment",
                            "valueCodeableConcept",
                            "valueCodeableReference",
                            "valueCoding",
                            "valueContactPoint",
                            "valueCount",
                            "valueDistance",
                            "valueDuration",
                            "valueHumanName",
                            "valueIdentifier",
                            "valueMoney",
                            "valuePeriod",
                            "valueQuantity",
                            "valueRange",
                            "valueRatio",
                            "valueRatioRange",
                            "valueReference",
                            "valueSampledData",
                            "valueSignature",
                            "valueTiming",
                            "valueContactDetail",
                            "valueContributor",
                            "valueDataRequirement",
                            "valueExpression",
                            "valueParameterDefinition",
                            "valueRelatedArtifact",
                            "valueTriggerDefinition",
                            "valueUsageContext",
                            "valueDosage",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#label: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#value: Option<fhirbolt_model::r4b::types::ElementDefinitionExampleValue> =
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
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Label => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#label = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::ValueBase64Binary => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Base64Binary>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Base64Binary(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBase64Binary",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBase64Binary");
                            }
                        }
                        Field::ValueBoolean => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Boolean>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueCanonical => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCanonical",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Canonical>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueCanonicalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCanonical");
                            }
                        }
                        Field::ValueCode => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueCode"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Code>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Code(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueCodePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueCode",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueCode");
                            }
                        }
                        Field::ValueDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
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
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueId => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueId"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueId"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Id>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Id(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueIdPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueId"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueId");
                            }
                        }
                        Field::ValueInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInstant"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInstant");
                            }
                        }
                        Field::ValueInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueMarkdown => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueMarkdown",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Markdown>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::Markdown(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueMarkdown",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueOid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueOid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueOid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Oid>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Oid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueOidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueOid"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueOid");
                            }
                        }
                        Field::ValuePositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valuePositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valuePositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValuePositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valuePositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valuePositiveInt");
                            }
                        }
                        Field::ValueString => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueTime"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#value = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::ValueUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#value.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUnsignedInt");
                            }
                        }
                        Field::ValueUri => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUri"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uri>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUri"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueUrl => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUrl"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUrl"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Url>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Url(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUrlPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_valueUrl"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUrl");
                            }
                        }
                        Field::ValueUuid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueUuid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("value[x]"));
                                }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUuid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uuid>,
                                > = self.0.transmute();
                                r#value =
                                    Some(_Enum::Uuid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::ValueUuidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_valueUuid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_value[x]"));
                                }
                            } else {
                                return unknown_field_error("valueUuid");
                            }
                        }
                        Field::ValueAddress => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Address>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAge => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Age>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Age(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueAnnotation => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAnnotation"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Annotation>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Annotation(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueAttachment => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Attachment>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCodeableConcept => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCodeableReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::CodeableReference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCoding => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Coding>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueContactPoint => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContactPoint"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactPoint>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ContactPoint(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueCount => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Count>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Count(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueDistance => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDistance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Distance>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Distance(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueDuration => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueHumanName => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueHumanName"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::HumanName>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::HumanName(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueIdentifier => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Identifier>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueMoney => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValuePeriod => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valuePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueRatio => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueRatioRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRatioRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RatioRange>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueSampledData => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSampledData"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::SampledData>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::SampledData(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueSignature => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueSignature"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Signature>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Signature(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueTiming => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueTiming"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Timing>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueContactDetail => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueContactDetail",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactDetail>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ContactDetail(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueContributor => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueContributor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Contributor>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Contributor(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueDataRequirement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueDataRequirement",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::DataRequirement>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::DataRequirement(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueExpression => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueExpression"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Expression>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::Expression(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueParameterDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueParameterDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ParameterDefinition>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::ParameterDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueRelatedArtifact => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueRelatedArtifact",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RelatedArtifact>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::RelatedArtifact(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueTriggerDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueTriggerDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::TriggerDefinition>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::TriggerDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueUsageContext => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueUsageContext"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::UsageContext>,
                            > = self.0.transmute();
                            r#value = Some(_Enum::UsageContext(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::ValueDosage => {
                            use fhirbolt_model::r4b::types::ElementDefinitionExampleValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueDosage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Dosage>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Dosage(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ElementDefinitionExample {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#label: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#label.unwrap_or(Default::default())
                    } else {
                        r#label.ok_or(serde::de::Error::missing_field("label"))?
                    },
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionExample>>
{
    type Value = Box<ElementDefinitionExample>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionExample>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionExample>>
{
    type Value = Vec<ElementDefinitionExample>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionExample>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionExample>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionExample> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionConstraint;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionConstraint> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.constraint", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#key.id.as_deref() == Some("$invalid") {
                return missing_field_error("key");
            }
            if let Some(some) = self.value.r#key.value.as_ref().map(Ok) {
                state.serialize_entry("key", &some?)?;
            }
            if self.value.r#key.id.is_some() || !self.value.r#key.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#key.id.as_ref(),
                    extension: &self.value.r#key.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_key", ctx))?;
            }
        } else if self.value.r#key.id.as_deref() == Some("$invalid") {
            return missing_field_error("key");
        }
        self.with_context(&self.value.r#key, |ctx| state.serialize_entry("key", ctx))?;
        if self.output_json {
            if let Some(some) = self.value.r#requirements.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requirements", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requirements", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#requirements.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requirements", ctx))?;
        }
        if self.output_json {
            if self.value.r#severity.id.as_deref() == Some("$invalid") {
                return missing_field_error("severity");
            }
            if let Some(some) = self.value.r#severity.value.as_ref().map(Ok) {
                state.serialize_entry("severity", &some?)?;
            }
            if self.value.r#severity.id.is_some() || !self.value.r#severity.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#severity.id.as_ref(),
                    extension: &self.value.r#severity.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_severity", ctx)
                })?;
            }
        } else if self.value.r#severity.id.as_deref() == Some("$invalid") {
            return missing_field_error("severity");
        }
        self.with_context(&self.value.r#severity, |ctx| {
            state.serialize_entry("severity", ctx)
        })?;
        if self.output_json {
            if self.value.r#human.id.as_deref() == Some("$invalid") {
                return missing_field_error("human");
            }
            if let Some(some) = self.value.r#human.value.as_ref().map(Ok) {
                state.serialize_entry("human", &some?)?;
            }
            if self.value.r#human.id.is_some() || !self.value.r#human.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#human.id.as_ref(),
                    extension: &self.value.r#human.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_human", ctx)
                })?;
            }
        } else if self.value.r#human.id.as_deref() == Some("$invalid") {
            return missing_field_error("human");
        }
        self.with_context(&self.value.r#human, |ctx| {
            state.serialize_entry("human", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#expression.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("expression", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_expression", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#expression.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expression", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#xpath.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("xpath", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_xpath", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#xpath.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("xpath", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#source.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("source", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_source", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#source.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("source", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionConstraint>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionConstraint>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionConstraint>
{
    type Value = ElementDefinitionConstraint;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionConstraint>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionConstraint;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionConstraint")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<ElementDefinitionConstraint, V::Error>
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
                    #[serde(rename = "key")]
                    Key,
                    #[serde(rename = "_key")]
                    KeyPrimitiveElement,
                    #[serde(rename = "requirements")]
                    Requirements,
                    #[serde(rename = "_requirements")]
                    RequirementsPrimitiveElement,
                    #[serde(rename = "severity")]
                    Severity,
                    #[serde(rename = "_severity")]
                    SeverityPrimitiveElement,
                    #[serde(rename = "human")]
                    Human,
                    #[serde(rename = "_human")]
                    HumanPrimitiveElement,
                    #[serde(rename = "expression")]
                    Expression,
                    #[serde(rename = "_expression")]
                    ExpressionPrimitiveElement,
                    #[serde(rename = "xpath")]
                    Xpath,
                    #[serde(rename = "_xpath")]
                    XpathPrimitiveElement,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "_source")]
                    SourcePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "key",
                            "requirements",
                            "severity",
                            "human",
                            "expression",
                            "xpath",
                            "source",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#key: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#requirements: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#severity: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#human: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#expression: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#xpath: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#source: Option<fhirbolt_model::r4b::types::Canonical> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Key => {
                            if self.0.from_json {
                                let some = r#key.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("key"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#key.is_some() {
                                    return Err(serde::de::Error::duplicate_field("key"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                r#key = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::KeyPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#key.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_key"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("key");
                            }
                        }
                        Field::Requirements => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#requirements.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#requirements = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequirementsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requirements"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requirements");
                            }
                        }
                        Field::Severity => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#severity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#severity = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SeverityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_severity"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("severity");
                            }
                        }
                        Field::Human => {
                            if self.0.from_json {
                                let some = r#human.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("human"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#human.is_some() {
                                    return Err(serde::de::Error::duplicate_field("human"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#human = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::HumanPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#human.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_human"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("human");
                            }
                        }
                        Field::Expression => {
                            if self.0.from_json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#expression.is_some() {
                                    return Err(serde::de::Error::duplicate_field("expression"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#expression = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExpressionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#expression.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_expression"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("expression");
                            }
                        }
                        Field::Xpath => {
                            if self.0.from_json {
                                let some = r#xpath.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("xpath"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#xpath.is_some() {
                                    return Err(serde::de::Error::duplicate_field("xpath"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#xpath = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::XpathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#xpath.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_xpath"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("xpath");
                            }
                        }
                        Field::Source => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Canonical,
                                > = self.0.transmute();
                                r#source = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SourcePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_source"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("source");
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
                Ok(ElementDefinitionConstraint {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#key: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#key.unwrap_or(Default::default())
                    } else {
                        r#key.ok_or(serde::de::Error::missing_field("key"))?
                    },
                    r#requirements,
                    r#severity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#severity.unwrap_or(Default::default())
                    } else {
                        r#severity.ok_or(serde::de::Error::missing_field("severity"))?
                    },
                    r#human: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#human.unwrap_or(Default::default())
                    } else {
                        r#human.ok_or(serde::de::Error::missing_field("human"))?
                    },
                    r#expression,
                    r#xpath,
                    r#source,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionConstraint>>
{
    type Value = Box<ElementDefinitionConstraint>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionConstraint>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionConstraint>>
{
    type Value = Vec<ElementDefinitionConstraint>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionConstraint>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionConstraint>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionConstraint> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionBinding;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionBinding> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.binding", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#strength.id.as_deref() == Some("$invalid") {
                return missing_field_error("strength");
            }
            if let Some(some) = self.value.r#strength.value.as_ref().map(Ok) {
                state.serialize_entry("strength", &some?)?;
            }
            if self.value.r#strength.id.is_some() || !self.value.r#strength.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#strength.id.as_ref(),
                    extension: &self.value.r#strength.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_strength", ctx)
                })?;
            }
        } else if self.value.r#strength.id.as_deref() == Some("$invalid") {
            return missing_field_error("strength");
        }
        self.with_context(&self.value.r#strength, |ctx| {
            state.serialize_entry("strength", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#value_set.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("valueSet", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_valueSet", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#value_set.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("valueSet", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionBinding>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionBinding>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionBinding>
{
    type Value = ElementDefinitionBinding;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionBinding>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionBinding;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionBinding")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionBinding, V::Error>
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
                    #[serde(rename = "strength")]
                    Strength,
                    #[serde(rename = "_strength")]
                    StrengthPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "valueSet")]
                    ValueSet,
                    #[serde(rename = "_valueSet")]
                    ValueSetPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "strength", "description", "valueSet"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#strength: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#value_set: Option<fhirbolt_model::r4b::types::Canonical> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Strength => {
                            if self.0.from_json {
                                let some = r#strength.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#strength = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StrengthPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#strength.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_strength"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("strength");
                            }
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::ValueSet => {
                            if self.0.from_json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#value_set.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueSet"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Canonical,
                                > = self.0.transmute();
                                r#value_set = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ValueSetPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#value_set.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_valueSet"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("valueSet");
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
                Ok(ElementDefinitionBinding {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#strength: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#strength.unwrap_or(Default::default())
                    } else {
                        r#strength.ok_or(serde::de::Error::missing_field("strength"))?
                    },
                    r#description,
                    r#value_set,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionBinding>>
{
    type Value = Box<ElementDefinitionBinding>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionBinding>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionBinding>>
{
    type Value = Vec<ElementDefinitionBinding>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionBinding>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionBinding>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionBinding> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinitionMapping;
impl serde::ser::Serialize for SerializationContext<&ElementDefinitionMapping> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition.mapping", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#identity.id.as_deref() == Some("$invalid") {
                return missing_field_error("identity");
            }
            if let Some(some) = self.value.r#identity.value.as_ref().map(Ok) {
                state.serialize_entry("identity", &some?)?;
            }
            if self.value.r#identity.id.is_some() || !self.value.r#identity.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#identity.id.as_ref(),
                    extension: &self.value.r#identity.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_identity", ctx)
                })?;
            }
        } else if self.value.r#identity.id.as_deref() == Some("$invalid") {
            return missing_field_error("identity");
        }
        self.with_context(&self.value.r#identity, |ctx| {
            state.serialize_entry("identity", ctx)
        })?;
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
        }
        if self.output_json {
            if self.value.r#map.id.as_deref() == Some("$invalid") {
                return missing_field_error("map");
            }
            if let Some(some) = self.value.r#map.value.as_ref().map(Ok) {
                state.serialize_entry("map", &some?)?;
            }
            if self.value.r#map.id.is_some() || !self.value.r#map.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#map.id.as_ref(),
                    extension: &self.value.r#map.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_map", ctx))?;
            }
        } else if self.value.r#map.id.as_deref() == Some("$invalid") {
            return missing_field_error("map");
        }
        self.with_context(&self.value.r#map, |ctx| state.serialize_entry("map", ctx))?;
        if self.output_json {
            if let Some(some) = self.value.r#comment.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("comment", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_comment", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#comment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("comment", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinitionMapping>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinitionMapping>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<ElementDefinitionMapping>
{
    type Value = ElementDefinitionMapping;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinitionMapping>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinitionMapping;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinitionMapping")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinitionMapping, V::Error>
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
                    #[serde(rename = "identity")]
                    Identity,
                    #[serde(rename = "_identity")]
                    IdentityPrimitiveElement,
                    #[serde(rename = "language")]
                    Language,
                    #[serde(rename = "_language")]
                    LanguagePrimitiveElement,
                    #[serde(rename = "map")]
                    Map,
                    #[serde(rename = "_map")]
                    MapPrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "identity", "language", "map", "comment"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#identity: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#map: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#comment: Option<fhirbolt_model::r4b::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identity => {
                            if self.0.from_json {
                                let some = r#identity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identity"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#identity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identity"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                r#identity = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdentityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#identity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_identity"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("identity");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Map => {
                            if self.0.from_json {
                                let some = r#map.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("map"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#map.is_some() {
                                    return Err(serde::de::Error::duplicate_field("map"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#map = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MapPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#map.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_map"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("map");
                            }
                        }
                        Field::Comment => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#comment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#comment = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comment"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comment");
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
                Ok(ElementDefinitionMapping {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#identity: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#identity.unwrap_or(Default::default())
                    } else {
                        r#identity.ok_or(serde::de::Error::missing_field("identity"))?
                    },
                    r#language,
                    r#map: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#map.unwrap_or(Default::default())
                    } else {
                        r#map.ok_or(serde::de::Error::missing_field("map"))?
                    },
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<ElementDefinitionMapping>>
{
    type Value = Box<ElementDefinitionMapping>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinitionMapping>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<ElementDefinitionMapping>>
{
    type Value = Vec<ElementDefinitionMapping>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinitionMapping>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinitionMapping>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinitionMapping> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::types::ElementDefinition;
impl serde::ser::Serialize for SerializationContext<&ElementDefinition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ElementDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if self.output_json {
            if self.value.r#path.id.as_deref() == Some("$invalid") {
                return missing_field_error("path");
            }
            if let Some(some) = self.value.r#path.value.as_ref().map(Ok) {
                state.serialize_entry("path", &some?)?;
            }
            if self.value.r#path.id.is_some() || !self.value.r#path.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#path.id.as_ref(),
                    extension: &self.value.r#path.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_path", ctx)
                })?;
            }
        } else if self.value.r#path.id.as_deref() == Some("$invalid") {
            return missing_field_error("path");
        }
        self.with_context(&self.value.r#path, |ctx| state.serialize_entry("path", ctx))?;
        if self.output_json {
            if !self.value.r#representation.is_empty() {
                let values = self
                    .value
                    .r#representation
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("representation", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#representation
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#representation
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_representation", ctx)
                    })?;
                }
            }
        } else if !self.value.r#representation.is_empty() {
            self.with_context(&self.value.r#representation, |ctx| {
                state.serialize_entry("representation", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#slice_name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sliceName", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sliceName", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#slice_name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sliceName", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#slice_is_constraining.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("sliceIsConstraining", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sliceIsConstraining", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#slice_is_constraining.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("sliceIsConstraining", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#label.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("label", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_label", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#label.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("label", ctx))?;
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if let Some(some) = self.value.r#slicing.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("slicing", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#short.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("short", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_short", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#short.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("short", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#definition.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("definition", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_definition", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#definition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("definition", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#comment.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("comment", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_comment", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#comment.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("comment", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#requirements.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requirements", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requirements", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#requirements.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requirements", ctx))?;
        }
        if self.output_json {
            if !self.value.r#alias.is_empty() {
                let values = self
                    .value
                    .r#alias
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("alias", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#alias
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#alias
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_alias", ctx)
                    })?;
                }
            }
        } else if !self.value.r#alias.is_empty() {
            self.with_context(&self.value.r#alias, |ctx| {
                state.serialize_entry("alias", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#min.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("min", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_min", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#min.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("min", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#max.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("max", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_max", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#max.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("max", ctx))?;
        }
        if let Some(some) = self.value.r#base.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("base", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#content_reference.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("contentReference", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_contentReference", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#content_reference.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("contentReference", ctx))?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        {
            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
            if let Some(some) = self.value.r#default_value.as_ref() {
                match some {
                    _Enum::Base64Binary(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueBase64Binary", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueBase64Binary", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueBase64Binary", ctx)
                            })?;
                        }
                    }
                    _Enum::Boolean(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueBoolean", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::Canonical(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueCanonical", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueCanonical", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueCanonical", ctx)
                            })?;
                        }
                    }
                    _Enum::Code(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueCode", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueCode", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueCode", ctx)
                            })?;
                        }
                    }
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueDateTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueDateTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Decimal(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("defaultValueDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Id(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueId", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueId", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueId", ctx)
                            })?;
                        }
                    }
                    _Enum::Instant(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::Markdown(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueMarkdown", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueMarkdown", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueMarkdown", ctx)
                            })?;
                        }
                    }
                    _Enum::Oid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueOid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueOid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueOid", ctx)
                            })?;
                        }
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValuePositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValuePositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValuePositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueString", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueTime", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Uri(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueUri", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueUri", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueUri", ctx)
                            })?;
                        }
                    }
                    _Enum::Url(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueUrl", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueUrl", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueUrl", ctx)
                            })?;
                        }
                    }
                    _Enum::Uuid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("defaultValueUuid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_defaultValueUuid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("defaultValueUuid", ctx)
                            })?;
                        }
                    }
                    _Enum::Address(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueAddress", ctx)
                        })?;
                    }
                    _Enum::Age(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueAge", ctx)
                        })?;
                    }
                    _Enum::Annotation(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueAnnotation", ctx)
                        })?;
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueAttachment", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::CodeableReference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCodeableReference", ctx)
                        })?;
                    }
                    _Enum::Coding(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCoding", ctx)
                        })?;
                    }
                    _Enum::ContactPoint(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueContactPoint", ctx)
                        })?;
                    }
                    _Enum::Count(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueCount", ctx)
                        })?;
                    }
                    _Enum::Distance(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDistance", ctx)
                        })?;
                    }
                    _Enum::Duration(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDuration", ctx)
                        })?;
                    }
                    _Enum::HumanName(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueHumanName", ctx)
                        })?;
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueIdentifier", ctx)
                        })?;
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueMoney", ctx)
                        })?;
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValuePeriod", ctx)
                        })?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueRange", ctx)
                        })?;
                    }
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueRatio", ctx)
                        })?;
                    }
                    _Enum::RatioRange(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueRatioRange", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueReference", ctx)
                        })?;
                    }
                    _Enum::SampledData(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueSampledData", ctx)
                        })?;
                    }
                    _Enum::Signature(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueSignature", ctx)
                        })?;
                    }
                    _Enum::Timing(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueTiming", ctx)
                        })?;
                    }
                    _Enum::ContactDetail(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueContactDetail", ctx)
                        })?;
                    }
                    _Enum::Contributor(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueContributor", ctx)
                        })?;
                    }
                    _Enum::DataRequirement(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDataRequirement", ctx)
                        })?;
                    }
                    _Enum::Expression(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueExpression", ctx)
                        })?;
                    }
                    _Enum::ParameterDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueParameterDefinition", ctx)
                        })?;
                    }
                    _Enum::RelatedArtifact(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueRelatedArtifact", ctx)
                        })?;
                    }
                    _Enum::TriggerDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueTriggerDefinition", ctx)
                        })?;
                    }
                    _Enum::UsageContext(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueUsageContext", ctx)
                        })?;
                    }
                    _Enum::Dosage(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("defaultValueDosage", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("default_value is invalid"))
                    }
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#meaning_when_missing.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("meaningWhenMissing", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_meaningWhenMissing", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#meaning_when_missing.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meaningWhenMissing", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#order_meaning.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("orderMeaning", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_orderMeaning", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#order_meaning.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("orderMeaning", ctx))?;
        }
        {
            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
            if let Some(some) = self.value.r#fixed.as_ref() {
                match some {
                    _Enum::Base64Binary(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedBase64Binary", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedBase64Binary", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedBase64Binary", ctx)
                            })?;
                        }
                    }
                    _Enum::Boolean(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedBoolean", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::Canonical(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedCanonical", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedCanonical", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedCanonical", ctx)
                            })?;
                        }
                    }
                    _Enum::Code(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedCode", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedCode", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedCode", ctx)
                            })?;
                        }
                    }
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedDateTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedDateTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Decimal(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("fixedDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Id(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedId", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedId", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| state.serialize_entry("fixedId", ctx))?;
                        }
                    }
                    _Enum::Instant(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::Markdown(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedMarkdown", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedMarkdown", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedMarkdown", ctx)
                            })?;
                        }
                    }
                    _Enum::Oid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedOid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedOid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| state.serialize_entry("fixedOid", ctx))?;
                        }
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedPositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedPositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedPositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedString", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedTime", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Uri(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedUri", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedUri", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| state.serialize_entry("fixedUri", ctx))?;
                        }
                    }
                    _Enum::Url(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedUrl", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedUrl", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| state.serialize_entry("fixedUrl", ctx))?;
                        }
                    }
                    _Enum::Uuid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("fixedUuid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_fixedUuid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("fixedUuid", ctx)
                            })?;
                        }
                    }
                    _Enum::Address(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedAddress", ctx))?;
                    }
                    _Enum::Age(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedAge", ctx))?;
                    }
                    _Enum::Annotation(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedAnnotation", ctx)
                        })?;
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedAttachment", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::CodeableReference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedCodeableReference", ctx)
                        })?;
                    }
                    _Enum::Coding(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedCoding", ctx))?;
                    }
                    _Enum::ContactPoint(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedContactPoint", ctx)
                        })?;
                    }
                    _Enum::Count(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedCount", ctx))?;
                    }
                    _Enum::Distance(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedDistance", ctx)
                        })?;
                    }
                    _Enum::Duration(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedDuration", ctx)
                        })?;
                    }
                    _Enum::HumanName(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedHumanName", ctx)
                        })?;
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedIdentifier", ctx)
                        })?;
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedMoney", ctx))?;
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedPeriod", ctx))?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedRange", ctx))?;
                    }
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedRatio", ctx))?;
                    }
                    _Enum::RatioRange(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedRatioRange", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedReference", ctx)
                        })?;
                    }
                    _Enum::SampledData(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedSampledData", ctx)
                        })?;
                    }
                    _Enum::Signature(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedSignature", ctx)
                        })?;
                    }
                    _Enum::Timing(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedTiming", ctx))?;
                    }
                    _Enum::ContactDetail(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedContactDetail", ctx)
                        })?;
                    }
                    _Enum::Contributor(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedContributor", ctx)
                        })?;
                    }
                    _Enum::DataRequirement(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedDataRequirement", ctx)
                        })?;
                    }
                    _Enum::Expression(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedExpression", ctx)
                        })?;
                    }
                    _Enum::ParameterDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedParameterDefinition", ctx)
                        })?;
                    }
                    _Enum::RelatedArtifact(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedRelatedArtifact", ctx)
                        })?;
                    }
                    _Enum::TriggerDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedTriggerDefinition", ctx)
                        })?;
                    }
                    _Enum::UsageContext(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("fixedUsageContext", ctx)
                        })?;
                    }
                    _Enum::Dosage(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("fixedDosage", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("fixed is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
            if let Some(some) = self.value.r#pattern.as_ref() {
                match some {
                    _Enum::Base64Binary(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternBase64Binary", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternBase64Binary", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternBase64Binary", ctx)
                            })?;
                        }
                    }
                    _Enum::Boolean(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternBoolean", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternBoolean", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::Canonical(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternCanonical", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternCanonical", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternCanonical", ctx)
                            })?;
                        }
                    }
                    _Enum::Code(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternCode", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternCode", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternCode", ctx)
                            })?;
                        }
                    }
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternDateTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternDateTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Decimal(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("patternDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Id(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternId", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternId", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternId", ctx)
                            })?;
                        }
                    }
                    _Enum::Instant(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::Markdown(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternMarkdown", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternMarkdown", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternMarkdown", ctx)
                            })?;
                        }
                    }
                    _Enum::Oid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternOid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternOid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternOid", ctx)
                            })?;
                        }
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternPositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternPositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternPositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternString", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternTime", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Uri(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternUri", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternUri", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternUri", ctx)
                            })?;
                        }
                    }
                    _Enum::Url(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternUrl", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternUrl", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternUrl", ctx)
                            })?;
                        }
                    }
                    _Enum::Uuid(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("patternUuid", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_patternUuid", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("patternUuid", ctx)
                            })?;
                        }
                    }
                    _Enum::Address(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternAddress", ctx)
                        })?;
                    }
                    _Enum::Age(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("patternAge", ctx))?;
                    }
                    _Enum::Annotation(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternAnnotation", ctx)
                        })?;
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternAttachment", ctx)
                        })?;
                    }
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::CodeableReference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternCodeableReference", ctx)
                        })?;
                    }
                    _Enum::Coding(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternCoding", ctx)
                        })?;
                    }
                    _Enum::ContactPoint(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternContactPoint", ctx)
                        })?;
                    }
                    _Enum::Count(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("patternCount", ctx))?;
                    }
                    _Enum::Distance(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDistance", ctx)
                        })?;
                    }
                    _Enum::Duration(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDuration", ctx)
                        })?;
                    }
                    _Enum::HumanName(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternHumanName", ctx)
                        })?;
                    }
                    _Enum::Identifier(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternIdentifier", ctx)
                        })?;
                    }
                    _Enum::Money(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("patternMoney", ctx))?;
                    }
                    _Enum::Period(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternPeriod", ctx)
                        })?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternQuantity", ctx)
                        })?;
                    }
                    _Enum::Range(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("patternRange", ctx))?;
                    }
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("patternRatio", ctx))?;
                    }
                    _Enum::RatioRange(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternRatioRange", ctx)
                        })?;
                    }
                    _Enum::Reference(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternReference", ctx)
                        })?;
                    }
                    _Enum::SampledData(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternSampledData", ctx)
                        })?;
                    }
                    _Enum::Signature(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternSignature", ctx)
                        })?;
                    }
                    _Enum::Timing(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternTiming", ctx)
                        })?;
                    }
                    _Enum::ContactDetail(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternContactDetail", ctx)
                        })?;
                    }
                    _Enum::Contributor(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternContributor", ctx)
                        })?;
                    }
                    _Enum::DataRequirement(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDataRequirement", ctx)
                        })?;
                    }
                    _Enum::Expression(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternExpression", ctx)
                        })?;
                    }
                    _Enum::ParameterDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternParameterDefinition", ctx)
                        })?;
                    }
                    _Enum::RelatedArtifact(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternRelatedArtifact", ctx)
                        })?;
                    }
                    _Enum::TriggerDefinition(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternTriggerDefinition", ctx)
                        })?;
                    }
                    _Enum::UsageContext(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternUsageContext", ctx)
                        })?;
                    }
                    _Enum::Dosage(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("patternDosage", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("pattern is invalid")),
                }
            }
        }
        if !self.value.r#example.is_empty() {
            self.with_context(&self.value.r#example, |ctx| {
                state.serialize_entry("example", ctx)
            })?;
        }
        {
            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
            if let Some(some) = self.value.r#min_value.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueDateTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueDateTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Instant(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Decimal(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("minValueDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValuePositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValuePositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValuePositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minValueUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minValueUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minValueUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minValueQuantity", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("min_value is invalid"))
                    }
                }
            }
        }
        {
            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
            if let Some(some) = self.value.r#max_value.as_ref() {
                match some {
                    _Enum::Date(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueDate", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueDate", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueDate", ctx)
                            })?;
                        }
                    }
                    _Enum::DateTime(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueDateTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueDateTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueDateTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Instant(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueInstant", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueInstant", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueInstant", ctx)
                            })?;
                        }
                    }
                    _Enum::Time(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueTime", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueTime", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueTime", ctx)
                            })?;
                        }
                    }
                    _Enum::Decimal(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            }) {
                                state.serialize_entry("maxValueDecimal", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueDecimal", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueDecimal", ctx)
                            })?;
                        }
                    }
                    _Enum::Integer(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueInteger", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueInteger", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueInteger", ctx)
                            })?;
                        }
                    }
                    _Enum::PositiveInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValuePositiveInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValuePositiveInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValuePositiveInt", ctx)
                            })?;
                        }
                    }
                    _Enum::UnsignedInt(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("maxValueUnsignedInt", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_maxValueUnsignedInt", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("maxValueUnsignedInt", ctx)
                            })?;
                        }
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("maxValueQuantity", ctx)
                        })?;
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("max_value is invalid"))
                    }
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#max_length.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("maxLength", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_maxLength", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#max_length.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxLength", ctx))?;
        }
        if self.output_json {
            if !self.value.r#condition.is_empty() {
                let values = self
                    .value
                    .r#condition
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("condition", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#condition
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#condition
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_condition", ctx)
                    })?;
                }
            }
        } else if !self.value.r#condition.is_empty() {
            self.with_context(&self.value.r#condition, |ctx| {
                state.serialize_entry("condition", ctx)
            })?;
        }
        if !self.value.r#constraint.is_empty() {
            self.with_context(&self.value.r#constraint, |ctx| {
                state.serialize_entry("constraint", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#must_support.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("mustSupport", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_mustSupport", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#must_support.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("mustSupport", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_modifier.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("isModifier", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isModifier", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#is_modifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("isModifier", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_modifier_reason.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("isModifierReason", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isModifierReason", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#is_modifier_reason.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("isModifierReason", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_summary.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("isSummary", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isSummary", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#is_summary.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("isSummary", ctx))?;
        }
        if let Some(some) = self.value.r#binding.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("binding", ctx))?;
        }
        if !self.value.r#mapping.is_empty() {
            self.with_context(&self.value.r#mapping, |ctx| {
                state.serialize_entry("mapping", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ElementDefinition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ElementDefinition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ElementDefinition> {
    type Value = ElementDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ElementDefinition>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ElementDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ElementDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ElementDefinition, V::Error>
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
                    #[serde(rename = "modifierExtension")]
                    ModifierExtension,
                    #[serde(rename = "path")]
                    Path,
                    #[serde(rename = "_path")]
                    PathPrimitiveElement,
                    #[serde(rename = "representation")]
                    Representation,
                    #[serde(rename = "_representation")]
                    RepresentationPrimitiveElement,
                    #[serde(rename = "sliceName")]
                    SliceName,
                    #[serde(rename = "_sliceName")]
                    SliceNamePrimitiveElement,
                    #[serde(rename = "sliceIsConstraining")]
                    SliceIsConstraining,
                    #[serde(rename = "_sliceIsConstraining")]
                    SliceIsConstrainingPrimitiveElement,
                    #[serde(rename = "label")]
                    Label,
                    #[serde(rename = "_label")]
                    LabelPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "slicing")]
                    Slicing,
                    #[serde(rename = "short")]
                    Short,
                    #[serde(rename = "_short")]
                    ShortPrimitiveElement,
                    #[serde(rename = "definition")]
                    Definition,
                    #[serde(rename = "_definition")]
                    DefinitionPrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    #[serde(rename = "requirements")]
                    Requirements,
                    #[serde(rename = "_requirements")]
                    RequirementsPrimitiveElement,
                    #[serde(rename = "alias")]
                    Alias,
                    #[serde(rename = "_alias")]
                    AliasPrimitiveElement,
                    #[serde(rename = "min")]
                    Min,
                    #[serde(rename = "_min")]
                    MinPrimitiveElement,
                    #[serde(rename = "max")]
                    Max,
                    #[serde(rename = "_max")]
                    MaxPrimitiveElement,
                    #[serde(rename = "base")]
                    Base,
                    #[serde(rename = "contentReference")]
                    ContentReference,
                    #[serde(rename = "_contentReference")]
                    ContentReferencePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "defaultValueBase64Binary")]
                    DefaultValueBase64Binary,
                    #[serde(rename = "_defaultValueBase64Binary")]
                    DefaultValueBase64BinaryPrimitiveElement,
                    #[serde(rename = "defaultValueBoolean")]
                    DefaultValueBoolean,
                    #[serde(rename = "_defaultValueBoolean")]
                    DefaultValueBooleanPrimitiveElement,
                    #[serde(rename = "defaultValueCanonical")]
                    DefaultValueCanonical,
                    #[serde(rename = "_defaultValueCanonical")]
                    DefaultValueCanonicalPrimitiveElement,
                    #[serde(rename = "defaultValueCode")]
                    DefaultValueCode,
                    #[serde(rename = "_defaultValueCode")]
                    DefaultValueCodePrimitiveElement,
                    #[serde(rename = "defaultValueDate")]
                    DefaultValueDate,
                    #[serde(rename = "_defaultValueDate")]
                    DefaultValueDatePrimitiveElement,
                    #[serde(rename = "defaultValueDateTime")]
                    DefaultValueDateTime,
                    #[serde(rename = "_defaultValueDateTime")]
                    DefaultValueDateTimePrimitiveElement,
                    #[serde(rename = "defaultValueDecimal")]
                    DefaultValueDecimal,
                    #[serde(rename = "_defaultValueDecimal")]
                    DefaultValueDecimalPrimitiveElement,
                    #[serde(rename = "defaultValueId")]
                    DefaultValueId,
                    #[serde(rename = "_defaultValueId")]
                    DefaultValueIdPrimitiveElement,
                    #[serde(rename = "defaultValueInstant")]
                    DefaultValueInstant,
                    #[serde(rename = "_defaultValueInstant")]
                    DefaultValueInstantPrimitiveElement,
                    #[serde(rename = "defaultValueInteger")]
                    DefaultValueInteger,
                    #[serde(rename = "_defaultValueInteger")]
                    DefaultValueIntegerPrimitiveElement,
                    #[serde(rename = "defaultValueMarkdown")]
                    DefaultValueMarkdown,
                    #[serde(rename = "_defaultValueMarkdown")]
                    DefaultValueMarkdownPrimitiveElement,
                    #[serde(rename = "defaultValueOid")]
                    DefaultValueOid,
                    #[serde(rename = "_defaultValueOid")]
                    DefaultValueOidPrimitiveElement,
                    #[serde(rename = "defaultValuePositiveInt")]
                    DefaultValuePositiveInt,
                    #[serde(rename = "_defaultValuePositiveInt")]
                    DefaultValuePositiveIntPrimitiveElement,
                    #[serde(rename = "defaultValueString")]
                    DefaultValueString,
                    #[serde(rename = "_defaultValueString")]
                    DefaultValueStringPrimitiveElement,
                    #[serde(rename = "defaultValueTime")]
                    DefaultValueTime,
                    #[serde(rename = "_defaultValueTime")]
                    DefaultValueTimePrimitiveElement,
                    #[serde(rename = "defaultValueUnsignedInt")]
                    DefaultValueUnsignedInt,
                    #[serde(rename = "_defaultValueUnsignedInt")]
                    DefaultValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "defaultValueUri")]
                    DefaultValueUri,
                    #[serde(rename = "_defaultValueUri")]
                    DefaultValueUriPrimitiveElement,
                    #[serde(rename = "defaultValueUrl")]
                    DefaultValueUrl,
                    #[serde(rename = "_defaultValueUrl")]
                    DefaultValueUrlPrimitiveElement,
                    #[serde(rename = "defaultValueUuid")]
                    DefaultValueUuid,
                    #[serde(rename = "_defaultValueUuid")]
                    DefaultValueUuidPrimitiveElement,
                    #[serde(rename = "defaultValueAddress")]
                    DefaultValueAddress,
                    #[serde(rename = "defaultValueAge")]
                    DefaultValueAge,
                    #[serde(rename = "defaultValueAnnotation")]
                    DefaultValueAnnotation,
                    #[serde(rename = "defaultValueAttachment")]
                    DefaultValueAttachment,
                    #[serde(rename = "defaultValueCodeableConcept")]
                    DefaultValueCodeableConcept,
                    #[serde(rename = "defaultValueCodeableReference")]
                    DefaultValueCodeableReference,
                    #[serde(rename = "defaultValueCoding")]
                    DefaultValueCoding,
                    #[serde(rename = "defaultValueContactPoint")]
                    DefaultValueContactPoint,
                    #[serde(rename = "defaultValueCount")]
                    DefaultValueCount,
                    #[serde(rename = "defaultValueDistance")]
                    DefaultValueDistance,
                    #[serde(rename = "defaultValueDuration")]
                    DefaultValueDuration,
                    #[serde(rename = "defaultValueHumanName")]
                    DefaultValueHumanName,
                    #[serde(rename = "defaultValueIdentifier")]
                    DefaultValueIdentifier,
                    #[serde(rename = "defaultValueMoney")]
                    DefaultValueMoney,
                    #[serde(rename = "defaultValuePeriod")]
                    DefaultValuePeriod,
                    #[serde(rename = "defaultValueQuantity")]
                    DefaultValueQuantity,
                    #[serde(rename = "defaultValueRange")]
                    DefaultValueRange,
                    #[serde(rename = "defaultValueRatio")]
                    DefaultValueRatio,
                    #[serde(rename = "defaultValueRatioRange")]
                    DefaultValueRatioRange,
                    #[serde(rename = "defaultValueReference")]
                    DefaultValueReference,
                    #[serde(rename = "defaultValueSampledData")]
                    DefaultValueSampledData,
                    #[serde(rename = "defaultValueSignature")]
                    DefaultValueSignature,
                    #[serde(rename = "defaultValueTiming")]
                    DefaultValueTiming,
                    #[serde(rename = "defaultValueContactDetail")]
                    DefaultValueContactDetail,
                    #[serde(rename = "defaultValueContributor")]
                    DefaultValueContributor,
                    #[serde(rename = "defaultValueDataRequirement")]
                    DefaultValueDataRequirement,
                    #[serde(rename = "defaultValueExpression")]
                    DefaultValueExpression,
                    #[serde(rename = "defaultValueParameterDefinition")]
                    DefaultValueParameterDefinition,
                    #[serde(rename = "defaultValueRelatedArtifact")]
                    DefaultValueRelatedArtifact,
                    #[serde(rename = "defaultValueTriggerDefinition")]
                    DefaultValueTriggerDefinition,
                    #[serde(rename = "defaultValueUsageContext")]
                    DefaultValueUsageContext,
                    #[serde(rename = "defaultValueDosage")]
                    DefaultValueDosage,
                    #[serde(rename = "meaningWhenMissing")]
                    MeaningWhenMissing,
                    #[serde(rename = "_meaningWhenMissing")]
                    MeaningWhenMissingPrimitiveElement,
                    #[serde(rename = "orderMeaning")]
                    OrderMeaning,
                    #[serde(rename = "_orderMeaning")]
                    OrderMeaningPrimitiveElement,
                    #[serde(rename = "fixedBase64Binary")]
                    FixedBase64Binary,
                    #[serde(rename = "_fixedBase64Binary")]
                    FixedBase64BinaryPrimitiveElement,
                    #[serde(rename = "fixedBoolean")]
                    FixedBoolean,
                    #[serde(rename = "_fixedBoolean")]
                    FixedBooleanPrimitiveElement,
                    #[serde(rename = "fixedCanonical")]
                    FixedCanonical,
                    #[serde(rename = "_fixedCanonical")]
                    FixedCanonicalPrimitiveElement,
                    #[serde(rename = "fixedCode")]
                    FixedCode,
                    #[serde(rename = "_fixedCode")]
                    FixedCodePrimitiveElement,
                    #[serde(rename = "fixedDate")]
                    FixedDate,
                    #[serde(rename = "_fixedDate")]
                    FixedDatePrimitiveElement,
                    #[serde(rename = "fixedDateTime")]
                    FixedDateTime,
                    #[serde(rename = "_fixedDateTime")]
                    FixedDateTimePrimitiveElement,
                    #[serde(rename = "fixedDecimal")]
                    FixedDecimal,
                    #[serde(rename = "_fixedDecimal")]
                    FixedDecimalPrimitiveElement,
                    #[serde(rename = "fixedId")]
                    FixedId,
                    #[serde(rename = "_fixedId")]
                    FixedIdPrimitiveElement,
                    #[serde(rename = "fixedInstant")]
                    FixedInstant,
                    #[serde(rename = "_fixedInstant")]
                    FixedInstantPrimitiveElement,
                    #[serde(rename = "fixedInteger")]
                    FixedInteger,
                    #[serde(rename = "_fixedInteger")]
                    FixedIntegerPrimitiveElement,
                    #[serde(rename = "fixedMarkdown")]
                    FixedMarkdown,
                    #[serde(rename = "_fixedMarkdown")]
                    FixedMarkdownPrimitiveElement,
                    #[serde(rename = "fixedOid")]
                    FixedOid,
                    #[serde(rename = "_fixedOid")]
                    FixedOidPrimitiveElement,
                    #[serde(rename = "fixedPositiveInt")]
                    FixedPositiveInt,
                    #[serde(rename = "_fixedPositiveInt")]
                    FixedPositiveIntPrimitiveElement,
                    #[serde(rename = "fixedString")]
                    FixedString,
                    #[serde(rename = "_fixedString")]
                    FixedStringPrimitiveElement,
                    #[serde(rename = "fixedTime")]
                    FixedTime,
                    #[serde(rename = "_fixedTime")]
                    FixedTimePrimitiveElement,
                    #[serde(rename = "fixedUnsignedInt")]
                    FixedUnsignedInt,
                    #[serde(rename = "_fixedUnsignedInt")]
                    FixedUnsignedIntPrimitiveElement,
                    #[serde(rename = "fixedUri")]
                    FixedUri,
                    #[serde(rename = "_fixedUri")]
                    FixedUriPrimitiveElement,
                    #[serde(rename = "fixedUrl")]
                    FixedUrl,
                    #[serde(rename = "_fixedUrl")]
                    FixedUrlPrimitiveElement,
                    #[serde(rename = "fixedUuid")]
                    FixedUuid,
                    #[serde(rename = "_fixedUuid")]
                    FixedUuidPrimitiveElement,
                    #[serde(rename = "fixedAddress")]
                    FixedAddress,
                    #[serde(rename = "fixedAge")]
                    FixedAge,
                    #[serde(rename = "fixedAnnotation")]
                    FixedAnnotation,
                    #[serde(rename = "fixedAttachment")]
                    FixedAttachment,
                    #[serde(rename = "fixedCodeableConcept")]
                    FixedCodeableConcept,
                    #[serde(rename = "fixedCodeableReference")]
                    FixedCodeableReference,
                    #[serde(rename = "fixedCoding")]
                    FixedCoding,
                    #[serde(rename = "fixedContactPoint")]
                    FixedContactPoint,
                    #[serde(rename = "fixedCount")]
                    FixedCount,
                    #[serde(rename = "fixedDistance")]
                    FixedDistance,
                    #[serde(rename = "fixedDuration")]
                    FixedDuration,
                    #[serde(rename = "fixedHumanName")]
                    FixedHumanName,
                    #[serde(rename = "fixedIdentifier")]
                    FixedIdentifier,
                    #[serde(rename = "fixedMoney")]
                    FixedMoney,
                    #[serde(rename = "fixedPeriod")]
                    FixedPeriod,
                    #[serde(rename = "fixedQuantity")]
                    FixedQuantity,
                    #[serde(rename = "fixedRange")]
                    FixedRange,
                    #[serde(rename = "fixedRatio")]
                    FixedRatio,
                    #[serde(rename = "fixedRatioRange")]
                    FixedRatioRange,
                    #[serde(rename = "fixedReference")]
                    FixedReference,
                    #[serde(rename = "fixedSampledData")]
                    FixedSampledData,
                    #[serde(rename = "fixedSignature")]
                    FixedSignature,
                    #[serde(rename = "fixedTiming")]
                    FixedTiming,
                    #[serde(rename = "fixedContactDetail")]
                    FixedContactDetail,
                    #[serde(rename = "fixedContributor")]
                    FixedContributor,
                    #[serde(rename = "fixedDataRequirement")]
                    FixedDataRequirement,
                    #[serde(rename = "fixedExpression")]
                    FixedExpression,
                    #[serde(rename = "fixedParameterDefinition")]
                    FixedParameterDefinition,
                    #[serde(rename = "fixedRelatedArtifact")]
                    FixedRelatedArtifact,
                    #[serde(rename = "fixedTriggerDefinition")]
                    FixedTriggerDefinition,
                    #[serde(rename = "fixedUsageContext")]
                    FixedUsageContext,
                    #[serde(rename = "fixedDosage")]
                    FixedDosage,
                    #[serde(rename = "patternBase64Binary")]
                    PatternBase64Binary,
                    #[serde(rename = "_patternBase64Binary")]
                    PatternBase64BinaryPrimitiveElement,
                    #[serde(rename = "patternBoolean")]
                    PatternBoolean,
                    #[serde(rename = "_patternBoolean")]
                    PatternBooleanPrimitiveElement,
                    #[serde(rename = "patternCanonical")]
                    PatternCanonical,
                    #[serde(rename = "_patternCanonical")]
                    PatternCanonicalPrimitiveElement,
                    #[serde(rename = "patternCode")]
                    PatternCode,
                    #[serde(rename = "_patternCode")]
                    PatternCodePrimitiveElement,
                    #[serde(rename = "patternDate")]
                    PatternDate,
                    #[serde(rename = "_patternDate")]
                    PatternDatePrimitiveElement,
                    #[serde(rename = "patternDateTime")]
                    PatternDateTime,
                    #[serde(rename = "_patternDateTime")]
                    PatternDateTimePrimitiveElement,
                    #[serde(rename = "patternDecimal")]
                    PatternDecimal,
                    #[serde(rename = "_patternDecimal")]
                    PatternDecimalPrimitiveElement,
                    #[serde(rename = "patternId")]
                    PatternId,
                    #[serde(rename = "_patternId")]
                    PatternIdPrimitiveElement,
                    #[serde(rename = "patternInstant")]
                    PatternInstant,
                    #[serde(rename = "_patternInstant")]
                    PatternInstantPrimitiveElement,
                    #[serde(rename = "patternInteger")]
                    PatternInteger,
                    #[serde(rename = "_patternInteger")]
                    PatternIntegerPrimitiveElement,
                    #[serde(rename = "patternMarkdown")]
                    PatternMarkdown,
                    #[serde(rename = "_patternMarkdown")]
                    PatternMarkdownPrimitiveElement,
                    #[serde(rename = "patternOid")]
                    PatternOid,
                    #[serde(rename = "_patternOid")]
                    PatternOidPrimitiveElement,
                    #[serde(rename = "patternPositiveInt")]
                    PatternPositiveInt,
                    #[serde(rename = "_patternPositiveInt")]
                    PatternPositiveIntPrimitiveElement,
                    #[serde(rename = "patternString")]
                    PatternString,
                    #[serde(rename = "_patternString")]
                    PatternStringPrimitiveElement,
                    #[serde(rename = "patternTime")]
                    PatternTime,
                    #[serde(rename = "_patternTime")]
                    PatternTimePrimitiveElement,
                    #[serde(rename = "patternUnsignedInt")]
                    PatternUnsignedInt,
                    #[serde(rename = "_patternUnsignedInt")]
                    PatternUnsignedIntPrimitiveElement,
                    #[serde(rename = "patternUri")]
                    PatternUri,
                    #[serde(rename = "_patternUri")]
                    PatternUriPrimitiveElement,
                    #[serde(rename = "patternUrl")]
                    PatternUrl,
                    #[serde(rename = "_patternUrl")]
                    PatternUrlPrimitiveElement,
                    #[serde(rename = "patternUuid")]
                    PatternUuid,
                    #[serde(rename = "_patternUuid")]
                    PatternUuidPrimitiveElement,
                    #[serde(rename = "patternAddress")]
                    PatternAddress,
                    #[serde(rename = "patternAge")]
                    PatternAge,
                    #[serde(rename = "patternAnnotation")]
                    PatternAnnotation,
                    #[serde(rename = "patternAttachment")]
                    PatternAttachment,
                    #[serde(rename = "patternCodeableConcept")]
                    PatternCodeableConcept,
                    #[serde(rename = "patternCodeableReference")]
                    PatternCodeableReference,
                    #[serde(rename = "patternCoding")]
                    PatternCoding,
                    #[serde(rename = "patternContactPoint")]
                    PatternContactPoint,
                    #[serde(rename = "patternCount")]
                    PatternCount,
                    #[serde(rename = "patternDistance")]
                    PatternDistance,
                    #[serde(rename = "patternDuration")]
                    PatternDuration,
                    #[serde(rename = "patternHumanName")]
                    PatternHumanName,
                    #[serde(rename = "patternIdentifier")]
                    PatternIdentifier,
                    #[serde(rename = "patternMoney")]
                    PatternMoney,
                    #[serde(rename = "patternPeriod")]
                    PatternPeriod,
                    #[serde(rename = "patternQuantity")]
                    PatternQuantity,
                    #[serde(rename = "patternRange")]
                    PatternRange,
                    #[serde(rename = "patternRatio")]
                    PatternRatio,
                    #[serde(rename = "patternRatioRange")]
                    PatternRatioRange,
                    #[serde(rename = "patternReference")]
                    PatternReference,
                    #[serde(rename = "patternSampledData")]
                    PatternSampledData,
                    #[serde(rename = "patternSignature")]
                    PatternSignature,
                    #[serde(rename = "patternTiming")]
                    PatternTiming,
                    #[serde(rename = "patternContactDetail")]
                    PatternContactDetail,
                    #[serde(rename = "patternContributor")]
                    PatternContributor,
                    #[serde(rename = "patternDataRequirement")]
                    PatternDataRequirement,
                    #[serde(rename = "patternExpression")]
                    PatternExpression,
                    #[serde(rename = "patternParameterDefinition")]
                    PatternParameterDefinition,
                    #[serde(rename = "patternRelatedArtifact")]
                    PatternRelatedArtifact,
                    #[serde(rename = "patternTriggerDefinition")]
                    PatternTriggerDefinition,
                    #[serde(rename = "patternUsageContext")]
                    PatternUsageContext,
                    #[serde(rename = "patternDosage")]
                    PatternDosage,
                    #[serde(rename = "example")]
                    Example,
                    #[serde(rename = "minValueDate")]
                    MinValueDate,
                    #[serde(rename = "_minValueDate")]
                    MinValueDatePrimitiveElement,
                    #[serde(rename = "minValueDateTime")]
                    MinValueDateTime,
                    #[serde(rename = "_minValueDateTime")]
                    MinValueDateTimePrimitiveElement,
                    #[serde(rename = "minValueInstant")]
                    MinValueInstant,
                    #[serde(rename = "_minValueInstant")]
                    MinValueInstantPrimitiveElement,
                    #[serde(rename = "minValueTime")]
                    MinValueTime,
                    #[serde(rename = "_minValueTime")]
                    MinValueTimePrimitiveElement,
                    #[serde(rename = "minValueDecimal")]
                    MinValueDecimal,
                    #[serde(rename = "_minValueDecimal")]
                    MinValueDecimalPrimitiveElement,
                    #[serde(rename = "minValueInteger")]
                    MinValueInteger,
                    #[serde(rename = "_minValueInteger")]
                    MinValueIntegerPrimitiveElement,
                    #[serde(rename = "minValuePositiveInt")]
                    MinValuePositiveInt,
                    #[serde(rename = "_minValuePositiveInt")]
                    MinValuePositiveIntPrimitiveElement,
                    #[serde(rename = "minValueUnsignedInt")]
                    MinValueUnsignedInt,
                    #[serde(rename = "_minValueUnsignedInt")]
                    MinValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "minValueQuantity")]
                    MinValueQuantity,
                    #[serde(rename = "maxValueDate")]
                    MaxValueDate,
                    #[serde(rename = "_maxValueDate")]
                    MaxValueDatePrimitiveElement,
                    #[serde(rename = "maxValueDateTime")]
                    MaxValueDateTime,
                    #[serde(rename = "_maxValueDateTime")]
                    MaxValueDateTimePrimitiveElement,
                    #[serde(rename = "maxValueInstant")]
                    MaxValueInstant,
                    #[serde(rename = "_maxValueInstant")]
                    MaxValueInstantPrimitiveElement,
                    #[serde(rename = "maxValueTime")]
                    MaxValueTime,
                    #[serde(rename = "_maxValueTime")]
                    MaxValueTimePrimitiveElement,
                    #[serde(rename = "maxValueDecimal")]
                    MaxValueDecimal,
                    #[serde(rename = "_maxValueDecimal")]
                    MaxValueDecimalPrimitiveElement,
                    #[serde(rename = "maxValueInteger")]
                    MaxValueInteger,
                    #[serde(rename = "_maxValueInteger")]
                    MaxValueIntegerPrimitiveElement,
                    #[serde(rename = "maxValuePositiveInt")]
                    MaxValuePositiveInt,
                    #[serde(rename = "_maxValuePositiveInt")]
                    MaxValuePositiveIntPrimitiveElement,
                    #[serde(rename = "maxValueUnsignedInt")]
                    MaxValueUnsignedInt,
                    #[serde(rename = "_maxValueUnsignedInt")]
                    MaxValueUnsignedIntPrimitiveElement,
                    #[serde(rename = "maxValueQuantity")]
                    MaxValueQuantity,
                    #[serde(rename = "maxLength")]
                    MaxLength,
                    #[serde(rename = "_maxLength")]
                    MaxLengthPrimitiveElement,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "_condition")]
                    ConditionPrimitiveElement,
                    #[serde(rename = "constraint")]
                    Constraint,
                    #[serde(rename = "mustSupport")]
                    MustSupport,
                    #[serde(rename = "_mustSupport")]
                    MustSupportPrimitiveElement,
                    #[serde(rename = "isModifier")]
                    IsModifier,
                    #[serde(rename = "_isModifier")]
                    IsModifierPrimitiveElement,
                    #[serde(rename = "isModifierReason")]
                    IsModifierReason,
                    #[serde(rename = "_isModifierReason")]
                    IsModifierReasonPrimitiveElement,
                    #[serde(rename = "isSummary")]
                    IsSummary,
                    #[serde(rename = "_isSummary")]
                    IsSummaryPrimitiveElement,
                    #[serde(rename = "binding")]
                    Binding,
                    #[serde(rename = "mapping")]
                    Mapping,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "path",
                            "representation",
                            "sliceName",
                            "sliceIsConstraining",
                            "label",
                            "code",
                            "slicing",
                            "short",
                            "definition",
                            "comment",
                            "requirements",
                            "alias",
                            "min",
                            "max",
                            "base",
                            "contentReference",
                            "type",
                            "defaultValueBase64Binary",
                            "defaultValueBoolean",
                            "defaultValueCanonical",
                            "defaultValueCode",
                            "defaultValueDate",
                            "defaultValueDateTime",
                            "defaultValueDecimal",
                            "defaultValueId",
                            "defaultValueInstant",
                            "defaultValueInteger",
                            "defaultValueMarkdown",
                            "defaultValueOid",
                            "defaultValuePositiveInt",
                            "defaultValueString",
                            "defaultValueTime",
                            "defaultValueUnsignedInt",
                            "defaultValueUri",
                            "defaultValueUrl",
                            "defaultValueUuid",
                            "defaultValueAddress",
                            "defaultValueAge",
                            "defaultValueAnnotation",
                            "defaultValueAttachment",
                            "defaultValueCodeableConcept",
                            "defaultValueCodeableReference",
                            "defaultValueCoding",
                            "defaultValueContactPoint",
                            "defaultValueCount",
                            "defaultValueDistance",
                            "defaultValueDuration",
                            "defaultValueHumanName",
                            "defaultValueIdentifier",
                            "defaultValueMoney",
                            "defaultValuePeriod",
                            "defaultValueQuantity",
                            "defaultValueRange",
                            "defaultValueRatio",
                            "defaultValueRatioRange",
                            "defaultValueReference",
                            "defaultValueSampledData",
                            "defaultValueSignature",
                            "defaultValueTiming",
                            "defaultValueContactDetail",
                            "defaultValueContributor",
                            "defaultValueDataRequirement",
                            "defaultValueExpression",
                            "defaultValueParameterDefinition",
                            "defaultValueRelatedArtifact",
                            "defaultValueTriggerDefinition",
                            "defaultValueUsageContext",
                            "defaultValueDosage",
                            "meaningWhenMissing",
                            "orderMeaning",
                            "fixedBase64Binary",
                            "fixedBoolean",
                            "fixedCanonical",
                            "fixedCode",
                            "fixedDate",
                            "fixedDateTime",
                            "fixedDecimal",
                            "fixedId",
                            "fixedInstant",
                            "fixedInteger",
                            "fixedMarkdown",
                            "fixedOid",
                            "fixedPositiveInt",
                            "fixedString",
                            "fixedTime",
                            "fixedUnsignedInt",
                            "fixedUri",
                            "fixedUrl",
                            "fixedUuid",
                            "fixedAddress",
                            "fixedAge",
                            "fixedAnnotation",
                            "fixedAttachment",
                            "fixedCodeableConcept",
                            "fixedCodeableReference",
                            "fixedCoding",
                            "fixedContactPoint",
                            "fixedCount",
                            "fixedDistance",
                            "fixedDuration",
                            "fixedHumanName",
                            "fixedIdentifier",
                            "fixedMoney",
                            "fixedPeriod",
                            "fixedQuantity",
                            "fixedRange",
                            "fixedRatio",
                            "fixedRatioRange",
                            "fixedReference",
                            "fixedSampledData",
                            "fixedSignature",
                            "fixedTiming",
                            "fixedContactDetail",
                            "fixedContributor",
                            "fixedDataRequirement",
                            "fixedExpression",
                            "fixedParameterDefinition",
                            "fixedRelatedArtifact",
                            "fixedTriggerDefinition",
                            "fixedUsageContext",
                            "fixedDosage",
                            "patternBase64Binary",
                            "patternBoolean",
                            "patternCanonical",
                            "patternCode",
                            "patternDate",
                            "patternDateTime",
                            "patternDecimal",
                            "patternId",
                            "patternInstant",
                            "patternInteger",
                            "patternMarkdown",
                            "patternOid",
                            "patternPositiveInt",
                            "patternString",
                            "patternTime",
                            "patternUnsignedInt",
                            "patternUri",
                            "patternUrl",
                            "patternUuid",
                            "patternAddress",
                            "patternAge",
                            "patternAnnotation",
                            "patternAttachment",
                            "patternCodeableConcept",
                            "patternCodeableReference",
                            "patternCoding",
                            "patternContactPoint",
                            "patternCount",
                            "patternDistance",
                            "patternDuration",
                            "patternHumanName",
                            "patternIdentifier",
                            "patternMoney",
                            "patternPeriod",
                            "patternQuantity",
                            "patternRange",
                            "patternRatio",
                            "patternRatioRange",
                            "patternReference",
                            "patternSampledData",
                            "patternSignature",
                            "patternTiming",
                            "patternContactDetail",
                            "patternContributor",
                            "patternDataRequirement",
                            "patternExpression",
                            "patternParameterDefinition",
                            "patternRelatedArtifact",
                            "patternTriggerDefinition",
                            "patternUsageContext",
                            "patternDosage",
                            "example",
                            "minValueDate",
                            "minValueDateTime",
                            "minValueInstant",
                            "minValueTime",
                            "minValueDecimal",
                            "minValueInteger",
                            "minValuePositiveInt",
                            "minValueUnsignedInt",
                            "minValueQuantity",
                            "maxValueDate",
                            "maxValueDateTime",
                            "maxValueInstant",
                            "maxValueTime",
                            "maxValueDecimal",
                            "maxValueInteger",
                            "maxValuePositiveInt",
                            "maxValueUnsignedInt",
                            "maxValueQuantity",
                            "maxLength",
                            "condition",
                            "constraint",
                            "mustSupport",
                            "isModifier",
                            "isModifierReason",
                            "isSummary",
                            "binding",
                            "mapping",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#path: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#representation: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#slice_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#slice_is_constraining: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#label: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#code: Option<Vec<fhirbolt_model::r4b::types::Coding>> = None;
                let mut r#slicing: Option<fhirbolt_model::r4b::types::ElementDefinitionSlicing> =
                    None;
                let mut r#short: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#definition: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#comment: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#requirements: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#alias: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
                let mut r#min: Option<fhirbolt_model::r4b::types::UnsignedInt> = None;
                let mut r#max: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#base: Option<fhirbolt_model::r4b::types::ElementDefinitionBase> = None;
                let mut r#content_reference: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r4b::types::ElementDefinitionType>> =
                    None;
                let mut r#default_value: Option<
                    fhirbolt_model::r4b::types::ElementDefinitionDefaultValue,
                > = None;
                let mut r#meaning_when_missing: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#order_meaning: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#fixed: Option<fhirbolt_model::r4b::types::ElementDefinitionFixed> = None;
                let mut r#pattern: Option<fhirbolt_model::r4b::types::ElementDefinitionPattern> =
                    None;
                let mut r#example: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
                > = None;
                let mut r#min_value: Option<fhirbolt_model::r4b::types::ElementDefinitionMinValue> =
                    None;
                let mut r#max_value: Option<fhirbolt_model::r4b::types::ElementDefinitionMaxValue> =
                    None;
                let mut r#max_length: Option<fhirbolt_model::r4b::types::Integer> = None;
                let mut r#condition: Option<Vec<fhirbolt_model::r4b::types::Id>> = None;
                let mut r#constraint: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
                > = None;
                let mut r#must_support: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#is_modifier: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#is_modifier_reason: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#is_summary: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#binding: Option<fhirbolt_model::r4b::types::ElementDefinitionBinding> =
                    None;
                let mut r#mapping: Option<
                    Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Path => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#path.is_some() {
                                    return Err(serde::de::Error::duplicate_field("path"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#path = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PathPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#path.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_path"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("path");
                            }
                        }
                        Field::Representation => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#representation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "representation",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#representation.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Code,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RepresentationPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#representation.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_representation",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("representation");
                            }
                        }
                        Field::SliceName => {
                            if self.0.from_json {
                                let some = r#slice_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sliceName"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#slice_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sliceName"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#slice_name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SliceNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#slice_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sliceName"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sliceName");
                            }
                        }
                        Field::SliceIsConstraining => {
                            if self.0.from_json {
                                let some =
                                    r#slice_is_constraining.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sliceIsConstraining",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#slice_is_constraining.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "sliceIsConstraining",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#slice_is_constraining =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SliceIsConstrainingPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#slice_is_constraining.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_sliceIsConstraining",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sliceIsConstraining");
                            }
                        }
                        Field::Label => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("label"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#label = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LabelPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#label.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_label"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("label");
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Coding>,
                                > = self.0.transmute();
                                r#code = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Coding,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Slicing => {
                            if r#slicing.is_some() {
                                return Err(serde::de::Error::duplicate_field("slicing"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::types::ElementDefinitionSlicing,
                            > = self.0.transmute();
                            r#slicing = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Short => {
                            if self.0.from_json {
                                let some = r#short.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("short"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#short.is_some() {
                                    return Err(serde::de::Error::duplicate_field("short"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#short = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ShortPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#short.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_short"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("short");
                            }
                        }
                        Field::Definition => {
                            if self.0.from_json {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#definition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("definition"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#definition = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DefinitionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#definition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_definition"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("definition");
                            }
                        }
                        Field::Comment => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#comment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comment"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#comment = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#comment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_comment"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("comment");
                            }
                        }
                        Field::Requirements => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#requirements.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirements"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#requirements = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequirementsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#requirements.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requirements"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requirements");
                            }
                        }
                        Field::Alias => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#alias.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AliasPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_alias"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("alias");
                            }
                        }
                        Field::Min => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#min.is_some() {
                                    return Err(serde::de::Error::duplicate_field("min"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::UnsignedInt,
                                > = self.0.transmute();
                                r#min = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MinPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#min.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_min"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("min");
                            }
                        }
                        Field::Max => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#max.is_some() {
                                    return Err(serde::de::Error::duplicate_field("max"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#max = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MaxPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_max"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("max");
                            }
                        }
                        Field::Base => {
                            if r#base.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::types::ElementDefinitionBase,
                            > = self.0.transmute();
                            r#base = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ContentReference => {
                            if self.0.from_json {
                                let some = r#content_reference.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#content_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contentReference",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#content_reference =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ContentReferencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#content_reference.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_contentReference",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("contentReference");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ElementDefinitionType>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ElementDefinitionType,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DefaultValueBase64Binary => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Base64Binary>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Base64Binary(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueBase64Binary",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueBase64Binary");
                            }
                        }
                        Field::DefaultValueBoolean => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueBoolean",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Boolean>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueBooleanPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueBoolean");
                            }
                        }
                        Field::DefaultValueCanonical => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCanonical",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Canonical>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueCanonicalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueCanonical");
                            }
                        }
                        Field::DefaultValueCode => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueCode",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueCode",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Code>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Code(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueCodePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueCode",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueCode");
                            }
                        }
                        Field::DefaultValueDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDate",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueDate");
                            }
                        }
                        Field::DefaultValueDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueDateTime");
                            }
                        }
                        Field::DefaultValueDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueDecimal",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueDecimal");
                            }
                        }
                        Field::DefaultValueId => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueId",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueId",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Id>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Id(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueIdPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueId",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueId");
                            }
                        }
                        Field::DefaultValueInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInstant",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueInstant");
                            }
                        }
                        Field::DefaultValueInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueInteger",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueInteger");
                            }
                        }
                        Field::DefaultValueMarkdown => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueMarkdown",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueMarkdown",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Markdown>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::Markdown(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueMarkdownPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueMarkdown",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueMarkdown");
                            }
                        }
                        Field::DefaultValueOid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueOid",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueOid",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Oid>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Oid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueOidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueOid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueOid");
                            }
                        }
                        Field::DefaultValuePositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValuePositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValuePositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValuePositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValuePositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValuePositiveInt");
                            }
                        }
                        Field::DefaultValueString => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueStringPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueString");
                            }
                        }
                        Field::DefaultValueTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueTime");
                            }
                        }
                        Field::DefaultValueUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#default_value = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::DefaultValueUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#default_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueUnsignedInt");
                            }
                        }
                        Field::DefaultValueUri => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUri",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUri",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uri>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueUriPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUri",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueUri");
                            }
                        }
                        Field::DefaultValueUrl => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUrl",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUrl",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Url>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Url(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueUrlPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUrl",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueUrl");
                            }
                        }
                        Field::DefaultValueUuid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "defaultValueUuid",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValue[x]",
                                    ));
                                }
                            } else {
                                if r#default_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "defaultValueUuid",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uuid>,
                                > = self.0.transmute();
                                r#default_value =
                                    Some(_Enum::Uuid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::DefaultValueUuidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#default_value.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_defaultValueUuid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_defaultValue[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("defaultValueUuid");
                            }
                        }
                        Field::DefaultValueAddress => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAddress",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Address>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueAge => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Age>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Age(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueAnnotation => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAnnotation",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Annotation>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Annotation(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueAttachment => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueAttachment",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Attachment>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueCodeableConcept => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueCodeableReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCodeableReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::CodeableReference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueCoding => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueCoding",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Coding>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueContactPoint => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactPoint",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactPoint>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::ContactPoint(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueCount => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueCount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Count>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Count(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueDistance => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDistance",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Distance>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Distance(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueDuration => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDuration",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueHumanName => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueHumanName",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::HumanName>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::HumanName(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueIdentifier => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueIdentifier",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Identifier>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueMoney => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValuePeriod => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValuePeriod",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueQuantity",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueRatio => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultValueRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueRatioRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRatioRange",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RatioRange>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueSampledData => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSampledData",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::SampledData>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::SampledData(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueSignature => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueSignature",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Signature>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Signature(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueTiming => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTiming",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Timing>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::DefaultValueContactDetail => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContactDetail",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactDetail>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::ContactDetail(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueContributor => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueContributor",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Contributor>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Contributor(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueDataRequirement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDataRequirement",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::DataRequirement>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::DataRequirement(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueExpression => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueExpression",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Expression>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::Expression(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueParameterDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueParameterDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ParameterDefinition>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::ParameterDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueRelatedArtifact => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueRelatedArtifact",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RelatedArtifact>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::RelatedArtifact(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueTriggerDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueTriggerDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::TriggerDefinition>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::TriggerDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueUsageContext => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueUsageContext",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::UsageContext>,
                            > = self.0.transmute();
                            r#default_value = Some(_Enum::UsageContext(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::DefaultValueDosage => {
                            use fhirbolt_model::r4b::types::ElementDefinitionDefaultValue as _Enum;
                            if r#default_value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "defaultValueDosage",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Dosage>,
                            > = self.0.transmute();
                            r#default_value =
                                Some(_Enum::Dosage(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::MeaningWhenMissing => {
                            if self.0.from_json {
                                let some = r#meaning_when_missing.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "meaningWhenMissing",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#meaning_when_missing.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "meaningWhenMissing",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#meaning_when_missing =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MeaningWhenMissingPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#meaning_when_missing.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_meaningWhenMissing",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("meaningWhenMissing");
                            }
                        }
                        Field::OrderMeaning => {
                            if self.0.from_json {
                                let some = r#order_meaning.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderMeaning"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#order_meaning.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orderMeaning"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#order_meaning = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OrderMeaningPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#order_meaning.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_orderMeaning"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("orderMeaning");
                            }
                        }
                        Field::FixedBase64Binary => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Base64Binary>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Base64Binary(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedBase64Binary",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedBase64Binary");
                            }
                        }
                        Field::FixedBoolean => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedBoolean"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Boolean>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedBooleanPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedBoolean");
                            }
                        }
                        Field::FixedCanonical => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedCanonical",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Canonical>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedCanonicalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedCanonical");
                            }
                        }
                        Field::FixedCode => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedCode"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Code>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Code(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedCodePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedCode",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedCode");
                            }
                        }
                        Field::FixedDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedDate"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedDate");
                            }
                        }
                        Field::FixedDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDateTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedDateTime");
                            }
                        }
                        Field::FixedDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedDecimal"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedDecimal");
                            }
                        }
                        Field::FixedId => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedId"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedId"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Id>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Id(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedIdPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedId"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedId");
                            }
                        }
                        Field::FixedInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInstant"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedInstant");
                            }
                        }
                        Field::FixedInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedInteger"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedInteger");
                            }
                        }
                        Field::FixedMarkdown => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedMarkdown",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedMarkdown"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Markdown>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::Markdown(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedMarkdownPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedMarkdown",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedMarkdown");
                            }
                        }
                        Field::FixedOid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedOid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedOid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Oid>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Oid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedOidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedOid"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedOid");
                            }
                        }
                        Field::FixedPositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedPositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedPositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedPositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedPositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedPositiveInt");
                            }
                        }
                        Field::FixedString => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedStringPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedString");
                            }
                        }
                        Field::FixedTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedTime"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedTime");
                            }
                        }
                        Field::FixedUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "fixedUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "fixedUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#fixed = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::FixedUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#fixed.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUnsignedInt");
                            }
                        }
                        Field::FixedUri => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUri"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUri"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uri>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedUriPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedUri"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUri");
                            }
                        }
                        Field::FixedUrl => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUrl"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUrl"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Url>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Url(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedUrlPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_fixedUrl"));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUrl");
                            }
                        }
                        Field::FixedUuid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("fixedUuid"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("fixed[x]"));
                                }
                            } else {
                                if r#fixed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fixedUuid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uuid>,
                                > = self.0.transmute();
                                r#fixed =
                                    Some(_Enum::Uuid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::FixedUuidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if self.0.from_json {
                                let r#enum = r#fixed.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_fixedUuid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_fixed[x]"));
                                }
                            } else {
                                return unknown_field_error("fixedUuid");
                            }
                        }
                        Field::FixedAddress => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Address>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedAge => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Age>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Age(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedAnnotation => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAnnotation"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Annotation>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Annotation(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedAttachment => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Attachment>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedCodeableConcept => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedCodeableReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedCodeableReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::CodeableReference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedCoding => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCoding"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Coding>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedContactPoint => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContactPoint"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactPoint>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::ContactPoint(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedCount => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedCount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Count>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Count(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedDistance => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDistance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Distance>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Distance(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedDuration => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedHumanName => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedHumanName"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::HumanName>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::HumanName(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedIdentifier => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Identifier>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedMoney => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedPeriod => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedRatio => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedRatioRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedRatioRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RatioRange>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedSampledData => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSampledData"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::SampledData>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::SampledData(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedSignature => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedSignature"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Signature>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Signature(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedTiming => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedTiming"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Timing>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::FixedContactDetail => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedContactDetail",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactDetail>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::ContactDetail(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedContributor => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedContributor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Contributor>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Contributor(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedDataRequirement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedDataRequirement",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::DataRequirement>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::DataRequirement(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedExpression => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedExpression"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Expression>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::Expression(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedParameterDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedParameterDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ParameterDefinition>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::ParameterDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedRelatedArtifact => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedRelatedArtifact",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RelatedArtifact>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::RelatedArtifact(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedTriggerDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "fixedTriggerDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::TriggerDefinition>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::TriggerDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedUsageContext => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedUsageContext"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::UsageContext>,
                            > = self.0.transmute();
                            r#fixed = Some(_Enum::UsageContext(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::FixedDosage => {
                            use fhirbolt_model::r4b::types::ElementDefinitionFixed as _Enum;
                            if r#fixed.is_some() {
                                return Err(serde::de::Error::duplicate_field("fixedDosage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Dosage>,
                            > = self.0.transmute();
                            r#fixed =
                                Some(_Enum::Dosage(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternBase64Binary => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum = r#pattern
                                    .get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternBase64Binary",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBase64Binary",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Base64Binary>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Base64Binary(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternBase64BinaryPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum = r#pattern
                                    .get_or_insert(_Enum::Base64Binary(Default::default()));
                                if let _Enum::Base64Binary(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternBase64Binary",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternBase64Binary");
                            }
                        }
                        Field::PatternBoolean => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternBoolean",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternBoolean",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Boolean>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Boolean(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternBooleanPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternBoolean",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternBoolean");
                            }
                        }
                        Field::PatternCanonical => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternCanonical",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternCanonical",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Canonical>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Canonical(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternCanonicalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Canonical(Default::default()));
                                if let _Enum::Canonical(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternCanonical",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternCanonical");
                            }
                        }
                        Field::PatternCode => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternCode",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternCode"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Code>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Code(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternCodePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Code(Default::default()));
                                if let _Enum::Code(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternCode",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternCode");
                            }
                        }
                        Field::PatternDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternDate");
                            }
                        }
                        Field::PatternDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternDateTime");
                            }
                        }
                        Field::PatternDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternDecimal",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternDecimal");
                            }
                        }
                        Field::PatternId => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("patternId"));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternId"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Id>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Id(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternIdPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum = r#pattern.get_or_insert(_Enum::Id(Default::default()));
                                if let _Enum::Id(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternId",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternId");
                            }
                        }
                        Field::PatternInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInstant",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternInstant");
                            }
                        }
                        Field::PatternInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternInteger",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternInteger");
                            }
                        }
                        Field::PatternMarkdown => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternMarkdown",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternMarkdown",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Markdown>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::Markdown(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternMarkdownPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Markdown(Default::default()));
                                if let _Enum::Markdown(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternMarkdown",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternMarkdown");
                            }
                        }
                        Field::PatternOid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternOid",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternOid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Oid>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Oid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternOidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Oid(Default::default()));
                                if let _Enum::Oid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternOid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternOid");
                            }
                        }
                        Field::PatternPositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternPositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternPositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternPositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternPositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternPositiveInt");
                            }
                        }
                        Field::PatternString => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternString"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::String>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternStringPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternString");
                            }
                        }
                        Field::PatternTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternTime");
                            }
                        }
                        Field::PatternUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patternUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#pattern = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::PatternUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUnsignedInt");
                            }
                        }
                        Field::PatternUri => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUri",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUri"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uri>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Uri(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternUriPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Uri(Default::default()));
                                if let _Enum::Uri(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUri",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUri");
                            }
                        }
                        Field::PatternUrl => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUrl",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUrl"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Url>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Url(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternUrlPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Url(Default::default()));
                                if let _Enum::Url(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUrl",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUrl");
                            }
                        }
                        Field::PatternUuid => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "patternUuid",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("pattern[x]"));
                                }
                            } else {
                                if r#pattern.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patternUuid"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Uuid>,
                                > = self.0.transmute();
                                r#pattern =
                                    Some(_Enum::Uuid(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::PatternUuidPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#pattern.get_or_insert(_Enum::Uuid(Default::default()));
                                if let _Enum::Uuid(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_patternUuid",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_pattern[x]"));
                                }
                            } else {
                                return unknown_field_error("patternUuid");
                            }
                        }
                        Field::PatternAddress => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAddress"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Address>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Address(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternAge => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAge"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Age>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Age(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternAnnotation => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAnnotation"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Annotation>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Annotation(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternAttachment => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternAttachment"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Attachment>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Attachment(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternCodeableConcept => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternCodeableReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternCodeableReference",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::CodeableReference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternCoding => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCoding"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Coding>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Coding(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternContactPoint => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactPoint",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactPoint>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::ContactPoint(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternCount => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternCount"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Count>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Count(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternDistance => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDistance"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Distance>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Distance(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternDuration => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Duration>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Duration(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternHumanName => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternHumanName"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::HumanName>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::HumanName(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternIdentifier => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternIdentifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Identifier>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Identifier(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternMoney => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternMoney"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Money>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Money(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternPeriod => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Range>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Range(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternRatio => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Ratio>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternRatioRange => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternRatioRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RatioRange>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::RatioRange(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternReference => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternSampledData => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternSampledData",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::SampledData>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::SampledData(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternSignature => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternSignature"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Signature>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Signature(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternTiming => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternTiming"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Timing>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::PatternContactDetail => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContactDetail",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ContactDetail>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::ContactDetail(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternContributor => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternContributor",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Contributor>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Contributor(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternDataRequirement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternDataRequirement",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::DataRequirement>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::DataRequirement(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternExpression => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternExpression"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Expression>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::Expression(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternParameterDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternParameterDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::ParameterDefinition>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::ParameterDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternRelatedArtifact => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternRelatedArtifact",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::RelatedArtifact>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::RelatedArtifact(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternTriggerDefinition => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternTriggerDefinition",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::TriggerDefinition>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::TriggerDefinition(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternUsageContext => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "patternUsageContext",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::UsageContext>,
                            > = self.0.transmute();
                            r#pattern = Some(_Enum::UsageContext(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::PatternDosage => {
                            use fhirbolt_model::r4b::types::ElementDefinitionPattern as _Enum;
                            if r#pattern.is_some() {
                                return Err(serde::de::Error::duplicate_field("patternDosage"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Dosage>,
                            > = self.0.transmute();
                            r#pattern =
                                Some(_Enum::Dosage(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Example => {
                            if self.0.from_json {
                                if r#example.is_some() {
                                    return Err(serde::de::Error::duplicate_field("example"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ElementDefinitionExample>,
                                > = self.0.transmute();
                                r#example = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#example.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ElementDefinitionExample,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MinValueDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#min_value =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::MinValueDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueDate");
                            }
                        }
                        Field::MinValueDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueDateTime");
                            }
                        }
                        Field::MinValueInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInstant",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValueInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueInstant");
                            }
                        }
                        Field::MinValueTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("minValueTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#min_value =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::MinValueTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueTime");
                            }
                        }
                        Field::MinValueDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueDecimal",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValueDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueDecimal");
                            }
                        }
                        Field::MinValueInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueInteger",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#min_value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueInteger");
                            }
                        }
                        Field::MinValuePositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#min_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValuePositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValuePositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValuePositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#min_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValuePositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValuePositiveInt");
                            }
                        }
                        Field::MinValueUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#min_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minValueUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("minValue[x]"));
                                }
                            } else {
                                if r#min_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minValueUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#min_value = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinValueUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#min_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minValueUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_minValue[x]"));
                                }
                            } else {
                                return unknown_field_error("minValueUnsignedInt");
                            }
                        }
                        Field::MinValueQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMinValue as _Enum;
                            if r#min_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("minValueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#min_value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::MaxValueDate => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueDate",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Date>,
                                > = self.0.transmute();
                                r#max_value =
                                    Some(_Enum::Date(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::MaxValueDatePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Date(Default::default()));
                                if let _Enum::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueDate",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueDate");
                            }
                        }
                        Field::MaxValueDateTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::DateTime>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValueDateTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueDateTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueDateTime");
                            }
                        }
                        Field::MaxValueInstant => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueInstant",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInstant",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Instant>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::Instant(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValueInstantPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Instant(Default::default()));
                                if let _Enum::Instant(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueInstant",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueInstant");
                            }
                        }
                        Field::MaxValueTime => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxValueTime"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Time>,
                                > = self.0.transmute();
                                r#max_value =
                                    Some(_Enum::Time(map_access.next_value_seed(&mut *_context)?));
                            }
                        }
                        Field::MaxValueTimePrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Time(Default::default()));
                                if let _Enum::Time(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueTime",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueTime");
                            }
                        }
                        Field::MaxValueDecimal => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueDecimal",
                                        ));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    variant.value = Some(format!("{}", value));
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueDecimal",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Decimal>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::Decimal(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValueDecimalPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Decimal(Default::default()));
                                if let _Enum::Decimal(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueDecimal",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueDecimal");
                            }
                        }
                        Field::MaxValueInteger => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueInteger",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueInteger",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::Integer>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::Integer(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValueIntegerPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum =
                                    r#max_value.get_or_insert(_Enum::Integer(Default::default()));
                                if let _Enum::Integer(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueInteger",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueInteger");
                            }
                        }
                        Field::MaxValuePositiveInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#max_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValuePositiveInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValuePositiveInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::PositiveInt>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::PositiveInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValuePositiveIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#max_value
                                    .get_or_insert(_Enum::PositiveInt(Default::default()));
                                if let _Enum::PositiveInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValuePositiveInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValuePositiveInt");
                            }
                        }
                        Field::MaxValueUnsignedInt => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#max_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "maxValueUnsignedInt",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("maxValue[x]"));
                                }
                            } else {
                                if r#max_value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxValueUnsignedInt",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4b::types::UnsignedInt>,
                                > = self.0.transmute();
                                r#max_value = Some(_Enum::UnsignedInt(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MaxValueUnsignedIntPrimitiveElement => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if self.0.from_json {
                                let r#enum = r#max_value
                                    .get_or_insert(_Enum::UnsignedInt(Default::default()));
                                if let _Enum::UnsignedInt(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_maxValueUnsignedInt",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_maxValue[x]"));
                                }
                            } else {
                                return unknown_field_error("maxValueUnsignedInt");
                            }
                        }
                        Field::MaxValueQuantity => {
                            use fhirbolt_model::r4b::types::ElementDefinitionMaxValue as _Enum;
                            if r#max_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxValueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#max_value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::MaxLength => {
                            if self.0.from_json {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxLength"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#max_length.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxLength"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Integer,
                                > = self.0.transmute();
                                r#max_length = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MaxLengthPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#max_length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_maxLength"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("maxLength");
                            }
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#condition.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#condition.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#condition.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("condition");
                            }
                        }
                        Field::Constraint => {
                            if self.0.from_json {
                                if r#constraint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("constraint"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ElementDefinitionConstraint>,
                                > = self.0.transmute();
                                r#constraint = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#constraint.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ElementDefinitionConstraint,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MustSupport => {
                            if self.0.from_json {
                                let some = r#must_support.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#must_support.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mustSupport"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#must_support = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MustSupportPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#must_support.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mustSupport"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("mustSupport");
                            }
                        }
                        Field::IsModifier => {
                            if self.0.from_json {
                                let some = r#is_modifier.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isModifier"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#is_modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isModifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#is_modifier = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsModifierPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_modifier.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isModifier"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isModifier");
                            }
                        }
                        Field::IsModifierReason => {
                            if self.0.from_json {
                                let some = r#is_modifier_reason.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "isModifierReason",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#is_modifier_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "isModifierReason",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#is_modifier_reason =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsModifierReasonPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_modifier_reason.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_isModifierReason",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isModifierReason");
                            }
                        }
                        Field::IsSummary => {
                            if self.0.from_json {
                                let some = r#is_summary.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSummary"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#is_summary.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSummary"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Boolean,
                                > = self.0.transmute();
                                r#is_summary = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsSummaryPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_summary.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isSummary"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isSummary");
                            }
                        }
                        Field::Binding => {
                            if r#binding.is_some() {
                                return Err(serde::de::Error::duplicate_field("binding"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4b::types::ElementDefinitionBinding,
                            > = self.0.transmute();
                            r#binding = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Mapping => {
                            if self.0.from_json {
                                if r#mapping.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mapping"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::ElementDefinitionMapping>,
                                > = self.0.transmute();
                                r#mapping = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#mapping.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::ElementDefinitionMapping,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(ElementDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#path: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#path.unwrap_or(Default::default())
                    } else {
                        r#path.ok_or(serde::de::Error::missing_field("path"))?
                    },
                    r#representation: r#representation.unwrap_or(vec![]),
                    r#slice_name,
                    r#slice_is_constraining,
                    r#label,
                    r#code: r#code.unwrap_or(vec![]),
                    r#slicing,
                    r#short,
                    r#definition,
                    r#comment,
                    r#requirements,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#min,
                    r#max,
                    r#base,
                    r#content_reference,
                    r#type: r#type.unwrap_or(vec![]),
                    r#default_value,
                    r#meaning_when_missing,
                    r#order_meaning,
                    r#fixed,
                    r#pattern,
                    r#example: r#example.unwrap_or(vec![]),
                    r#min_value,
                    r#max_value,
                    r#max_length,
                    r#condition: r#condition.unwrap_or(vec![]),
                    r#constraint: r#constraint.unwrap_or(vec![]),
                    r#must_support,
                    r#is_modifier,
                    r#is_modifier_reason,
                    r#is_summary,
                    r#binding,
                    r#mapping: r#mapping.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ElementDefinition>> {
    type Value = Box<ElementDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ElementDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ElementDefinition>> {
    type Value = Vec<ElementDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ElementDefinition>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ElementDefinition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ElementDefinition> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
