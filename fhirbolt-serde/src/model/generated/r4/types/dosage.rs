// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::types::DosageDoseAndRate;
impl serde::ser::Serialize for SerializationContext<&DosageDoseAndRate> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Dosage.doseAndRate", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("type", ctx)));
        }
        {
            use fhirbolt_model::r4::types::DosageDoseAndRateDose as _Enum;
            if let Some(some) = self.value.r#dose.as_ref() {
                match some {
                    _Enum::Range(ref value) => {
                        tri!(
                            self.with_context(value, |ctx| state.serialize_entry("doseRange", ctx))
                        );
                    }
                    _Enum::Quantity(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("doseQuantity", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("dose is invalid")),
                }
            }
        }
        {
            use fhirbolt_model::r4::types::DosageDoseAndRateRate as _Enum;
            if let Some(some) = self.value.r#rate.as_ref() {
                match some {
                    _Enum::Ratio(ref value) => {
                        tri!(
                            self.with_context(value, |ctx| state.serialize_entry("rateRatio", ctx))
                        );
                    }
                    _Enum::Range(ref value) => {
                        tri!(
                            self.with_context(value, |ctx| state.serialize_entry("rateRange", ctx))
                        );
                    }
                    _Enum::Quantity(ref value) => {
                        tri!(self
                            .with_context(value, |ctx| state.serialize_entry("rateQuantity", ctx)));
                    }
                    _Enum::Invalid => return Err(serde::ser::Error::custom("rate is invalid")),
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<DosageDoseAndRate>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<DosageDoseAndRate>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<DosageDoseAndRate> {
    type Value = DosageDoseAndRate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<DosageDoseAndRate>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = DosageDoseAndRate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DosageDoseAndRate")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DosageDoseAndRate, V::Error>
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
                    #[serde(rename = "doseRange")]
                    DoseRange,
                    #[serde(rename = "doseQuantity")]
                    DoseQuantity,
                    #[serde(rename = "rateRatio")]
                    RateRatio,
                    #[serde(rename = "rateRange")]
                    RateRange,
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
                            "type",
                            "doseRange",
                            "doseQuantity",
                            "rateRatio",
                            "rateRange",
                            "rateQuantity",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#dose: Option<fhirbolt_model::r4::types::DosageDoseAndRateDose> = None;
                let mut r#rate: Option<fhirbolt_model::r4::types::DosageDoseAndRateRate> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::DoseRange => {
                            use fhirbolt_model::r4::types::DosageDoseAndRateDose as _Enum;
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#dose = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::DoseQuantity => {
                            use fhirbolt_model::r4::types::DosageDoseAndRateDose as _Enum;
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#dose = Some(_Enum::Quantity(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::RateRatio => {
                            use fhirbolt_model::r4::types::DosageDoseAndRateRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Ratio>,
                            > = self.0.transmute();
                            r#rate = Some(_Enum::Ratio(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::RateRange => {
                            use fhirbolt_model::r4::types::DosageDoseAndRateRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#rate = Some(_Enum::Range(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::RateQuantity => {
                            use fhirbolt_model::r4::types::DosageDoseAndRateRate as _Enum;
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#rate = Some(_Enum::Quantity(tri!(
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
                Ok(DosageDoseAndRate {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#type,
                    r#dose,
                    r#rate,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<DosageDoseAndRate>> {
    type Value = Box<DosageDoseAndRate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<DosageDoseAndRate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<DosageDoseAndRate>> {
    type Value = Vec<DosageDoseAndRate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<DosageDoseAndRate>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<DosageDoseAndRate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<DosageDoseAndRate> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::types::Dosage;
impl serde::ser::Serialize for SerializationContext<&Dosage> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Dosage", field
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
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#sequence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("sequence", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_sequence", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#sequence.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sequence", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("text", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_text", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#text.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("text", ctx)));
        }
        if !self.value.r#additional_instruction.is_empty() {
            tri!(
                self.with_context(&self.value.r#additional_instruction, |ctx| state
                    .serialize_entry("additionalInstruction", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#patient_instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("patientInstruction", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_patientInstruction", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#patient_instruction.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("patientInstruction", ctx)));
        }
        if let Some(some) = self.value.r#timing.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("timing", ctx)));
        }
        {
            use fhirbolt_model::r4::types::DosageAsNeeded as _Enum;
            if let Some(some) = self.value.r#as_needed.as_ref() {
                match some {
                    _Enum::Boolean(ref value) => {
                        if self.output == crate::context::Format::Json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                tri!(state.serialize_entry("asNeededBoolean", &some?));
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                tri!(self.with_context(&primitive_element, |ctx| state
                                    .serialize_entry("_asNeededBoolean", ctx)));
                            }
                        } else {
                            tri!(self.with_context(value, |ctx| state
                                .serialize_entry("asNeededBoolean", ctx)));
                        }
                    }
                    _Enum::CodeableConcept(ref value) => {
                        tri!(self.with_context(value, |ctx| state
                            .serialize_entry("asNeededCodeableConcept", ctx)));
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("as_needed is invalid"))
                    }
                }
            }
        }
        if let Some(some) = self.value.r#site.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("site", ctx)));
        }
        if let Some(some) = self.value.r#route.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("route", ctx)));
        }
        if let Some(some) = self.value.r#method.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("method", ctx)));
        }
        if !self.value.r#dose_and_rate.is_empty() {
            tri!(self.with_context(&self.value.r#dose_and_rate, |ctx| state
                .serialize_entry("doseAndRate", ctx)));
        }
        if let Some(some) = self.value.r#max_dose_per_period.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("maxDosePerPeriod", ctx)));
        }
        if let Some(some) = self.value.r#max_dose_per_administration.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("maxDosePerAdministration", ctx)));
        }
        if let Some(some) = self.value.r#max_dose_per_lifetime.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("maxDosePerLifetime", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<Dosage>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Dosage>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Dosage> {
    type Value = Dosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Dosage>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Dosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Dosage")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Dosage, V::Error>
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
                    #[serde(rename = "sequence")]
                    Sequence,
                    #[serde(rename = "_sequence")]
                    SequencePrimitiveElement,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "additionalInstruction")]
                    AdditionalInstruction,
                    #[serde(rename = "patientInstruction")]
                    PatientInstruction,
                    #[serde(rename = "_patientInstruction")]
                    PatientInstructionPrimitiveElement,
                    #[serde(rename = "timing")]
                    Timing,
                    #[serde(rename = "asNeededBoolean")]
                    AsNeededBoolean,
                    #[serde(rename = "_asNeededBoolean")]
                    AsNeededBooleanPrimitiveElement,
                    #[serde(rename = "asNeededCodeableConcept")]
                    AsNeededCodeableConcept,
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "route")]
                    Route,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "doseAndRate")]
                    DoseAndRate,
                    #[serde(rename = "maxDosePerPeriod")]
                    MaxDosePerPeriod,
                    #[serde(rename = "maxDosePerAdministration")]
                    MaxDosePerAdministration,
                    #[serde(rename = "maxDosePerLifetime")]
                    MaxDosePerLifetime,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "sequence",
                            "text",
                            "additionalInstruction",
                            "patientInstruction",
                            "timing",
                            "asNeededBoolean",
                            "asNeededCodeableConcept",
                            "site",
                            "route",
                            "method",
                            "doseAndRate",
                            "maxDosePerPeriod",
                            "maxDosePerAdministration",
                            "maxDosePerLifetime",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#additional_instruction: Option<
                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#patient_instruction: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#timing: Option<Box<fhirbolt_model::r4::types::Timing>> = None;
                let mut r#as_needed: Option<fhirbolt_model::r4::types::DosageAsNeeded> = None;
                let mut r#site: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#dose_and_rate: Option<Vec<fhirbolt_model::r4::types::DosageDoseAndRate>> =
                    None;
                let mut r#max_dose_per_period: Option<Box<fhirbolt_model::r4::types::Ratio>> = None;
                let mut r#max_dose_per_administration: Option<
                    Box<fhirbolt_model::r4::types::Quantity>,
                > = None;
                let mut r#max_dose_per_lifetime: Option<Box<fhirbolt_model::r4::types::Quantity>> =
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Sequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#sequence = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sequence");
                            }
                        }
                        Field::Text => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
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
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::AdditionalInstruction => {
                            if self.0.from == crate::context::Format::Json {
                                if r#additional_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#additional_instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec =
                                    r#additional_instruction.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PatientInstruction => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#patient_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#patient_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#patient_instruction =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::PatientInstructionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#patient_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patientInstruction",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("patientInstruction");
                            }
                        }
                        Field::Timing => {
                            if r#timing.is_some() {
                                return Err(serde::de::Error::duplicate_field("timing"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Timing>,
                            > = self.0.transmute();
                            r#timing = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::AsNeededBoolean => {
                            use fhirbolt_model::r4::types::DosageAsNeeded as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#as_needed.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "asNeededBoolean",
                                        ));
                                    }
                                    variant.value = Some(tri!(map_access.next_value()))
                                } else {
                                    return Err(serde::de::Error::duplicate_field("asNeeded[x]"));
                                }
                            } else {
                                if r#as_needed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededBoolean",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#as_needed = Some(_Enum::Boolean(tri!(
                                    map_access.next_value_seed(&mut *_context)
                                )));
                            }
                        }
                        Field::AsNeededBooleanPrimitiveElement => {
                            use fhirbolt_model::r4::types::DosageAsNeeded as _Enum;
                            if self.0.from == crate::context::Format::Json {
                                let r#enum =
                                    r#as_needed.get_or_insert(_Enum::Boolean(Default::default()));
                                if let _Enum::Boolean(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_asNeededBoolean",
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
                                    return Err(serde::de::Error::duplicate_field("_asNeeded[x]"));
                                }
                            } else {
                                return unknown_field_error("asNeededBoolean");
                            }
                        }
                        Field::AsNeededCodeableConcept => {
                            use fhirbolt_model::r4::types::DosageAsNeeded as _Enum;
                            if r#as_needed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "asNeededCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#as_needed = Some(_Enum::CodeableConcept(tri!(
                                map_access.next_value_seed(&mut *_context)
                            )));
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#site = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#route = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::DoseAndRate => {
                            if self.0.from == crate::context::Format::Json {
                                if r#dose_and_rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseAndRate"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::DosageDoseAndRate>,
                                > = self.0.transmute();
                                r#dose_and_rate =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#dose_and_rate.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::DosageDoseAndRate,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::MaxDosePerPeriod => {
                            if r#max_dose_per_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDosePerPeriod"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Ratio>,
                            > = self.0.transmute();
                            r#max_dose_per_period =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::MaxDosePerAdministration => {
                            if r#max_dose_per_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerAdministration",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#max_dose_per_administration =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::MaxDosePerLifetime => {
                            if r#max_dose_per_lifetime.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerLifetime",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#max_dose_per_lifetime =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(Dosage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence,
                    r#text,
                    r#additional_instruction: r#additional_instruction.unwrap_or(vec![]),
                    r#patient_instruction,
                    r#timing,
                    r#as_needed,
                    r#site,
                    r#route,
                    r#method,
                    r#dose_and_rate: r#dose_and_rate.unwrap_or(vec![]),
                    r#max_dose_per_period,
                    r#max_dose_per_administration,
                    r#max_dose_per_lifetime,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<Dosage>> {
    type Value = Box<Dosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<Dosage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Dosage>> {
    type Value = Vec<Dosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Dosage>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Dosage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<Dosage> = self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
