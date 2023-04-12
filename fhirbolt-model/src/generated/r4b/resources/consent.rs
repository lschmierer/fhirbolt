// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
#[doc = "The source on which this consent statement is based. The source might be a scanned original paper form, or a reference to a consent that links back to such a source, a reference to a document repository (e.g. XDS) that stores the original consent document."]
#[derive(Debug, Clone, PartialEq)]
pub enum ConsentSource {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for ConsentSource {
    fn default() -> ConsentSource {
        ConsentSource::Invalid
    }
}
#[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConsentPolicy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Entity or Organization having regulatory jurisdiction or accountability for  enforcing policies pertaining to Consent Directives."]
    pub r#authority: Option<super::super::types::Uri>,
    #[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
    pub r#uri: Option<super::super::types::Uri>,
}
impl serde::ser::Serialize for ConsentPolicy {
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
                if let Some(some) = self.r#authority.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("authority", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_authority", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#authority.as_ref() {
                    state.serialize_entry("authority", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#uri.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("uri", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_uri", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#uri.as_ref() {
                    state.serialize_entry("uri", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Whether a treatment instruction (e.g. artificial respiration yes or no) was verified with the patient, his/her family or another authorized person."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConsentVerification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Has the instruction been verified."]
    pub r#verified: super::super::types::Boolean,
    #[doc = "Who verified the instruction (Patient, Relative or other Authorized Person)."]
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    #[doc = "Date verification was collected."]
    pub r#verification_date: Option<super::super::types::DateTime>,
}
impl serde::ser::Serialize for ConsentVerification {
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
                if let Some(some) = self.r#verified.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("verified", &some)?;
                }
                if self.r#verified.id.is_some() || !self.r#verified.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#verified.id.as_ref(),
                        extension: &self.r#verified.extension,
                    };
                    state.serialize_entry("_verified", &primitive_element)?;
                }
            } else {
                state.serialize_entry("verified", &self.r#verified)?;
            }
            if let Some(some) = self.r#verified_with.as_ref() {
                state.serialize_entry("verifiedWith", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#verification_date.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("verificationDate", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_verificationDate", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#verification_date.as_ref() {
                    state.serialize_entry("verificationDate", some)?;
                }
            }
            state.end()
        })
    }
}
#[doc = "Who or what is controlled by this rule. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConsentProvisionActor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the individual is involved in the resources content that is described in the exception."]
    pub r#role: Box<super::super::types::CodeableConcept>,
    #[doc = "The resource that identifies the actor. To identify actors by type, use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#reference: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ConsentProvisionActor {
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
            state.serialize_entry("role", &self.r#role)?;
            state.serialize_entry("reference", &self.r#reference)?;
            state.end()
        })
    }
}
#[doc = "The resources controlled by this rule if specific resources are referenced."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConsentProvisionData {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "How the resource reference is interpreted when testing consent restrictions."]
    pub r#meaning: super::super::types::Code,
    #[doc = "A reference to a specific resource that defines which resources are covered by this consent."]
    pub r#reference: Box<super::super::types::Reference>,
}
impl serde::ser::Serialize for ConsentProvisionData {
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
                if let Some(some) = self.r#meaning.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("meaning", &some)?;
                }
                if self.r#meaning.id.is_some() || !self.r#meaning.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#meaning.id.as_ref(),
                        extension: &self.r#meaning.extension,
                    };
                    state.serialize_entry("_meaning", &primitive_element)?;
                }
            } else {
                state.serialize_entry("meaning", &self.r#meaning)?;
            }
            state.serialize_entry("reference", &self.r#reference)?;
            state.end()
        })
    }
}
#[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ConsentProvision {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Action  to take - permit or deny - when the rule conditions are met.  Not permitted in root rule, required in all nested rules."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "The timeframe in this rule is valid."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Who or what is controlled by this rule. Use group to identify a set of actors by some property they share (e.g. 'admitting officers')."]
    pub r#actor: Vec<ConsentProvisionActor>,
    #[doc = "Actions controlled by this Rule."]
    pub r#action: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A security label, comprised of 0..* security label fields (Privacy tags), which define which resources are controlled by this exception."]
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    #[doc = "The context of the activities a user is taking - why the user is accessing the data - that are controlled by this rule."]
    pub r#purpose: Vec<Box<super::super::types::Coding>>,
    #[doc = "The class of information covered by this rule. The type can be a FHIR resource type, a profile on a type, or a CDA document, or some other type that indicates what sort of information the consent relates to."]
    pub r#class: Vec<Box<super::super::types::Coding>>,
    #[doc = "If this code is found in an instance, then the rule applies."]
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Clinical or Operational Relevant period of time that bounds the data controlled by this rule."]
    pub r#data_period: Option<Box<super::super::types::Period>>,
    #[doc = "The resources controlled by this rule if specific resources are referenced."]
    pub r#data: Vec<ConsentProvisionData>,
    #[doc = "Rules which provide exceptions to the base rule or subrules."]
    pub r#provision: Vec<ConsentProvision>,
}
impl serde::ser::Serialize for ConsentProvision {
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
                if let Some(some) = self.r#type.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("type", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_type", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#type.as_ref() {
                    state.serialize_entry("type", some)?;
                }
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if !self.r#actor.is_empty() {
                state.serialize_entry("actor", &self.r#actor)?;
            }
            if !self.r#action.is_empty() {
                state.serialize_entry("action", &self.r#action)?;
            }
            if !self.r#security_label.is_empty() {
                state.serialize_entry("securityLabel", &self.r#security_label)?;
            }
            if !self.r#purpose.is_empty() {
                state.serialize_entry("purpose", &self.r#purpose)?;
            }
            if !self.r#class.is_empty() {
                state.serialize_entry("class", &self.r#class)?;
            }
            if !self.r#code.is_empty() {
                state.serialize_entry("code", &self.r#code)?;
            }
            if let Some(some) = self.r#data_period.as_ref() {
                state.serialize_entry("dataPeriod", some)?;
            }
            if !self.r#data.is_empty() {
                state.serialize_entry("data", &self.r#data)?;
            }
            if !self.r#provision.is_empty() {
                state.serialize_entry("provision", &self.r#provision)?;
            }
            state.end()
        })
    }
}
#[doc = "A record of a healthcare consumer’s  choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Consent {
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
    #[doc = "Unique identifier for this copy of the Consent Statement."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Indicates the current state of this consent."]
    pub r#status: super::super::types::Code,
    #[doc = "A selector of the type of consent being presented: ADR, Privacy, Treatment, Research.  This list is now extensible."]
    pub r#scope: Box<super::super::types::CodeableConcept>,
    #[doc = "A classification of the type of consents found in the statement. This element supports indexing and retrieval of consent statements."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient/healthcare consumer to whom this consent applies."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "When this  Consent was issued / created / indexed."]
    pub r#date_time: Option<super::super::types::DateTime>,
    #[doc = "Either the Grantor, which is the entity responsible for granting the rights listed in a Consent Directive or the Grantee, which is the entity responsible for complying with the Consent Directive, including any obligations or limitations on authorizations and enforcement of prohibitions."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "The organization that manages the consent, and the framework within which it is executed."]
    pub r#organization: Vec<Box<super::super::types::Reference>>,
    #[doc = "The source on which this consent statement is based. The source might be a scanned original paper form, or a reference to a consent that links back to such a source, a reference to a document repository (e.g. XDS) that stores the original consent document."]
    pub r#source: Option<ConsentSource>,
    #[doc = "The references to the policies that are included in this consent scope. Policies may be organizational, but are often defined jurisdictionally, or in law."]
    pub r#policy: Vec<ConsentPolicy>,
    #[doc = "A reference to the specific base computable regulation or policy."]
    pub r#policy_rule: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether a treatment instruction (e.g. artificial respiration yes or no) was verified with the patient, his/her family or another authorized person."]
    pub r#verification: Vec<ConsentVerification>,
    #[doc = "An exception to the base policy of this consent. An exception can be an addition or removal of access permissions."]
    pub r#provision: Option<ConsentProvision>,
}
impl serde::ser::Serialize for Consent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "Consent")?;
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
            state.serialize_entry("scope", &self.r#scope)?;
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if let Some(some) = self.r#patient.as_ref() {
                state.serialize_entry("patient", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#date_time.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("dateTime", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_dateTime", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#date_time.as_ref() {
                    state.serialize_entry("dateTime", some)?;
                }
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if !self.r#organization.is_empty() {
                state.serialize_entry("organization", &self.r#organization)?;
            }
            if let Some(some) = self.r#source.as_ref() {
                match some {
                    ConsentSource::Attachment(ref value) => {
                        state.serialize_entry("sourceAttachment", value)?;
                    }
                    ConsentSource::Reference(ref value) => {
                        state.serialize_entry("sourceReference", value)?;
                    }
                    ConsentSource::Invalid => {
                        return Err(serde::ser::Error::custom("source is invalid"))
                    }
                }
            }
            if !self.r#policy.is_empty() {
                state.serialize_entry("policy", &self.r#policy)?;
            }
            if let Some(some) = self.r#policy_rule.as_ref() {
                state.serialize_entry("policyRule", some)?;
            }
            if !self.r#verification.is_empty() {
                state.serialize_entry("verification", &self.r#verification)?;
            }
            if let Some(some) = self.r#provision.as_ref() {
                state.serialize_entry("provision", some)?;
            }
            state.end()
        })
    }
}
