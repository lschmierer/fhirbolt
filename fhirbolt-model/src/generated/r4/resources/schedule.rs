// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "A container for slots of time that may be available for booking appointments."]
#[derive(Debug, Clone, PartialEq)]
pub struct Schedule {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "External Ids for this item."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Whether this schedule record is in active use or should not be used (such as was entered in error)."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "A broad categorization of the service that is to be performed during this appointment."]
    pub r#service_category: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specific service that is to be performed during this appointment."]
    pub r#service_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The specialty of a practitioner that would be required to perform the service requested in this appointment."]
    pub r#specialty: Vec<super::super::types::CodeableConcept>,
    #[doc = "Slots that reference this schedule resource provide the availability details to these referenced resource(s)."]
    pub r#actor: Vec<super::super::types::Reference>,
    #[doc = "The period of time that the slots that reference this Schedule resource cover (even if none exist). These  cover the amount of time that an organization's planning horizon; the interval for which they are currently accepting appointments. This does not define a \"template\" for planning outside these dates."]
    pub r#planning_horizon: Option<Box<super::super::types::Period>>,
    #[doc = "Comments on the availability to describe any extended information. Such as custom constraints on the slots that may be associated."]
    pub r#comment: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for Schedule {
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
            r#active: Default::default(),
            r#service_category: Default::default(),
            r#service_type: Default::default(),
            r#specialty: Default::default(),
            r#actor: Default::default(),
            r#planning_horizon: Default::default(),
            r#comment: Default::default(),
        }
    }
}
