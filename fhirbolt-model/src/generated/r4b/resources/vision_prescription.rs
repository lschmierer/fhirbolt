// Generated on 2023-03-28 by fhirbolt-codegen v0.1.0
#[doc = "Allows for adjustment on two axis."]
#[derive(Default, Debug, Clone)]
pub struct VisionPrescriptionLensSpecificationPrism {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Amount of prism to compensate for eye alignment in fractional units."]
    pub r#amount: super::super::types::Decimal,
    #[doc = "The relative base, or reference lens edge, for the prism."]
    pub r#base: super::super::types::Code,
}
impl serde::ser::Serialize for VisionPrescriptionLensSpecificationPrism {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
                if let Some(some) = self.r#amount.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("amount", &some)?;
                }
                if self.r#amount.id.is_some() || !self.r#amount.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#amount.id.as_ref(),
                        extension: &self.r#amount.extension,
                    };
                    state.serialize_entry("_amount", &primitive_element)?;
                }
            } else {
                state.serialize_entry("amount", &self.r#amount)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#base.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("base", &some)?;
                }
                if self.r#base.id.is_some() || !self.r#base.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#base.id.as_ref(),
                        extension: &self.r#base.extension,
                    };
                    state.serialize_entry("_base", &primitive_element)?;
                }
            } else {
                state.serialize_entry("base", &self.r#base)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescriptionLensSpecificationPrism {
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
            #[serde(rename = "amount")]
            Amount,
            #[serde(rename = "_amount")]
            AmountPrimitiveElement,
            #[serde(rename = "base")]
            Base,
            #[serde(rename = "_base")]
            BasePrimitiveElement,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisionPrescriptionLensSpecificationPrism;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescriptionLensSpecificationPrism")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VisionPrescriptionLensSpecificationPrism, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#amount: Option<super::super::types::Decimal> = None;
                let mut r#base: Option<super::super::types::Code> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
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
                            Field::Amount => {
                                if _ctx.from_json {
                                    let some = r#amount.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("amount"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#amount.is_some() {
                                        return Err(serde::de::Error::duplicate_field("amount"));
                                    }
                                    r#amount = Some(map_access.next_value()?);
                                }
                            }
                            Field::AmountPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#amount.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_amount"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "amount",
                                        &["id", "extension", "modifierExtension", "amount", "base"],
                                    ));
                                }
                            }
                            Field::Base => {
                                if _ctx.from_json {
                                    let some = r#base.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("base"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#base.is_some() {
                                        return Err(serde::de::Error::duplicate_field("base"));
                                    }
                                    r#base = Some(map_access.next_value()?);
                                }
                            }
                            Field::BasePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#base.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_base"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "base",
                                        &["id", "extension", "modifierExtension", "amount", "base"],
                                    ));
                                }
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "amount", "base"],
                                ));
                            },
                        }
                    }
                    Ok(VisionPrescriptionLensSpecificationPrism {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#amount: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#amount.unwrap_or(Default::default())
                        } else {
                            r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                        },
                        r#base: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#base.unwrap_or(Default::default())
                        } else {
                            r#base.ok_or(serde::de::Error::missing_field("base"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Contain the details of  the individual lens specifications and serves as the authorization for the fullfillment by certified professionals."]
#[derive(Default, Debug, Clone)]
pub struct VisionPrescriptionLensSpecification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the type of vision correction product which is required for the patient."]
    pub r#product: Box<super::super::types::CodeableConcept>,
    #[doc = "The eye for which the lens specification applies."]
    pub r#eye: super::super::types::Code,
    #[doc = "Lens power measured in dioptres (0.25 units)."]
    pub r#sphere: Option<super::super::types::Decimal>,
    #[doc = "Power adjustment for astigmatism measured in dioptres (0.25 units)."]
    pub r#cylinder: Option<super::super::types::Decimal>,
    #[doc = "Adjustment for astigmatism measured in integer degrees."]
    pub r#axis: Option<super::super::types::Integer>,
    #[doc = "Allows for adjustment on two axis."]
    pub r#prism: Vec<VisionPrescriptionLensSpecificationPrism>,
    #[doc = "Power adjustment for multifocal lenses measured in dioptres (0.25 units)."]
    pub r#add: Option<super::super::types::Decimal>,
    #[doc = "Contact lens power measured in dioptres (0.25 units)."]
    pub r#power: Option<super::super::types::Decimal>,
    #[doc = "Back curvature measured in millimetres."]
    pub r#back_curve: Option<super::super::types::Decimal>,
    #[doc = "Contact lens diameter measured in millimetres."]
    pub r#diameter: Option<super::super::types::Decimal>,
    #[doc = "The recommended maximum wear period for the lens."]
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Special color or pattern."]
    pub r#color: Option<super::super::types::String>,
    #[doc = "Brand recommendations or restrictions."]
    pub r#brand: Option<super::super::types::String>,
    #[doc = "Notes for special requirements such as coatings and lens materials."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for VisionPrescriptionLensSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
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
            state.serialize_entry("product", &self.r#product)?;
            if _ctx.output_json {
                if let Some(some) = self.r#eye.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("eye", &some)?;
                }
                if self.r#eye.id.is_some() || !self.r#eye.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#eye.id.as_ref(),
                        extension: &self.r#eye.extension,
                    };
                    state.serialize_entry("_eye", &primitive_element)?;
                }
            } else {
                state.serialize_entry("eye", &self.r#eye)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#sphere.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("sphere", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_sphere", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#sphere.as_ref() {
                    state.serialize_entry("sphere", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#cylinder.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("cylinder", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_cylinder", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#cylinder.as_ref() {
                    state.serialize_entry("cylinder", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#axis.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("axis", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_axis", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#axis.as_ref() {
                    state.serialize_entry("axis", some)?;
                }
            }
            if !self.r#prism.is_empty() {
                state.serialize_entry("prism", &self.r#prism)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#add.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("add", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_add", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#add.as_ref() {
                    state.serialize_entry("add", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#power.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("power", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_power", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#power.as_ref() {
                    state.serialize_entry("power", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#back_curve.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("backCurve", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_backCurve", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#back_curve.as_ref() {
                    state.serialize_entry("backCurve", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#diameter.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("diameter", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_diameter", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#diameter.as_ref() {
                    state.serialize_entry("diameter", some)?;
                }
            }
            if let Some(some) = self.r#duration.as_ref() {
                state.serialize_entry("duration", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#color.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("color", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_color", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#color.as_ref() {
                    state.serialize_entry("color", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#brand.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("brand", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_brand", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#brand.as_ref() {
                    state.serialize_entry("brand", some)?;
                }
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescriptionLensSpecification {
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
            #[serde(rename = "product")]
            Product,
            #[serde(rename = "eye")]
            Eye,
            #[serde(rename = "_eye")]
            EyePrimitiveElement,
            #[serde(rename = "sphere")]
            Sphere,
            #[serde(rename = "_sphere")]
            SpherePrimitiveElement,
            #[serde(rename = "cylinder")]
            Cylinder,
            #[serde(rename = "_cylinder")]
            CylinderPrimitiveElement,
            #[serde(rename = "axis")]
            Axis,
            #[serde(rename = "_axis")]
            AxisPrimitiveElement,
            #[serde(rename = "prism")]
            Prism,
            #[serde(rename = "add")]
            Add,
            #[serde(rename = "_add")]
            AddPrimitiveElement,
            #[serde(rename = "power")]
            Power,
            #[serde(rename = "_power")]
            PowerPrimitiveElement,
            #[serde(rename = "backCurve")]
            BackCurve,
            #[serde(rename = "_backCurve")]
            BackCurvePrimitiveElement,
            #[serde(rename = "diameter")]
            Diameter,
            #[serde(rename = "_diameter")]
            DiameterPrimitiveElement,
            #[serde(rename = "duration")]
            Duration,
            #[serde(rename = "color")]
            Color,
            #[serde(rename = "_color")]
            ColorPrimitiveElement,
            #[serde(rename = "brand")]
            Brand,
            #[serde(rename = "_brand")]
            BrandPrimitiveElement,
            #[serde(rename = "note")]
            Note,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisionPrescriptionLensSpecification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescriptionLensSpecification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VisionPrescriptionLensSpecification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#eye: Option<super::super::types::Code> = None;
                let mut r#sphere: Option<super::super::types::Decimal> = None;
                let mut r#cylinder: Option<super::super::types::Decimal> = None;
                let mut r#axis: Option<super::super::types::Integer> = None;
                let mut r#prism: Option<Vec<VisionPrescriptionLensSpecificationPrism>> = None;
                let mut r#add: Option<super::super::types::Decimal> = None;
                let mut r#power: Option<super::super::types::Decimal> = None;
                let mut r#back_curve: Option<super::super::types::Decimal> = None;
                let mut r#diameter: Option<super::super::types::Decimal> = None;
                let mut r#duration: Option<Box<super::super::types::Quantity>> = None;
                let mut r#color: Option<super::super::types::String> = None;
                let mut r#brand: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
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
                            Field::Product => {
                                if r#product.is_some() {
                                    return Err(serde::de::Error::duplicate_field("product"));
                                }
                                r#product = Some(map_access.next_value()?);
                            }
                            Field::Eye => {
                                if _ctx.from_json {
                                    let some = r#eye.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("eye"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#eye.is_some() {
                                        return Err(serde::de::Error::duplicate_field("eye"));
                                    }
                                    r#eye = Some(map_access.next_value()?);
                                }
                            }
                            Field::EyePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#eye.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_eye"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "eye",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Sphere => {
                                if _ctx.from_json {
                                    let some = r#sphere.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sphere"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#sphere.is_some() {
                                        return Err(serde::de::Error::duplicate_field("sphere"));
                                    }
                                    r#sphere = Some(map_access.next_value()?);
                                }
                            }
                            Field::SpherePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#sphere.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_sphere"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "sphere",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Cylinder => {
                                if _ctx.from_json {
                                    let some = r#cylinder.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("cylinder"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#cylinder.is_some() {
                                        return Err(serde::de::Error::duplicate_field("cylinder"));
                                    }
                                    r#cylinder = Some(map_access.next_value()?);
                                }
                            }
                            Field::CylinderPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#cylinder.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_cylinder"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "cylinder",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Axis => {
                                if _ctx.from_json {
                                    let some = r#axis.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("axis"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#axis.is_some() {
                                        return Err(serde::de::Error::duplicate_field("axis"));
                                    }
                                    r#axis = Some(map_access.next_value()?);
                                }
                            }
                            Field::AxisPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#axis.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_axis"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "axis",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Prism => {
                                if r#prism.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prism"));
                                }
                                r#prism = Some(map_access.next_value()?);
                            }
                            Field::Add => {
                                if _ctx.from_json {
                                    let some = r#add.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("add"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#add.is_some() {
                                        return Err(serde::de::Error::duplicate_field("add"));
                                    }
                                    r#add = Some(map_access.next_value()?);
                                }
                            }
                            Field::AddPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#add.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_add"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "add",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Power => {
                                if _ctx.from_json {
                                    let some = r#power.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("power"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#power.is_some() {
                                        return Err(serde::de::Error::duplicate_field("power"));
                                    }
                                    r#power = Some(map_access.next_value()?);
                                }
                            }
                            Field::PowerPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#power.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_power"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "power",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::BackCurve => {
                                if _ctx.from_json {
                                    let some = r#back_curve.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("backCurve"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#back_curve.is_some() {
                                        return Err(serde::de::Error::duplicate_field("backCurve"));
                                    }
                                    r#back_curve = Some(map_access.next_value()?);
                                }
                            }
                            Field::BackCurvePrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#back_curve.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_backCurve",
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
                                        "backCurve",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Diameter => {
                                if _ctx.from_json {
                                    let some = r#diameter.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("diameter"));
                                    }
                                    let value: serde_json::Number = map_access.next_value()?;
                                    some.value = Some(format!("{}", value));
                                } else {
                                    if r#diameter.is_some() {
                                        return Err(serde::de::Error::duplicate_field("diameter"));
                                    }
                                    r#diameter = Some(map_access.next_value()?);
                                }
                            }
                            Field::DiameterPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#diameter.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_diameter"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "diameter",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Duration => {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                r#duration = Some(map_access.next_value()?);
                            }
                            Field::Color => {
                                if _ctx.from_json {
                                    let some = r#color.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("color"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#color.is_some() {
                                        return Err(serde::de::Error::duplicate_field("color"));
                                    }
                                    r#color = Some(map_access.next_value()?);
                                }
                            }
                            Field::ColorPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#color.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_color"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "color",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Brand => {
                                if _ctx.from_json {
                                    let some = r#brand.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("brand"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#brand.is_some() {
                                        return Err(serde::de::Error::duplicate_field("brand"));
                                    }
                                    r#brand = Some(map_access.next_value()?);
                                }
                            }
                            Field::BrandPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#brand.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_brand"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "brand",
                                        &[
                                            "id",
                                            "extension",
                                            "modifierExtension",
                                            "product",
                                            "eye",
                                            "sphere",
                                            "cylinder",
                                            "axis",
                                            "prism",
                                            "add",
                                            "power",
                                            "backCurve",
                                            "diameter",
                                            "duration",
                                            "color",
                                            "brand",
                                            "note",
                                        ],
                                    ));
                                }
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
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
                                        "product",
                                        "eye",
                                        "sphere",
                                        "cylinder",
                                        "axis",
                                        "prism",
                                        "add",
                                        "power",
                                        "backCurve",
                                        "diameter",
                                        "duration",
                                        "color",
                                        "brand",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VisionPrescriptionLensSpecification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#product: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#product.unwrap_or(Default::default())
                        } else {
                            r#product.ok_or(serde::de::Error::missing_field("product"))?
                        },
                        r#eye: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#eye.unwrap_or(Default::default())
                        } else {
                            r#eye.ok_or(serde::de::Error::missing_field("eye"))?
                        },
                        r#sphere,
                        r#cylinder,
                        r#axis,
                        r#prism: r#prism.unwrap_or(vec![]),
                        r#add,
                        r#power,
                        r#back_curve,
                        r#diameter,
                        r#duration,
                        r#color,
                        r#brand,
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "An authorization for the provision of glasses and/or contact lenses to a patient."]
#[derive(Default, Debug, Clone)]
pub struct VisionPrescription {
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
    #[doc = "A unique identifier assigned to this vision prescription."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "A resource reference to the person to whom the vision prescription applies."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "A reference to a resource that identifies the particular occurrence of contact between patient and health care provider during which the prescription was issued."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the prescription was written."]
    pub r#date_written: super::super::types::DateTime,
    #[doc = "The healthcare professional responsible for authorizing the prescription."]
    pub r#prescriber: Box<super::super::types::Reference>,
    #[doc = "Contain the details of  the individual lens specifications and serves as the authorization for the fullfillment by certified professionals."]
    pub r#lens_specification: Vec<VisionPrescriptionLensSpecification>,
}
impl crate::AnyResource for VisionPrescription {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4B;
}
impl serde::ser::Serialize for VisionPrescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "VisionPrescription")?;
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
            if _ctx.output_json {
                if let Some(some) = self.r#created.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("created", &some)?;
                }
                if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#created.id.as_ref(),
                        extension: &self.r#created.extension,
                    };
                    state.serialize_entry("_created", &primitive_element)?;
                }
            } else {
                state.serialize_entry("created", &self.r#created)?;
            }
            state.serialize_entry("patient", &self.r#patient)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date_written.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("dateWritten", &some)?;
                }
                if self.r#date_written.id.is_some() || !self.r#date_written.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#date_written.id.as_ref(),
                        extension: &self.r#date_written.extension,
                    };
                    state.serialize_entry("_dateWritten", &primitive_element)?;
                }
            } else {
                state.serialize_entry("dateWritten", &self.r#date_written)?;
            }
            state.serialize_entry("prescriber", &self.r#prescriber)?;
            if !self.r#lens_specification.is_empty() {
                state.serialize_entry("lensSpecification", &self.r#lens_specification)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescription {
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
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "dateWritten")]
            DateWritten,
            #[serde(rename = "_dateWritten")]
            DateWrittenPrimitiveElement,
            #[serde(rename = "prescriber")]
            Prescriber,
            #[serde(rename = "lensSpecification")]
            LensSpecification,
            Unknown(std::string::String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisionPrescription;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescription")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<VisionPrescription, V::Error>
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
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#date_written: Option<super::super::types::DateTime> = None;
                let mut r#prescriber: Option<Box<super::super::types::Reference>> = None;
                let mut r#lens_specification: Option<Vec<VisionPrescriptionLensSpecification>> =
                    None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.borrow();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "VisionPrescription" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"VisionPrescription",
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
                                            "created",
                                            "patient",
                                            "encounter",
                                            "dateWritten",
                                            "prescriber",
                                            "lensSpecification",
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
                                            "created",
                                            "patient",
                                            "encounter",
                                            "dateWritten",
                                            "prescriber",
                                            "lensSpecification",
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
                                            "created",
                                            "patient",
                                            "encounter",
                                            "dateWritten",
                                            "prescriber",
                                            "lensSpecification",
                                        ],
                                    ));
                                }
                            }
                            Field::Created => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#created.is_some() {
                                        return Err(serde::de::Error::duplicate_field("created"));
                                    }
                                    r#created = Some(map_access.next_value()?);
                                }
                            }
                            Field::CreatedPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#created.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field("_created"));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    some.id = id;
                                    some.extension = extension;
                                } else {
                                    return Err(serde::de::Error::unknown_field(
                                        "created",
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
                                            "created",
                                            "patient",
                                            "encounter",
                                            "dateWritten",
                                            "prescriber",
                                            "lensSpecification",
                                        ],
                                    ));
                                }
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::DateWritten => {
                                if _ctx.from_json {
                                    let some = r#date_written.get_or_insert(Default::default());
                                    if some.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dateWritten",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    some.value = Some(value);
                                } else {
                                    if r#date_written.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "dateWritten",
                                        ));
                                    }
                                    r#date_written = Some(map_access.next_value()?);
                                }
                            }
                            Field::DateWrittenPrimitiveElement => {
                                if _ctx.from_json {
                                    let some = r#date_written.get_or_insert(Default::default());
                                    if some.id.is_some() || !some.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_dateWritten",
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
                                        "dateWritten",
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
                                            "created",
                                            "patient",
                                            "encounter",
                                            "dateWritten",
                                            "prescriber",
                                            "lensSpecification",
                                        ],
                                    ));
                                }
                            }
                            Field::Prescriber => {
                                if r#prescriber.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prescriber"));
                                }
                                r#prescriber = Some(map_access.next_value()?);
                            }
                            Field::LensSpecification => {
                                if r#lens_specification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lensSpecification",
                                    ));
                                }
                                r#lens_specification = Some(map_access.next_value()?);
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
                                        "created",
                                        "patient",
                                        "encounter",
                                        "dateWritten",
                                        "prescriber",
                                        "lensSpecification",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(VisionPrescription {
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
                        r#created: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#patient: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#encounter,
                        r#date_written: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#date_written.unwrap_or(Default::default())
                        } else {
                            r#date_written.ok_or(serde::de::Error::missing_field("dateWritten"))?
                        },
                        r#prescriber: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#prescriber.unwrap_or(Default::default())
                        } else {
                            r#prescriber.ok_or(serde::de::Error::missing_field("prescriber"))?
                        },
                        r#lens_specification: r#lens_specification.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
