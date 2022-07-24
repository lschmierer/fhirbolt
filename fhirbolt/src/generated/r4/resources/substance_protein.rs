// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct SubstanceProteinSubunit {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subunit: Option<super::super::types::Integer>,
    pub r#sequence: Option<super::super::types::String>,
    pub r#length: Option<super::super::types::Integer>,
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    pub r#n_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    pub r#n_terminal_modification: Option<super::super::types::String>,
    pub r#c_terminal_modification_id: Option<Box<super::super::types::Identifier>>,
    pub r#c_terminal_modification: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SubstanceProteinSubunit {
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
        if let Some(some) = self.r#subunit.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("subunit", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_subunit", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#sequence.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sequence", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sequence", &primitive_element)?;
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
        if let Some(some) = self.r#sequence_attachment.as_ref() {
            state.serialize_entry("sequenceAttachment", some)?;
        }
        if let Some(some) = self.r#n_terminal_modification_id.as_ref() {
            state.serialize_entry("nTerminalModificationId", some)?;
        }
        if let Some(some) = self.r#n_terminal_modification.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("nTerminalModification", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_nTerminalModification", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#c_terminal_modification_id.as_ref() {
            state.serialize_entry("cTerminalModificationId", some)?;
        }
        if let Some(some) = self.r#c_terminal_modification.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("cTerminalModification", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cTerminalModification", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceProteinSubunit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
            #[serde(rename = "subunit")]
            Subunit,
            #[serde(rename = "_subunit")]
            SubunitPrimitiveElement,
            #[serde(rename = "sequence")]
            Sequence,
            #[serde(rename = "_sequence")]
            SequencePrimitiveElement,
            #[serde(rename = "length")]
            Length,
            #[serde(rename = "_length")]
            LengthPrimitiveElement,
            #[serde(rename = "sequenceAttachment")]
            SequenceAttachment,
            #[serde(rename = "nTerminalModificationId")]
            NTerminalModificationId,
            #[serde(rename = "nTerminalModification")]
            NTerminalModification,
            #[serde(rename = "_nTerminalModification")]
            NTerminalModificationPrimitiveElement,
            #[serde(rename = "cTerminalModificationId")]
            CTerminalModificationId,
            #[serde(rename = "cTerminalModification")]
            CTerminalModification,
            #[serde(rename = "_cTerminalModification")]
            CTerminalModificationPrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceProteinSubunit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceProteinSubunit")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceProteinSubunit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#subunit: Option<super::super::types::Integer> = None;
                let mut r#sequence: Option<super::super::types::String> = None;
                let mut r#length: Option<super::super::types::Integer> = None;
                let mut r#sequence_attachment: Option<Box<super::super::types::Attachment>> = None;
                let mut r#n_terminal_modification_id: Option<Box<super::super::types::Identifier>> =
                    None;
                let mut r#n_terminal_modification: Option<super::super::types::String> = None;
                let mut r#c_terminal_modification_id: Option<Box<super::super::types::Identifier>> =
                    None;
                let mut r#c_terminal_modification: Option<super::super::types::String> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Subunit => {
                            let some = r#subunit.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("subunit"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SubunitPrimitiveElement => {
                            let some = r#subunit.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_subunit"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Sequence => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequence"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::SequencePrimitiveElement => {
                            let some = r#sequence.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sequence"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Length => {
                            let some = r#length.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("length"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LengthPrimitiveElement => {
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
                        Field::SequenceAttachment => {
                            if r#sequence_attachment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sequenceAttachment",
                                ));
                            }
                            r#sequence_attachment = Some(map_access.next_value()?);
                        }
                        Field::NTerminalModificationId => {
                            if r#n_terminal_modification_id.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "nTerminalModificationId",
                                ));
                            }
                            r#n_terminal_modification_id = Some(map_access.next_value()?);
                        }
                        Field::NTerminalModification => {
                            let some = r#n_terminal_modification.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "nTerminalModification",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NTerminalModificationPrimitiveElement => {
                            let some = r#n_terminal_modification.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_nTerminalModification",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::CTerminalModificationId => {
                            if r#c_terminal_modification_id.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cTerminalModificationId",
                                ));
                            }
                            r#c_terminal_modification_id = Some(map_access.next_value()?);
                        }
                        Field::CTerminalModification => {
                            let some = r#c_terminal_modification.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "cTerminalModification",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::CTerminalModificationPrimitiveElement => {
                            let some = r#c_terminal_modification.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_cTerminalModification",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(SubstanceProteinSubunit {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#subunit,
                    r#sequence,
                    r#length,
                    r#sequence_attachment,
                    r#n_terminal_modification_id,
                    r#n_terminal_modification,
                    r#c_terminal_modification_id,
                    r#c_terminal_modification,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct SubstanceProtein {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    pub r#disulfide_linkage: Vec<super::super::types::String>,
    pub r#subunit: Vec<SubstanceProteinSubunit>,
}
impl serde::ser::Serialize for SubstanceProtein {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SubstanceProtein")?;
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
        if let Some(some) = self.r#sequence_type.as_ref() {
            state.serialize_entry("sequenceType", some)?;
        }
        if let Some(some) = self.r#number_of_subunits.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("numberOfSubunits", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_numberOfSubunits", &primitive_element)?;
            }
        }
        if !self.r#disulfide_linkage.is_empty() {
            let values: Vec<_> = self.r#disulfide_linkage.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("disulfideLinkage", &values)?;
            }
            let requires_elements = self
                .r#disulfide_linkage
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#disulfide_linkage
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
                state.serialize_entry("_disulfideLinkage", &primitive_elements)?;
            }
        }
        if !self.r#subunit.is_empty() {
            state.serialize_entry("subunit", &self.r#subunit)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for SubstanceProtein {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
            #[serde(rename = "sequenceType")]
            SequenceType,
            #[serde(rename = "numberOfSubunits")]
            NumberOfSubunits,
            #[serde(rename = "_numberOfSubunits")]
            NumberOfSubunitsPrimitiveElement,
            #[serde(rename = "disulfideLinkage")]
            DisulfideLinkage,
            #[serde(rename = "_disulfideLinkage")]
            DisulfideLinkagePrimitiveElement,
            #[serde(rename = "subunit")]
            Subunit,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SubstanceProtein;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceProtein")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceProtein, V::Error>
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
                let mut r#sequence_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#number_of_subunits: Option<super::super::types::Integer> = None;
                let mut r#disulfide_linkage: Option<Vec<super::super::types::String>> = None;
                let mut r#subunit: Option<Vec<SubstanceProteinSubunit>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "SubstanceProtein" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SubstanceProtein",
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
                            r#meta = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRules => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
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
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
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
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        Field::Contained => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        Field::ModifierExtension => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::SequenceType => {
                            if r#sequence_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceType"));
                            }
                            r#sequence_type = Some(map_access.next_value()?);
                        }
                        Field::NumberOfSubunits => {
                            let some = r#number_of_subunits.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfSubunits"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NumberOfSubunitsPrimitiveElement => {
                            let some = r#number_of_subunits.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_numberOfSubunits"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::DisulfideLinkage => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#disulfide_linkage.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("disulfideLinkage"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::DisulfideLinkagePrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#disulfide_linkage.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_disulfideLinkage"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::Subunit => {
                            if r#subunit.is_some() {
                                return Err(serde::de::Error::duplicate_field("subunit"));
                            }
                            r#subunit = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(SubstanceProtein {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence_type,
                    r#number_of_subunits,
                    r#disulfide_linkage: r#disulfide_linkage.unwrap_or(vec![]),
                    r#subunit: r#subunit.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
