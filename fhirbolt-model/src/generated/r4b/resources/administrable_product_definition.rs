// Generated on 2022-12-28 by fhirbolt-codegen v0.1.0
#[doc = "A value for the characteristic."]
#[derive(Debug, Clone)]
pub enum AdministrableProductDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    Invalid,
}
impl Default for AdministrableProductDefinitionPropertyValue {
    fn default() -> AdministrableProductDefinitionPropertyValue {
        AdministrableProductDefinitionPropertyValue::Invalid
    }
}
#[doc = "Characteristics e.g. a product's onset of action."]
#[derive(Default, Debug, Clone)]
pub struct AdministrableProductDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code expressing the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the characteristic."]
    pub r#value: Option<AdministrableProductDefinitionPropertyValue>,
    #[doc = "The status of characteristic e.g. assigned or pending."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for AdministrableProductDefinitionProperty {
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
            state.serialize_entry("type", &self.r#type)?;
            if let Some(some) = self.r#value.as_ref() {
                match some {
                    AdministrableProductDefinitionPropertyValue::CodeableConcept(ref value) => {
                        state.serialize_entry("valueCodeableConcept", value)?;
                    }
                    AdministrableProductDefinitionPropertyValue::Quantity(ref value) => {
                        state.serialize_entry("valueQuantity", value)?;
                    }
                    AdministrableProductDefinitionPropertyValue::Date(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueDate", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueDate", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueDate", value)?;
                        }
                    }
                    AdministrableProductDefinitionPropertyValue::Boolean(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("valueBoolean", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_valueBoolean", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("valueBoolean", value)?;
                        }
                    }
                    AdministrableProductDefinitionPropertyValue::Attachment(ref value) => {
                        state.serialize_entry("valueAttachment", value)?;
                    }
                    AdministrableProductDefinitionPropertyValue::Invalid => {
                        return Err(serde::ser::Error::custom("value is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#status.as_ref() {
                state.serialize_entry("status", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdministrableProductDefinitionProperty {
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
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "valueCodeableConcept")]
            ValueCodeableConcept,
            #[serde(rename = "valueQuantity")]
            ValueQuantity,
            #[serde(rename = "valueDate")]
            ValueDate,
            #[serde(rename = "_valueDate")]
            ValueDatePrimitiveElement,
            #[serde(rename = "valueBoolean")]
            ValueBoolean,
            #[serde(rename = "_valueBoolean")]
            ValueBooleanPrimitiveElement,
            #[serde(rename = "valueAttachment")]
            ValueAttachment,
            #[serde(rename = "status")]
            Status,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdministrableProductDefinitionProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinitionProperty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdministrableProductDefinitionProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<AdministrableProductDefinitionPropertyValue> = None;
                let mut r#status: Option<Box<super::super::types::CodeableConcept>> = None;
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
                            Field::Type => {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some(map_access.next_value()?);
                            }
                            Field::ValueCodeableConcept => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueCodeableConcept",
                                    ));
                                }
                                r#value = Some(
                                    AdministrableProductDefinitionPropertyValue::CodeableConcept(
                                        map_access.next_value()?,
                                    ),
                                );
                            }
                            Field::ValueQuantity => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueQuantity"));
                                }
                                r#value =
                                    Some(AdministrableProductDefinitionPropertyValue::Quantity(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::ValueDate => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        AdministrableProductDefinitionPropertyValue::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let AdministrableProductDefinitionPropertyValue::Date(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueDate",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("valueDate"));
                                    }
                                    r#value =
                                        Some(AdministrableProductDefinitionPropertyValue::Date(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::ValueDatePrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        AdministrableProductDefinitionPropertyValue::Date(
                                            Default::default(),
                                        ),
                                    );
                                    if let AdministrableProductDefinitionPropertyValue::Date(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueDate",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueDate",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "valueCodeableConcept",
                                            "valueQuantity",
                                            "valueDate",
                                            "valueBoolean",
                                            "valueAttachment",
                                            "status",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueBoolean => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        AdministrableProductDefinitionPropertyValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let AdministrableProductDefinitionPropertyValue::Boolean(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.value.is_some() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "valueBoolean",
                                            ));
                                        }
                                        let value: _ = map_access.next_value()?;
                                        variant.value = Some(value);
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("value[x]"));
                                    }
                                } else {
                                    if r#value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "valueBoolean",
                                        ));
                                    }
                                    r#value =
                                        Some(AdministrableProductDefinitionPropertyValue::Boolean(
                                            map_access.next_value()?,
                                        ));
                                }
                            }
                            Field::ValueBooleanPrimitiveElement => {
                                if _ctx.from_json {
                                    let r#enum = r#value.get_or_insert(
                                        AdministrableProductDefinitionPropertyValue::Boolean(
                                            Default::default(),
                                        ),
                                    );
                                    if let AdministrableProductDefinitionPropertyValue::Boolean(
                                        variant,
                                    ) = r#enum
                                    {
                                        if variant.id.is_some() || !variant.extension.is_empty() {
                                            return Err(serde::de::Error::duplicate_field(
                                                "_valueBoolean",
                                            ));
                                        }
                                        let super::super::serde_helpers::PrimitiveElementOwned {
                                            id,
                                            extension,
                                        } = map_access.next_value()?;
                                        variant.id = id;
                                        variant.extension = extension;
                                    } else {
                                        return Err(serde::de::Error::duplicate_field("_value[x]"));
                                    }
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "valueBoolean",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "type",
                                            "valueCodeableConcept",
                                            "valueQuantity",
                                            "valueDate",
                                            "valueBoolean",
                                            "valueAttachment",
                                            "status",
                                        ],
                                    ));
                                }
                            }
                            Field::ValueAttachment => {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueAttachment",
                                    ));
                                }
                                r#value =
                                    Some(AdministrableProductDefinitionPropertyValue::Attachment(
                                        map_access.next_value()?,
                                    ));
                            }
                            Field::Status => {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value()?);
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
                                        "type",
                                        "valueCodeableConcept",
                                        "valueQuantity",
                                        "valueDate",
                                        "valueBoolean",
                                        "valueAttachment",
                                        "status",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdministrableProductDefinitionProperty {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value,
                        r#status,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A species specific time during which consumption of animal product is not appropriate."]
#[derive(Default, Debug, Clone)]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the type of tissue for which the withdrawal period applies, e.g. meat, milk."]
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the time."]
    pub r#value: Box<super::super::types::Quantity>,
    #[doc = "Extra information about the withdrawal period."]
    pub r#supporting_information: Option<super::super::types::String>,
}
impl serde::ser::Serialize
    for AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod
{
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
            state.serialize_entry("tissue", &self.r#tissue)?;
            state.serialize_entry("value", &self.r#value)?;
            if _ctx.output_json {
                if let Some(some) = self.r#supporting_information.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("supportingInformation", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_supportingInformation", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#supporting_information.as_ref() {
                    state.serialize_entry("supportingInformation", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de>
    for AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod
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
            #[serde(rename = "tissue")]
            Tissue,
            #[serde(rename = "value")]
            Value,
            #[serde(rename = "supportingInformation")]
            SupportingInformation,
            #[serde(rename = "_supportingInformation")]
            SupportingInformationPrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value =
                AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter . write_str ("AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#tissue: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value: Option<Box<super::super::types::Quantity>> = None;
                let mut r#supporting_information: Option<super::super::types::String> = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Tissue => { if r#tissue . is_some () { return Err (serde :: de :: Error :: duplicate_field ("tissue")) ; } r#tissue = Some (map_access . next_value () ?) ; } , Field :: Value => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("value")) ; } r#value = Some (map_access . next_value () ?) ; } , Field :: SupportingInformation => { if _ctx . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#supporting_information . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } r#supporting_information = Some (map_access . next_value () ?) ; } } , Field :: SupportingInformationPrimitiveElement => { if _ctx . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_supportingInformation")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; some . id = id ; some . extension = extension ; } else { return Err (serde :: de :: Error :: unknown_field ("supportingInformation" , & ["id" , "extension" , "modifierExtension" , "tissue" , "value" , "supportingInformation" ,])) ; } } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "tissue" , "value" , "supportingInformation" ,])) ; } } } Ok (AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#tissue : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#tissue . unwrap_or (Default :: default ()) } else { r#tissue . ok_or (serde :: de :: Error :: missing_field ("tissue")) ? } , r#value : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#value . unwrap_or (Default :: default ()) } else { r#value . ok_or (serde :: de :: Error :: missing_field ("value")) ? } , r#supporting_information , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A species for which this route applies."]
#[derive(Default, Debug, Clone)]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the species."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A species specific time during which consumption of animal product is not appropriate."]
    pub r#withdrawal_period:
        Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}
impl serde::ser::Serialize for AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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
            if !self.r#withdrawal_period.is_empty() {
                state.serialize_entry("withdrawalPeriod", &self.r#withdrawal_period)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de>
    for AdministrableProductDefinitionRouteOfAdministrationTargetSpecies
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
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "withdrawalPeriod")]
            WithdrawalPeriod,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdministrableProductDefinitionRouteOfAdministrationTargetSpecies;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter
                    .write_str("AdministrableProductDefinitionRouteOfAdministrationTargetSpecies")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#withdrawal_period : Option < Vec < AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > > = None ;
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
                            Field::WithdrawalPeriod => {
                                if r#withdrawal_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "withdrawalPeriod",
                                    ));
                                }
                                r#withdrawal_period = Some(map_access.next_value()?);
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
                                        "code",
                                        "withdrawalPeriod",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(
                        AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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
                            r#withdrawal_period: r#withdrawal_period.unwrap_or(vec![]),
                        },
                    )
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The path by which the product is taken into or makes contact with the body. In some regions this is referred to as the licenced or approved route. RouteOfAdministration cannot be used when the 'formOf' product already uses MedicinalProductDefinition.route (and vice versa)."]
#[derive(Default, Debug, Clone)]
pub struct AdministrableProductDefinitionRouteOfAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Coded expression for the route."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The first dose (dose quantity) administered can be specified for the product, using a numerical value and its unit of measurement."]
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum single dose that can be administered, specified using a numerical value and its unit of measurement."]
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered."]
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per treatment period that can be administered."]
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "The maximum treatment period during which the product can be administered."]
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    #[doc = "A species for which this route applies."]
    pub r#target_species: Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>,
}
impl serde::ser::Serialize for AdministrableProductDefinitionRouteOfAdministration {
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
            if let Some(some) = self.r#first_dose.as_ref() {
                state.serialize_entry("firstDose", some)?;
            }
            if let Some(some) = self.r#max_single_dose.as_ref() {
                state.serialize_entry("maxSingleDose", some)?;
            }
            if let Some(some) = self.r#max_dose_per_day.as_ref() {
                state.serialize_entry("maxDosePerDay", some)?;
            }
            if let Some(some) = self.r#max_dose_per_treatment_period.as_ref() {
                state.serialize_entry("maxDosePerTreatmentPeriod", some)?;
            }
            if let Some(some) = self.r#max_treatment_period.as_ref() {
                state.serialize_entry("maxTreatmentPeriod", some)?;
            }
            if !self.r#target_species.is_empty() {
                state.serialize_entry("targetSpecies", &self.r#target_species)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdministrableProductDefinitionRouteOfAdministration {
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
            #[serde(rename = "firstDose")]
            FirstDose,
            #[serde(rename = "maxSingleDose")]
            MaxSingleDose,
            #[serde(rename = "maxDosePerDay")]
            MaxDosePerDay,
            #[serde(rename = "maxDosePerTreatmentPeriod")]
            MaxDosePerTreatmentPeriod,
            #[serde(rename = "maxTreatmentPeriod")]
            MaxTreatmentPeriod,
            #[serde(rename = "targetSpecies")]
            TargetSpecies,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdministrableProductDefinitionRouteOfAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinitionRouteOfAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdministrableProductDefinitionRouteOfAdministration, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#first_dose: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_single_dose: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_dose_per_day: Option<Box<super::super::types::Quantity>> = None;
                let mut r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>> =
                    None;
                let mut r#max_treatment_period: Option<Box<super::super::types::Duration>> = None;
                let mut r#target_species: Option<
                    Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>,
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
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::FirstDose => {
                                if r#first_dose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("firstDose"));
                                }
                                r#first_dose = Some(map_access.next_value()?);
                            }
                            Field::MaxSingleDose => {
                                if r#max_single_dose.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxSingleDose"));
                                }
                                r#max_single_dose = Some(map_access.next_value()?);
                            }
                            Field::MaxDosePerDay => {
                                if r#max_dose_per_day.is_some() {
                                    return Err(serde::de::Error::duplicate_field("maxDosePerDay"));
                                }
                                r#max_dose_per_day = Some(map_access.next_value()?);
                            }
                            Field::MaxDosePerTreatmentPeriod => {
                                if r#max_dose_per_treatment_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxDosePerTreatmentPeriod",
                                    ));
                                }
                                r#max_dose_per_treatment_period = Some(map_access.next_value()?);
                            }
                            Field::MaxTreatmentPeriod => {
                                if r#max_treatment_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "maxTreatmentPeriod",
                                    ));
                                }
                                r#max_treatment_period = Some(map_access.next_value()?);
                            }
                            Field::TargetSpecies => {
                                if r#target_species.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetSpecies"));
                                }
                                r#target_species = Some(map_access.next_value()?);
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
                                        "code",
                                        "firstDose",
                                        "maxSingleDose",
                                        "maxDosePerDay",
                                        "maxDosePerTreatmentPeriod",
                                        "maxTreatmentPeriod",
                                        "targetSpecies",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdministrableProductDefinitionRouteOfAdministration {
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
                        r#first_dose,
                        r#max_single_dose,
                        r#max_dose_per_day,
                        r#max_dose_per_treatment_period,
                        r#max_treatment_period,
                        r#target_species: r#target_species.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed)."]
#[derive(Default, Debug, Clone)]
pub struct AdministrableProductDefinition {
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
    #[doc = "An identifier for the administrable product."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of this administrable product. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product.  If this administrable product describes the administration of a crushed tablet, the 'formOf' would be the product representing a distribution containing tablets and possibly also a cream.  This is distinct from the 'producedFrom' which refers to the specific components of the product that are used in this preparation, rather than the product as a whole."]
    pub r#form_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "The dose form of the final product after necessary reconstitution or processing. Contrasts to the manufactured dose form (see ManufacturedItemDefinition). If the manufactured form was 'powder for solution for injection', the administrable dose form could be 'solution for injection' (once mixed with another item having manufactured form 'solvent for solution for injection')."]
    pub r#administrable_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The presentation type in which this item is given to a patient. e.g. for a spray - 'puff' (as in 'contains 100 mcg per puff'), or for a liquid - 'vial' (as in 'contains 5 ml per vial')."]
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form.  In some cases, an administrable form might use all of the items from the overall product (or there might only be one item), while in other cases, an administrable form might use only a subset of the items available in the overall product.  For example, an administrable form might involve combining a liquid and a powder available as part of an overall product, but not involve applying the also supplied cream."]
    pub r#produced_from: Vec<Box<super::super::types::Reference>>,
    #[doc = "The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton (via AdministrableProductDefinition.producedFrom) to state which component items are used to make this, or using by incoming references from the Ingredient resource, to state in detail which substances exist within this. This element allows a basic coded ingredient to be used."]
    pub r#ingredient: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A device that is integral to the medicinal product, in effect being considered as an \"ingredient\" of the medicinal product. This is not intended for devices that are just co-packaged."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Characteristics e.g. a product's onset of action."]
    pub r#property: Vec<AdministrableProductDefinitionProperty>,
    #[doc = "The path by which the product is taken into or makes contact with the body. In some regions this is referred to as the licenced or approved route. RouteOfAdministration cannot be used when the 'formOf' product already uses MedicinalProductDefinition.route (and vice versa)."]
    pub r#route_of_administration: Vec<AdministrableProductDefinitionRouteOfAdministration>,
}
impl crate::AnyResource for AdministrableProductDefinition {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4B
    }
}
impl serde::ser::Serialize for AdministrableProductDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "AdministrableProductDefinition")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
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
            if !self.r#form_of.is_empty() {
                state.serialize_entry("formOf", &self.r#form_of)?;
            }
            if let Some(some) = self.r#administrable_dose_form.as_ref() {
                state.serialize_entry("administrableDoseForm", some)?;
            }
            if let Some(some) = self.r#unit_of_presentation.as_ref() {
                state.serialize_entry("unitOfPresentation", some)?;
            }
            if !self.r#produced_from.is_empty() {
                state.serialize_entry("producedFrom", &self.r#produced_from)?;
            }
            if !self.r#ingredient.is_empty() {
                state.serialize_entry("ingredient", &self.r#ingredient)?;
            }
            if let Some(some) = self.r#device.as_ref() {
                state.serialize_entry("device", some)?;
            }
            if !self.r#property.is_empty() {
                state.serialize_entry("property", &self.r#property)?;
            }
            if !self.r#route_of_administration.is_empty() {
                state.serialize_entry("routeOfAdministration", &self.r#route_of_administration)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for AdministrableProductDefinition {
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
            #[serde(rename = "formOf")]
            FormOf,
            #[serde(rename = "administrableDoseForm")]
            AdministrableDoseForm,
            #[serde(rename = "unitOfPresentation")]
            UnitOfPresentation,
            #[serde(rename = "producedFrom")]
            ProducedFrom,
            #[serde(rename = "ingredient")]
            Ingredient,
            #[serde(rename = "device")]
            Device,
            #[serde(rename = "property")]
            Property,
            #[serde(rename = "routeOfAdministration")]
            RouteOfAdministration,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = AdministrableProductDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<AdministrableProductDefinition, V::Error>
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
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#form_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#administrable_dose_form: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#produced_from: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#ingredient: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#device: Option<Box<super::super::types::Reference>> = None;
                let mut r#property: Option<Vec<AdministrableProductDefinitionProperty>> = None;
                let mut r#route_of_administration: Option<
                    Vec<AdministrableProductDefinitionRouteOfAdministration>,
                > = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "AdministrableProductDefinition" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"AdministrableProductDefinition",
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
                                            "formOf",
                                            "administrableDoseForm",
                                            "unitOfPresentation",
                                            "producedFrom",
                                            "ingredient",
                                            "device",
                                            "property",
                                            "routeOfAdministration",
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
                                            "formOf",
                                            "administrableDoseForm",
                                            "unitOfPresentation",
                                            "producedFrom",
                                            "ingredient",
                                            "device",
                                            "property",
                                            "routeOfAdministration",
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
                                            "formOf",
                                            "administrableDoseForm",
                                            "unitOfPresentation",
                                            "producedFrom",
                                            "ingredient",
                                            "device",
                                            "property",
                                            "routeOfAdministration",
                                        ],
                                    ));
                                }
                            }
                            Field::FormOf => {
                                if r#form_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("formOf"));
                                }
                                r#form_of = Some(map_access.next_value()?);
                            }
                            Field::AdministrableDoseForm => {
                                if r#administrable_dose_form.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administrableDoseForm",
                                    ));
                                }
                                r#administrable_dose_form = Some(map_access.next_value()?);
                            }
                            Field::UnitOfPresentation => {
                                if r#unit_of_presentation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "unitOfPresentation",
                                    ));
                                }
                                r#unit_of_presentation = Some(map_access.next_value()?);
                            }
                            Field::ProducedFrom => {
                                if r#produced_from.is_some() {
                                    return Err(serde::de::Error::duplicate_field("producedFrom"));
                                }
                                r#produced_from = Some(map_access.next_value()?);
                            }
                            Field::Ingredient => {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some(map_access.next_value()?);
                            }
                            Field::Device => {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                r#device = Some(map_access.next_value()?);
                            }
                            Field::Property => {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some(map_access.next_value()?);
                            }
                            Field::RouteOfAdministration => {
                                if r#route_of_administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "routeOfAdministration",
                                    ));
                                }
                                r#route_of_administration = Some(map_access.next_value()?);
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
                                        "formOf",
                                        "administrableDoseForm",
                                        "unitOfPresentation",
                                        "producedFrom",
                                        "ingredient",
                                        "device",
                                        "property",
                                        "routeOfAdministration",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(AdministrableProductDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#form_of: r#form_of.unwrap_or(vec![]),
                        r#administrable_dose_form,
                        r#unit_of_presentation,
                        r#produced_from: r#produced_from.unwrap_or(vec![]),
                        r#ingredient: r#ingredient.unwrap_or(vec![]),
                        r#device,
                        r#property: r#property.unwrap_or(vec![]),
                        r#route_of_administration: r#route_of_administration.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
