// Generated on 2023-04-13 by fhirbolt-codegen v0.1.0
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#substance.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("substance", ctx))?;
        }
        if !self.value.r#manifestation.is_empty() {
            self.with_context(&self.value.r#manifestation, |ctx| {
                state.serialize_entry("manifestation", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#description.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#onset.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("onset", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_onset", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#onset.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("onset", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#severity.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("severity", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_severity", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#severity.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("severity", ctx))?;
            }
        }
        if let Some(some) = self.value.r#exposure_route.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("exposureRoute", ctx))?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
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
    for crate::SerializationContext<
        &Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
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
impl serde::ser::Serialize
    for crate::SerializationContext<
        &Vec<Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>>,
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
        fhirbolt_model::r4b::resources::AllergyIntoleranceReaction,
    >
{
    type Value = fhirbolt_model::r4b::resources::AllergyIntoleranceReaction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::AllergyIntoleranceReaction,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::AllergyIntoleranceReaction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AllergyIntoleranceReaction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction, V::Error>
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
                    #[serde(rename = "substance")]
                    Substance,
                    #[serde(rename = "manifestation")]
                    Manifestation,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "onset")]
                    Onset,
                    #[serde(rename = "_onset")]
                    OnsetPrimitiveElement,
                    #[serde(rename = "severity")]
                    Severity,
                    #[serde(rename = "_severity")]
                    SeverityPrimitiveElement,
                    #[serde(rename = "exposureRoute")]
                    ExposureRoute,
                    #[serde(rename = "note")]
                    Note,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "substance",
                            "manifestation",
                            "description",
                            "onset",
                            "severity",
                            "exposureRoute",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#substance: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#manifestation: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#onset: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#severity: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#exposure_route: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Manifestation => {
                            if self.0.from_json {
                                if r#manifestation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manifestation"));
                                }
                                r#manifestation =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#manifestation.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Onset => {
                            if self.0.from_json {
                                let some = r#onset.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onset"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onset"));
                                }
                                r#onset = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::OnsetPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#onset.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_onset"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("onset");
                            }
                        }
                        Field::Severity => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#severity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("severity"));
                                }
                                r#severity = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::SeverityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#severity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_severity"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("severity");
                            }
                        }
                        Field::ExposureRoute => {
                            if r#exposure_route.is_some() {
                                return Err(serde::de::Error::duplicate_field("exposureRoute"));
                            }
                            r#exposure_route = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::AllergyIntoleranceReaction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#substance,
                    r#manifestation: r#manifestation.unwrap_or(vec![]),
                    r#description,
                    r#onset,
                    r#severity,
                    r#exposure_route,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>;
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
                        .transmute::<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: AllergyIntoleranceReaction >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::AllergyIntolerance {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl serde::ser::Serialize
    for crate::SerializationContext<&fhirbolt_model::r4b::resources::AllergyIntolerance>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "AllergyIntolerance")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
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
        if let Some(some) = self.value.r#clinical_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("clinicalStatus", ctx))?;
        }
        if let Some(some) = self.value.r#verification_status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("verificationStatus", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("type", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_type", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#type.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#category.is_empty() {
                let values = self
                    .value
                    .r#category
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("category", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#category
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#category
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(super::super::serde_helpers::PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_category", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#category.is_empty() {
                self.with_context(&self.value.r#category, |ctx| {
                    state.serialize_entry("category", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#criticality.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("criticality", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_criticality", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#criticality.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("criticality", ctx))?;
            }
        }
        if let Some(some) = self.value.r#code.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("code", ctx))?;
        }
        self.with_context(&self.value.r#patient, |ctx| {
            state.serialize_entry("patient", ctx)
        })?;
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if let Some(some) = self.value.r#onset.as_ref() {
            match some {
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::DateTime(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("onsetDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_onsetDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("onsetDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Age(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("onsetAge", ctx))?;
                }
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("onsetPeriod", ctx))?;
                }
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Range(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("onsetRange", ctx))?;
                }
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::String(ref value) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("onsetString", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_onsetString", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| state.serialize_entry("onsetString", ctx))?;
                    }
                }
                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Invalid => {
                    return Err(serde::ser::Error::custom("onset is invalid"))
                }
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#recorded_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("recordedDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_recordedDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#recorded_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("recordedDate", ctx))?;
            }
        }
        if let Some(some) = self.value.r#recorder.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("recorder", ctx))?;
        }
        if let Some(some) = self.value.r#asserter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("asserter", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#last_occurrence.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("lastOccurrence", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_lastOccurrence", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#last_occurrence.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("lastOccurrence", ctx))?;
            }
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if !self.value.r#reaction.is_empty() {
            self.with_context(&self.value.r#reaction, |ctx| {
                state.serialize_entry("reaction", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<fhirbolt_model::r4b::resources::AllergyIntolerance>>
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
impl serde::ser::Serialize
    for crate::SerializationContext<&Vec<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>>
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
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::AllergyIntolerance,
    >
{
    type Value = fhirbolt_model::r4b::resources::AllergyIntolerance;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::AllergyIntolerance,
    >
{
    type Value = fhirbolt_model::r4b::resources::AllergyIntolerance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::AllergyIntolerance,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::AllergyIntolerance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AllergyIntolerance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::AllergyIntolerance, V::Error>
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
                    #[serde(rename = "clinicalStatus")]
                    ClinicalStatus,
                    #[serde(rename = "verificationStatus")]
                    VerificationStatus,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "_category")]
                    CategoryPrimitiveElement,
                    #[serde(rename = "criticality")]
                    Criticality,
                    #[serde(rename = "_criticality")]
                    CriticalityPrimitiveElement,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "onsetDateTime")]
                    OnsetDateTime,
                    #[serde(rename = "_onsetDateTime")]
                    OnsetDateTimePrimitiveElement,
                    #[serde(rename = "onsetAge")]
                    OnsetAge,
                    #[serde(rename = "onsetPeriod")]
                    OnsetPeriod,
                    #[serde(rename = "onsetRange")]
                    OnsetRange,
                    #[serde(rename = "onsetString")]
                    OnsetString,
                    #[serde(rename = "_onsetString")]
                    OnsetStringPrimitiveElement,
                    #[serde(rename = "recordedDate")]
                    RecordedDate,
                    #[serde(rename = "_recordedDate")]
                    RecordedDatePrimitiveElement,
                    #[serde(rename = "recorder")]
                    Recorder,
                    #[serde(rename = "asserter")]
                    Asserter,
                    #[serde(rename = "lastOccurrence")]
                    LastOccurrence,
                    #[serde(rename = "_lastOccurrence")]
                    LastOccurrencePrimitiveElement,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "reaction")]
                    Reaction,
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
                            "clinicalStatus",
                            "verificationStatus",
                            "type",
                            "category",
                            "criticality",
                            "code",
                            "patient",
                            "encounter",
                            "onsetDateTime",
                            "onsetAge",
                            "onsetPeriod",
                            "onsetRange",
                            "onsetString",
                            "recordedDate",
                            "recorder",
                            "asserter",
                            "lastOccurrence",
                            "note",
                            "reaction",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4b::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#clinical_status: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#verification_status: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#type: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#category: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#criticality: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#onset: Option<fhirbolt_model::r4b::resources::AllergyIntoleranceOnset> =
                    None;
                let mut r#recorded_date: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#recorder: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#asserter: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#last_occurrence: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
                let mut r#reaction: Option<
                    Vec<fhirbolt_model::r4b::resources::AllergyIntoleranceReaction>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "AllergyIntolerance" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"AllergyIntolerance",
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
                            r#meta = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Meta>>(),
                            )?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                r#implicit_rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
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
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<Box<fhirbolt_model::r4b::Resource>>>(),
                                    )?,
                                );
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::ClinicalStatus => {
                            if r#clinical_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("clinicalStatus"));
                            }
                            r#clinical_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::VerificationStatus => {
                            if r#verification_status.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "verificationStatus",
                                ));
                            }
                            r#verification_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Type => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::TypePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#type.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_type"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#category.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::CategoryPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#category.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_category"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("category");
                            }
                        }
                        Field::Criticality => {
                            if self.0.from_json {
                                let some = r#criticality.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("criticality"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#criticality.is_some() {
                                    return Err(serde::de::Error::duplicate_field("criticality"));
                                }
                                r#criticality = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::CriticalityPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#criticality.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_criticality"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("criticality");
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::OnsetDateTime => {
                            if self.0.from_json {
                                let r#enum = r#onset . get_or_insert (fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("onsetDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("onset[x]")) ; }
                            } else {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetDateTime"));
                                }
                                r#onset = Some (fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::OnsetDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#onset . get_or_insert (fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_onsetDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_onset[x]")) ; }
                            } else {
                                return unknown_field_error("onsetDateTime");
                            }
                        }
                        Field::OnsetAge => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetAge"));
                            }
                            r#onset = Some(
                                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Age(
                                    map_access.next_value_seed(
                                        self.0.transmute::<Box<fhirbolt_model::r4b::types::Age>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::OnsetPeriod => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetPeriod"));
                            }
                            r#onset = Some(
                                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::OnsetRange => {
                            if r#onset.is_some() {
                                return Err(serde::de::Error::duplicate_field("onsetRange"));
                            }
                            r#onset = Some(
                                fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::Range(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Range>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::OnsetString => {
                            if self.0.from_json {
                                let r#enum = r#onset.get_or_insert(
                                    fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("onsetString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("onset[x]")) ; }
                            } else {
                                if r#onset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("onsetString"));
                                }
                                r#onset = Some (fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::OnsetStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#onset.get_or_insert(
                                    fhirbolt_model::r4b::resources::AllergyIntoleranceOnset::String(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: AllergyIntoleranceOnset :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_onsetString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_onset[x]")) ; }
                            } else {
                                return unknown_field_error("onsetString");
                            }
                        }
                        Field::RecordedDate => {
                            if self.0.from_json {
                                let some = r#recorded_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recordedDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#recorded_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recordedDate"));
                                }
                                r#recorded_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::RecordedDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#recorded_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recordedDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recordedDate");
                            }
                        }
                        Field::Recorder => {
                            if r#recorder.is_some() {
                                return Err(serde::de::Error::duplicate_field("recorder"));
                            }
                            r#recorder = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Asserter => {
                            if r#asserter.is_some() {
                                return Err(serde::de::Error::duplicate_field("asserter"));
                            }
                            r#asserter = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::LastOccurrence => {
                            if self.0.from_json {
                                let some = r#last_occurrence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastOccurrence",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#last_occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lastOccurrence",
                                    ));
                                }
                                r#last_occurrence = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::LastOccurrencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#last_occurrence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_lastOccurrence",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("lastOccurrence");
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::Reaction => {
                            if self.0.from_json {
                                if r#reaction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reaction"));
                                }
                                r#reaction =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::AllergyIntoleranceReaction,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#reaction.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: AllergyIntoleranceReaction > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::AllergyIntolerance {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#clinical_status,
                    r#verification_status,
                    r#type,
                    r#category: r#category.unwrap_or(vec![]),
                    r#criticality,
                    r#code,
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                    },
                    r#encounter,
                    r#onset,
                    r#recorded_date,
                    r#recorder,
                    r#asserter,
                    r#last_occurrence,
                    r#note: r#note.unwrap_or(vec![]),
                    r#reaction: r#reaction.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::AllergyIntolerance>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::AllergyIntolerance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::AllergyIntolerance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::AllergyIntolerance>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::AllergyIntolerance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::AllergyIntolerance>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::AllergyIntolerance>;
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
                        .transmute::<fhirbolt_model::r4b::resources::AllergyIntolerance>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
