// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "The history of statuses that the EpisodeOfCare has been through (without requiring processing the history of the resource)."]
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCareStatusHistory {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "planned | waitlist | active | onhold | finished | cancelled."]
    pub r#status: super::super::types::Code,
    #[doc = "The period during this EpisodeOfCare that the specific status applied."]
    pub r#period: Box<super::super::types::Period>,
}
impl Default for EpisodeOfCareStatusHistory {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: {
                let mut default: Box<super::super::types::Period> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "The list of diagnosis relevant to this episode of care."]
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCareDiagnosis {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A list of conditions/problems/diagnoses that this episode of care is intended to be providing care for."]
    pub r#condition: Box<super::super::types::Reference>,
    #[doc = "Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ranking of the diagnosis (for each role type)."]
    pub r#rank: Option<super::super::types::PositiveInt>,
}
impl Default for EpisodeOfCareDiagnosis {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#condition: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#role: Default::default(),
            r#rank: Default::default(),
        }
    }
}
#[doc = "An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time."]
#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeOfCare {
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
    #[doc = "The EpisodeOfCare may be known by different identifiers for different contexts of use, such as when an external agency is tracking the Episode for funding purposes."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "planned | waitlist | active | onhold | finished | cancelled."]
    pub r#status: super::super::types::Code,
    #[doc = "The history of statuses that the EpisodeOfCare has been through (without requiring processing the history of the resource)."]
    pub r#status_history: Vec<EpisodeOfCareStatusHistory>,
    #[doc = "A classification of the type of episode of care; e.g. specialist referral, disease management, type of funded care."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The list of diagnosis relevant to this episode of care."]
    pub r#diagnosis: Vec<EpisodeOfCareDiagnosis>,
    #[doc = "The patient who is the focus of this episode of care."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The organization that has assumed the specific responsibilities for the specified duration."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The interval during which the managing organization assumes the defined responsibility."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Referral Request(s) that are fulfilled by this EpisodeOfCare, incoming referrals."]
    pub r#referral_request: Vec<Box<super::super::types::Reference>>,
    #[doc = "The practitioner that is the care manager/care coordinator for this patient."]
    pub r#care_manager: Option<Box<super::super::types::Reference>>,
    #[doc = "The list of practitioners that may be facilitating this episode of care for specific purposes."]
    pub r#team: Vec<Box<super::super::types::Reference>>,
    #[doc = "The set of accounts that may be used for billing for this EpisodeOfCare."]
    pub r#account: Vec<Box<super::super::types::Reference>>,
}
impl Default for EpisodeOfCare {
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
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#status_history: Default::default(),
            r#type: Default::default(),
            r#diagnosis: Default::default(),
            r#patient: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#managing_organization: Default::default(),
            r#period: Default::default(),
            r#referral_request: Default::default(),
            r#care_manager: Default::default(),
            r#team: Default::default(),
            r#account: Default::default(),
        }
    }
}
