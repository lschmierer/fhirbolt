// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "The minimum volume to be conditioned in the container."]
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerMinimumVolume {
    fn default() -> SpecimenDefinitionTypeTestedContainerMinimumVolume {
        SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid
    }
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    fn default() -> SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
        SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid
    }
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainerAdditive {
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
            match self.r#additive {
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::CodeableConcept(
                    ref value,
                ) => {
                    state.serialize_entry("additiveCodeableConcept", value)?;
                }
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Reference(ref value) => {
                    state.serialize_entry("additiveReference", value)?;
                }
                SpecimenDefinitionTypeTestedContainerAdditiveAdditive::Invalid => {
                    return Err(serde::ser::Error::custom("additive is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedContainerAdditive {
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
            #[serde(rename = "additiveCodeableConcept")]
            AdditiveCodeableConcept,
            #[serde(rename = "additiveReference")]
            AdditiveReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedContainerAdditive;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainerAdditive")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainerAdditive, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#additive: Option<SpecimenDefinitionTypeTestedContainerAdditiveAdditive> =
                    None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: AdditiveCodeableConcept => { if r#additive . is_some () { return Err (serde :: de :: Error :: duplicate_field ("additiveCodeableConcept")) ; } r#additive = Some (SpecimenDefinitionTypeTestedContainerAdditiveAdditive :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: AdditiveReference => { if r#additive . is_some () { return Err (serde :: de :: Error :: duplicate_field ("additiveReference")) ; } r#additive = Some (SpecimenDefinitionTypeTestedContainerAdditiveAdditive :: Reference (map_access . next_value () ?)) ; } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "additiveCodeableConcept" , "additiveReference" ,])) ; } } } Ok (SpecimenDefinitionTypeTestedContainerAdditive { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#additive : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#additive . unwrap_or (Default :: default ()) } else { r#additive . ok_or (serde :: de :: Error :: missing_field ("additive[x]")) ? } , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The specimen's container."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of material of the container."]
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of container used to contain this kind of specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Color of container cap."]
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The textual description of the kind of container."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The capacity (volume or other measure) of this kind of container."]
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The minimum volume to be conditioned in the container."]
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    #[doc = "Special processing that should be applied to the container for this kind of specimen."]
    pub r#preparation: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedContainer {
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
            if let Some(some) = self.r#material.as_ref() {
                state.serialize_entry("material", some)?;
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if let Some(some) = self.r#cap.as_ref() {
                state.serialize_entry("cap", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            if let Some(some) = self.r#capacity.as_ref() {
                state.serialize_entry("capacity", some)?;
            }
            if let Some(some) = self.r#minimum_volume.as_ref() {
                match some {
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::Quantity(ref value) => {
                        state.serialize_entry("minimumVolumeQuantity", value)?;
                    }
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("minimumVolumeString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state
                                    .serialize_entry("_minimumVolumeString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("minimumVolumeString", value)?;
                        }
                    }
                    SpecimenDefinitionTypeTestedContainerMinimumVolume::Invalid => {
                        return Err(serde::ser::Error::custom("minimum_volume is invalid"))
                    }
                }
            }
            if !self.r#additive.is_empty() {
                state.serialize_entry("additive", &self.r#additive)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preparation.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("preparation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_preparation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#preparation.as_ref() {
                    state.serialize_entry("preparation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedContainer {
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
            #[serde(rename = "material")]
            Material,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "cap")]
            Cap,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "capacity")]
            Capacity,
            #[serde(rename = "minimumVolumeQuantity")]
            MinimumVolumeQuantity,
            #[serde(rename = "minimumVolumeString")]
            MinimumVolumeString,
            #[serde(rename = "_minimumVolumeString")]
            MinimumVolumeStringPrimitiveElement,
            #[serde(rename = "additive")]
            Additive,
            #[serde(rename = "preparation")]
            Preparation,
            #[serde(rename = "_preparation")]
            PreparationPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedContainer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#material: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#cap: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#capacity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#minimum_volume: Option<
                    SpecimenDefinitionTypeTestedContainerMinimumVolume,
                > = None;
                let mut r#additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditive>> =
                    None;
                let mut r#preparation: Option<super::super::types::String> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Material => { if r#material . is_some () { return Err (serde :: de :: Error :: duplicate_field ("material")) ; } r#material = Some (map_access . next_value () ?) ; } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value () ?) ; } , Field :: Cap => { if r#cap . is_some () { return Err (serde :: de :: Error :: duplicate_field ("cap")) ; } r#cap = Some (map_access . next_value () ?) ; } , Field :: Description => { if _ctx . from_json { let some = r#description . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("description")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#description . is_some () { return Err (serde :: de :: Error :: duplicate_field ("description")) ; } r#description = Some (map_access . next_value () ?) ; } } , Field :: DescriptionPrimitiveElement => { if _ctx . from_json { let some = r#description . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_description")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("description" , & ["id" , "extension" , "modifierExtension" , "material" , "type" , "cap" , "description" , "capacity" , "minimumVolumeQuantity" , "minimumVolumeString" , "additive" , "preparation" ,])) ; } } , Field :: Capacity => { if r#capacity . is_some () { return Err (serde :: de :: Error :: duplicate_field ("capacity")) ; } r#capacity = Some (map_access . next_value () ?) ; } , Field :: MinimumVolumeQuantity => { if r#minimum_volume . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minimumVolumeQuantity")) ; } r#minimum_volume = Some (SpecimenDefinitionTypeTestedContainerMinimumVolume :: Quantity (map_access . next_value () ?)) ; } , Field :: MinimumVolumeString => { if _ctx . from_json { let r#enum = r#minimum_volume . get_or_insert (SpecimenDefinitionTypeTestedContainerMinimumVolume :: String (Default :: default ())) ; if let SpecimenDefinitionTypeTestedContainerMinimumVolume :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minimumVolumeString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("minimumVolume[x]")) ; } } else { if r#minimum_volume . is_some () { return Err (serde :: de :: Error :: duplicate_field ("minimumVolumeString")) ; } r#minimum_volume = Some (SpecimenDefinitionTypeTestedContainerMinimumVolume :: String (map_access . next_value () ?)) ; } } , Field :: MinimumVolumeStringPrimitiveElement => { if _ctx . from_json { let r#enum = r#minimum_volume . get_or_insert (SpecimenDefinitionTypeTestedContainerMinimumVolume :: String (Default :: default ())) ; if let SpecimenDefinitionTypeTestedContainerMinimumVolume :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_minimumVolumeString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_minimumVolume[x]")) ; } } else { return Err (serde :: de :: Error :: unknown_field ("minimumVolumeString" , & ["id" , "extension" , "modifierExtension" , "material" , "type" , "cap" , "description" , "capacity" , "minimumVolumeQuantity" , "minimumVolumeString" , "additive" , "preparation" ,])) ; } } , Field :: Additive => { if r#additive . is_some () { return Err (serde :: de :: Error :: duplicate_field ("additive")) ; } r#additive = Some (map_access . next_value () ?) ; } , Field :: Preparation => { if _ctx . from_json { let some = r#preparation . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("preparation")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#preparation . is_some () { return Err (serde :: de :: Error :: duplicate_field ("preparation")) ; } r#preparation = Some (map_access . next_value () ?) ; } } , Field :: PreparationPrimitiveElement => { if _ctx . from_json { let some = r#preparation . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_preparation")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("preparation" , & ["id" , "extension" , "modifierExtension" , "material" , "type" , "cap" , "description" , "capacity" , "minimumVolumeQuantity" , "minimumVolumeString" , "additive" , "preparation" ,])) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "material" , "type" , "cap" , "description" , "capacity" , "minimumVolumeQuantity" , "minimumVolumeString" , "additive" , "preparation" ,])) ; } } } Ok (SpecimenDefinitionTypeTestedContainer { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#material , r#type , r#cap , r#description , r#capacity , r#minimum_volume , r#additive : r#additive . unwrap_or (vec ! []) , r#preparation , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedHandling {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "It qualifies the interval of temperature, which characterizes an occurrence of handling. Conditions that are not related to temperature may be handled in the instruction element."]
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The temperature interval for this set of handling instructions."]
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
    #[doc = "The maximum time interval of preservation of the specimen with these conditions."]
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Additional textual instructions for the preservation or transport of the specimen. For instance, 'Protect from light exposure'."]
    pub r#instruction: Option<super::super::types::String>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTestedHandling {
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
            if let Some(some) = self.r#temperature_qualifier.as_ref() {
                state.serialize_entry("temperatureQualifier", some)?;
            }
            if let Some(some) = self.r#temperature_range.as_ref() {
                state.serialize_entry("temperatureRange", some)?;
            }
            if let Some(some) = self.r#max_duration.as_ref() {
                state.serialize_entry("maxDuration", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#instruction.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("instruction", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_instruction", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#instruction.as_ref() {
                    state.serialize_entry("instruction", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTestedHandling {
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
            #[serde(rename = "temperatureQualifier")]
            TemperatureQualifier,
            #[serde(rename = "temperatureRange")]
            TemperatureRange,
            #[serde(rename = "maxDuration")]
            MaxDuration,
            #[serde(rename = "instruction")]
            Instruction,
            #[serde(rename = "_instruction")]
            InstructionPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTestedHandling;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedHandling")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedHandling, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#temperature_range: Option<Box<super::super::types::Range>> = None;
                let mut r#max_duration: Option<Box<super::super::types::Duration>> = None;
                let mut r#instruction: Option<super::super::types::String> = None;
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
                            Field::TemperatureQualifier => {
                                if r#temperature_qualifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "temperatureQualifier",
                                    ));
                                }
                                r#temperature_qualifier = Some(map_access.next_value()?);
                            }
                            Field::TemperatureRange => {
                                if r#temperature_range.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "temperatureRange",
                                    ));
                                }
                                r#temperature_range = Some(map_access.next_value()?);
                            }
                            Field::MaxDuration => {
                                if r#max_duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxDuration"));
                                }
                                r#max_duration = Some(map_access.next_value()?);
                            }
                            Field::Instruction => {
                                if _ctx.from_json {
                                    let some = r#instruction.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instruction",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#instruction.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "instruction",
                                        ));
                                    }
                                    r#instruction = Some(map_access.next_value()?);
                                }
                            }
                            Field::InstructionPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#instruction.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_instruction",
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
                                        "instruction",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "temperatureQualifier",
                                            "temperatureRange",
                                            "maxDuration",
                                            "instruction",
                                        ],
                                    ));
                                }
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
                                        "temperatureQualifier",
                                        "temperatureRange",
                                        "maxDuration",
                                        "instruction",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SpecimenDefinitionTypeTestedHandling {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#temperature_qualifier,
                        r#temperature_range,
                        r#max_duration,
                        r#instruction,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinitionTypeTested {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Primary of secondary specimen."]
    pub r#is_derived: Option<super::super::types::Boolean>,
    #[doc = "The kind of specimen conditioned for testing expected by lab."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The preference for this type of conditioned specimen."]
    pub r#preference: super::super::types::Code,
    #[doc = "The specimen's container."]
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
    #[doc = "Requirements for delivery and special handling of this kind of conditioned specimen."]
    pub r#requirement: Option<super::super::types::String>,
    #[doc = "The usual time that a specimen of this kind is retained after the ordered tests are completed, for the purpose of additional testing."]
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    #[doc = "Criterion for rejection of the specimen in its container by the laboratory."]
    pub r#rejection_criterion: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
}
impl serde::ser::Serialize for SpecimenDefinitionTypeTested {
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
                if let Some(some) = self.r#is_derived.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("isDerived", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_isDerived", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#is_derived.as_ref() {
                    state.serialize_entry("isDerived", some)?;
                }
            }
            if let Some(some) = self.r#type.as_ref() {
                state.serialize_entry("type", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#preference.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("preference", &some)?;
                }
                if self.r#preference.id.is_some() || !self.r#preference.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#preference.id.as_ref(),
                        extension: &self.r#preference.extension,
                    };
                    state.serialize_entry("_preference", &primitive_element)?;
                }
            } else {
                state.serialize_entry("preference", &self.r#preference)?;
            }
            if let Some(some) = self.r#container.as_ref() {
                state.serialize_entry("container", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#requirement.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("requirement", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_requirement", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#requirement.as_ref() {
                    state.serialize_entry("requirement", some)?;
                }
            }
            if let Some(some) = self.r#retention_time.as_ref() {
                state.serialize_entry("retentionTime", some)?;
            }
            if !self.r#rejection_criterion.is_empty() {
                state.serialize_entry("rejectionCriterion", &self.r#rejection_criterion)?;
            }
            if !self.r#handling.is_empty() {
                state.serialize_entry("handling", &self.r#handling)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinitionTypeTested {
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
            #[serde(rename = "isDerived")]
            IsDerived,
            #[serde(rename = "_isDerived")]
            IsDerivedPrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "preference")]
            Preference,
            #[serde(rename = "_preference")]
            PreferencePrimitiveElement,
            #[serde(rename = "container")]
            Container,
            #[serde(rename = "requirement")]
            Requirement,
            #[serde(rename = "_requirement")]
            RequirementPrimitiveElement,
            #[serde(rename = "retentionTime")]
            RetentionTime,
            #[serde(rename = "rejectionCriterion")]
            RejectionCriterion,
            #[serde(rename = "handling")]
            Handling,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinitionTypeTested;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTested")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTested, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#is_derived: Option<super::super::types::Boolean> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#preference: Option<super::super::types::Code> = None;
                let mut r#container: Option<SpecimenDefinitionTypeTestedContainer> = None;
                let mut r#requirement: Option<super::super::types::String> = None;
                let mut r#retention_time: Option<Box<super::super::types::Duration>> = None;
                let mut r#rejection_criterion: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#handling: Option<Vec<SpecimenDefinitionTypeTestedHandling>> = None;
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
                            Field::IsDerived => {
                                if _ctx.from_json {
                                    let some = r#is_derived.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("isDerived"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#is_derived.is_some() {
                                        return Err(serde::de::Error::duplicate_field("isDerived"));
                                    }
                                    r#is_derived = Some(map_access.next_value()?);
                                }
                            }
                            Field::IsDerivedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#is_derived.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_isDerived",
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
                                        "isDerived",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "isDerived",
                                            "type",
                                            "preference",
                                            "container",
                                            "requirement",
                                            "retentionTime",
                                            "rejectionCriterion",
                                            "handling",
                                        ],
                                    ));
                                }
                            }
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::Preference => {
                                if _ctx.from_json {
                                    let some = r#preference.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preference",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#preference.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "preference",
                                        ));
                                    }
                                    r#preference = Some(map_access.next_value()?);
                                }
                            }
                            Field::PreferencePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#preference.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_preference",
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
                                        "preference",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "isDerived",
                                            "type",
                                            "preference",
                                            "container",
                                            "requirement",
                                            "retentionTime",
                                            "rejectionCriterion",
                                            "handling",
                                        ],
                                    ));
                                }
                            }
                            Field::Container => {
                                if r#container.is_some() {
                                    return Err(serde::de::Error::duplicate_field("container"));
                                }
                                r#container = Some(map_access.next_value()?);
                            }
                            Field::Requirement => {
                                if _ctx.from_json {
                                    let some = r#requirement.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requirement",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#requirement.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "requirement",
                                        ));
                                    }
                                    r#requirement = Some(map_access.next_value()?);
                                }
                            }
                            Field::RequirementPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#requirement.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_requirement",
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
                                        "requirement",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "isDerived",
                                            "type",
                                            "preference",
                                            "container",
                                            "requirement",
                                            "retentionTime",
                                            "rejectionCriterion",
                                            "handling",
                                        ],
                                    ));
                                }
                            }
                            Field::RetentionTime => {
                                if r#retention_time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("retentionTime"));
                                }
                                r#retention_time = Some(map_access.next_value()?);
                            }
                            Field::RejectionCriterion => {
                                if r#rejection_criterion.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "rejectionCriterion",
                                    ));
                                }
                                r#rejection_criterion = Some(map_access.next_value()?);
                            }
                            Field::Handling => {
                                if r#handling.is_some() {
                                    return Err(serde::de::Error::duplicate_field("handling"));
                                }
                                r#handling = Some(map_access.next_value()?);
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
                                        "isDerived",
                                        "type",
                                        "preference",
                                        "container",
                                        "requirement",
                                        "retentionTime",
                                        "rejectionCriterion",
                                        "handling",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SpecimenDefinitionTypeTested {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#is_derived,
                        r#type,
                        r#preference: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#preference.unwrap_or(Default::default())
                        } else {
                            r#preference.ok_or(serde::de::Error::missing_field("preference"))?
                        },
                        r#container,
                        r#requirement,
                        r#retention_time,
                        r#rejection_criterion: r#rejection_criterion.unwrap_or(vec![]),
                        r#handling: r#handling.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A kind of specimen with associated set of requirements."]
