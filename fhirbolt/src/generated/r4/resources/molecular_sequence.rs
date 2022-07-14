// Generated on 2022-07-14 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceReferenceSeq {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#chromosome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#genome_build: Option<super::super::types::String>,
    pub r#orientation: Option<super::super::types::Code>,
    pub r#reference_seq_id: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference_seq_pointer: Option<Box<super::super::types::Reference>>,
    pub r#reference_seq_string: Option<super::super::types::String>,
    pub r#strand: Option<super::super::types::Code>,
    pub r#window_start: Option<super::super::types::Integer>,
    pub r#window_end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceReferenceSeq {
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
        if let Some(some) = self.r#chromosome.as_ref() {
            state.serialize_entry("chromosome", some)?;
        }
        if let Some(some) = self.r#genome_build.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("genomeBuild", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_genomeBuild", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#orientation.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("orientation", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_orientation", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#reference_seq_id.as_ref() {
            state.serialize_entry("referenceSeqId", some)?;
        }
        if let Some(some) = self.r#reference_seq_pointer.as_ref() {
            state.serialize_entry("referenceSeqPointer", some)?;
        }
        if let Some(some) = self.r#reference_seq_string.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("referenceSeqString", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_referenceSeqString", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#strand.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("strand", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_strand", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#window_start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("windowStart", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_windowStart", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#window_end.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("windowEnd", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_windowEnd", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceReferenceSeq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#chromosome: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#genome_build: Option<super::super::types::String> = None;
                let mut r#orientation: Option<super::super::types::Code> = None;
                let mut r#reference_seq_id: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#reference_seq_pointer: Option<Box<super::super::types::Reference>> = None;
                let mut r#reference_seq_string: Option<super::super::types::String> = None;
                let mut r#strand: Option<super::super::types::Code> = None;
                let mut r#window_start: Option<super::super::types::Integer> = None;
                let mut r#window_end: Option<super::super::types::Integer> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "chromosome" => {
                            if r#chromosome.is_some() {
                                return Err(serde::de::Error::duplicate_field("chromosome"));
                            }
                            r#chromosome = Some(map_access.next_value()?);
                        }
                        "genomeBuild" => {
                            let some = r#genome_build.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("genomeBuild"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_genomeBuild" => {
                            let some = r#genome_build.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_genomeBuild"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "orientation" => {
                            let some = r#orientation.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("orientation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_orientation" => {
                            let some = r#orientation.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_orientation"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "referenceSeqId" => {
                            if r#reference_seq_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceSeqId"));
                            }
                            r#reference_seq_id = Some(map_access.next_value()?);
                        }
                        "referenceSeqPointer" => {
                            if r#reference_seq_pointer.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "referenceSeqPointer",
                                ));
                            }
                            r#reference_seq_pointer = Some(map_access.next_value()?);
                        }
                        "referenceSeqString" => {
                            let some = r#reference_seq_string.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "referenceSeqString",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_referenceSeqString" => {
                            let some = r#reference_seq_string.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_referenceSeqString",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "strand" => {
                            let some = r#strand.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("strand"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_strand" => {
                            let some = r#strand.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_strand"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "windowStart" => {
                            let some = r#window_start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowStart"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_windowStart" => {
                            let some = r#window_start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_windowStart"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "windowEnd" => {
                            let some = r#window_end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("windowEnd"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_windowEnd" => {
                            let some = r#window_end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_windowEnd"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceVariant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#observed_allele: Option<super::super::types::String>,
    pub r#reference_allele: Option<super::super::types::String>,
    pub r#cigar: Option<super::super::types::String>,
    pub r#variant_pointer: Option<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for MolecularSequenceVariant {
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
        if let Some(some) = self.r#start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("start", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_start", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#end.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("end", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_end", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#observed_allele.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("observedAllele", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_observedAllele", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#reference_allele.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("referenceAllele", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_referenceAllele", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#cigar.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("cigar", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cigar", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#variant_pointer.as_ref() {
            state.serialize_entry("variantPointer", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MolecularSequenceVariant;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceVariant")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequenceVariant, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#start: Option<super::super::types::Integer> = None;
                let mut r#end: Option<super::super::types::Integer> = None;
                let mut r#observed_allele: Option<super::super::types::String> = None;
                let mut r#reference_allele: Option<super::super::types::String> = None;
                let mut r#cigar: Option<super::super::types::String> = None;
                let mut r#variant_pointer: Option<Box<super::super::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_end"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "observedAllele" => {
                            let some = r#observed_allele.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("observedAllele"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_observedAllele" => {
                            let some = r#observed_allele.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_observedAllele"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "referenceAllele" => {
                            let some = r#reference_allele.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceAllele"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_referenceAllele" => {
                            let some = r#reference_allele.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_referenceAllele"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "cigar" => {
                            let some = r#cigar.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("cigar"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_cigar" => {
                            let some = r#cigar.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_cigar"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "variantPointer" => {
                            if r#variant_pointer.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantPointer"));
                            }
                            r#variant_pointer = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceQualityRoc {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#score: Vec<super::super::types::Integer>,
    pub r#num_tp: Vec<super::super::types::Integer>,
    pub r#num_fp: Vec<super::super::types::Integer>,
    pub r#num_fn: Vec<super::super::types::Integer>,
    pub r#precision: Vec<super::super::types::Decimal>,
    pub r#sensitivity: Vec<super::super::types::Decimal>,
    pub r#f_measure: Vec<super::super::types::Decimal>,
}
impl serde::ser::Serialize for MolecularSequenceQualityRoc {
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
        if !self.r#score.is_empty() {
            let values: Vec<_> = self.r#score.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("score", &values)?;
            }
            let requires_elements = self
                .r#score
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#score
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_score", &primitive_elements)?;
            }
        }
        if !self.r#num_tp.is_empty() {
            let values: Vec<_> = self.r#num_tp.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("numTP", &values)?;
            }
            let requires_elements = self
                .r#num_tp
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#num_tp
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_numTP", &primitive_elements)?;
            }
        }
        if !self.r#num_fp.is_empty() {
            let values: Vec<_> = self.r#num_fp.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("numFP", &values)?;
            }
            let requires_elements = self
                .r#num_fp
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#num_fp
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_numFP", &primitive_elements)?;
            }
        }
        if !self.r#num_fn.is_empty() {
            let values: Vec<_> = self.r#num_fn.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("numFN", &values)?;
            }
            let requires_elements = self
                .r#num_fn
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#num_fn
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_numFN", &primitive_elements)?;
            }
        }
        if !self.r#precision.is_empty() {
            let values: Vec<_> = self.r#precision.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("precision", &values)?;
            }
            let requires_elements = self
                .r#precision
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#precision
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_precision", &primitive_elements)?;
            }
        }
        if !self.r#sensitivity.is_empty() {
            let values: Vec<_> = self.r#sensitivity.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("sensitivity", &values)?;
            }
            let requires_elements = self
                .r#sensitivity
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#sensitivity
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_sensitivity", &primitive_elements)?;
            }
        }
        if !self.r#f_measure.is_empty() {
            let values: Vec<_> = self.r#f_measure.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("fMeasure", &values)?;
            }
            let requires_elements = self
                .r#f_measure
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#f_measure
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_fMeasure", &primitive_elements)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceQualityRoc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#score: Option<Vec<super::super::types::Integer>> = None;
                let mut r#num_tp: Option<Vec<super::super::types::Integer>> = None;
                let mut r#num_fp: Option<Vec<super::super::types::Integer>> = None;
                let mut r#num_fn: Option<Vec<super::super::types::Integer>> = None;
                let mut r#precision: Option<Vec<super::super::types::Decimal>> = None;
                let mut r#sensitivity: Option<Vec<super::super::types::Decimal>> = None;
                let mut r#f_measure: Option<Vec<super::super::types::Decimal>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "score" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#score.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_score" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#score.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "numTP" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#num_tp.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_numTP" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#num_tp.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "numFP" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#num_fp.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_numFP" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#num_fp.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "numFN" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#num_fn.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_numFN" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#num_fn.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "precision" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#precision.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_precision" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#precision.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "sensitivity" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#sensitivity.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_sensitivity" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec =
                                r#sensitivity.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "fMeasure" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#f_measure.get_or_insert(Vec::with_capacity(values.len()));
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
                                vec[i].value = value;
                            }
                        }
                        "_fMeasure" => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
                            let vec = r#f_measure.get_or_insert(Vec::with_capacity(elements.len()));
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceQuality {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#standard_sequence: Option<Box<super::super::types::CodeableConcept>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
    pub r#score: Option<Box<super::super::types::Quantity>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#truth_tp: Option<super::super::types::Decimal>,
    pub r#query_tp: Option<super::super::types::Decimal>,
    pub r#truth_fn: Option<super::super::types::Decimal>,
    pub r#query_fp: Option<super::super::types::Decimal>,
    pub r#gt_fp: Option<super::super::types::Decimal>,
    pub r#precision: Option<super::super::types::Decimal>,
    pub r#recall: Option<super::super::types::Decimal>,
    pub r#f_score: Option<super::super::types::Decimal>,
    pub r#roc: Option<MolecularSequenceQualityRoc>,
}
impl serde::ser::Serialize for MolecularSequenceQuality {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#standard_sequence.as_ref() {
            state.serialize_entry("standardSequence", some)?;
        }
        if let Some(some) = self.r#start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("start", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_start", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#end.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("end", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_end", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#score.as_ref() {
            state.serialize_entry("score", some)?;
        }
        if let Some(some) = self.r#method.as_ref() {
            state.serialize_entry("method", some)?;
        }
        if let Some(some) = self.r#truth_tp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("truthTP", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_truthTP", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#query_tp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("queryTP", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_queryTP", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#truth_fn.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("truthFN", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_truthFN", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#query_fp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("queryFP", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_queryFP", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#gt_fp.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("gtFP", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_gtFP", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#precision.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("precision", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_precision", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#recall.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("recall", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_recall", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#f_score.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("fScore", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_fScore", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#roc.as_ref() {
            state.serialize_entry("roc", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceQuality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MolecularSequenceQuality;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequenceQuality")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequenceQuality, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#standard_sequence: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#start: Option<super::super::types::Integer> = None;
                let mut r#end: Option<super::super::types::Integer> = None;
                let mut r#score: Option<Box<super::super::types::Quantity>> = None;
                let mut r#method: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#truth_tp: Option<super::super::types::Decimal> = None;
                let mut r#query_tp: Option<super::super::types::Decimal> = None;
                let mut r#truth_fn: Option<super::super::types::Decimal> = None;
                let mut r#query_fp: Option<super::super::types::Decimal> = None;
                let mut r#gt_fp: Option<super::super::types::Decimal> = None;
                let mut r#precision: Option<super::super::types::Decimal> = None;
                let mut r#recall: Option<super::super::types::Decimal> = None;
                let mut r#f_score: Option<super::super::types::Decimal> = None;
                let mut r#roc: Option<MolecularSequenceQualityRoc> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
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
                        "standardSequence" => {
                            if r#standard_sequence.is_some() {
                                return Err(serde::de::Error::duplicate_field("standardSequence"));
                            }
                            r#standard_sequence = Some(map_access.next_value()?);
                        }
                        "start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_end"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "score" => {
                            if r#score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            r#score = Some(map_access.next_value()?);
                        }
                        "method" => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some(map_access.next_value()?);
                        }
                        "truthTP" => {
                            let some = r#truth_tp.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("truthTP"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_truthTP" => {
                            let some = r#truth_tp.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_truthTP"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "queryTP" => {
                            let some = r#query_tp.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryTP"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_queryTP" => {
                            let some = r#query_tp.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_queryTP"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "truthFN" => {
                            let some = r#truth_fn.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("truthFN"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_truthFN" => {
                            let some = r#truth_fn.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_truthFN"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "queryFP" => {
                            let some = r#query_fp.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryFP"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_queryFP" => {
                            let some = r#query_fp.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_queryFP"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "gtFP" => {
                            let some = r#gt_fp.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("gtFP"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_gtFP" => {
                            let some = r#gt_fp.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_gtFP"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "precision" => {
                            let some = r#precision.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("precision"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_precision" => {
                            let some = r#precision.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_precision"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "recall" => {
                            let some = r#recall.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("recall"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_recall" => {
                            let some = r#recall.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_recall"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "fScore" => {
                            let some = r#f_score.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("fScore"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_fScore" => {
                            let some = r#f_score.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_fScore"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "roc" => {
                            if r#roc.is_some() {
                                return Err(serde::de::Error::duplicate_field("roc"));
                            }
                            r#roc = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                    }
                }
                Ok(MolecularSequenceQuality {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceRepository {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#url: Option<super::super::types::Uri>,
    pub r#name: Option<super::super::types::String>,
    pub r#dataset_id: Option<super::super::types::String>,
    pub r#variantset_id: Option<super::super::types::String>,
    pub r#readset_id: Option<super::super::types::String>,
}
impl serde::ser::Serialize for MolecularSequenceRepository {
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
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_url", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#dataset_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("datasetId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_datasetId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#variantset_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("variantsetId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_variantsetId", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#readset_id.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("readsetId", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_readsetId", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceRepository {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#dataset_id: Option<super::super::types::String> = None;
                let mut r#variantset_id: Option<super::super::types::String> = None;
                let mut r#readset_id: Option<super::super::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
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
                        "url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_url" => {
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
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "datasetId" => {
                            let some = r#dataset_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("datasetId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_datasetId" => {
                            let some = r#dataset_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_datasetId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "variantsetId" => {
                            let some = r#variantset_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantsetId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_variantsetId" => {
                            let some = r#variantset_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_variantsetId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "readsetId" => {
                            let some = r#readset_id.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("readsetId"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_readsetId" => {
                            let some = r#readset_id.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_readsetId"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                    }
                }
                Ok(MolecularSequenceRepository {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#url,
                    r#name,
                    r#dataset_id,
                    r#variantset_id,
                    r#readset_id,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceStructureVariantOuter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariantOuter {
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
        if let Some(some) = self.r#start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("start", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_start", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#end.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("end", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_end", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceStructureVariantOuter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#start: Option<super::super::types::Integer> = None;
                let mut r#end: Option<super::super::types::Integer> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_end"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifierExtension", "start", "end"],
                            ))
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceStructureVariantInner {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Integer>,
    pub r#end: Option<super::super::types::Integer>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariantInner {
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
        if let Some(some) = self.r#start.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("start", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_start", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#end.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("end", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_end", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceStructureVariantInner {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#start: Option<super::super::types::Integer> = None;
                let mut r#end: Option<super::super::types::Integer> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("start"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_start" => {
                            let some = r#start.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_start"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("end"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_end" => {
                            let some = r#end.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_end"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifierExtension", "start", "end"],
                            ))
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequenceStructureVariant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#variant_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#exact: Option<super::super::types::Boolean>,
    pub r#length: Option<super::super::types::Integer>,
    pub r#outer: Option<MolecularSequenceStructureVariantOuter>,
    pub r#inner: Option<MolecularSequenceStructureVariantInner>,
}
impl serde::ser::Serialize for MolecularSequenceStructureVariant {
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
        if let Some(some) = self.r#variant_type.as_ref() {
            state.serialize_entry("variantType", some)?;
        }
        if let Some(some) = self.r#exact.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("exact", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_exact", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#length.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("length", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_length", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#outer.as_ref() {
            state.serialize_entry("outer", some)?;
        }
        if let Some(some) = self.r#inner.as_ref() {
            state.serialize_entry("inner", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequenceStructureVariant {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#variant_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#exact: Option<super::super::types::Boolean> = None;
                let mut r#length: Option<super::super::types::Integer> = None;
                let mut r#outer: Option<MolecularSequenceStructureVariantOuter> = None;
                let mut r#inner: Option<MolecularSequenceStructureVariantInner> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "variantType" => {
                            if r#variant_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("variantType"));
                            }
                            r#variant_type = Some(map_access.next_value()?);
                        }
                        "exact" => {
                            let some = r#exact.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("exact"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_exact" => {
                            let some = r#exact.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_exact"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "length" => {
                            let some = r#length.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_length" => {
                            let some = r#length.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_length"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "outer" => {
                            if r#outer.is_some() {
                                return Err(serde::de::Error::duplicate_field("outer"));
                            }
                            r#outer = Some(map_access.next_value()?);
                        }
                        "inner" => {
                            if r#inner.is_some() {
                                return Err(serde::de::Error::duplicate_field("inner"));
                            }
                            r#inner = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MolecularSequence {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#coordinate_system: super::super::types::Integer,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#reference_seq: Option<MolecularSequenceReferenceSeq>,
    pub r#variant: Vec<MolecularSequenceVariant>,
    pub r#observed_seq: Option<super::super::types::String>,
    pub r#quality: Vec<MolecularSequenceQuality>,
    pub r#read_coverage: Option<super::super::types::Integer>,
    pub r#repository: Vec<MolecularSequenceRepository>,
    pub r#pointer: Vec<Box<super::super::types::Reference>>,
    pub r#structure_variant: Vec<MolecularSequenceStructureVariant>,
}
impl serde::ser::Serialize for MolecularSequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MolecularSequence")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
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
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#type.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_type", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#coordinate_system.value.as_ref() {
            state.serialize_entry("coordinateSystem", some)?;
        }
        if self.r#coordinate_system.id.is_some() || !self.r#coordinate_system.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#coordinate_system.id,
                extension: &self.r#coordinate_system.extension,
            };
            state.serialize_entry("_coordinateSystem", &primitive_element)?;
        }
        if let Some(some) = self.r#patient.as_ref() {
            state.serialize_entry("patient", some)?;
        }
        if let Some(some) = self.r#specimen.as_ref() {
            state.serialize_entry("specimen", some)?;
        }
        if let Some(some) = self.r#device.as_ref() {
            state.serialize_entry("device", some)?;
        }
        if let Some(some) = self.r#performer.as_ref() {
            state.serialize_entry("performer", some)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#reference_seq.as_ref() {
            state.serialize_entry("referenceSeq", some)?;
        }
        if !self.r#variant.is_empty() {
            state.serialize_entry("variant", &self.r#variant)?;
        }
        if let Some(some) = self.r#observed_seq.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("observedSeq", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_observedSeq", &primitive_element)?;
            }
        }
        if !self.r#quality.is_empty() {
            state.serialize_entry("quality", &self.r#quality)?;
        }
        if let Some(some) = self.r#read_coverage.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("readCoverage", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_readCoverage", &primitive_element)?;
            }
        }
        if !self.r#repository.is_empty() {
            state.serialize_entry("repository", &self.r#repository)?;
        }
        if !self.r#pointer.is_empty() {
            state.serialize_entry("pointer", &self.r#pointer)?;
        }
        if !self.r#structure_variant.is_empty() {
            state.serialize_entry("structureVariant", &self.r#structure_variant)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MolecularSequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MolecularSequence;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MolecularSequence")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MolecularSequence, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#type: Option<super::super::types::Code> = None;
                let mut r#coordinate_system: Option<super::super::types::Integer> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#specimen: Option<Box<super::super::types::Reference>> = None;
                let mut r#device: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Box<super::super::types::Reference>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#reference_seq: Option<MolecularSequenceReferenceSeq> = None;
                let mut r#variant: Option<Vec<MolecularSequenceVariant>> = None;
                let mut r#observed_seq: Option<super::super::types::String> = None;
                let mut r#quality: Option<Vec<MolecularSequenceQuality>> = None;
                let mut r#read_coverage: Option<super::super::types::Integer> = None;
                let mut r#repository: Option<Vec<MolecularSequenceRepository>> = None;
                let mut r#pointer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#structure_variant: Option<Vec<MolecularSequenceStructureVariant>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
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
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
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
                        "coordinateSystem" => {
                            let some = r#coordinate_system.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("coordinateSystem"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_coordinateSystem" => {
                            let some = r#coordinate_system.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_coordinateSystem"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "specimen" => {
                            if r#specimen.is_some() {
                                return Err(serde::de::Error::duplicate_field("specimen"));
                            }
                            r#specimen = Some(map_access.next_value()?);
                        }
                        "device" => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(map_access.next_value()?);
                        }
                        "performer" => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "referenceSeq" => {
                            if r#reference_seq.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceSeq"));
                            }
                            r#reference_seq = Some(map_access.next_value()?);
                        }
                        "variant" => {
                            if r#variant.is_some() {
                                return Err(serde::de::Error::duplicate_field("variant"));
                            }
                            r#variant = Some(map_access.next_value()?);
                        }
                        "observedSeq" => {
                            let some = r#observed_seq.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("observedSeq"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_observedSeq" => {
                            let some = r#observed_seq.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_observedSeq"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "quality" => {
                            if r#quality.is_some() {
                                return Err(serde::de::Error::duplicate_field("quality"));
                            }
                            r#quality = Some(map_access.next_value()?);
                        }
                        "readCoverage" => {
                            let some = r#read_coverage.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("readCoverage"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_readCoverage" => {
                            let some = r#read_coverage.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_readCoverage"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "repository" => {
                            if r#repository.is_some() {
                                return Err(serde::de::Error::duplicate_field("repository"));
                            }
                            r#repository = Some(map_access.next_value()?);
                        }
                        "pointer" => {
                            if r#pointer.is_some() {
                                return Err(serde::de::Error::duplicate_field("pointer"));
                            }
                            r#pointer = Some(map_access.next_value()?);
                        }
                        "structureVariant" => {
                            if r#structure_variant.is_some() {
                                return Err(serde::de::Error::duplicate_field("structureVariant"));
                            }
                            r#structure_variant = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
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
                    r#coordinate_system: r#coordinate_system
                        .ok_or(serde::de::Error::missing_field("coordinate_system"))?,
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
        deserializer.deserialize_map(Visitor)
    }
}
