// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "Indicates who or what performed an action."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductDispensePerformer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the function of the performer during the dispense."]
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies the person responsible for the action."]
    pub r#actor: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductDispensePerformer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#function: Default::default(),
            r#actor: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "A record of dispensation of a biologically derived product."]
#[derive(Debug, Clone, PartialEq)]
pub struct BiologicallyDerivedProductDispense {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique instance identifiers assigned to a biologically derived product dispense. Note: This is a business identifier, not a resource identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The order or request that the dispense is fulfilling. This is a reference to a ServiceRequest resource."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular event is a component."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code specifying the state of the dispense event."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the relationship between the donor of the biologically derived product and the intended recipient."]
    pub r#origin_relationship_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A link to a resource identifying the biologically derived product that is being dispensed."]
    pub r#product: Box<super::super::types::Reference>,
    #[doc = "A link to a resource representing the patient that the product is dispensed for."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "Indicates the type of matching associated with the dispense."]
    pub r#match_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates who or what performed an action."]
    pub r#performer: Vec<BiologicallyDerivedProductDispensePerformer>,
    #[doc = "The physical location where the dispense was performed."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The amount of product in the dispense. Quantity will depend on the product being dispensed. Examples are: volume; cell count; concentration."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "When the product was selected/ matched."]
    pub r#prepared_date: Option<super::super::types::DateTime>,
    #[doc = "When the product was dispatched for clinical use."]
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    #[doc = "Link to a resource identifying the physical location that the product was dispatched to."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Additional notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Specific instructions for use."]
    pub r#usage_instruction: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for BiologicallyDerivedProductDispense {
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
            r#part_of: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#origin_relationship_type: Default::default(),
            r#product: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#match_status: Default::default(),
            r#performer: Default::default(),
            r#location: Default::default(),
            r#quantity: Default::default(),
            r#prepared_date: Default::default(),
            r#when_handed_over: Default::default(),
            r#destination: Default::default(),
            r#note: Default::default(),
            r#usage_instruction: Default::default(),
        }
    }
}
