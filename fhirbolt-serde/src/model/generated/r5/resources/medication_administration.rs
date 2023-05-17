// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::MedicationAdministrationPerformer;
impl serde::ser::Serialize for SerializationContext<&MedicationAdministrationPerformer> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationAdministration.performer", field
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
        if let Some(some) = self.value.r#function.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("function", ctx))?;
        }
        if self.value.r#actor.id.as_deref() == Some("$invalid") {
            return missing_field_error("actor");
        } else {
            self.with_context(&self.value.r#actor, |ctx| {
                state.serialize_entry("actor", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationAdministrationPerformer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationAdministrationPerformer>> {
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
    for &mut DeserializationContext<MedicationAdministrationPerformer>
{
    type Value = MedicationAdministrationPerformer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationAdministrationPerformer>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationAdministrationPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationPerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationAdministrationPerformer, V::Error>
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
                    #[serde(rename = "function")]
                    Function,
                    #[serde(rename = "actor")]
                    Actor,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "function", "actor"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#function: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
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
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#function = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Actor => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#actor = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicationAdministrationPerformer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#function,
                    r#actor: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#actor.unwrap_or(Default::default())
                    } else {
                        r#actor.ok_or(serde::de::Error::missing_field("actor"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicationAdministrationPerformer>>
{
    type Value = Box<MedicationAdministrationPerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationAdministrationPerformer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicationAdministrationPerformer>>
{
    type Value = Vec<MedicationAdministrationPerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationAdministrationPerformer>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationAdministrationPerformer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationAdministrationPerformer> =
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
use fhirbolt_model::r5::resources::MedicationAdministrationDosage;
impl serde::ser::Serialize for SerializationContext<&MedicationAdministrationDosage> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationAdministration.dosage", field
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
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("text", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if let Some(some) = self.value.r#site.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("site", ctx))?;
        }
        if let Some(some) = self.value.r#route.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("route", ctx))?;
        }
        if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if let Some(some) = self.value.r#dose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("dose", ctx))?;
        }
        {
            use fhirbolt_model::r5::resources::MedicationAdministrationDosageRate as _Enum;
            if let Some(some) = self.value.r#rate.as_ref() {
                match some {
                    _Enum::Ratio(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("rateRatio", ctx))?;
                    }
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| state.serialize_entry("rateQuantity", ctx))?;
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("rate is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationAdministrationDosage>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationAdministrationDosage>> {
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
    for &mut DeserializationContext<MedicationAdministrationDosage>
{
    type Value = MedicationAdministrationDosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationAdministrationDosage>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationAdministrationDosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationDosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicationAdministrationDosage, V::Error>
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
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "route")]
                    Route,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "dose")]
                    Dose,
                    #[serde(rename = "rateRatio")]
                    RateRatio,
                    #[serde(rename = "rateQuantity")]
                    RateQuantity,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "text",
                            "site",
                            "route",
                            "method",
                            "dose",
                            "rateRatio",
                            "rateQuantity",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#text: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#site: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#dose: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#rate: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationDosageRate,
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
                        Field::Text => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#text = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#site = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#route = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Dose => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("dose"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#dose = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::RateRatio => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationDosageRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Ratio>,
                            > = self.0.transmute();
                            r#rate =
                                Some(_Enum::Ratio(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::RateQuantity => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationDosageRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#rate =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MedicationAdministrationDosage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#text,
                    r#site,
                    r#route,
                    r#method,
                    r#dose,
                    r#rate,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicationAdministrationDosage>>
{
    type Value = Box<MedicationAdministrationDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationAdministrationDosage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicationAdministrationDosage>>
{
    type Value = Vec<MedicationAdministrationDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationAdministrationDosage>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationAdministrationDosage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationAdministrationDosage> =
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
use fhirbolt_model::r5::resources::MedicationAdministration;
impl crate::Resource for MedicationAdministration {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&MedicationAdministration> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MedicationAdministration", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationAdministration")?;
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
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if !self.value.r#part_of.is_empty() {
            self.with_context(&self.value.r#part_of, |ctx| {
                state.serialize_entry("partOf", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                state.serialize_entry("status", &some?)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if !self.value.r#status_reason.is_empty() {
            self.with_context(&self.value.r#status_reason, |ctx| {
                state.serialize_entry("statusReason", ctx)
            })?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if self.value.r#medication.id.as_deref() == Some("$invalid") {
            return missing_field_error("medication");
        } else {
            self.with_context(&self.value.r#medication, |ctx| {
                state.serialize_entry("medication", ctx)
            })?;
        }
        if self.value.r#subject.id.as_deref() == Some("$invalid") {
            return missing_field_error("subject");
        } else {
            self.with_context(&self.value.r#subject, |ctx| {
                state.serialize_entry("subject", ctx)
            })?;
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if !self.value.r#supporting_information.is_empty() {
            self.with_context(&self.value.r#supporting_information, |ctx| {
                state.serialize_entry("supportingInformation", ctx)
            })?;
        }
        {
            use fhirbolt_model::r5::resources::MedicationAdministrationOccurence as _Enum;
            match self.value.r#occurence {
                _Enum::DateTime(ref value) => {
                    if self.output == crate::context::Format::Json {
                        if let Some(some) = value.value.as_ref().map(Ok) {
                            state.serialize_entry("occurenceDateTime", &some?)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            use super::super::serde_helpers::PrimitiveElement;
                            let primitive_element = PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_occurenceDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("occurenceDateTime", ctx)
                        })?;
                    }
                }
                _Enum::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurencePeriod", ctx))?;
                }
                _Enum::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurenceTiming", ctx))?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("occurence is a required field"))
                }
            }
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#recorded.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("recorded", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_recorded", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#recorded.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("recorded", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#is_sub_potent.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("isSubPotent", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isSubPotent", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#is_sub_potent.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("isSubPotent", ctx))?;
        }
        if !self.value.r#sub_potent_reason.is_empty() {
            self.with_context(&self.value.r#sub_potent_reason, |ctx| {
                state.serialize_entry("subPotentReason", ctx)
            })?;
        }
        if !self.value.r#performer.is_empty() {
            self.with_context(&self.value.r#performer, |ctx| {
                state.serialize_entry("performer", ctx)
            })?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if let Some(some) = self.value.r#request.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("request", ctx))?;
        }
        if !self.value.r#device.is_empty() {
            self.with_context(&self.value.r#device, |ctx| {
                state.serialize_entry("device", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if let Some(some) = self.value.r#dosage.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("dosage", ctx))?;
        }
        if !self.value.r#event_history.is_empty() {
            self.with_context(&self.value.r#event_history, |ctx| {
                state.serialize_entry("eventHistory", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MedicationAdministration>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MedicationAdministration>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<MedicationAdministration> {
    type Value = MedicationAdministration;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<MedicationAdministration>
{
    type Value = MedicationAdministration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MedicationAdministration>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MedicationAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministration")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicationAdministration, V::Error>
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
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "statusReason")]
                    StatusReason,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "medication")]
                    Medication,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "supportingInformation")]
                    SupportingInformation,
                    #[serde(rename = "occurenceDateTime")]
                    OccurenceDateTime,
                    #[serde(rename = "_occurenceDateTime")]
                    OccurenceDateTimePrimitiveElement,
                    #[serde(rename = "occurencePeriod")]
                    OccurencePeriod,
                    #[serde(rename = "occurenceTiming")]
                    OccurenceTiming,
                    #[serde(rename = "recorded")]
                    Recorded,
                    #[serde(rename = "_recorded")]
                    RecordedPrimitiveElement,
                    #[serde(rename = "isSubPotent")]
                    IsSubPotent,
                    #[serde(rename = "_isSubPotent")]
                    IsSubPotentPrimitiveElement,
                    #[serde(rename = "subPotentReason")]
                    SubPotentReason,
                    #[serde(rename = "performer")]
                    Performer,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "dosage")]
                    Dosage,
                    #[serde(rename = "eventHistory")]
                    EventHistory,
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
                            "basedOn",
                            "partOf",
                            "status",
                            "statusReason",
                            "category",
                            "medication",
                            "subject",
                            "encounter",
                            "supportingInformation",
                            "occurenceDateTime",
                            "occurencePeriod",
                            "occurenceTiming",
                            "recorded",
                            "isSubPotent",
                            "subPotentReason",
                            "performer",
                            "reason",
                            "request",
                            "device",
                            "note",
                            "dosage",
                            "eventHistory",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#based_on: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#part_of: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#status_reason: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#category: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#medication: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#supporting_information: Option<
                    Vec<fhirbolt_model::r5::types::Reference>,
                > = None;
                let mut r#occurence: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationOccurence,
                > = None;
                let mut r#recorded: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#is_sub_potent: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#sub_potent_reason: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#performer: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
                > = None;
                let mut r#reason: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#request: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#device: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                let mut r#dosage: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationDosage,
                > = None;
                let mut r#event_history: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationAdministration" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationAdministration",
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
                                    Box<fhirbolt_model::r5::types::Id>,
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
                        Field::BasedOn => {
                            if self.0.from == crate::context::Format::Json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#based_on = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PartOf => {
                            if self.0.from == crate::context::Format::Json {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#part_of = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#part_of.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::StatusReason => {
                            if self.0.from == crate::context::Format::Json {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#status_reason = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#status_reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
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
                        Field::Medication => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field("medication"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableReference>,
                            > = self.0.transmute();
                            r#medication = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#subject = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::SupportingInformation => {
                            if self.0.from == crate::context::Format::Json {
                                if r#supporting_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInformation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#supporting_information =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec =
                                    r#supporting_information.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OccurenceDateTime => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationOccurence as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#occurence.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "occurenceDateTime",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field("occurence[x]"));
                                }
                            } else {
                                if r#occurence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurenceDateTime",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r5::types::DateTime>,
                                > = self.0.transmute();
                                r#occurence = Some(_Enum::DateTime(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::OccurenceDateTimePrimitiveElement => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationOccurence as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#occurence.get_or_insert(_Enum::DateTime(Default::default()));
                                if let _Enum::DateTime(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_occurenceDateTime",
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
                                    return Err(serde::de::Error::duplicate_field("_occurence[x]"));
                                }
                            } else {
                                return unknown_field_error("occurenceDateTime");
                            }
                        }
                        Field::OccurencePeriod => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationOccurence as _Enum;
                            if r#occurence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurencePeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Period>,
                            > = self.0.transmute();
                            r#occurence =
                                Some(_Enum::Period(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::OccurenceTiming => {
                            use fhirbolt_model::r5::resources::MedicationAdministrationOccurence as _Enum;
                            if r#occurence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurenceTiming"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Timing>,
                            > = self.0.transmute();
                            r#occurence =
                                Some(_Enum::Timing(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::Recorded => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#recorded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#recorded = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RecordedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recorded"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recorded");
                            }
                        }
                        Field::IsSubPotent => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#is_sub_potent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSubPotent"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#is_sub_potent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSubPotent"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Boolean,
                                > = self.0.transmute();
                                r#is_sub_potent = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsSubPotentPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#is_sub_potent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isSubPotent"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isSubPotent");
                            }
                        }
                        Field::SubPotentReason => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sub_potent_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subPotentReason",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#sub_potent_reason =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#sub_potent_reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Performer => {
                            if self.0.from == crate::context::Format::Json {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer >> = self . 0 . transmute () ;
                                r#performer = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#performer.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Reason => {
                            if self.0.from == crate::context::Format::Json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#reason = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#request = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Device => {
                            if self.0.from == crate::context::Format::Json {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::CodeableReference>,
                                > = self.0.transmute();
                                r#device = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#device.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::CodeableReference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Dosage => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r5::resources::MedicationAdministrationDosage,
                            > = self.0.transmute();
                            r#dosage = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::EventHistory => {
                            if self.0.from == crate::context::Format::Json {
                                if r#event_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventHistory"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Reference>,
                                > = self.0.transmute();
                                r#event_history = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#event_history.get_or_insert(Default::default());
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
                Ok(MedicationAdministration {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#status_reason: r#status_reason.unwrap_or(vec![]),
                    r#category: r#category.unwrap_or(vec![]),
                    r#medication: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#medication.unwrap_or(Default::default())
                    } else {
                        r#medication.ok_or(serde::de::Error::missing_field("medication"))?
                    },
                    r#subject: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#subject.unwrap_or(Default::default())
                    } else {
                        r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                    },
                    r#encounter,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#occurence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#occurence.unwrap_or(Default::default())
                    } else {
                        r#occurence.ok_or(serde::de::Error::missing_field("occurence[x]"))?
                    },
                    r#recorded,
                    r#is_sub_potent,
                    r#sub_potent_reason: r#sub_potent_reason.unwrap_or(vec![]),
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#request,
                    r#device: r#device.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#dosage,
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MedicationAdministration>>
{
    type Value = Box<MedicationAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MedicationAdministration>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MedicationAdministration>>
{
    type Value = Vec<MedicationAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MedicationAdministration>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MedicationAdministration>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MedicationAdministration> =
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