#[derive(Default, Debug, Clone)]
pub struct SpecimenDefinition {
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
    #[doc = "A business identifier associated with the kind of specimen."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The kind of material to be collected."]
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Preparation of the patient for specimen collection."]
    pub r#patient_preparation: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Time aspect of specimen collection (duration or offset)."]
    pub r#time_aspect: Option<super::super::types::String>,
    #[doc = "The action to be performed for collecting the specimen."]
    pub r#collection: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
}
impl crate::AnyResource for SpecimenDefinition {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for SpecimenDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "SpecimenDefinition")?;
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
            if let Some(some) = self.r#type_collected.as_ref() {
                state.serialize_entry("typeCollected", some)?;
            }
            if !self.r#patient_preparation.is_empty() {
                state.serialize_entry("patientPreparation", &self.r#patient_preparation)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#time_aspect.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("timeAspect", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_timeAspect", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#time_aspect.as_ref() {
                    state.serialize_entry("timeAspect", some)?;
                }
            }
            if !self.r#collection.is_empty() {
                state.serialize_entry("collection", &self.r#collection)?;
            }
            if !self.r#type_tested.is_empty() {
                state.serialize_entry("typeTested", &self.r#type_tested)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for SpecimenDefinition {
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
            #[serde(rename = "typeCollected")]
            TypeCollected,
            #[serde(rename = "patientPreparation")]
            PatientPreparation,
            #[serde(rename = "timeAspect")]
            TimeAspect,
            #[serde(rename = "_timeAspect")]
            TimeAspectPrimitiveElement,
            #[serde(rename = "collection")]
            Collection,
            #[serde(rename = "typeTested")]
            TypeTested,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = SpecimenDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenDefinition, V::Error>
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
                let mut r#type_collected: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#patient_preparation: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#time_aspect: Option<super::super::types::String> = None;
                let mut r#collection: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#type_tested: Option<Vec<SpecimenDefinitionTypeTested>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "SpecimenDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"SpecimenDefinition",
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
                                            "typeCollected",
                                            "patientPreparation",
                                            "timeAspect",
                                            "collection",
                                            "typeTested",
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
                                            "typeCollected",
                                            "patientPreparation",
                                            "timeAspect",
                                            "collection",
                                            "typeTested",
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
                            Field::TypeCollected => {
                                if r#type_collected.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeCollected"));
                                }
                                r#type_collected = Some(map_access.next_value()?);
                            }
                            Field::PatientPreparation => {
                                if r#patient_preparation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientPreparation",
                                    ));
                                }
                                r#patient_preparation = Some(map_access.next_value()?);
                            }
                            Field::TimeAspect => {
                                if _ctx.from_json {
                                    let some = r#time_aspect.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeAspect",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#time_aspect.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "timeAspect",
                                        ));
                                    }
                                    r#time_aspect = Some(map_access.next_value()?);
                                }
                            }
                            Field::TimeAspectPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#time_aspect.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_timeAspect",
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
                                        "timeAspect",
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
                                            "typeCollected",
                                            "patientPreparation",
                                            "timeAspect",
                                            "collection",
                                            "typeTested",
                                        ],
                                    ));
                                }
                            }
                            Field::Collection => {
                                if r#collection.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collection"));
                                }
                                r#collection = Some(map_access.next_value()?);
                            }
                            Field::TypeTested => {
                                if r#type_tested.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeTested"));
                                }
                                r#type_tested = Some(map_access.next_value()?);
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
                                        "typeCollected",
                                        "patientPreparation",
                                        "timeAspect",
                                        "collection",
                                        "typeTested",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(SpecimenDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier,
                        r#type_collected,
                        r#patient_preparation: r#patient_preparation.unwrap_or(vec![]),
                        r#time_aspect,
                        r#collection: r#collection.unwrap_or(vec![]),
                        r#type_tested: r#type_tested.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
