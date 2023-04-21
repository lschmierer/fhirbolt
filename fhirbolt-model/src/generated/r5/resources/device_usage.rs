// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "How often the device was used."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DeviceUsageTiming {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
    #[default]
    Invalid,
}
#[doc = "This indicates how or if the device is being used."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUsageAdherence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of adherence."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Reason for adherence type."]
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
}
impl Default for DeviceUsageAdherence {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#reason: Default::default(),
        }
    }
}
#[doc = "A record of a device being used by a patient where the record is the result of a report from the patient or a clinician."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceUsage {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An external identifier for this statement such as an IRI."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this DeviceUsage."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A code representing the patient or other source's judgment about the state of the device used that this statement is about.  Generally this will be active or completed."]
    pub r#status: super::super::types::Code,
    #[doc = "This attribute indicates a category for the statement - The device statement may be made in an inpatient or outpatient settting (inpatient | outpatient | community | patientspecified)."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient who used the device."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "Allows linking the DeviceUsage to the underlying Request, or to other information that supports or is used to derive the DeviceUsage."]
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    #[doc = "The encounter or episode of care that establishes the context for this device use statement."]
    pub r#context: Option<Box<super::super::types::Reference>>,
    #[doc = "How often the device was used."]
    pub r#timing: Option<DeviceUsageTiming>,
    #[doc = "The time at which the statement was recorded by informationSource."]
    pub r#date_asserted: Option<super::super::types::DateTime>,
    #[doc = "The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement."]
    pub r#usage_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reason for asserting the usage status - for example forgot, lost, stolen, broken."]
    pub r#usage_reason: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "This indicates how or if the device is being used."]
    pub r#adherence: Option<DeviceUsageAdherence>,
    #[doc = "Who reported the device was being used by the patient."]
    pub r#information_source: Option<Box<super::super::types::Reference>>,
    #[doc = "Code or Reference to device used."]
    pub r#device: Box<super::super::types::CodeableReference>,
    #[doc = "Reason or justification for the use of the device. A coded concept, or another resource whose existence justifies this DeviceUsage."]
    pub r#reason: Vec<Box<super::super::types::CodeableReference>>,
    #[doc = "Indicates the anotomic location on the subject's body where the device was used ( i.e. the target)."]
    pub r#body_site: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Details about the device statement that were not represented at all or sufficiently in one of the attributes provided in a class. These may include for example a comment, an instruction, or a note associated with the statement."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl Default for DeviceUsage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#based_on: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#category: Default::default(),
            r#patient: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#derived_from: Default::default(),
            r#context: Default::default(),
            r#timing: Default::default(),
            r#date_asserted: Default::default(),
            r#usage_status: Default::default(),
            r#usage_reason: Default::default(),
            r#adherence: Default::default(),
            r#information_source: Default::default(),
            r#device: {
                let mut default: Box<super::super::types::CodeableReference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#reason: Default::default(),
            r#body_site: Default::default(),
            r#note: Default::default(),
        }
    }
}
