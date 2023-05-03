// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Estimated or actual date,  date-time, or age when allergy or intolerance was identified."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AllergyIntoleranceOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Details about each adverse reaction event linked to exposure to the identified substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct AllergyIntoleranceReaction {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identification of the specific substance (or pharmaceutical product) considered to be responsible for the Adverse Reaction event. Note: the substance for a specific reaction may be different from the substance identified as the cause of the risk, but it must be consistent with it. For instance, it may be a more specific substance (e.g. a brand medication) or a composite product that includes the identified substance. It must be clinically safe to only process the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable to confirm that AllergyIntolerance.reaction.substance falls within the semantic scope of AllergyIntolerance.code, then the receiving system should ignore AllergyIntolerance.reaction.substance."]
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Clinical symptoms and/or signs that are observed or associated with the adverse reaction event."]
    pub r#manifestation: Vec<super::super::types::CodeableConcept>,
    #[doc = "Text description about the reaction as a whole, including details of the manifestation if required."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Record of the date and/or time of the onset of the Reaction."]
    pub r#onset: Option<super::super::types::DateTime>,
    #[doc = "Clinical assessment of the severity of the reaction event as a whole, potentially considering multiple different manifestations."]
    pub r#severity: Option<super::super::types::Code>,
    #[doc = "Identification of the route by which the subject was exposed to the substance."]
    pub r#exposure_route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional text about the adverse reaction event not captured in other fields."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for AllergyIntoleranceReaction {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#substance: Default::default(),
            r#manifestation: Default::default(),
            r#description: Default::default(),
            r#onset: Default::default(),
            r#severity: Default::default(),
            r#exposure_route: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.\n\nTo record a clinical assessment of a propensity, or potential risk to an individual, of an adverse reaction upon future exposure to the specified substance, or class of substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct AllergyIntolerance {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Business identifiers assigned to this AllergyIntolerance by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The clinical status of the allergy or intolerance."]
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Assertion about certainty associated with the propensity, or potential risk, of a reaction to the identified substance (including pharmaceutical product)."]
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identification of the underlying physiological mechanism for the reaction risk."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Category of the identified substance."]
    pub r#category: Vec<super::super::types::Code>,
    #[doc = "Estimate of the potential clinical harm, or seriousness, of the reaction to the identified substance."]
    pub r#criticality: Option<super::super::types::Code>,
    #[doc = "Code for an allergy or intolerance statement (either a positive or a negated/excluded statement).  This may be a code for a substance or pharmaceutical product that is considered to be responsible for the adverse reaction risk (e.g., \"Latex\"), an allergy or intolerance condition (e.g., \"Latex allergy\"), or a negated/excluded code for a specific substance or class (e.g., \"No latex allergy\") or a general or categorical negated statement (e.g.,  \"No known allergy\", \"No known drug allergies\").  Note: the substance for a specific reaction may be different from the substance identified as the cause of the risk, but it must be consistent with it. For instance, it may be a more specific substance (e.g. a brand medication) or a composite product that includes the identified substance. It must be clinically safe to only process the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable to confirm that AllergyIntolerance.reaction.substance falls within the semantic scope of AllergyIntolerance.code, then the receiving system should ignore AllergyIntolerance.reaction.substance."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The patient who has the allergy or intolerance."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The encounter when the allergy or intolerance was asserted."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Estimated or actual date,  date-time, or age when allergy or intolerance was identified."]
    pub r#onset: Option<AllergyIntoleranceOnset>,
    #[doc = "The recordedDate represents when this particular AllergyIntolerance record was created in the system, which is often a system-generated date."]
    pub r#recorded_date: Option<super::super::types::DateTime>,
    #[doc = "Individual who recorded the record and takes responsibility for its content."]
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    #[doc = "The source of the information about the allergy that is recorded."]
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    #[doc = "Represents the date and/or time of the last known occurrence of a reaction event."]
    pub r#last_occurrence: Option<super::super::types::DateTime>,
    #[doc = "Additional narrative about the propensity for the Adverse Reaction, not captured in other fields."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Details about each adverse reaction event linked to exposure to the identified substance."]
    pub r#reaction: Vec<AllergyIntoleranceReaction>,
}
#[allow(clippy::derivable_impls)]
impl Default for AllergyIntolerance {
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
            r#clinical_status: Default::default(),
            r#verification_status: Default::default(),
            r#type: Default::default(),
            r#category: Default::default(),
            r#criticality: Default::default(),
            r#code: Default::default(),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#onset: Default::default(),
            r#recorded_date: Default::default(),
            r#recorder: Default::default(),
            r#asserter: Default::default(),
            r#last_occurrence: Default::default(),
            r#note: Default::default(),
            r#reaction: Default::default(),
        }
    }
}
