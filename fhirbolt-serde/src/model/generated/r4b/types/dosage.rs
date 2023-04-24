// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::types::DosageDoseAndRate>
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
                "Dosage.doseAndRate", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#dose.as_ref() {
            match some {
                fhirbolt_model::r4b::types::DosageDoseAndRateDose::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("doseRange", ctx))?;
                }
                fhirbolt_model::r4b::types::DosageDoseAndRateDose::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("doseQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::DosageDoseAndRateDose::Invalid => {
                    return Err(serde::ser::Error::custom("dose is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#rate.as_ref() {
            match some {
                fhirbolt_model::r4b::types::DosageDoseAndRateRate::Ratio(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("rateRatio", ctx))?;
                }
                fhirbolt_model::r4b::types::DosageDoseAndRateRate::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("rateRange", ctx))?;
                }
                fhirbolt_model::r4b::types::DosageDoseAndRateRate::Quantity(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("rateQuantity", ctx))?;
                }
                fhirbolt_model::r4b::types::DosageDoseAndRateRate::Invalid => {
                    return Err(serde::ser::Error::custom("rate is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4b::types::DosageDoseAndRate>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::types::DosageDoseAndRate,
    >
{
    type Value = fhirbolt_model::r4b::types::DosageDoseAndRate;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::types::DosageDoseAndRate,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::DosageDoseAndRate;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DosageDoseAndRate")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::DosageDoseAndRate, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#dose: Option<fhirbolt_model::r4b::types::DosageDoseAndRateDose> = None;
                let mut r#rate: Option<fhirbolt_model::r4b::types::DosageDoseAndRateRate> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::DoseRange => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseRange"));
                            }
                            r#dose =
                                Some(fhirbolt_model::r4b::types::DosageDoseAndRateDose::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::DoseQuantity => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseQuantity"));
                            }
                            r#dose =
                                Some(fhirbolt_model::r4b::types::DosageDoseAndRateDose::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::RateRatio => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            r#rate =
                                Some(fhirbolt_model::r4b::types::DosageDoseAndRateRate::Ratio(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                                    )?,
                                ));
                        }
                        Field::RateRange => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRange"));
                            }
                            r#rate =
                                Some(fhirbolt_model::r4b::types::DosageDoseAndRateRate::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ));
                        }
                        Field::RateQuantity => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            r#rate =
                                Some(fhirbolt_model::r4b::types::DosageDoseAndRateRate::Quantity(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
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
                Ok(fhirbolt_model::r4b::types::DosageDoseAndRate {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::types::DosageDoseAndRate>,
    >
{
    type Value = Box<fhirbolt_model::r4b::types::DosageDoseAndRate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::DosageDoseAndRate>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r4b::types::DosageDoseAndRate>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4b::types::Dosage>
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
                "Dosage", field
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
            if let Some(some) = self.value.r#sequence.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("sequence", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sequence", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#sequence.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("sequence", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        if !self.value.r#additional_instruction.is_empty() {
            self.with_context(&self.value.r#additional_instruction, |ctx| {
                state.serialize_entry("additionalInstruction", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#patient_instruction.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("patientInstruction", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_patientInstruction", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#patient_instruction.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("patientInstruction", ctx))?;
            }
        }
        if let Some(some) = self.value.r#timing.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("timing", ctx))?;
        }
        if let Some(some) = self.value.r#as_needed.as_ref() {
            match some {
                fhirbolt_model::r4b::types::DosageAsNeeded::Boolean(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("asNeededBoolean", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_asNeededBoolean", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("asNeededBoolean", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::types::DosageAsNeeded::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("asNeededCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4b::types::DosageAsNeeded::Invalid => {
                    return Err(serde::ser::Error::custom("as_needed is invalid"))
                }
            }
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
        if !self.value.r#dose_and_rate.is_empty() {
            self.with_context(&self.value.r#dose_and_rate, |ctx| {
                state.serialize_entry("doseAndRate", ctx)
            })?;
        }
        if let Some(some) = self.value.r#max_dose_per_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxDosePerPeriod", ctx))?;
        }
        if let Some(some) = self.value.r#max_dose_per_administration.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("maxDosePerAdministration", ctx)
            })?;
        }
        if let Some(some) = self.value.r#max_dose_per_lifetime.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxDosePerLifetime", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4b::types::Dosage>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4b::types::Dosage>>
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::Dosage>
{
    type Value = fhirbolt_model::r4b::types::Dosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::types::Dosage>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::types::Dosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Dosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::types::Dosage, V::Error>
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
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#sequence: Option<fhirbolt_model::r4b::types::Integer> = None;
                let mut r#text: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#additional_instruction: Option<
                    Vec<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#patient_instruction: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#timing: Option<Box<fhirbolt_model::r4b::types::Timing>> = None;
                let mut r#as_needed: Option<fhirbolt_model::r4b::types::DosageAsNeeded> = None;
                let mut r#site: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#dose_and_rate: Option<
                    Vec<fhirbolt_model::r4b::types::DosageDoseAndRate>,
                > = None;
                let mut r#max_dose_per_period: Option<Box<fhirbolt_model::r4b::types::Ratio>> =
                    None;
                let mut r#max_dose_per_administration: Option<
                    Box<fhirbolt_model::r4b::types::Quantity>,
                > = None;
                let mut r#max_dose_per_lifetime: Option<Box<fhirbolt_model::r4b::types::Quantity>> =
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: Extension >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Sequence => {
                            if self.0.from_json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                r#sequence = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Integer>(),
                                )?);
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sequence");
                            }
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::AdditionalInstruction => {
                            if self.0.from_json {
                                if r#additional_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "additionalInstruction",
                                    ));
                                }
                                r#additional_instruction = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec =
                                    r#additional_instruction.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::PatientInstruction => {
                            if self.0.from_json {
                                let some = r#patient_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#patient_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientInstruction",
                                    ));
                                }
                                r#patient_instruction = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PatientInstructionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#patient_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_patientInstruction",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#timing = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Timing>>(),
                                )?,
                            );
                        }
                        Field::AsNeededBoolean => {
                            if self.0.from_json {
                                let r#enum = r#as_needed.get_or_insert(
                                    fhirbolt_model::r4b::types::DosageAsNeeded::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::DosageAsNeeded::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "asNeededBoolean",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("asNeeded[x]"));
                                }
                            } else {
                                if r#as_needed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "asNeededBoolean",
                                    ));
                                }
                                r#as_needed = Some (fhirbolt_model :: r4b :: types :: DosageAsNeeded :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::AsNeededBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#as_needed.get_or_insert(
                                    fhirbolt_model::r4b::types::DosageAsNeeded::Boolean(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4b::types::DosageAsNeeded::Boolean(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_asNeededBoolean",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            if r#as_needed.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "asNeededCodeableConcept",
                                ));
                            }
                            r#as_needed = Some (fhirbolt_model :: r4b :: types :: DosageAsNeeded :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            r#route = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::DoseAndRate => {
                            if self.0.from_json {
                                if r#dose_and_rate.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doseAndRate"));
                                }
                                r#dose_and_rate = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: types :: DosageDoseAndRate >> ()) ?) ;
                            } else {
                                let vec = r#dose_and_rate.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: DosageDoseAndRate > ()) ?) ;
                            }
                        }
                        Field::MaxDosePerPeriod => {
                            if r#max_dose_per_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDosePerPeriod"));
                            }
                            r#max_dose_per_period = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                            )?);
                        }
                        Field::MaxDosePerAdministration => {
                            if r#max_dose_per_administration.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerAdministration",
                                ));
                            }
                            r#max_dose_per_administration = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::MaxDosePerLifetime => {
                            if r#max_dose_per_lifetime.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerLifetime",
                                ));
                            }
                            r#max_dose_per_lifetime = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::types::Dosage {
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4b::types::Dosage>>
{
    type Value = Box<fhirbolt_model::r4b::types::Dosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::types::Dosage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4b::types::Dosage>>
{
    type Value = Vec<fhirbolt_model::r4b::types::Dosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::types::Dosage>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::types::Dosage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<fhirbolt_model::r4b::types::Dosage>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
