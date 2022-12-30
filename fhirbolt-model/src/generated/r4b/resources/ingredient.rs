// Generated on 2022-12-29 by fhirbolt-codegen v0.1.0
#[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. Unit of presentation refers to the quantity that the item occurs in e.g. a strength per tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally normalized as a unitary unit, which would be 'per mg')."]
#[derive(Debug, Clone)]
pub enum IngredientSubstanceStrengthPresentation {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthPresentation {
    fn default() -> IngredientSubstanceStrengthPresentation {
        IngredientSubstanceStrengthPresentation::Invalid
    }
}
#[doc = "The strength per unitary volume (or mass)."]
#[derive(Debug, Clone)]
pub enum IngredientSubstanceStrengthConcentration {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthConcentration {
    fn default() -> IngredientSubstanceStrengthConcentration {
        IngredientSubstanceStrengthConcentration::Invalid
    }
}
#[doc = "Strength expressed in terms of a reference substance."]
#[derive(Debug, Clone)]
pub enum IngredientSubstanceStrengthReferenceStrengthStrength {
    Ratio(Box<super::super::types::Ratio>),
    RatioRange(Box<super::super::types::RatioRange>),
    Invalid,
}
impl Default for IngredientSubstanceStrengthReferenceStrengthStrength {
    fn default() -> IngredientSubstanceStrengthReferenceStrengthStrength {
        IngredientSubstanceStrengthReferenceStrengthStrength::Invalid
    }
}
#[doc = "The organization(s) that manufacture this ingredient. Can be used to indicate:         1) Organizations we are aware of that manufacture this ingredient         2) Specific Manufacturer(s) currently being used         3) Set of organisations allowed to manufacture this ingredient for this product         Users must be clear on the application of context relevant to their use case."]
#[derive(Default, Debug, Clone)]
pub struct IngredientManufacturer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The way in which this manufacturer is associated with the ingredient. For example whether it is a possible one (others allowed), or an exclusive authorized one for this ingredient. Note that this is not the manufacturing process role."]
    pub r#role: Option<super::super::types::Code>,
    #[doc = "An organization that manufactures this ingredient."]
    pub r#manufacturer: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for IngredientManufacturer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if _ctx.output_json {
                if let Some(some) = self.r#role.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("role", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_role", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#role.as_ref() {
                    state.serialize_entry("role", some)?;
                }
            }
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for IngredientManufacturer {
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
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "_role")]
            RolePrimitiveElement,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngredientManufacturer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientManufacturer")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<IngredientManufacturer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#role: Option<super::super::types::Code> = None;
                let mut r#manufacturer: Option<Box<super::super::types::Reference>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#role.is_some() {
                                        return Err(serde::de::Error::duplicate_field("role"));
                                    }
                                    r#role = Some(map_access.next_value()?);
                                }
                            }
                            Field::RolePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#role.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_role"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "role",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "role",
                                            "manufacturer",
                                        ],
                                    ));
                                }
                            }
                            Field::Manufacturer => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "role",
                                        "manufacturer",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(IngredientManufacturer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#role,
                        r#manufacturer: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#manufacturer.unwrap_or(Default::default())
                        } else {
                            r#manufacturer.ok_or(serde::de::Error::missing_field("manufacturer"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Strength expressed in terms of a reference substance. For when the ingredient strength is additionally expressed as equivalent to the strength of some other closely related substance (e.g. salt vs. base). Reference strength represents the strength (quantitative composition) of the active moiety of the active substance. There are situations when the active substance and active moiety are different, therefore both a strength and a reference strength are needed."]
#[derive(Default, Debug, Clone)]
pub struct IngredientSubstanceStrengthReferenceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Relevant reference substance."]
    pub r#substance: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Strength expressed in terms of a reference substance."]
    pub r#strength: IngredientSubstanceStrengthReferenceStrengthStrength,
    #[doc = "For when strength is measured at a particular point or distance."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for IngredientSubstanceStrengthReferenceStrength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            match self.r#strength {
                IngredientSubstanceStrengthReferenceStrengthStrength::Ratio(ref value) => {
                    state.serialize_entry("strengthRatio", value)?;
                }
                IngredientSubstanceStrengthReferenceStrengthStrength::RatioRange(ref value) => {
                    state.serialize_entry("strengthRatioRange", value)?;
                }
                IngredientSubstanceStrengthReferenceStrengthStrength::Invalid => {
                    return Err(serde::ser::Error::custom("strength is a required field"))
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#measurement_point.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("measurementPoint", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_measurementPoint", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#measurement_point.as_ref() {
                    state.serialize_entry("measurementPoint", some)?;
                }
            }
            if !self.r#country.is_empty() {
                state.serialize_entry("country", &self.r#country)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for IngredientSubstanceStrengthReferenceStrength {
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
            #[serde(rename = "strengthRatio")]
            StrengthRatio,
            #[serde(rename = "strengthRatioRange")]
            StrengthRatioRange,
            #[serde(rename = "measurementPoint")]
            MeasurementPoint,
            #[serde(rename = "_measurementPoint")]
            MeasurementPointPrimitiveElement,
            #[serde(rename = "country")]
            Country,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngredientSubstanceStrengthReferenceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstanceStrengthReferenceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<IngredientSubstanceStrengthReferenceStrength, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#strength: Option<IngredientSubstanceStrengthReferenceStrengthStrength> =
                    None;
                let mut r#measurement_point: Option<super::super::types::String> = None;
                let mut r#country: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Substance => { if r#substance . is_some () { return Err (serde :: de :: Error :: duplicate_field ("substance")) ; } r#substance = Some (map_access . next_value () ?) ; } , Field :: StrengthRatio => { if r#strength . is_some () { return Err (serde :: de :: Error :: duplicate_field ("strengthRatio")) ; } r#strength = Some (IngredientSubstanceStrengthReferenceStrengthStrength :: Ratio (map_access . next_value () ?)) ; } , Field :: StrengthRatioRange => { if r#strength . is_some () { return Err (serde :: de :: Error :: duplicate_field ("strengthRatioRange")) ; } r#strength = Some (IngredientSubstanceStrengthReferenceStrengthStrength :: RatioRange (map_access . next_value () ?)) ; } , Field :: MeasurementPoint => { if _ctx . from_json { let some = r#measurement_point . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("measurementPoint")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#measurement_point . is_some () { return Err (serde :: de :: Error :: duplicate_field ("measurementPoint")) ; } r#measurement_point = Some (map_access . next_value () ?) ; } } , Field :: MeasurementPointPrimitiveElement => { if _ctx . from_json { let some = r#measurement_point . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_measurementPoint")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("measurementPoint" , & ["id" , "extension" , "modifierExtension" , "substance" , "strengthRatio" , "strengthRatioRange" , "measurementPoint" , "country" ,])) ; } } , Field :: Country => { if r#country . is_some () { return Err (serde :: de :: Error :: duplicate_field ("country")) ; } r#country = Some (map_access . next_value () ?) ; } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "substance" , "strengthRatio" , "strengthRatioRange" , "measurementPoint" , "country" ,])) ; } } } Ok (IngredientSubstanceStrengthReferenceStrength { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#substance , r#strength : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#strength . unwrap_or (Default :: default ()) } else { r#strength . ok_or (serde :: de :: Error :: missing_field ("strength[x]")) ? } , r#measurement_point , r#country : r#country . unwrap_or (vec ! []) , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. The allowed repetitions do not represent different strengths, but are different representations - mathematically equivalent - of a single strength."]
#[derive(Default, Debug, Clone)]
pub struct IngredientSubstanceStrength {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. Unit of presentation refers to the quantity that the item occurs in e.g. a strength per tablet size, perhaps 'per 20mg' (the size of the tablet). It is not generally normalized as a unitary unit, which would be 'per mg')."]
    pub r#presentation: Option<IngredientSubstanceStrengthPresentation>,
    #[doc = "A textual represention of either the whole of the presentation strength or a part of it - with the rest being in Strength.presentation as a ratio."]
    pub r#text_presentation: Option<super::super::types::String>,
    #[doc = "The strength per unitary volume (or mass)."]
    pub r#concentration: Option<IngredientSubstanceStrengthConcentration>,
    #[doc = "A textual represention of either the whole of the concentration strength or a part of it - with the rest being in Strength.concentration as a ratio."]
    pub r#text_concentration: Option<super::super::types::String>,
    #[doc = "For when strength is measured at a particular point or distance. There are products where strength is measured at a particular point. For example, the strength of the ingredient in some inhalers is measured at a particular position relative to the point of aerosolization."]
    pub r#measurement_point: Option<super::super::types::String>,
    #[doc = "The country or countries for which the strength range applies."]
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Strength expressed in terms of a reference substance. For when the ingredient strength is additionally expressed as equivalent to the strength of some other closely related substance (e.g. salt vs. base). Reference strength represents the strength (quantitative composition) of the active moiety of the active substance. There are situations when the active substance and active moiety are different, therefore both a strength and a reference strength are needed."]
    pub r#reference_strength: Vec<IngredientSubstanceStrengthReferenceStrength>,
}
impl serde::ser::Serialize for IngredientSubstanceStrength {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            if let Some(some) = self.r#presentation.as_ref() {
                match some {
                    IngredientSubstanceStrengthPresentation::Ratio(ref value) => {
                        state.serialize_entry("presentationRatio", value)?;
                    }
                    IngredientSubstanceStrengthPresentation::RatioRange(ref value) => {
                        state.serialize_entry("presentationRatioRange", value)?;
                    }
                    IngredientSubstanceStrengthPresentation::Invalid => {
                        return Err(serde::ser::Error::custom("presentation is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text_presentation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("textPresentation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_textPresentation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text_presentation.as_ref() {
                    state.serialize_entry("textPresentation", some)?;
                }
            }
            if let Some(some) = self.r#concentration.as_ref() {
                match some {
                    IngredientSubstanceStrengthConcentration::Ratio(ref value) => {
                        state.serialize_entry("concentrationRatio", value)?;
                    }
                    IngredientSubstanceStrengthConcentration::RatioRange(ref value) => {
                        state.serialize_entry("concentrationRatioRange", value)?;
                    }
                    IngredientSubstanceStrengthConcentration::Invalid => {
                        return Err(serde::ser::Error::custom("concentration is invalid"))
                    }
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#text_concentration.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("textConcentration", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_textConcentration", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#text_concentration.as_ref() {
                    state.serialize_entry("textConcentration", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#measurement_point.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("measurementPoint", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_measurementPoint", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#measurement_point.as_ref() {
                    state.serialize_entry("measurementPoint", some)?;
                }
            }
            if !self.r#country.is_empty() {
                state.serialize_entry("country", &self.r#country)?;
            }
            if !self.r#reference_strength.is_empty() {
                state.serialize_entry("referenceStrength", &self.r#reference_strength)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for IngredientSubstanceStrength {
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
            #[serde(rename = "presentationRatio")]
            PresentationRatio,
            #[serde(rename = "presentationRatioRange")]
            PresentationRatioRange,
            #[serde(rename = "textPresentation")]
            TextPresentation,
            #[serde(rename = "_textPresentation")]
            TextPresentationPrimitiveElement,
            #[serde(rename = "concentrationRatio")]
            ConcentrationRatio,
            #[serde(rename = "concentrationRatioRange")]
            ConcentrationRatioRange,
            #[serde(rename = "textConcentration")]
            TextConcentration,
            #[serde(rename = "_textConcentration")]
            TextConcentrationPrimitiveElement,
            #[serde(rename = "measurementPoint")]
            MeasurementPoint,
            #[serde(rename = "_measurementPoint")]
            MeasurementPointPrimitiveElement,
            #[serde(rename = "country")]
            Country,
            #[serde(rename = "referenceStrength")]
            ReferenceStrength,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngredientSubstanceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstanceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<IngredientSubstanceStrength, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#presentation: Option<IngredientSubstanceStrengthPresentation> = None;
                let mut r#text_presentation: Option<super::super::types::String> = None;
                let mut r#concentration: Option<IngredientSubstanceStrengthConcentration> = None;
                let mut r#text_concentration: Option<super::super::types::String> = None;
                let mut r#measurement_point: Option<super::super::types::String> = None;
                let mut r#country: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#reference_strength: Option<
                    Vec<IngredientSubstanceStrengthReferenceStrength>,
                > = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::PresentationRatio => {
                                if r#presentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "presentationRatio",
                                    ));
                                }
                                r#presentation =
                                    Some(IngredientSubstanceStrengthPresentation::Ratio(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::PresentationRatioRange => {
                                if r#presentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "presentationRatioRange",
                                    ));
                                }
                                r#presentation =
                                    Some(IngredientSubstanceStrengthPresentation::RatioRange(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::TextPresentation => {
                                if _ctx.from_json {
                                    let some =
                                        r#text_presentation.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "textPresentation",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text_presentation.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "textPresentation",
                                        ));
                                    }
                                    r#text_presentation = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextPresentationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#text_presentation.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_textPresentation",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "textPresentation",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "presentationRatio",
                                            "presentationRatioRange",
                                            "textPresentation",
                                            "concentrationRatio",
                                            "concentrationRatioRange",
                                            "textConcentration",
                                            "measurementPoint",
                                            "country",
                                            "referenceStrength",
                                        ],
                                    ));
                                }
                            }
                            Field::ConcentrationRatio => {
                                if r#concentration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "concentrationRatio",
                                    ));
                                }
                                r#concentration =
                                    Some(IngredientSubstanceStrengthConcentration::Ratio(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ConcentrationRatioRange => {
                                if r#concentration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "concentrationRatioRange",
                                    ));
                                }
                                r#concentration =
                                    Some(IngredientSubstanceStrengthConcentration::RatioRange(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::TextConcentration => {
                                if _ctx.from_json {
                                    let some =
                                        r#text_concentration.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "textConcentration",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#text_concentration.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "textConcentration",
                                        ));
                                    }
                                    r#text_concentration = Some(map_access.next_value()?);
                                }
                            }
                            Field::TextConcentrationPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#text_concentration.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_textConcentration",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "textConcentration",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "presentationRatio",
                                            "presentationRatioRange",
                                            "textPresentation",
                                            "concentrationRatio",
                                            "concentrationRatioRange",
                                            "textConcentration",
                                            "measurementPoint",
                                            "country",
                                            "referenceStrength",
                                        ],
                                    ));
                                }
                            }
                            Field::MeasurementPoint => {
                                if _ctx.from_json {
                                    let some =
                                        r#measurement_point.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "measurementPoint",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#measurement_point.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "measurementPoint",
                                        ));
                                    }
                                    r#measurement_point = Some(map_access.next_value()?);
                                }
                            }
                            Field::MeasurementPointPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#measurement_point.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_measurementPoint",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "measurementPoint",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "presentationRatio",
                                            "presentationRatioRange",
                                            "textPresentation",
                                            "concentrationRatio",
                                            "concentrationRatioRange",
                                            "textConcentration",
                                            "measurementPoint",
                                            "country",
                                            "referenceStrength",
                                        ],
                                    ));
                                }
                            }
                            Field::Country => {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                r#country = Some(map_access.next_value()?);
                            }
                            Field::ReferenceStrength => {
                                if r#reference_strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceStrength",
                                    ));
                                }
                                r#reference_strength = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "presentationRatio",
                                        "presentationRatioRange",
                                        "textPresentation",
                                        "concentrationRatio",
                                        "concentrationRatioRange",
                                        "textConcentration",
                                        "measurementPoint",
                                        "country",
                                        "referenceStrength",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(IngredientSubstanceStrength {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#presentation,
                        r#text_presentation,
                        r#concentration,
                        r#text_concentration,
                        r#measurement_point,
                        r#country: r#country.unwrap_or(vec![]),
                        r#reference_strength: r#reference_strength.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The substance that comprises this ingredient."]
#[derive(Default, Debug, Clone)]
pub struct IngredientSubstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code or full resource that represents the ingredient's substance."]
    pub r#code: Box<super::super::types::CodeableReference>,
    #[doc = "The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item. The allowed repetitions do not represent different strengths, but are different representations - mathematically equivalent - of a single strength."]
    pub r#strength: Vec<IngredientSubstanceStrength>,
}
impl serde::ser::Serialize for IngredientSubstance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for IngredientSubstance {
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
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = IngredientSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("IngredientSubstance")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<IngredientSubstance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableReference>> = None;
                let mut r#strength: Option<Vec<IngredientSubstanceStrength>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "code", "strength"],
                                ));
                            },
                        }
                    }
                    Ok(IngredientSubstance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#strength: r#strength.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An ingredient of a manufactured item or pharmaceutical product."]
#[derive(Default, Debug, Clone)]
pub struct Ingredient {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<std::string::String>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The identifier(s) of this Ingredient that are assigned by business processes and/or used to refer to it when a direct URL reference to the resource itself is not appropriate."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The status of this ingredient. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The product which this ingredient is a constituent part of."]
    pub r#for: Vec<Box<super::super::types::Reference>>,
    #[doc = "A classification of the ingredient identifying its purpose within the product, e.g. active, inactive."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "A classification of the ingredient identifying its precise purpose(s) in the drug product. This extends the Ingredient.role to add more detail. Example: antioxidant, alkalizing agent."]
    pub r#function: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the ingredient is a known or suspected allergen. Note that this is a property of the substance, so if a reference to a SubstanceDefinition is used to decribe that (rather than just a code), the allergen information should go there, not here."]
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    #[doc = "The organization(s) that manufacture this ingredient. Can be used to indicate:         1) Organizations we are aware of that manufacture this ingredient         2) Specific Manufacturer(s) currently being used         3) Set of organisations allowed to manufacture this ingredient for this product         Users must be clear on the application of context relevant to their use case."]
    pub r#manufacturer: Vec<IngredientManufacturer>,
    #[doc = "The substance that comprises this ingredient."]
    pub r#substance: IngredientSubstance,
}
impl crate::AnyResource for Ingredient {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for Ingredient {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Ingredient")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if !self.r#for.is_empty() {
                state.serialize_entry("for", &self.r#for)?;
            }
            state.serialize_entry("role", &self.r#role)?;
            if !self.r#function.is_empty() {
                state.serialize_entry("function", &self.r#function)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#allergenic_indicator.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allergenicIndicator", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_allergenicIndicator", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#allergenic_indicator.as_ref() {
                    state.serialize_entry("allergenicIndicator", some)?;
                }
            }
            if !self.r#manufacturer.is_empty() {
                state.serialize_entry("manufacturer", &self.r#manufacturer)?;
            }
            state.serialize_entry("substance", &self.r#substance)?;
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for Ingredient {
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "for")]
            For,
            #[serde(rename = "role")]
            Role,
            #[serde(rename = "function")]
            Function,
            #[serde(rename = "allergenicIndicator")]
            AllergenicIndicator,
            #[serde(rename = "_allergenicIndicator")]
            AllergenicIndicatorPrimitiveElement,
            #[serde(rename = "manufacturer")]
            Manufacturer,
            #[serde(rename = "substance")]
            Substance,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Ingredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Ingredient")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Ingredient, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Box<super::super::types::Identifier>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#for: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#role: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#function: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#allergenic_indicator: Option<super::super::types::Boolean> = None;
                let mut r#manufacturer: Option<Vec<IngredientManufacturer>> = None;
                let mut r#substance: Option<IngredientSubstance> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "Ingredient" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"Ingredient",
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
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#implicit_rules.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "implicitRules",
                                        ));
                                    }
                                    r#implicit_rules = Some(map_access.next_value()?);
                                }
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#implicit_rules.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_implicitRules",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "implicitRules",
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
                                            "status",
                                            "for",
                                            "role",
                                            "function",
                                            "allergenicIndicator",
                                            "manufacturer",
                                            "substance",
                                        ],
                                    ));
                                }
                            }
                            Field::Language => {
                                if _ctx.from_json {
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
                                    r#language = Some(map_access.next_value()?);
                                }
                            }
                            Field::LanguagePrimitiveElement => {
                                if _ctx.from_json {
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "language",
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
                                            "status",
                                            "for",
                                            "role",
                                            "function",
                                            "allergenicIndicator",
                                            "manufacturer",
                                            "substance",
                                        ],
                                    ));
                                }
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#status.is_some() {
                                        return Err(serde::de::Error::duplicate_field("status"));
                                    }
                                    r#status = Some(map_access.next_value()?);
                                }
                            }
                            Field::StatusPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#status.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_status"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "status",
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
                                            "status",
                                            "for",
                                            "role",
                                            "function",
                                            "allergenicIndicator",
                                            "manufacturer",
                                            "substance",
                                        ],
                                    ));
                                }
                            }
                            Field::For => {
                                if r#for.is_some() {
                                    return Err(serde::de::Error::duplicate_field("for"));
                                }
                                r#for = Some(map_access.next_value()?);
                            }
                            Field::Role => {
                                if r#role.is_some() {
                                    return Err(serde::de::Error::duplicate_field("role"));
                                }
                                r#role = Some(map_access.next_value()?);
                            }
                            Field::Function => {
                                if r#function.is_some() {
                                    return Err(serde::de::Error::duplicate_field("function"));
                                }
                                r#function = Some(map_access.next_value()?);
                            }
                            Field::AllergenicIndicator => {
                                if _ctx.from_json {
                                    let some =
                                        r#allergenic_indicator.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allergenicIndicator",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#allergenic_indicator.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "allergenicIndicator",
                                        ));
                                    }
                                    r#allergenic_indicator = Some(map_access.next_value()?);
                                }
                            }
                            Field::AllergenicIndicatorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some =
                                        r#allergenic_indicator.get_or_insert(Default::default());
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
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "allergenicIndicator",
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
                                            "status",
                                            "for",
                                            "role",
                                            "function",
                                            "allergenicIndicator",
                                            "manufacturer",
                                            "substance",
                                        ],
                                    ));
                                }
                            }
                            Field::Manufacturer => {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some(map_access.next_value()?);
                            }
                            Field::Substance => {
                                if r#substance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substance"));
                                }
                                r#substance = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
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
                                        "status",
                                        "for",
                                        "role",
                                        "function",
                                        "allergenicIndicator",
                                        "manufacturer",
                                        "substance",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(Ingredient {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#for: r#for.unwrap_or(vec![]),
                        r#role: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#role.unwrap_or(Default::default())
                        } else {
                            r#role.ok_or(serde::de::Error::missing_field("role"))?
                        },
                        r#function: r#function.unwrap_or(vec![]),
                        r#allergenic_indicator,
                        r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                        r#substance: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#substance.unwrap_or(Default::default())
                        } else {
                            r#substance.ok_or(serde::de::Error::missing_field("substance"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
