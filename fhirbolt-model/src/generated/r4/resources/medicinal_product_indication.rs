// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicinalProductIndicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductIndicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#medication: MedicinalProductIndicationOtherTherapyMedication,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductIndicationOtherTherapy {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#therapy_relationship_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#medication: Default::default(),
        }
    }
}
#[doc = "Indication for the Medicinal Product."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductIndication {
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
    #[doc = "The medication for which this is an indication."]
    pub r#subject: Vec<super::super::types::Reference>,
    #[doc = "The disease, symptom or procedure that is the indication for treatment."]
    pub r#disease_symptom_procedure: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the disease or symptom for which the indication applies."]
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Comorbidity (concurrent condition) or co-infection as part of the indication."]
    pub r#comorbidity: Vec<super::super::types::CodeableConcept>,
    #[doc = "The intended effect, aim or strategy to be achieved by the indication."]
    pub r#intended_effect: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Timing or duration information as part of the indication."]
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<MedicinalProductIndicationOtherTherapy>,
    #[doc = "Describe the undesirable effects of the medicinal product."]
    pub r#undesirable_effect: Vec<super::super::types::Reference>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<super::super::types::Population>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductIndication {
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
            r#subject: Default::default(),
            r#disease_symptom_procedure: Default::default(),
            r#disease_status: Default::default(),
            r#comorbidity: Default::default(),
            r#intended_effect: Default::default(),
            r#duration: Default::default(),
            r#other_therapy: Default::default(),
            r#undesirable_effect: Default::default(),
            r#population: Default::default(),
        }
    }
}
