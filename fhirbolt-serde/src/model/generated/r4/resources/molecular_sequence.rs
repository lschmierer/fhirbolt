// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::MolecularSequenceReferenceSeq;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceReferenceSeq> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.referenceSeq", field
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
        if let Some(some) = self.value.r#chromosome.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("chromosome", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#genome_build.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("genomeBuild", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_genomeBuild", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#genome_build.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("genomeBuild", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#orientation.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("orientation", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_orientation", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#orientation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("orientation", ctx))?;
        }
        if let Some(some) = self.value.r#reference_seq_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceSeqId", ctx))?;
        }
        if let Some(some) = self.value.r#reference_seq_pointer.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("referenceSeqPointer", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#reference_seq_string.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("referenceSeqString", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_referenceSeqString", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#reference_seq_string.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceSeqString", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#strand.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("strand", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_strand", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#strand.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("strand", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#window_start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("windowStart", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_windowStart", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#window_start.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("windowStart", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#window_end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("windowEnd", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_windowEnd", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#window_end.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("windowEnd", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceReferenceSeq>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceReferenceSeq>> {
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
    for &mut DeserializationContext<MolecularSequenceReferenceSeq>
{
    type Value = MolecularSequenceReferenceSeq;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceReferenceSeq>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceReferenceSeq;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceReferenceSeq")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceReferenceSeq, V::Error>
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
                    #[serde(rename = "chromosome")]
                    Chromosome,
                    #[serde(rename = "genomeBuild")]
                    GenomeBuild,
                    #[serde(rename = "_genomeBuild")]
                    GenomeBuildPrimitiveElement,
                    #[serde(rename = "orientation")]
                    Orientation,
                    #[serde(rename = "_orientation")]
                    OrientationPrimitiveElement,
                    #[serde(rename = "referenceSeqId")]
                    ReferenceSeqId,
                    #[serde(rename = "referenceSeqPointer")]
                    ReferenceSeqPointer,
                    #[serde(rename = "referenceSeqString")]
                    ReferenceSeqString,
                    #[serde(rename = "_referenceSeqString")]
                    ReferenceSeqStringPrimitiveElement,
                    #[serde(rename = "strand")]
                    Strand,
                    #[serde(rename = "_strand")]
                    StrandPrimitiveElement,
                    #[serde(rename = "windowStart")]
                    WindowStart,
                    #[serde(rename = "_windowStart")]
                    WindowStartPrimitiveElement,
                    #[serde(rename = "windowEnd")]
                    WindowEnd,
                    #[serde(rename = "_windowEnd")]
                    WindowEndPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "chromosome",
                            "genomeBuild",
                            "orientation",
                            "referenceSeqId",
                            "referenceSeqPointer",
                            "referenceSeqString",
                            "strand",
                            "windowStart",
                            "windowEnd",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#chromosome: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#genome_build: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#orientation: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#reference_seq_id: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#reference_seq_pointer: Option<Box<fhirbolt_model::r4::types::Reference>> =
                    None;
                let mut r#reference_seq_string: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#strand: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#window_start: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#window_end: Option<fhirbolt_model::r4::types::Integer> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Chromosome => {
                            if r#chromosome.is_some() {
                                return Err(serde::de::Error::duplicate_field("chromosome"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#chromosome = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::GenomeBuild => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#genome_build.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("genomeBuild"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#genome_build.is_some() {
                                    return Err(serde::de::Error::duplicate_field("genomeBuild"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#genome_build = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::GenomeBuildPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#genome_build.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_genomeBuild"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("genomeBuild");
                            }
                        }
                        Field::Orientation => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#orientation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orientation"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#orientation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("orientation"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#orientation = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::OrientationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#orientation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_orientation"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("orientation");
                            }
                        }
                        Field::ReferenceSeqId => {
                            if r#reference_seq_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceSeqId"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#reference_seq_id = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ReferenceSeqPointer => {
                            if r#reference_seq_pointer.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "referenceSeqPointer",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#reference_seq_pointer =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ReferenceSeqString => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#reference_seq_string.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceSeqString",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#reference_seq_string.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceSeqString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#reference_seq_string =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReferenceSeqStringPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#reference_seq_string.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_referenceSeqString",
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
                                return unknown_field_error("referenceSeqString");
                            }
                        }
                        Field::Strand => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#strand.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strand"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#strand.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strand"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#strand = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StrandPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#strand.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_strand"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("strand");
                            }
                        }
                        Field::WindowStart => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#window_start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("windowStart"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#window_start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("windowStart"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#window_start = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WindowStartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#window_start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_windowStart"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("windowStart");
                            }
                        }
                        Field::WindowEnd => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#window_end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("windowEnd"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#window_end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("windowEnd"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#window_end = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::WindowEndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#window_end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_windowEnd"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("windowEnd");
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
                Ok(MolecularSequenceReferenceSeq {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#chromosome,
                    r#genome_build,
                    r#orientation,
                    r#reference_seq_id,
                    r#reference_seq_pointer,
                    r#reference_seq_string,
                    r#strand,
                    r#window_start,
                    r#window_end,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceReferenceSeq>>
{
    type Value = Box<MolecularSequenceReferenceSeq>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceReferenceSeq>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceReferenceSeq>>
{
    type Value = Vec<MolecularSequenceReferenceSeq>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceReferenceSeq>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceReferenceSeq>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceReferenceSeq> =
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
use fhirbolt_model::r4::resources::MolecularSequenceVariant;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceVariant> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.variant", field
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
            if let Some(some) = self.value.r#start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("start", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_start", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#start.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("start", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("end", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_end", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#end.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("end", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#observed_allele.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("observedAllele", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_observedAllele", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#observed_allele.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("observedAllele", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#reference_allele.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("referenceAllele", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_referenceAllele", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#reference_allele.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceAllele", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#cigar.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("cigar", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_cigar", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#cigar.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("cigar", ctx))?;
        }
        if let Some(some) = self.value.r#variant_pointer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("variantPointer", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceVariant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceVariant>> {
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
    for &mut DeserializationContext<MolecularSequenceVariant>
{
    type Value = MolecularSequenceVariant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceVariant>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceVariant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceVariant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequenceVariant, V::Error>
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
                    #[serde(rename = "start")]
                    Start,
                    #[serde(rename = "_start")]
                    StartPrimitiveElement,
                    #[serde(rename = "end")]
                    End,
                    #[serde(rename = "_end")]
                    EndPrimitiveElement,
                    #[serde(rename = "observedAllele")]
                    ObservedAllele,
                    #[serde(rename = "_observedAllele")]
                    ObservedAllelePrimitiveElement,
                    #[serde(rename = "referenceAllele")]
                    ReferenceAllele,
                    #[serde(rename = "_referenceAllele")]
                    ReferenceAllelePrimitiveElement,
                    #[serde(rename = "cigar")]
                    Cigar,
                    #[serde(rename = "_cigar")]
                    CigarPrimitiveElement,
                    #[serde(rename = "variantPointer")]
                    VariantPointer,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "start",
                            "end",
                            "observedAllele",
                            "referenceAllele",
                            "cigar",
                            "variantPointer",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#start: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#end: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#observed_allele: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#reference_allele: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#cigar: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#variant_pointer: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Start => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#start = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_start"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("start");
                            }
                        }
                        Field::End => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#end = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_end"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("end");
                            }
                        }
                        Field::ObservedAllele => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#observed_allele.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "observedAllele",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#observed_allele.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "observedAllele",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#observed_allele =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ObservedAllelePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#observed_allele.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_observedAllele",
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
                                return unknown_field_error("observedAllele");
                            }
                        }
                        Field::ReferenceAllele => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#reference_allele.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceAllele",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#reference_allele.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceAllele",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#reference_allele =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReferenceAllelePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#reference_allele.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_referenceAllele",
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
                                return unknown_field_error("referenceAllele");
                            }
                        }
                        Field::Cigar => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cigar.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cigar"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#cigar.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cigar"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#cigar = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CigarPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cigar.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_cigar"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("cigar");
                            }
                        }
                        Field::VariantPointer => {
                            if r#variant_pointer.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantPointer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#variant_pointer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MolecularSequenceVariant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#start,
                    r#end,
                    r#observed_allele,
                    r#reference_allele,
                    r#cigar,
                    r#variant_pointer,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceVariant>>
{
    type Value = Box<MolecularSequenceVariant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceVariant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceVariant>>
{
    type Value = Vec<MolecularSequenceVariant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceVariant>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceVariant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceVariant> =
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
use fhirbolt_model::r4::resources::MolecularSequenceQualityRoc;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceQualityRoc> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.quality.roc", field
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
            if !self.value.r#score.is_empty() {
                let values = self
                    .value
                    .r#score
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("score", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#score
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#score
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
                        state.serialize_entry("_score", ctx)
                    })?;
                }
            }
        } else if !self.value.r#score.is_empty() {
            self.with_context(&self.value.r#score, |ctx| {
                state.serialize_entry("score", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#num_tp.is_empty() {
                let values = self
                    .value
                    .r#num_tp
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("numTP", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#num_tp
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#num_tp
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
                        state.serialize_entry("_numTP", ctx)
                    })?;
                }
            }
        } else if !self.value.r#num_tp.is_empty() {
            self.with_context(&self.value.r#num_tp, |ctx| {
                state.serialize_entry("numTP", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#num_fp.is_empty() {
                let values = self
                    .value
                    .r#num_fp
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("numFP", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#num_fp
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#num_fp
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
                        state.serialize_entry("_numFP", ctx)
                    })?;
                }
            }
        } else if !self.value.r#num_fp.is_empty() {
            self.with_context(&self.value.r#num_fp, |ctx| {
                state.serialize_entry("numFP", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#num_fn.is_empty() {
                let values = self
                    .value
                    .r#num_fn
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("numFN", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#num_fn
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#num_fn
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
                        state.serialize_entry("_numFN", ctx)
                    })?;
                }
            }
        } else if !self.value.r#num_fn.is_empty() {
            self.with_context(&self.value.r#num_fn, |ctx| {
                state.serialize_entry("numFN", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#precision.is_empty() {
                let values = self
                    .value
                    .r#precision
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| {
                        v.as_ref()
                            .map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            })
                            .transpose()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("precision", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#precision
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#precision
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
                        state.serialize_entry("_precision", ctx)
                    })?;
                }
            }
        } else if !self.value.r#precision.is_empty() {
            self.with_context(&self.value.r#precision, |ctx| {
                state.serialize_entry("precision", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#sensitivity.is_empty() {
                let values = self
                    .value
                    .r#sensitivity
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| {
                        v.as_ref()
                            .map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            })
                            .transpose()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("sensitivity", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#sensitivity
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#sensitivity
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
                        state.serialize_entry("_sensitivity", ctx)
                    })?;
                }
            }
        } else if !self.value.r#sensitivity.is_empty() {
            self.with_context(&self.value.r#sensitivity, |ctx| {
                state.serialize_entry("sensitivity", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if !self.value.r#f_measure.is_empty() {
                let values = self
                    .value
                    .r#f_measure
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| {
                        v.as_ref()
                            .map(|v| {
                                v.parse::<serde_json::Number>().map_err(|_| {
                                    serde::ser::Error::custom("error serializing decimal")
                                })
                            })
                            .transpose()
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("fMeasure", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#f_measure
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#f_measure
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
                        state.serialize_entry("_fMeasure", ctx)
                    })?;
                }
            }
        } else if !self.value.r#f_measure.is_empty() {
            self.with_context(&self.value.r#f_measure, |ctx| {
                state.serialize_entry("fMeasure", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceQualityRoc>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceQualityRoc>> {
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
    for &mut DeserializationContext<MolecularSequenceQualityRoc>
{
    type Value = MolecularSequenceQualityRoc;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceQualityRoc>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceQualityRoc;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceQualityRoc")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceQualityRoc, V::Error>
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
                    #[serde(rename = "score")]
                    Score,
                    #[serde(rename = "_score")]
                    ScorePrimitiveElement,
                    #[serde(rename = "numTP")]
                    NumTp,
                    #[serde(rename = "_numTP")]
                    NumTpPrimitiveElement,
                    #[serde(rename = "numFP")]
                    NumFp,
                    #[serde(rename = "_numFP")]
                    NumFpPrimitiveElement,
                    #[serde(rename = "numFN")]
                    NumFn,
                    #[serde(rename = "_numFN")]
                    NumFnPrimitiveElement,
                    #[serde(rename = "precision")]
                    Precision,
                    #[serde(rename = "_precision")]
                    PrecisionPrimitiveElement,
                    #[serde(rename = "sensitivity")]
                    Sensitivity,
                    #[serde(rename = "_sensitivity")]
                    SensitivityPrimitiveElement,
                    #[serde(rename = "fMeasure")]
                    FMeasure,
                    #[serde(rename = "_fMeasure")]
                    FMeasurePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "score",
                            "numTP",
                            "numFP",
                            "numFN",
                            "precision",
                            "sensitivity",
                            "fMeasure",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#score: Option<Vec<fhirbolt_model::r4::types::Integer>> = None;
                let mut r#num_tp: Option<Vec<fhirbolt_model::r4::types::Integer>> = None;
                let mut r#num_fp: Option<Vec<fhirbolt_model::r4::types::Integer>> = None;
                let mut r#num_fn: Option<Vec<fhirbolt_model::r4::types::Integer>> = None;
                let mut r#precision: Option<Vec<fhirbolt_model::r4::types::Decimal>> = None;
                let mut r#sensitivity: Option<Vec<fhirbolt_model::r4::types::Decimal>> = None;
                let mut r#f_measure: Option<Vec<fhirbolt_model::r4::types::Decimal>> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Score => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#score.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("score"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#score.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ScorePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#score.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_score"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("score");
                            }
                        }
                        Field::NumTp => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#num_tp.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("numTP"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#num_tp.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NumTpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#num_tp.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_numTP"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("numTP");
                            }
                        }
                        Field::NumFp => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#num_fp.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("numFP"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#num_fp.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NumFpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#num_fp.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_numFP"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("numFP");
                            }
                        }
                        Field::NumFn => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#num_fn.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("numFN"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#num_fn.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::NumFnPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#num_fn.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_numFN"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("numFN");
                            }
                        }
                        Field::Precision => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<serde_json::Number>> =
                                    map_access.next_value()?;
                                let vec = r#precision.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("precision"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(format!("{}", value));
                                    }
                                }
                            } else {
                                let vec = r#precision.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PrecisionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#precision.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_precision"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("precision");
                            }
                        }
                        Field::Sensitivity => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<serde_json::Number>> =
                                    map_access.next_value()?;
                                let vec = r#sensitivity.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("sensitivity"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(format!("{}", value));
                                    }
                                }
                            } else {
                                let vec = r#sensitivity.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SensitivityPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#sensitivity.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_sensitivity"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("sensitivity");
                            }
                        }
                        Field::FMeasure => {
                            if self.0.from == crate::context::Format::Json {
                                let values: Vec<Option<serde_json::Number>> =
                                    map_access.next_value()?;
                                let vec = r#f_measure.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("fMeasure"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(format!("{}", value));
                                    }
                                }
                            } else {
                                let vec = r#f_measure.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FMeasurePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#f_measure.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_fMeasure"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("fMeasure");
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
                Ok(MolecularSequenceQualityRoc {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#score: r#score.unwrap_or(vec![]),
                    r#num_tp: r#num_tp.unwrap_or(vec![]),
                    r#num_fp: r#num_fp.unwrap_or(vec![]),
                    r#num_fn: r#num_fn.unwrap_or(vec![]),
                    r#precision: r#precision.unwrap_or(vec![]),
                    r#sensitivity: r#sensitivity.unwrap_or(vec![]),
                    r#f_measure: r#f_measure.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceQualityRoc>>
{
    type Value = Box<MolecularSequenceQualityRoc>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceQualityRoc>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceQualityRoc>>
{
    type Value = Vec<MolecularSequenceQualityRoc>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceQualityRoc>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceQualityRoc>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceQualityRoc> =
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
use fhirbolt_model::r4::resources::MolecularSequenceQuality;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceQuality> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.quality", field
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
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#standard_sequence.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("standardSequence", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("start", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_start", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#start.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("start", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("end", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_end", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#end.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("end", ctx))?;
        }
        if let Some(some) = self.value.r#score.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("score", ctx))?;
        }
        if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#truth_tp.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("truthTP", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_truthTP", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#truth_tp.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("truthTP", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#query_tp.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("queryTP", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_queryTP", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#query_tp.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("queryTP", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#truth_fn.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("truthFN", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_truthFN", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#truth_fn.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("truthFN", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#query_fp.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("queryFP", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_queryFP", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#query_fp.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("queryFP", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#gt_fp.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("gtFP", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_gtFP", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#gt_fp.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("gtFP", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#precision.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("precision", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_precision", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#precision.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("precision", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#recall.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("recall", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_recall", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#recall.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("recall", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#f_score.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("fScore", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_fScore", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#f_score.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("fScore", ctx))?;
        }
        if let Some(some) = self.value.r#roc.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("roc", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceQuality>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceQuality>> {
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
    for &mut DeserializationContext<MolecularSequenceQuality>
{
    type Value = MolecularSequenceQuality;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceQuality>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceQuality;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceQuality")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequenceQuality, V::Error>
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
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "standardSequence")]
                    StandardSequence,
                    #[serde(rename = "start")]
                    Start,
                    #[serde(rename = "_start")]
                    StartPrimitiveElement,
                    #[serde(rename = "end")]
                    End,
                    #[serde(rename = "_end")]
                    EndPrimitiveElement,
                    #[serde(rename = "score")]
                    Score,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "truthTP")]
                    TruthTp,
                    #[serde(rename = "_truthTP")]
                    TruthTpPrimitiveElement,
                    #[serde(rename = "queryTP")]
                    QueryTp,
                    #[serde(rename = "_queryTP")]
                    QueryTpPrimitiveElement,
                    #[serde(rename = "truthFN")]
                    TruthFn,
                    #[serde(rename = "_truthFN")]
                    TruthFnPrimitiveElement,
                    #[serde(rename = "queryFP")]
                    QueryFp,
                    #[serde(rename = "_queryFP")]
                    QueryFpPrimitiveElement,
                    #[serde(rename = "gtFP")]
                    GtFp,
                    #[serde(rename = "_gtFP")]
                    GtFpPrimitiveElement,
                    #[serde(rename = "precision")]
                    Precision,
                    #[serde(rename = "_precision")]
                    PrecisionPrimitiveElement,
                    #[serde(rename = "recall")]
                    Recall,
                    #[serde(rename = "_recall")]
                    RecallPrimitiveElement,
                    #[serde(rename = "fScore")]
                    FScore,
                    #[serde(rename = "_fScore")]
                    FScorePrimitiveElement,
                    #[serde(rename = "roc")]
                    Roc,
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
                            "standardSequence",
                            "start",
                            "end",
                            "score",
                            "method",
                            "truthTP",
                            "queryTP",
                            "truthFN",
                            "queryFP",
                            "gtFP",
                            "precision",
                            "recall",
                            "fScore",
                            "roc",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#standard_sequence: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#start: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#end: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#score: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#truth_tp: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#query_tp: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#truth_fn: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#query_fp: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#gt_fp: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#precision: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#recall: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#f_score: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#roc: Option<fhirbolt_model::r4::resources::MolecularSequenceQualityRoc> =
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
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::StandardSequence => {
                            if r#standard_sequence.is_some() {
                                return Err(serde::de::Error::duplicate_field("standardSequence"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#standard_sequence = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Start => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#start = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_start"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("start");
                            }
                        }
                        Field::End => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#end = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_end"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("end");
                            }
                        }
                        Field::Score => {
                            if r#score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#score = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#method = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TruthTp => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#truth_tp.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("truthTP"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#truth_tp.is_some() {
                                    return Err(serde::de::Error::duplicate_field("truthTP"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#truth_tp = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TruthTpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#truth_tp.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_truthTP"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("truthTP");
                            }
                        }
                        Field::QueryTp => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query_tp.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("queryTP"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#query_tp.is_some() {
                                    return Err(serde::de::Error::duplicate_field("queryTP"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#query_tp = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::QueryTpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query_tp.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_queryTP"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("queryTP");
                            }
                        }
                        Field::TruthFn => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#truth_fn.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("truthFN"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#truth_fn.is_some() {
                                    return Err(serde::de::Error::duplicate_field("truthFN"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#truth_fn = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TruthFnPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#truth_fn.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_truthFN"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("truthFN");
                            }
                        }
                        Field::QueryFp => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query_fp.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("queryFP"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#query_fp.is_some() {
                                    return Err(serde::de::Error::duplicate_field("queryFP"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#query_fp = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::QueryFpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#query_fp.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_queryFP"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("queryFP");
                            }
                        }
                        Field::GtFp => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#gt_fp.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gtFP"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#gt_fp.is_some() {
                                    return Err(serde::de::Error::duplicate_field("gtFP"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#gt_fp = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::GtFpPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#gt_fp.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_gtFP"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("gtFP");
                            }
                        }
                        Field::Precision => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#precision.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("precision"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#precision.is_some() {
                                    return Err(serde::de::Error::duplicate_field("precision"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#precision = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PrecisionPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#precision.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_precision"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("precision");
                            }
                        }
                        Field::Recall => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recall.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recall"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#recall.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recall"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#recall = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RecallPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#recall.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recall"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recall");
                            }
                        }
                        Field::FScore => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#f_score.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fScore"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#f_score.is_some() {
                                    return Err(serde::de::Error::duplicate_field("fScore"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Decimal,
                                > = self.0.transmute();
                                r#f_score = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::FScorePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#f_score.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_fScore"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("fScore");
                            }
                        }
                        Field::Roc => {
                            if r#roc.is_some() {
                                return Err(serde::de::Error::duplicate_field("roc"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::MolecularSequenceQualityRoc,
                            > = self.0.transmute();
                            r#roc = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MolecularSequenceQuality {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#standard_sequence,
                    r#start,
                    r#end,
                    r#score,
                    r#method,
                    r#truth_tp,
                    r#query_tp,
                    r#truth_fn,
                    r#query_fp,
                    r#gt_fp,
                    r#precision,
                    r#recall,
                    r#f_score,
                    r#roc,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceQuality>>
{
    type Value = Box<MolecularSequenceQuality>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceQuality>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceQuality>>
{
    type Value = Vec<MolecularSequenceQuality>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceQuality>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceQuality>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceQuality> =
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
use fhirbolt_model::r4::resources::MolecularSequenceRepository;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceRepository> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.repository", field
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
        } else {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("url", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#url.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
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
            if let Some(some) = self.value.r#dataset_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("datasetId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_datasetId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#dataset_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("datasetId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#variantset_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("variantsetId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_variantsetId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#variantset_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("variantsetId", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#readset_id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("readsetId", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_readsetId", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#readset_id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("readsetId", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceRepository>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceRepository>> {
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
    for &mut DeserializationContext<MolecularSequenceRepository>
{
    type Value = MolecularSequenceRepository;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceRepository>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceRepository;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceRepository")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceRepository, V::Error>
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
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "datasetId")]
                    DatasetId,
                    #[serde(rename = "_datasetId")]
                    DatasetIdPrimitiveElement,
                    #[serde(rename = "variantsetId")]
                    VariantsetId,
                    #[serde(rename = "_variantsetId")]
                    VariantsetIdPrimitiveElement,
                    #[serde(rename = "readsetId")]
                    ReadsetId,
                    #[serde(rename = "_readsetId")]
                    ReadsetIdPrimitiveElement,
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
                            "url",
                            "name",
                            "datasetId",
                            "variantsetId",
                            "readsetId",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#url: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#dataset_id: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#variantset_id: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#readset_id: Option<fhirbolt_model::r4::types::String> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::Url => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#url = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
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
                                    fhirbolt_model::r4::types::String,
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
                        Field::DatasetId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#dataset_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("datasetId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#dataset_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("datasetId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#dataset_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DatasetIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#dataset_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_datasetId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("datasetId");
                            }
                        }
                        Field::VariantsetId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#variantset_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variantsetId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#variantset_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variantsetId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#variantset_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VariantsetIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#variantset_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_variantsetId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("variantsetId");
                            }
                        }
                        Field::ReadsetId => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#readset_id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("readsetId"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#readset_id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("readsetId"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#readset_id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReadsetIdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#readset_id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_readsetId"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("readsetId");
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
                Ok(MolecularSequenceRepository {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#url,
                    r#name,
                    r#dataset_id,
                    r#variantset_id,
                    r#readset_id,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceRepository>>
{
    type Value = Box<MolecularSequenceRepository>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceRepository>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceRepository>>
{
    type Value = Vec<MolecularSequenceRepository>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceRepository>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceRepository>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceRepository> =
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
use fhirbolt_model::r4::resources::MolecularSequenceStructureVariantOuter;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceStructureVariantOuter> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.structureVariant.outer", field
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
            if let Some(some) = self.value.r#start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("start", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_start", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#start.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("start", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("end", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_end", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#end.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("end", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceStructureVariantOuter>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceStructureVariantOuter>> {
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
    for &mut DeserializationContext<MolecularSequenceStructureVariantOuter>
{
    type Value = MolecularSequenceStructureVariantOuter;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceStructureVariantOuter>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceStructureVariantOuter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceStructureVariantOuter")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceStructureVariantOuter, V::Error>
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
                    #[serde(rename = "start")]
                    Start,
                    #[serde(rename = "_start")]
                    StartPrimitiveElement,
                    #[serde(rename = "end")]
                    End,
                    #[serde(rename = "_end")]
                    EndPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "start", "end"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#start: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#end: Option<fhirbolt_model::r4::types::Integer> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Start => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#start = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_start"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("start");
                            }
                        }
                        Field::End => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#end = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_end"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("end");
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
                Ok(MolecularSequenceStructureVariantOuter {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#start,
                    r#end,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceStructureVariantOuter>>
{
    type Value = Box<MolecularSequenceStructureVariantOuter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceStructureVariantOuter>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceStructureVariantOuter>>
{
    type Value = Vec<MolecularSequenceStructureVariantOuter>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MolecularSequenceStructureVariantOuter>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceStructureVariantOuter>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceStructureVariantOuter> =
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
use fhirbolt_model::r4::resources::MolecularSequenceStructureVariantInner;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceStructureVariantInner> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.structureVariant.inner", field
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
            if let Some(some) = self.value.r#start.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("start", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_start", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#start.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("start", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#end.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("end", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_end", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#end.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("end", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceStructureVariantInner>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceStructureVariantInner>> {
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
    for &mut DeserializationContext<MolecularSequenceStructureVariantInner>
{
    type Value = MolecularSequenceStructureVariantInner;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceStructureVariantInner>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceStructureVariantInner;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceStructureVariantInner")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceStructureVariantInner, V::Error>
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
                    #[serde(rename = "start")]
                    Start,
                    #[serde(rename = "_start")]
                    StartPrimitiveElement,
                    #[serde(rename = "end")]
                    End,
                    #[serde(rename = "_end")]
                    EndPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "start", "end"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#start: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#end: Option<fhirbolt_model::r4::types::Integer> = None;
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Start => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#start.is_some() {
                                    return Err(serde::de::Error::duplicate_field("start"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#start = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StartPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#start.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_start"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("start");
                            }
                        }
                        Field::End => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#end.is_some() {
                                    return Err(serde::de::Error::duplicate_field("end"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#end = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EndPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#end.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_end"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("end");
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
                Ok(MolecularSequenceStructureVariantInner {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#start,
                    r#end,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceStructureVariantInner>>
{
    type Value = Box<MolecularSequenceStructureVariantInner>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceStructureVariantInner>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceStructureVariantInner>>
{
    type Value = Vec<MolecularSequenceStructureVariantInner>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<MolecularSequenceStructureVariantInner>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceStructureVariantInner>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceStructureVariantInner> =
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
use fhirbolt_model::r4::resources::MolecularSequenceStructureVariant;
impl serde::ser::Serialize for SerializationContext<&MolecularSequenceStructureVariant> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence.structureVariant", field
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
        if let Some(some) = self.value.r#variant_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("variantType", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#exact.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("exact", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_exact", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#exact.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("exact", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#length.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("length", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_length", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#length.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("length", ctx))?;
        }
        if let Some(some) = self.value.r#outer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("outer", ctx))?;
        }
        if let Some(some) = self.value.r#inner.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("inner", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequenceStructureVariant>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequenceStructureVariant>> {
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
    for &mut DeserializationContext<MolecularSequenceStructureVariant>
{
    type Value = MolecularSequenceStructureVariant;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequenceStructureVariant>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequenceStructureVariant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceStructureVariant")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MolecularSequenceStructureVariant, V::Error>
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
                    #[serde(rename = "variantType")]
                    VariantType,
                    #[serde(rename = "exact")]
                    Exact,
                    #[serde(rename = "_exact")]
                    ExactPrimitiveElement,
                    #[serde(rename = "length")]
                    Length,
                    #[serde(rename = "_length")]
                    LengthPrimitiveElement,
                    #[serde(rename = "outer")]
                    Outer,
                    #[serde(rename = "inner")]
                    Inner,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "variantType",
                            "exact",
                            "length",
                            "outer",
                            "inner",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#variant_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#exact: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#length: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#outer: Option<
                    fhirbolt_model::r4::resources::MolecularSequenceStructureVariantOuter,
                > = None;
                let mut r#inner: Option<
                    fhirbolt_model::r4::resources::MolecularSequenceStructureVariantInner,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::VariantType => {
                            if r#variant_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#variant_type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Exact => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#exact.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exact"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#exact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exact"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#exact = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ExactPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#exact.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_exact"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("exact");
                            }
                        }
                        Field::Length => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#length.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#length = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LengthPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_length"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("length");
                            }
                        }
                        Field::Outer => {
                            if r#outer.is_some() {
                                return Err(serde::de::Error::duplicate_field("outer"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: MolecularSequenceStructureVariantOuter > = self . 0 . transmute () ;
                            r#outer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Inner => {
                            if r#inner.is_some() {
                                return Err(serde::de::Error::duplicate_field("inner"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: MolecularSequenceStructureVariantInner > = self . 0 . transmute () ;
                            r#inner = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(MolecularSequenceStructureVariant {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#variant_type,
                    r#exact,
                    r#length,
                    r#outer,
                    r#inner,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<MolecularSequenceStructureVariant>>
{
    type Value = Box<MolecularSequenceStructureVariant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequenceStructureVariant>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<MolecularSequenceStructureVariant>>
{
    type Value = Vec<MolecularSequenceStructureVariant>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequenceStructureVariant>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequenceStructureVariant>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequenceStructureVariant> =
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
use fhirbolt_model::r4::resources::MolecularSequence;
impl crate::Resource for MolecularSequence {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&MolecularSequence> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "MolecularSequence", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MolecularSequence")?;
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
            if let Some(some) = self.value.r#type.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("type", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_type", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#coordinate_system.id.as_deref() == Some("$invalid") {
                return missing_field_error("coordinateSystem");
            }
            if let Some(some) = self.value.r#coordinate_system.value.as_ref().map(Ok) {
                state.serialize_entry("coordinateSystem", &some?)?;
            }
            if self.value.r#coordinate_system.id.is_some()
                || !self.value.r#coordinate_system.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#coordinate_system.id.as_ref(),
                    extension: &self.value.r#coordinate_system.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_coordinateSystem", ctx)
                })?;
            }
        } else if self.value.r#coordinate_system.id.as_deref() == Some("$invalid") {
            return missing_field_error("coordinateSystem");
        } else {
            self.with_context(&self.value.r#coordinate_system, |ctx| {
                state.serialize_entry("coordinateSystem", ctx)
            })?;
        }
        if let Some(some) = self.value.r#patient.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("patient", ctx))?;
        }
        if let Some(some) = self.value.r#specimen.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("specimen", ctx))?;
        }
        if let Some(some) = self.value.r#device.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("device", ctx))?;
        }
        if let Some(some) = self.value.r#performer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("performer", ctx))?;
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#reference_seq.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("referenceSeq", ctx))?;
        }
        if !self.value.r#variant.is_empty() {
            self.with_context(&self.value.r#variant, |ctx| {
                state.serialize_entry("variant", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#observed_seq.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("observedSeq", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_observedSeq", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#observed_seq.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("observedSeq", ctx))?;
        }
        if !self.value.r#quality.is_empty() {
            self.with_context(&self.value.r#quality, |ctx| {
                state.serialize_entry("quality", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#read_coverage.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("readCoverage", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_readCoverage", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#read_coverage.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("readCoverage", ctx))?;
        }
        if !self.value.r#repository.is_empty() {
            self.with_context(&self.value.r#repository, |ctx| {
                state.serialize_entry("repository", ctx)
            })?;
        }
        if !self.value.r#pointer.is_empty() {
            self.with_context(&self.value.r#pointer, |ctx| {
                state.serialize_entry("pointer", ctx)
            })?;
        }
        if !self.value.r#structure_variant.is_empty() {
            self.with_context(&self.value.r#structure_variant, |ctx| {
                state.serialize_entry("structureVariant", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<MolecularSequence>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<MolecularSequence>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<MolecularSequence> {
    type Value = MolecularSequence;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<MolecularSequence> {
    type Value = MolecularSequence;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<MolecularSequence>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = MolecularSequence;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequence")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequence, V::Error>
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
                    #[serde(rename = "_type")]
                    TypePrimitiveElement,
                    #[serde(rename = "coordinateSystem")]
                    CoordinateSystem,
                    #[serde(rename = "_coordinateSystem")]
                    CoordinateSystemPrimitiveElement,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "specimen")]
                    Specimen,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "performer")]
                    Performer,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "referenceSeq")]
                    ReferenceSeq,
                    #[serde(rename = "variant")]
                    Variant,
                    #[serde(rename = "observedSeq")]
                    ObservedSeq,
                    #[serde(rename = "_observedSeq")]
                    ObservedSeqPrimitiveElement,
                    #[serde(rename = "quality")]
                    Quality,
                    #[serde(rename = "readCoverage")]
                    ReadCoverage,
                    #[serde(rename = "_readCoverage")]
                    ReadCoveragePrimitiveElement,
                    #[serde(rename = "repository")]
                    Repository,
                    #[serde(rename = "pointer")]
                    Pointer,
                    #[serde(rename = "structureVariant")]
                    StructureVariant,
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
                            "coordinateSystem",
                            "patient",
                            "specimen",
                            "device",
                            "performer",
                            "quantity",
                            "referenceSeq",
                            "variant",
                            "observedSeq",
                            "quality",
                            "readCoverage",
                            "repository",
                            "pointer",
                            "structureVariant",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r4::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#type: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#coordinate_system: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#specimen: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#device: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#performer: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#reference_seq: Option<
                    fhirbolt_model::r4::resources::MolecularSequenceReferenceSeq,
                > = None;
                let mut r#variant: Option<
                    Vec<fhirbolt_model::r4::resources::MolecularSequenceVariant>,
                > = None;
                let mut r#observed_seq: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#quality: Option<
                    Vec<fhirbolt_model::r4::resources::MolecularSequenceQuality>,
                > = None;
                let mut r#read_coverage: Option<fhirbolt_model::r4::types::Integer> = None;
                let mut r#repository: Option<
                    Vec<fhirbolt_model::r4::resources::MolecularSequenceRepository>,
                > = None;
                let mut r#pointer: Option<Vec<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#structure_variant: Option<
                    Vec<fhirbolt_model::r4::resources::MolecularSequenceStructureVariant>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MolecularSequence" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MolecularSequence",
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
                                    fhirbolt_model::r4::types::Id,
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
                                Box<fhirbolt_model::r4::types::Meta>,
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
                                    fhirbolt_model::r4::types::Uri,
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
                                    fhirbolt_model::r4::types::Code,
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
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
                                    Vec<fhirbolt_model::r4::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Type => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#type = Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("type");
                            }
                        }
                        Field::CoordinateSystem => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#coordinate_system.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "coordinateSystem",
                                    ));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#coordinate_system.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "coordinateSystem",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#coordinate_system =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CoordinateSystemPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#coordinate_system.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_coordinateSystem",
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
                                return unknown_field_error("coordinateSystem");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Specimen => {
                            if r#specimen.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimen"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#specimen = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#device = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#performer = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#quantity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ReferenceSeq => {
                            if r#reference_seq.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceSeq"));
                            }
                            let _context: &mut DeserializationContext<
                                fhirbolt_model::r4::resources::MolecularSequenceReferenceSeq,
                            > = self.0.transmute();
                            r#reference_seq = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Variant => {
                            if self.0.from == crate::context::Format::Json {
                                if r#variant.is_some() {
                                    return Err(serde::de::Error::duplicate_field("variant"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::MolecularSequenceVariant>,
                                > = self.0.transmute();
                                r#variant = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#variant.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::MolecularSequenceVariant,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ObservedSeq => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#observed_seq.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("observedSeq"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#observed_seq.is_some() {
                                    return Err(serde::de::Error::duplicate_field("observedSeq"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#observed_seq = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ObservedSeqPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#observed_seq.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_observedSeq"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("observedSeq");
                            }
                        }
                        Field::Quality => {
                            if self.0.from == crate::context::Format::Json {
                                if r#quality.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quality"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::MolecularSequenceQuality>,
                                > = self.0.transmute();
                                r#quality = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#quality.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::MolecularSequenceQuality,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReadCoverage => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#read_coverage.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("readCoverage"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#read_coverage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("readCoverage"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Integer,
                                > = self.0.transmute();
                                r#read_coverage = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ReadCoveragePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#read_coverage.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_readCoverage"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("readCoverage");
                            }
                        }
                        Field::Repository => {
                            if self.0.from == crate::context::Format::Json {
                                if r#repository.is_some() {
                                    return Err(serde::de::Error::duplicate_field("repository"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::resources::MolecularSequenceRepository>,
                                > = self.0.transmute();
                                r#repository = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#repository.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::MolecularSequenceRepository,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Pointer => {
                            if self.0.from == crate::context::Format::Json {
                                if r#pointer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("pointer"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Reference>,
                                > = self.0.transmute();
                                r#pointer = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#pointer.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Reference,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StructureVariant => {
                            if self.0.from == crate::context::Format::Json {
                                if r#structure_variant.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "structureVariant",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MolecularSequenceStructureVariant >> = self . 0 . transmute () ;
                                r#structure_variant =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#structure_variant.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: MolecularSequenceStructureVariant > = self . 0 . transmute () ;
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
                Ok(MolecularSequence {
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
                    r#coordinate_system: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#coordinate_system.unwrap_or(Default::default())
                    } else {
                        r#coordinate_system
                            .ok_or(serde::de::Error::missing_field("coordinateSystem"))?
                    },
                    r#patient,
                    r#specimen,
                    r#device,
                    r#performer,
                    r#quantity,
                    r#reference_seq,
                    r#variant: r#variant.unwrap_or(vec![]),
                    r#observed_seq,
                    r#quality: r#quality.unwrap_or(vec![]),
                    r#read_coverage,
                    r#repository: r#repository.unwrap_or(vec![]),
                    r#pointer: r#pointer.unwrap_or(vec![]),
                    r#structure_variant: r#structure_variant.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<MolecularSequence>> {
    type Value = Box<MolecularSequence>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<MolecularSequence>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<MolecularSequence>> {
    type Value = Vec<MolecularSequence>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<MolecularSequence>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<MolecularSequence>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<MolecularSequence> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
