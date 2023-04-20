// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "The actual or approximate date of birth of the relative."]
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyMemberHistoryBorn {
    Period(Box<super::super::types::Period>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryBorn {
    fn default() -> FamilyMemberHistoryBorn {
        FamilyMemberHistoryBorn::Invalid
    }
}
#[doc = "The age of the relative at the time the family member history is recorded."]
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyMemberHistoryAge {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryAge {
    fn default() -> FamilyMemberHistoryAge {
        FamilyMemberHistoryAge::Invalid
    }
}
#[doc = "Deceased flag or the actual or approximate age of the relative at the time of death for the family member history record."]
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyMemberHistoryDeceased {
    Boolean(Box<super::super::types::Boolean>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Date(Box<super::super::types::Date>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryDeceased {
    fn default() -> FamilyMemberHistoryDeceased {
        FamilyMemberHistoryDeceased::Invalid
    }
}
#[doc = "Either the age of onset, range of approximate age or descriptive string can be recorded.  For conditions with multiple occurrences, this describes the first known occurrence."]
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyMemberHistoryConditionOnset {
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for FamilyMemberHistoryConditionOnset {
    fn default() -> FamilyMemberHistoryConditionOnset {
        FamilyMemberHistoryConditionOnset::Invalid
    }
}
#[doc = "The significant Conditions (or condition) that the family member had. This is a repeating section to allow a system to represent more than one condition per resource, though there is nothing stopping multiple resources - one per condition."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FamilyMemberHistoryCondition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual condition specified. Could be a coded condition (like MI or Diabetes) or a less specific string like 'cancer' depending on how much is known about the condition and the capabilities of the creating system."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates what happened following the condition.  If the condition resulted in death, deceased date is captured on the relation."]
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This condition contributed to the cause of death of the related person. If contributedToDeath is not populated, then it is unknown."]
    pub r#contributed_to_death: Option<super::super::types::Boolean>,
    #[doc = "Either the age of onset, range of approximate age or descriptive string can be recorded.  For conditions with multiple occurrences, this describes the first known occurrence."]
    pub r#onset: Option<FamilyMemberHistoryConditionOnset>,
    #[doc = "An area where general notes can be placed about this specific condition."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
#[doc = "Significant health conditions for a person related to the patient relevant in the context of care for the patient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FamilyMemberHistory {
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
    #[doc = "Business identifiers assigned to this family member history by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, orderset or other definition that is adhered to in whole or in part by this FamilyMemberHistory."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, orderset or other definition that is adhered to in whole or in part by this FamilyMemberHistory."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A code specifying the status of the record of the family history of a specific family member."]
    pub r#status: super::super::types::Code,
    #[doc = "Describes why the family member's history is not available."]
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The person who this history concerns."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date (and possibly time) when the family member history was recorded or last updated."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "This will either be a name or a description; e.g. \"Aunt Susan\", \"my cousin with the red hair\"."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The type of relationship this person has to the patient (father, mother, brother etc.)."]
    pub r#relationship: Box<super::super::types::CodeableConcept>,
    #[doc = "The birth sex of the family member."]
    pub r#sex: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual or approximate date of birth of the relative."]
    pub r#born: Option<FamilyMemberHistoryBorn>,
    #[doc = "The age of the relative at the time the family member history is recorded."]
    pub r#age: Option<FamilyMemberHistoryAge>,
    #[doc = "If true, indicates that the age value specified is an estimated value."]
    pub r#estimated_age: Option<super::super::types::Boolean>,
    #[doc = "Deceased flag or the actual or approximate age of the relative at the time of death for the family member history record."]
    pub r#deceased: Option<FamilyMemberHistoryDeceased>,
    #[doc = "Describes why the family member history occurred in coded or textual form."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates a Condition, Observation, AllergyIntolerance, or QuestionnaireResponse that justifies this family member history event."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "This property allows a non condition-specific note to the made about the related person. Ideally, the note would be in the condition property, but this is not always possible."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The significant Conditions (or condition) that the family member had. This is a repeating section to allow a system to represent more than one condition per resource, though there is nothing stopping multiple resources - one per condition."]
    pub r#condition: Vec<FamilyMemberHistoryCondition>,
}
