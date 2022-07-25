// Generated on 2022-07-25 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    pub r#strength: Box<super::super::types::Ratio>,
    pub r#strength_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#measurement_point: Option<super::super::types::String>,
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize
    for MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength
{
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
        if let Some(some) = self.r#substance.as_ref() {
            state.serialize_entry("substance", some)?;
        }
        state.serialize_entry("strength", &self.r#strength)?;
        if let Some(some) = self.r#strength_low_limit.as_ref() {
            state.serialize_entry("strengthLowLimit", some)?;
        }
        if let Some(some) = self.r#measurement_point.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("measurementPoint", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_measurementPoint", &primitive_element)?;
            }
        }
        if !self.r#country.is_empty() {
            state.serialize_entry("country", &self.r#country)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de>
    for MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength
{
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
            #[serde(rename = "substance")]
            Substance,
            #[serde(rename = "strength")]
            Strength,
            #[serde(rename = "strengthLowLimit")]
            StrengthLowLimit,
            #[serde(rename = "measurementPoint")]
            MeasurementPoint,
            #[serde(rename = "_measurementPoint")]
            MeasurementPointPrimitiveElement,
            #[serde(rename = "country")]
            Country,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength",
                )
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#strength: Option<Box<super::super::types::Ratio>> = None;
                let mut r#strength_low_limit: Option<Box<super::super::types::Ratio>> = None;
                let mut r#measurement_point: Option<super::super::types::String> = None;
                let mut r#country: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
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
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some(map_access.next_value()?);
                        }
                        Field::Strength => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            r#strength = Some(map_access.next_value()?);
                        }
                        Field::StrengthLowLimit => {
                            if r#strength_low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field("strengthLowLimit"));
                            }
                            r#strength_low_limit = Some(map_access.next_value()?);
                        }
                        Field::MeasurementPoint => {
                            let some = r#measurement_point.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementPoint"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::MeasurementPointPrimitiveElement => {
                            let some = r#measurement_point.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_measurementPoint"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Country => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            r#country = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(
                    MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#substance,
                        r#strength: r#strength
                            .ok_or(serde::de::Error::missing_field("strength"))?,
                        r#strength_low_limit,
                        r#measurement_point,
                        r#country: r#country.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrength {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#presentation: Box<super::super::types::Ratio>,
    pub r#presentation_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#concentration: Option<Box<super::super::types::Ratio>>,
    pub r#concentration_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#measurement_point: Option<super::super::types::String>,
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reference_strength:
        Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
}
impl serde::ser::Serialize for MedicinalProductIngredientSpecifiedSubstanceStrength {
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
        state.serialize_entry("presentation", &self.r#presentation)?;
        if let Some(some) = self.r#presentation_low_limit.as_ref() {
            state.serialize_entry("presentationLowLimit", some)?;
        }
        if let Some(some) = self.r#concentration.as_ref() {
            state.serialize_entry("concentration", some)?;
        }
        if let Some(some) = self.r#concentration_low_limit.as_ref() {
            state.serialize_entry("concentrationLowLimit", some)?;
        }
        if let Some(some) = self.r#measurement_point.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("measurementPoint", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_measurementPoint", &primitive_element)?;
            }
        }
        if !self.r#country.is_empty() {
            state.serialize_entry("country", &self.r#country)?;
        }
        if !self.r#reference_strength.is_empty() {
            state.serialize_entry("referenceStrength", &self.r#reference_strength)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIngredientSpecifiedSubstanceStrength {
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
            #[serde(rename = "presentation")]
            Presentation,
            #[serde(rename = "presentationLowLimit")]
            PresentationLowLimit,
            #[serde(rename = "concentration")]
            Concentration,
            #[serde(rename = "concentrationLowLimit")]
            ConcentrationLowLimit,
            #[serde(rename = "measurementPoint")]
            MeasurementPoint,
            #[serde(rename = "_measurementPoint")]
            MeasurementPointPrimitiveElement,
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "referenceStrength")]
            ReferenceStrength,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIngredientSpecifiedSubstanceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSpecifiedSubstanceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductIngredientSpecifiedSubstanceStrength, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#presentation: Option<Box<super::super::types::Ratio>> = None;
                let mut r#presentation_low_limit: Option<Box<super::super::types::Ratio>> = None;
                let mut r#concentration: Option<Box<super::super::types::Ratio>> = None;
                let mut r#concentration_low_limit: Option<Box<super::super::types::Ratio>> = None;
                let mut r#measurement_point: Option<super::super::types::String> = None;
                let mut r#country: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#reference_strength: Option<
                    Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
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
                        Field::Presentation => {
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentation"));
                            }
                            r#presentation = Some(map_access.next_value()?);
                        }
                        Field::PresentationLowLimit => {
                            if r#presentation_low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "presentationLowLimit",
                                ));
                            }
                            r#presentation_low_limit = Some(map_access.next_value()?);
                        }
                        Field::Concentration => {
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field("concentration"));
                            }
                            r#concentration = Some(map_access.next_value()?);
                        }
                        Field::ConcentrationLowLimit => {
                            if r#concentration_low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationLowLimit",
                                ));
                            }
                            r#concentration_low_limit = Some(map_access.next_value()?);
                        }
                        Field::MeasurementPoint => {
                            let some = r#measurement_point.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("measurementPoint"));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::MeasurementPointPrimitiveElement => {
                            let some = r#measurement_point.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_measurementPoint"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Country => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            r#country = Some(map_access.next_value()?);
                        }
                        Field::ReferenceStrength => {
                            if r#reference_strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceStrength"));
                            }
                            r#reference_strength = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductIngredientSpecifiedSubstanceStrength {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#presentation: r#presentation
                        .ok_or(serde::de::Error::missing_field("presentation"))?,
                    r#presentation_low_limit,
                    r#concentration,
                    r#concentration_low_limit,
                    r#measurement_point,
                    r#country: r#country.unwrap_or(vec![]),
                    r#reference_strength: r#reference_strength.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#group: Box<super::super::types::CodeableConcept>,
    pub r#confidentiality: Option<Box<super::super::types::CodeableConcept>>,
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}
impl serde::ser::Serialize for MedicinalProductIngredientSpecifiedSubstance {
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
        state.serialize_entry("code", &self.r#code)?;
        state.serialize_entry("group", &self.r#group)?;
        if let Some(some) = self.r#confidentiality.as_ref() {
            state.serialize_entry("confidentiality", some)?;
        }
        if !self.r#strength.is_empty() {
            state.serialize_entry("strength", &self.r#strength)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIngredientSpecifiedSubstance {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "group")]
            Group,
            #[serde(rename = "confidentiality")]
            Confidentiality,
            #[serde(rename = "strength")]
            Strength,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIngredientSpecifiedSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSpecifiedSubstance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductIngredientSpecifiedSubstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#group: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#confidentiality: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#strength: Option<
                    Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Group => {
                            if r#group.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            r#group = Some(map_access.next_value()?);
                        }
                        Field::Confidentiality => {
                            if r#confidentiality.is_some() {
                                return Err(serde::de::Error::duplicate_field("confidentiality"));
                            }
                            r#confidentiality = Some(map_access.next_value()?);
                        }
                        Field::Strength => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            r#strength = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductIngredientSpecifiedSubstance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#group: r#group.ok_or(serde::de::Error::missing_field("group"))?,
                    r#confidentiality,
                    r#strength: r#strength.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIngredientSubstance {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
}
impl serde::ser::Serialize for MedicinalProductIngredientSubstance {
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
        state.serialize_entry("code", &self.r#code)?;
        if !self.r#strength.is_empty() {
            state.serialize_entry("strength", &self.r#strength)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIngredientSubstance {
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "strength")]
            Strength,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIngredientSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSubstance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductIngredientSubstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#strength: Option<
                    Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some(map_access.next_value()?);
                        }
                        Field::Strength => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            r#strength = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductIngredientSubstance {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: r#code.ok_or(serde::de::Error::missing_field("code"))?,
                    r#strength: r#strength.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductIngredient {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#role: Box<super::super::types::CodeableConcept>,
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#specified_substance: Vec<MedicinalProductIngredientSpecifiedSubstance>,
    pub r#substance: Option<MedicinalProductIngredientSubstance>,
}
impl serde::ser::Serialize for MedicinalProductIngredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductIngredient")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
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
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
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
        if let Some(some) = self.r#identifier.as_ref() {
            state.serialize_entry("identifier", some)?;
        }
        state.serialize_entry("role", &self.r#role)?;
        if let Some(some) = self.r#allergenic_indicator.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("allergenicIndicator", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_allergenicIndicator", &primitive_element)?;
            }
        }
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if !self.r#specified_substance.is_empty() {
            state.serialize_entry("specifiedSubstance", &self.r#specified_substance)?;
        }
        if let Some(some) = self.r#substance.as_ref() {
            state.serialize_entry("substance", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductIngredient {
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
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "allergenicIndicator")]
            AllergenicIndicator,
            #[serde(rename = "_allergenicIndicator")]
            AllergenicIndicatorPrimitiveElement,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "specifiedSubstance")]
            SpecifiedSubstance,
            #[serde(rename = "substance")]
            Substance,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<MedicinalProductIngredient, V::Error>
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
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#allergenic_indicator: Option<super::super::types::Boolean> = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#specified_substance: Option<
                    Vec<MedicinalProductIngredientSpecifiedSubstance>,
                > = None;
                let mut r#substance: Option<MedicinalProductIngredientSubstance> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductIngredient" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductIngredient",
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
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
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
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
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
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some(map_access.next_value()?);
                        }
                        Field::AllergenicIndicator => {
                            let some = r#allergenic_indicator.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allergenicIndicator",
                                ));
                            }
                            let value: _ = map_access.next_value()?;
                            some.value = Some(value);
                        }
                        Field::AllergenicIndicatorPrimitiveElement => {
                            let some = r#allergenic_indicator.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_allergenicIndicator",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        Field::SpecifiedSubstance => {
                            if r#specified_substance.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "specifiedSubstance",
                                ));
                            }
                            r#specified_substance = Some(map_access.next_value()?);
                        }
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(MedicinalProductIngredient {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#role: r#role.ok_or(serde::de::Error::missing_field("role"))?,
                    r#allergenic_indicator,
                    r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    r#specified_substance: r#specified_substance.unwrap_or(vec![]),
                    r#substance,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
