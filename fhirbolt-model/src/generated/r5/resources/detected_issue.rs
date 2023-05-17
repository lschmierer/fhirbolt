// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The date or period when the detected issue was initially identified."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DetectedIssueIdentified {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Supporting evidence or manifestations that provide the basis for identifying the detected issue such as a GuidanceResponse or MeasureReport."]
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssueEvidence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A manifestation that led to the recording of this detected issue."]
    pub r#code: Vec<super::super::types::CodeableConcept>,
    #[doc = "Links to resources that constitute evidence for the detected issue such as a GuidanceResponse or MeasureReport."]
    pub r#detail: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DetectedIssueEvidence {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "Indicates an action that has been taken or is committed to reduce or eliminate the likelihood of the risk identified by the detected issue from manifesting.  Can also reflect an observation of known mitigating factors that may reduce/eliminate the need for any action."]
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssueMitigation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the action that was taken or the observation that was made that reduces/eliminates the risk associated with the identified issue."]
    pub r#action: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates when the mitigating action was documented."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Identifies the practitioner who determined the mitigation and takes responsibility for the mitigation step occurring."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Clinicians may add additional notes or justifications about the mitigation action. For example, patient can have this drug because they have had it before without any issues. Multiple justifications may be provided."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for DetectedIssueMitigation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#action: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#date: Default::default(),
            r#author: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedIssue {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Business identifier associated with the detected issue record."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates the status of the detected issue."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies the general type of detected issue."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Identifies the specific type of issue identified."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the degree of importance associated with the identified issue based on the potential impact on the patient."]
    pub r#severity: Option<super::super::types::Code>,
    #[doc = "Indicates the subject whose record the detected issue is associated with."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter during which this issue was detected."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date or period when the detected issue was initially identified."]
    pub r#identified: Option<DetectedIssueIdentified>,
    #[doc = "Individual or device responsible for the issue being raised.  For example, a decision support application or a pharmacist conducting a medication review."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the resource representing the current activity or proposed activity that is potentially problematic."]
    pub r#implicated: Vec<super::super::types::Reference>,
    #[doc = "Supporting evidence or manifestations that provide the basis for identifying the detected issue such as a GuidanceResponse or MeasureReport."]
    pub r#evidence: Vec<DetectedIssueEvidence>,
    #[doc = "A textual explanation of the detected issue."]
    pub r#detail: Option<super::super::types::Markdown>,
    #[doc = "The literature, knowledge-base or similar reference that describes the propensity for the detected issue identified."]
    pub r#reference: Option<super::super::types::Uri>,
    #[doc = "Indicates an action that has been taken or is committed to reduce or eliminate the likelihood of the risk identified by the detected issue from manifesting.  Can also reflect an observation of known mitigating factors that may reduce/eliminate the need for any action."]
    pub r#mitigation: Vec<DetectedIssueMitigation>,
}
#[allow(clippy::derivable_impls)]
impl Default for DetectedIssue {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#category: Default::default(),
            r#code: Default::default(),
            r#severity: Default::default(),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#identified: Default::default(),
            r#author: Default::default(),
            r#implicated: Default::default(),
            r#evidence: Default::default(),
            r#detail: Default::default(),
            r#reference: Default::default(),
            r#mitigation: Default::default(),
        }
    }
}
