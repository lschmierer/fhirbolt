// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::HealthcareServiceEligibility;
impl serde::ser::Serialize for SerializationContext<&HealthcareServiceEligibility> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "HealthcareService.eligibility", field
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
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        if self.output == crate::context::Format::Json {
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
impl serde::ser::Serialize for SerializationContext<&Box<HealthcareServiceEligibility>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<HealthcareServiceEligibility>> {
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
    for &mut DeserializationContext<HealthcareServiceEligibility>
{
    type Value = HealthcareServiceEligibility;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<HealthcareServiceEligibility>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = HealthcareServiceEligibility;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareServiceEligibility")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<HealthcareServiceEligibility, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code", "comment"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#comment: Option<fhirbolt_model::r5::types::Markdown> = None;
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#code = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Comment => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#comment = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
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
                Ok(HealthcareServiceEligibility {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#comment,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<HealthcareServiceEligibility>>
{
    type Value = Box<HealthcareServiceEligibility>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<HealthcareServiceEligibility>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<HealthcareServiceEligibility>>
{
    type Value = Vec<HealthcareServiceEligibility>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<HealthcareServiceEligibility>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<HealthcareServiceEligibility>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<HealthcareServiceEligibility> =
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
use fhirbolt_model::r5::resources::HealthcareService;
impl crate::Resource for HealthcareService {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&HealthcareService> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "HealthcareService", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "HealthcareService")?;
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#active.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("active", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_active", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#active.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("active", ctx))?;
        }
        if let Some(some) = self.value.r#provided_by.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("providedBy", ctx))?;
        }
        if !self.value.r#offered_in.is_empty() {
            self.with_context(&self.value.r#offered_in, |ctx| {
                state.serialize_entry("offeredIn", ctx)
            })?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#specialty.is_empty() {
            self.with_context(&self.value.r#specialty, |ctx| {
                state.serialize_entry("specialty", ctx)
            })?;
        }
        if !self.value.r#location.is_empty() {
            self.with_context(&self.value.r#location, |ctx| {
                state.serialize_entry("location", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("name", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
        }
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#extra_details.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("extraDetails", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_extraDetails", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#extra_details.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("extraDetails", ctx))?;
        }
        if let Some(some) = self.value.r#photo.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("photo", ctx))?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if !self.value.r#coverage_area.is_empty() {
            self.with_context(&self.value.r#coverage_area, |ctx| {
                state.serialize_entry("coverageArea", ctx)
            })?;
        }
        if !self.value.r#service_provision_code.is_empty() {
            self.with_context(&self.value.r#service_provision_code, |ctx| {
                state.serialize_entry("serviceProvisionCode", ctx)
            })?;
        }
        if !self.value.r#eligibility.is_empty() {
            self.with_context(&self.value.r#eligibility, |ctx| {
                state.serialize_entry("eligibility", ctx)
            })?;
        }
        if !self.value.r#program.is_empty() {
            self.with_context(&self.value.r#program, |ctx| {
                state.serialize_entry("program", ctx)
            })?;
        }
        if !self.value.r#characteristic.is_empty() {
            self.with_context(&self.value.r#characteristic, |ctx| {
                state.serialize_entry("characteristic", ctx)
            })?;
        }
        if !self.value.r#communication.is_empty() {
            self.with_context(&self.value.r#communication, |ctx| {
                state.serialize_entry("communication", ctx)
            })?;
        }
        if !self.value.r#referral_method.is_empty() {
            self.with_context(&self.value.r#referral_method, |ctx| {
                state.serialize_entry("referralMethod", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#appointment_required.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("appointmentRequired", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_appointmentRequired", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#appointment_required.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("appointmentRequired", ctx)
            })?;
        }
        if !self.value.r#availability.is_empty() {
            self.with_context(&self.value.r#availability, |ctx| {
                state.serialize_entry("availability", ctx)
            })?;
        }
        if !self.value.r#endpoint.is_empty() {
            self.with_context(&self.value.r#endpoint, |ctx| {
                state.serialize_entry("endpoint", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<HealthcareService>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<HealthcareService>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<HealthcareService> {
    type Value = HealthcareService;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<HealthcareService> {
    type Value = HealthcareService;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<HealthcareService>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = HealthcareService;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("HealthcareService")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<HealthcareService, V::Error>
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
                    #[serde(rename = "active")]
                    Active,
                    #[serde(rename = "_active")]
                    ActivePrimitiveElement,
                    #[serde(rename = "providedBy")]
                    ProvidedBy,
                    #[serde(rename = "offeredIn")]
                    OfferedIn,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "specialty")]
                    Specialty,
                    #[serde(rename = "location")]
                    Location,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "comment")]
                    Comment,
                    #[serde(rename = "_comment")]
                    CommentPrimitiveElement,
                    #[serde(rename = "extraDetails")]
                    ExtraDetails,
                    #[serde(rename = "_extraDetails")]
                    ExtraDetailsPrimitiveElement,
                    #[serde(rename = "photo")]
                    Photo,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "coverageArea")]
                    CoverageArea,
                    #[serde(rename = "serviceProvisionCode")]
                    ServiceProvisionCode,
                    #[serde(rename = "eligibility")]
                    Eligibility,
                    #[serde(rename = "program")]
                    Program,
                    #[serde(rename = "characteristic")]
                    Characteristic,
                    #[serde(rename = "communication")]
                    Communication,
                    #[serde(rename = "referralMethod")]
                    ReferralMethod,
                    #[serde(rename = "appointmentRequired")]
                    AppointmentRequired,
                    #[serde(rename = "_appointmentRequired")]
                    AppointmentRequiredPrimitiveElement,
                    #[serde(rename = "availability")]
                    Availability,
                    #[serde(rename = "endpoint")]
                    Endpoint,
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
                            "active",
                            "providedBy",
                            "offeredIn",
                            "category",
                            "type",
                            "specialty",
                            "location",
                            "name",
                            "comment",
                            "extraDetails",
                            "photo",
                            "contact",
                            "coverageArea",
                            "serviceProvisionCode",
                            "eligibility",
                            "program",
                            "characteristic",
                            "communication",
                            "referralMethod",
                            "appointmentRequired",
                            "availability",
                            "endpoint",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#active: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#provided_by: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#offered_in: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#category: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#specialty: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#location: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#comment: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#extra_details: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#photo: Option<Box<fhirbolt_model::r5::types::Attachment>> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r5::types::ExtendedContactDetail>> =
                    None;
                let mut r#coverage_area: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#service_provision_code: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#eligibility: Option<
                    Vec<fhirbolt_model::r5::resources::HealthcareServiceEligibility>,
                > = None;
                let mut r#program: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#characteristic: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#communication: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#referral_method: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#appointment_required: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#availability: Option<Vec<fhirbolt_model::r5::types::Availability>> = None;
                let mut r#endpoint: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "HealthcareService" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"HealthcareService",
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
                                    fhirbolt_model::r5::types::Id,
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
                                Box<fhirbolt_model::r5::types::Meta>,
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
                                    fhirbolt_model::r5::types::Uri,
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
                                    fhirbolt_model::r5::types::Code,
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
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
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
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
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
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Active => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#active.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("active"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#active.is_some() {
                                    return Err(serde::de::Error::duplicate_field("active"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#active = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ActivePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#active.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_active"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("active");
                            }
                        }
                        Field::ProvidedBy => {
                            if r#provided_by.is_some() {
                                return Err(serde::de::Error::duplicate_field("providedBy"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#provided_by = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::OfferedIn => {
                            if self.0.from == crate::context::Format::Json {
                                if r#offered_in.is_some() {
                                    return Err(serde::de::Error::duplicate_field("offeredIn"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#offered_in = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#offered_in.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Category => {
                            if self.0.from == crate::context::Format::Json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#category = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Specialty => {
                            if self.0.from == crate::context::Format::Json {
                                if r#specialty.is_some() {
                                    return Err(serde::de::Error::duplicate_field("specialty"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#specialty = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#specialty.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Location => {
                            if self.0.from == crate::context::Format::Json {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#location = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#location.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Comment => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#comment = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CommentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
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
                        Field::ExtraDetails => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#extra_details.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extraDetails"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#extra_details.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extraDetails"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Markdown,
                                > = self.0.transmute();
                                r#extra_details = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExtraDetailsPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#extra_details.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_extraDetails"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("extraDetails");
                            }
                        }
                        Field::Photo => {
                            if r#photo.is_some() {
                                return Err(serde::de::Error::duplicate_field("photo"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Attachment>,
                            > = self.0.transmute();
                            r#photo = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contact => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::ExtendedContactDetail>,
                                > = self.0.transmute();
                                r#contact = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::ExtendedContactDetail,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CoverageArea => {
                            if self.0.from == crate::context::Format::Json {
                                if r#coverage_area.is_some() {
                                    return Err(serde::de::Error::duplicate_field("coverageArea"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#coverage_area = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#coverage_area.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ServiceProvisionCode => {
                            if self.0.from == crate::context::Format::Json {
                                if r#service_provision_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "serviceProvisionCode",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#service_provision_code =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#service_provision_code.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Eligibility => {
                            if self.0.from == crate::context::Format::Json {
                                if r#eligibility.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eligibility"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r5::resources::HealthcareServiceEligibility,
                                    >,
                                > = self.0.transmute();
                                r#eligibility = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#eligibility.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::HealthcareServiceEligibility,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Program => {
                            if self.0.from == crate::context::Format::Json {
                                if r#program.is_some() {
                                    return Err(serde::de::Error::duplicate_field("program"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#program = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#program.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#characteristic =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#characteristic.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Communication => {
                            if self.0.from == crate::context::Format::Json {
                                if r#communication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("communication"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#communication = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#communication.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReferralMethod => {
                            if self.0.from == crate::context::Format::Json {
                                if r#referral_method.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referralMethod",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#referral_method =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#referral_method.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AppointmentRequired => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#appointment_required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "appointmentRequired",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#appointment_required.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "appointmentRequired",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#appointment_required =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AppointmentRequiredPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#appointment_required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_appointmentRequired",
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
                                return unknown_field_error("appointmentRequired");
                            }
                        }
                        Field::Availability => {
                            if self.0.from == crate::context::Format::Json {
                                if r#availability.is_some() {
                                    return Err(serde::de::Error::duplicate_field("availability"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Availability>,
                                > = self.0.transmute();
                                r#availability = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#availability.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Availability,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Endpoint => {
                            if self.0.from == crate::context::Format::Json {
                                if r#endpoint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endpoint"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#endpoint = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#endpoint.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
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
                Ok(HealthcareService {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#active,
                    r#provided_by,
                    r#offered_in: r#offered_in.unwrap_or(vec![]),
                    r#category: r#category.unwrap_or(vec![]),
                    r#type: r#type.unwrap_or(vec![]),
                    r#specialty: r#specialty.unwrap_or(vec![]),
                    r#location: r#location.unwrap_or(vec![]),
                    r#name,
                    r#comment,
                    r#extra_details,
                    r#photo,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#coverage_area: r#coverage_area.unwrap_or(vec![]),
                    r#service_provision_code: r#service_provision_code.unwrap_or(vec![]),
                    r#eligibility: r#eligibility.unwrap_or(vec![]),
                    r#program: r#program.unwrap_or(vec![]),
                    r#characteristic: r#characteristic.unwrap_or(vec![]),
                    r#communication: r#communication.unwrap_or(vec![]),
                    r#referral_method: r#referral_method.unwrap_or(vec![]),
                    r#appointment_required,
                    r#availability: r#availability.unwrap_or(vec![]),
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<HealthcareService>> {
    type Value = Box<HealthcareService>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<HealthcareService>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<HealthcareService>> {
    type Value = Vec<HealthcareService>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<HealthcareService>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<HealthcareService>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<HealthcareService> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
