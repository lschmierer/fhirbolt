// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "Allows for adjustment on two axis."]
#[derive(Default, Debug, Clone, PartialEq)]
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
#[doc = "Contain the details of  the individual lens specifications and serves as the authorization for the fullfillment by certified professionals."]
#[derive(Default, Debug, Clone, PartialEq)]
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
#[doc = "An authorization for the provision of glasses and/or contact lenses to a patient."]
#[derive(Default, Debug, Clone, PartialEq)]
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
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirRelease::R4;
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
