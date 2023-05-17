// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionContact;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionContact> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.contact", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.value.r#contact.id.as_deref() == Some("$invalid") {
            return missing_field_error("contact");
        } else {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicinalProductDefinitionContact>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicinalProductDefinitionContact>> {
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
    for &mut DeserializationContext<MedicinalProductDefinitionContact>
{
    type Value = MedicinalProductDefinitionContact;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicinalProductDefinitionContact>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionContact;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionContact")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionContact, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "contact")]
                    Contact,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "contact"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#contact: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contact => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Reference>,
                            > = self.0.transmute();
                            r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionContact {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#contact: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#contact.unwrap_or(Default::default())
                    } else {
                        r#contact.ok_or(serde::de::Error::missing_field("contact"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionContact>>
{
    type Value = Box<MedicinalProductDefinitionContact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionContact>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionContact>>
{
    type Value = Vec<MedicinalProductDefinitionContact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicinalProductDefinitionContact>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionContact>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicinalProductDefinitionContact> =
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
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionNameNamePart;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionNameNamePart> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.name.namePart", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#part.id.as_deref() == Some("$invalid") {
                return missing_field_error("part");
            }
            if let Some(some) = self.value.r#part.value.as_ref().map(Ok) {
                state.serialize_entry("part", &some?)?;
            }
            if self.value.r#part.id.is_some() || !self.value.r#part.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#part.id.as_ref(),
                    extension: &self.value.r#part.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_part", ctx)
                })?;
            }
        } else if self.value.r#part.id.as_deref() == Some("$invalid") {
            return missing_field_error("part");
        } else {
            self.with_context(&self.value.r#part, |ctx| state.serialize_entry("part", ctx))?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicinalProductDefinitionNameNamePart>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicinalProductDefinitionNameNamePart>> {
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
    for &mut DeserializationContext<MedicinalProductDefinitionNameNamePart>
{
    type Value = MedicinalProductDefinitionNameNamePart;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicinalProductDefinitionNameNamePart>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionNameNamePart;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionNameNamePart")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionNameNamePart, V::Error>
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
                    #[serde(rename = "part")]
                    Part,
                    #[serde(rename = "_part")]
                    PartPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "part", "type"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#part: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Part => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#part.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#part = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#part.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_part"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("part");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionNameNamePart {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#part: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#part.unwrap_or(Default::default())
                    } else {
                        r#part.ok_or(serde::de::Error::missing_field("part"))?
                    },
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionNameNamePart>>
{
    type Value = Box<MedicinalProductDefinitionNameNamePart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionNameNamePart>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionNameNamePart>>
{
    type Value = Vec<MedicinalProductDefinitionNameNamePart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MedicinalProductDefinitionNameNamePart>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionNameNamePart>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicinalProductDefinitionNameNamePart> =
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
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionNameCountryLanguage;
impl serde::ser::Serialize
    for SerializationContext<&MedicinalProductDefinitionNameCountryLanguage>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.name.countryLanguage", field
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
        if self.value.r#country.id.as_deref() == Some("$invalid") {
            return missing_field_error("country");
        } else {
            self.with_context(&self.value.r#country, |ctx| {
                state.serialize_entry("country", ctx)
            })?;
        }
        if let Some(some) = self.value.r#jurisdiction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("jurisdiction", ctx))?;
        }
        if self.value.r#language.id.as_deref() == Some("$invalid") {
            return missing_field_error("language");
        } else {
            self.with_context(&self.value.r#language, |ctx| {
                state.serialize_entry("language", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<MedicinalProductDefinitionNameCountryLanguage>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<MedicinalProductDefinitionNameCountryLanguage>>
{
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
    for &mut DeserializationContext<MedicinalProductDefinitionNameCountryLanguage>
{
    type Value = MedicinalProductDefinitionNameCountryLanguage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<MedicinalProductDefinitionNameCountryLanguage>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionNameCountryLanguage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionNameCountryLanguage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionNameCountryLanguage, V::Error>
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
                    #[serde(rename = "country")]
                    Country,
                    #[serde(rename = "jurisdiction")]
                    Jurisdiction,
                    #[serde(rename = "language")]
                    Language,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "country",
                            "jurisdiction",
                            "language",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#country: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#language: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Country => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#country = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Jurisdiction => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#jurisdiction = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#language = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionNameCountryLanguage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#country: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#country.unwrap_or(Default::default())
                    } else {
                        r#country.ok_or(serde::de::Error::missing_field("country"))?
                    },
                    r#jurisdiction,
                    r#language: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#language.unwrap_or(Default::default())
                    } else {
                        r#language.ok_or(serde::de::Error::missing_field("language"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionNameCountryLanguage>>
{
    type Value = Box<MedicinalProductDefinitionNameCountryLanguage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionNameCountryLanguage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionNameCountryLanguage>>
{
    type Value = Vec<MedicinalProductDefinitionNameCountryLanguage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MedicinalProductDefinitionNameCountryLanguage>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionNameCountryLanguage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    MedicinalProductDefinitionNameCountryLanguage,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionName;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionName> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.name", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#product_name.id.as_deref() == Some("$invalid") {
                return missing_field_error("productName");
            }
            if let Some(some) = self.value.r#product_name.value.as_ref().map(Ok) {
                state.serialize_entry("productName", &some?)?;
            }
            if self.value.r#product_name.id.is_some()
                || !self.value.r#product_name.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#product_name.id.as_ref(),
                    extension: &self.value.r#product_name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_productName", ctx)
                })?;
            }
        } else if self.value.r#product_name.id.as_deref() == Some("$invalid") {
            return missing_field_error("productName");
        } else {
            self.with_context(&self.value.r#product_name, |ctx| {
                state.serialize_entry("productName", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#name_part.is_empty() {
            self.with_context(&self.value.r#name_part, |ctx| {
                state.serialize_entry("namePart", ctx)
            })?;
        }
        if !self.value.r#country_language.is_empty() {
            self.with_context(&self.value.r#country_language, |ctx| {
                state.serialize_entry("countryLanguage", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicinalProductDefinitionName>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicinalProductDefinitionName>> {
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
    for &mut DeserializationContext<MedicinalProductDefinitionName>
{
    type Value = MedicinalProductDefinitionName;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicinalProductDefinitionName>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionName")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionName, V::Error>
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
                    #[serde(rename = "productName")]
                    ProductName,
                    #[serde(rename = "_productName")]
                    ProductNamePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "namePart")]
                    NamePart,
                    #[serde(rename = "countryLanguage")]
                    CountryLanguage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "productName",
                            "type",
                            "namePart",
                            "countryLanguage",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#product_name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#name_part: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionNameNamePart>,
                > = None;
                let mut r#country_language : Option < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionNameCountryLanguage > > = None ;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::ProductName => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#product_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#product_name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ProductNamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_productName"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("productName");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NamePart => {
                            if self.0.from == crate::context::Format::Json {
                                if r#name_part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("namePart"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionNameNamePart >> = self . 0 . transmute () ;
                                r#name_part = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#name_part.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionNameNamePart > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CountryLanguage => {
                            if self.0.from == crate::context::Format::Json {
                                if r#country_language.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "countryLanguage",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionNameCountryLanguage >> = self . 0 . transmute () ;
                                r#country_language =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#country_language.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionNameCountryLanguage > = self . 0 . transmute () ;
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
                Ok(MedicinalProductDefinitionName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product_name: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#product_name.unwrap_or(Default::default())
                    } else {
                        r#product_name.ok_or(serde::de::Error::missing_field("productName"))?
                    },
                    r#type,
                    r#name_part: r#name_part.unwrap_or(vec![]),
                    r#country_language: r#country_language.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionName>>
{
    type Value = Box<MedicinalProductDefinitionName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionName>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionName>>
{
    type Value = Vec<MedicinalProductDefinitionName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicinalProductDefinitionName>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionName>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicinalProductDefinitionName> =
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
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCrossReference;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionCrossReference> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.crossReference", field
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
        if self.value.r#product.id.as_deref() == Some("$invalid") {
            return missing_field_error("product");
        } else {
            self.with_context(&self.value.r#product, |ctx| {
                state.serialize_entry("product", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<MedicinalProductDefinitionCrossReference>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<MedicinalProductDefinitionCrossReference>>
{
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
    for &mut DeserializationContext<MedicinalProductDefinitionCrossReference>
{
    type Value = MedicinalProductDefinitionCrossReference;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<MedicinalProductDefinitionCrossReference>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionCrossReference;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionCrossReference")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionCrossReference, V::Error>
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
                    #[serde(rename = "product")]
                    Product,
                    #[serde(rename = "type")]
                    Type,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "product", "type"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#product: Option<Box<fhirbolt_model::r4b::types::CodeableReference>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Product => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#product = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionCrossReference {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#product.unwrap_or(Default::default())
                    } else {
                        r#product.ok_or(serde::de::Error::missing_field("product"))?
                    },
                    r#type,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionCrossReference>>
{
    type Value = Box<MedicinalProductDefinitionCrossReference>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionCrossReference>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionCrossReference>>
{
    type Value = Vec<MedicinalProductDefinitionCrossReference>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MedicinalProductDefinitionCrossReference>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionCrossReference>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    MedicinalProductDefinitionCrossReference,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionOperation;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionOperation> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.operation", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#effective_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("effectiveDate", ctx))?;
        }
        if !self.value.r#organization.is_empty() {
            self.with_context(&self.value.r#organization, |ctx| {
                state.serialize_entry("organization", ctx)
            })?;
        }
        if let Some(some) = self.value.r#confidentiality_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("confidentialityIndicator", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicinalProductDefinitionOperation>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicinalProductDefinitionOperation>> {
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
    for &mut DeserializationContext<MedicinalProductDefinitionOperation>
{
    type Value = MedicinalProductDefinitionOperation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicinalProductDefinitionOperation>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionOperation, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "effectiveDate")]
                    EffectiveDate,
                    #[serde(rename = "organization")]
                    Organization,
                    #[serde(rename = "confidentialityIndicator")]
                    ConfidentialityIndicator,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "effectiveDate",
                            "organization",
                            "confidentialityIndicator",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableReference>> = None;
                let mut r#effective_date: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#organization: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#confidentiality_indicator: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
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
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableReference>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::EffectiveDate => {
                            if r#effective_date.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveDate"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Period>,
                            > = self.0.transmute();
                            r#effective_date = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Organization => {
                            if self.0.from == crate::context::Format::Json {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#organization = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#organization.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ConfidentialityIndicator => {
                            if r#confidentiality_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "confidentialityIndicator",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#confidentiality_indicator =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionOperation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type,
                    r#effective_date,
                    r#organization: r#organization.unwrap_or(vec![]),
                    r#confidentiality_indicator,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionOperation>>
{
    type Value = Box<MedicinalProductDefinitionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionOperation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionOperation>>
{
    type Value = Vec<MedicinalProductDefinitionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MedicinalProductDefinitionOperation>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionOperation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicinalProductDefinitionOperation> =
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
use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristic;
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinitionCharacteristic> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition.characteristic", field
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        {
            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
            if let Some(some) = self.value.r#value.as_ref() {
                match some {
                    _Enum::CodeableConcept(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueCodeableConcept", ctx)
                        })?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueQuantity", ctx)
                        })?;
                    }
                    _Enum::Date(ref value) => {
                        if self.output == crate::context::Format::Json {
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
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueDate", ctx)
                            })?;
                        }
                    }
                    _Enum::Boolean(ref value) => {
                        if self.output == crate::context::Format::Json {
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
                            self.with_context(value, |ctx| {
                                state.serialize_entry("valueBoolean", ctx)
                            })?;
                        }
                    }
                    _Enum::Attachment(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("valueAttachment", ctx)
                        })?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("value is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<MedicinalProductDefinitionCharacteristic>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<MedicinalProductDefinitionCharacteristic>>
{
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
    for &mut DeserializationContext<MedicinalProductDefinitionCharacteristic>
{
    type Value = MedicinalProductDefinitionCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<MedicinalProductDefinitionCharacteristic>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinitionCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductDefinitionCharacteristic, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "valueCodeableConcept",
                            "valueQuantity",
                            "valueDate",
                            "valueBoolean",
                            "valueAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue,
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
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ValueCodeableConcept => {
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
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
                        Field::ValueQuantity => {
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#value =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::ValueDate => {
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
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
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
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
                        Field::ValueBoolean => {
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
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
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
                            if self.0.from == crate::context::Format::Json {
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
                        Field::ValueAttachment => {
                            use fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristicValue as _Enum;
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicinalProductDefinitionCharacteristic {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#value,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinitionCharacteristic>>
{
    type Value = Box<MedicinalProductDefinitionCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinitionCharacteristic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinitionCharacteristic>>
{
    type Value = Vec<MedicinalProductDefinitionCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MedicinalProductDefinitionCharacteristic>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinitionCharacteristic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    MedicinalProductDefinitionCharacteristic,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4b::resources::MedicinalProductDefinition;
impl crate::Resource for MedicinalProductDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for SerializationContext<&MedicinalProductDefinition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicinalProductDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductDefinition")?;
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
        }
        if self.output == crate::context::Format::Json {
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
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
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
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#domain.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("domain", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("version", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#version.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
        }
        if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#status_date.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("statusDate", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusDate", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#status_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("statusDate", ctx))?;
        }
        if self.output == crate::context::Format::Json {
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
        if let Some(some) = self.value.r#combined_pharmaceutical_dose_form.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("combinedPharmaceuticalDoseForm", ctx)
            })?;
        }
        if !self.value.r#route.is_empty() {
            self.with_context(&self.value.r#route, |ctx| {
                state.serialize_entry("route", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#indication.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("indication", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_indication", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#indication.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("indication", ctx))?;
        }
        if let Some(some) = self.value.r#legal_status_of_supply.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("legalStatusOfSupply", ctx)
            })?;
        }
        if let Some(some) = self.value.r#additional_monitoring_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("additionalMonitoringIndicator", ctx)
            })?;
        }
        if !self.value.r#special_measures.is_empty() {
            self.with_context(&self.value.r#special_measures, |ctx| {
                state.serialize_entry("specialMeasures", ctx)
            })?;
        }
        if let Some(some) = self.value.r#pediatric_use_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("pediatricUseIndicator", ctx)
            })?;
        }
        if !self.value.r#classification.is_empty() {
            self.with_context(&self.value.r#classification, |ctx| {
                state.serialize_entry("classification", ctx)
            })?;
        }
        if !self.value.r#marketing_status.is_empty() {
            self.with_context(&self.value.r#marketing_status, |ctx| {
                state.serialize_entry("marketingStatus", ctx)
            })?;
        }
        if !self.value.r#packaged_medicinal_product.is_empty() {
            self.with_context(&self.value.r#packaged_medicinal_product, |ctx| {
                state.serialize_entry("packagedMedicinalProduct", ctx)
            })?;
        }
        if !self.value.r#ingredient.is_empty() {
            self.with_context(&self.value.r#ingredient, |ctx| {
                state.serialize_entry("ingredient", ctx)
            })?;
        }
        if !self.value.r#impurity.is_empty() {
            self.with_context(&self.value.r#impurity, |ctx| {
                state.serialize_entry("impurity", ctx)
            })?;
        }
        if !self.value.r#attached_document.is_empty() {
            self.with_context(&self.value.r#attached_document, |ctx| {
                state.serialize_entry("attachedDocument", ctx)
            })?;
        }
        if !self.value.r#master_file.is_empty() {
            self.with_context(&self.value.r#master_file, |ctx| {
                state.serialize_entry("masterFile", ctx)
            })?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if !self.value.r#clinical_trial.is_empty() {
            self.with_context(&self.value.r#clinical_trial, |ctx| {
                state.serialize_entry("clinicalTrial", ctx)
            })?;
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if !self.value.r#name.is_empty() {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if !self.value.r#cross_reference.is_empty() {
            self.with_context(&self.value.r#cross_reference, |ctx| {
                state.serialize_entry("crossReference", ctx)
            })?;
        }
        if !self.value.r#operation.is_empty() {
            self.with_context(&self.value.r#operation, |ctx| {
                state.serialize_entry("operation", ctx)
            })?;
        }
        if !self.value.r#characteristic.is_empty() {
            self.with_context(&self.value.r#characteristic, |ctx| {
                state.serialize_entry("characteristic", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicinalProductDefinition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicinalProductDefinition>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<MedicinalProductDefinition> {
    type Value = MedicinalProductDefinition;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<MedicinalProductDefinition>
{
    type Value = MedicinalProductDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicinalProductDefinition>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicinalProductDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProductDefinition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                #[derive(serde :: Deserialize)]
                #[serde(field_identifier)]
                enum Field {
                    #[serde(rename = "resourceType")]
                    ResourceType,
                    #[serde(rename = "id")]
                    Id,
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "domain")]
                    Domain,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "_version")]
                    VersionPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "statusDate")]
                    StatusDate,
                    #[serde(rename = "_statusDate")]
                    StatusDatePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "combinedPharmaceuticalDoseForm")]
                    CombinedPharmaceuticalDoseForm,
                    #[serde(rename = "route")]
                    Route,
                    #[serde(rename = "indication")]
                    Indication,
                    #[serde(rename = "_indication")]
                    IndicationPrimitiveElement,
                    #[serde(rename = "legalStatusOfSupply")]
                    LegalStatusOfSupply,
                    #[serde(rename = "additionalMonitoringIndicator")]
                    AdditionalMonitoringIndicator,
                    #[serde(rename = "specialMeasures")]
                    SpecialMeasures,
                    #[serde(rename = "pediatricUseIndicator")]
                    PediatricUseIndicator,
                    #[serde(rename = "classification")]
                    Classification,
                    #[serde(rename = "marketingStatus")]
                    MarketingStatus,
                    #[serde(rename = "packagedMedicinalProduct")]
                    PackagedMedicinalProduct,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "impurity")]
                    Impurity,
                    #[serde(rename = "attachedDocument")]
                    AttachedDocument,
                    #[serde(rename = "masterFile")]
                    MasterFile,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "clinicalTrial")]
                    ClinicalTrial,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "crossReference")]
                    CrossReference,
                    #[serde(rename = "operation")]
                    Operation,
                    #[serde(rename = "characteristic")]
                    Characteristic,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "meta",
                            "implicitRules",
                            "language",
                            "text",
                            "contained",
                            "extension",
                            "modifierExtension",
                            "identifier",
                            "type",
                            "domain",
                            "version",
                            "status",
                            "statusDate",
                            "description",
                            "combinedPharmaceuticalDoseForm",
                            "route",
                            "indication",
                            "legalStatusOfSupply",
                            "additionalMonitoringIndicator",
                            "specialMeasures",
                            "pediatricUseIndicator",
                            "classification",
                            "marketingStatus",
                            "packagedMedicinalProduct",
                            "ingredient",
                            "impurity",
                            "attachedDocument",
                            "masterFile",
                            "contact",
                            "clinicalTrial",
                            "code",
                            "name",
                            "crossReference",
                            "operation",
                            "characteristic",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r4b::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4b::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4b::types::Identifier>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#domain: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#version: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#status: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#status_date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#combined_pharmaceutical_dose_form: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#route: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#indication: Option<fhirbolt_model::r4b::types::Markdown> = None;
                let mut r#legal_status_of_supply: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#additional_monitoring_indicator: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#special_measures: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#pediatric_use_indicator: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#classification: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#marketing_status: Option<
                    Vec<fhirbolt_model::r4b::types::MarketingStatus>,
                > = None;
                let mut r#packaged_medicinal_product: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#ingredient: Option<Vec<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#impurity: Option<Vec<fhirbolt_model::r4b::types::CodeableReference>> =
                    None;
                let mut r#attached_document: Option<Vec<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#master_file: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#contact: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionContact>,
                > = None;
                let mut r#clinical_trial: Option<Vec<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#code: Option<Vec<fhirbolt_model::r4b::types::Coding>> = None;
                let mut r#name: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionName>,
                > = None;
                let mut r#cross_reference: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionCrossReference>,
                > = None;
                let mut r#operation: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionOperation>,
                > = None;
                let mut r#characteristic: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicinalProductDefinitionCharacteristic>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductDefinition",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Id,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
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
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
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
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Domain => {
                            if r#domain.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#domain = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Version => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#version = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#status = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::StatusDate => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::DateTime,
                                > = self.0.transmute();
                                r#status_date = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusDatePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusDate"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("statusDate");
                            }
                        }
                        Field::Description => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
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
                        Field::CombinedPharmaceuticalDoseForm => {
                            if r#combined_pharmaceutical_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "combinedPharmaceuticalDoseForm",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#combined_pharmaceutical_dose_form =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Route => {
                            if self.0.from == crate::context::Format::Json {
                                if r#route.is_some() {
                                    return Err(serde::de::Error::duplicate_field("route"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#route = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#route.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Indication => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#indication.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Markdown,
                                > = self.0.transmute();
                                r#indication = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IndicationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#indication.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_indication"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("indication");
                            }
                        }
                        Field::LegalStatusOfSupply => {
                            if r#legal_status_of_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legalStatusOfSupply",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#legal_status_of_supply =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::AdditionalMonitoringIndicator => {
                            if r#additional_monitoring_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalMonitoringIndicator",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#additional_monitoring_indicator =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SpecialMeasures => {
                            if self.0.from == crate::context::Format::Json {
                                if r#special_measures.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialMeasures",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#special_measures =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#special_measures.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PediatricUseIndicator => {
                            if r#pediatric_use_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "pediatricUseIndicator",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#pediatric_use_indicator =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Classification => {
                            if self.0.from == crate::context::Format::Json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#classification =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MarketingStatus => {
                            if self.0.from == crate::context::Format::Json {
                                if r#marketing_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "marketingStatus",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::MarketingStatus>,
                                > = self.0.transmute();
                                r#marketing_status =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#marketing_status.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::MarketingStatus,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PackagedMedicinalProduct => {
                            if self.0.from == crate::context::Format::Json {
                                if r#packaged_medicinal_product.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "packagedMedicinalProduct",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#packaged_medicinal_product =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#packaged_medicinal_product.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Ingredient => {
                            if self.0.from == crate::context::Format::Json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#ingredient = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Impurity => {
                            if self.0.from == crate::context::Format::Json {
                                if r#impurity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("impurity"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::CodeableReference>,
                                > = self.0.transmute();
                                r#impurity = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#impurity.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AttachedDocument => {
                            if self.0.from == crate::context::Format::Json {
                                if r#attached_document.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "attachedDocument",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#attached_document =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#attached_document.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::MasterFile => {
                            if self.0.from == crate::context::Format::Json {
                                if r#master_file.is_some() {
                                    return Err(serde::de::Error::duplicate_field("masterFile"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#master_file = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#master_file.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Contact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionContact >> = self . 0 . transmute () ;
                                r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionContact > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ClinicalTrial => {
                            if self.0.from == crate::context::Format::Json {
                                if r#clinical_trial.is_some() {
                                    return Err(serde::de::Error::duplicate_field("clinicalTrial"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Reference>,
                                > = self.0.transmute();
                                r#clinical_trial =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#clinical_trial.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Code => {
                            if self.0.from == crate::context::Format::Json {
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
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionName >> = self . 0 . transmute () ;
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#name.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::resources::MedicinalProductDefinitionName,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CrossReference => {
                            if self.0.from == crate::context::Format::Json {
                                if r#cross_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "crossReference",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionCrossReference >> = self . 0 . transmute () ;
                                r#cross_reference =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#cross_reference.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionCrossReference > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Operation => {
                            if self.0.from == crate::context::Format::Json {
                                if r#operation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operation"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionOperation >> = self . 0 . transmute () ;
                                r#operation = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#operation.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionOperation > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Characteristic => {
                            if self.0.from == crate::context::Format::Json {
                                if r#characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristic",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionCharacteristic >> = self . 0 . transmute () ;
                                r#characteristic =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#characteristic.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinitionCharacteristic > = self . 0 . transmute () ;
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
                Ok(MedicinalProductDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type,
                    r#domain,
                    r#version,
                    r#status,
                    r#status_date,
                    r#description,
                    r#combined_pharmaceutical_dose_form,
                    r#route: r#route.unwrap_or(vec![]),
                    r#indication,
                    r#legal_status_of_supply,
                    r#additional_monitoring_indicator,
                    r#special_measures: r#special_measures.unwrap_or(vec![]),
                    r#pediatric_use_indicator,
                    r#classification: r#classification.unwrap_or(vec![]),
                    r#marketing_status: r#marketing_status.unwrap_or(vec![]),
                    r#packaged_medicinal_product: r#packaged_medicinal_product.unwrap_or(vec![]),
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#impurity: r#impurity.unwrap_or(vec![]),
                    r#attached_document: r#attached_document.unwrap_or(vec![]),
                    r#master_file: r#master_file.unwrap_or(vec![]),
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#clinical_trial: r#clinical_trial.unwrap_or(vec![]),
                    r#code: r#code.unwrap_or(vec![]),
                    r#name: r#name.unwrap_or(vec![]),
                    r#cross_reference: r#cross_reference.unwrap_or(vec![]),
                    r#operation: r#operation.unwrap_or(vec![]),
                    r#characteristic: r#characteristic.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicinalProductDefinition>>
{
    type Value = Box<MedicinalProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicinalProductDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicinalProductDefinition>>
{
    type Value = Vec<MedicinalProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicinalProductDefinition>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicinalProductDefinition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicinalProductDefinition> =
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
