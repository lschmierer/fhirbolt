// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "Identifies the medication, substance or device being dispensed. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SupplyDeliverySuppliedItemItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The date or time(s) the activity occurred."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SupplyDeliveryOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "The item that is being delivered or has been supplied."]
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyDeliverySuppliedItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The amount of supply that has been dispensed. Includes unit of measure."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the medication, substance or device being dispensed. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
    pub r#item: Option<SupplyDeliverySuppliedItemItem>,
}
#[allow(clippy::derivable_impls)]
impl Default for SupplyDeliverySuppliedItem {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#quantity: Default::default(),
            r#item: Default::default(),
        }
    }
}
#[doc = "Record of delivery of what is supplied."]
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyDelivery {
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
    #[doc = "Identifier for the supply delivery event that is used to identify it across multiple disparate systems."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A plan, proposal or order that is fulfilled in whole or in part by this event."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<super::super::types::Reference>,
    #[doc = "A code specifying the state of the dispense event."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "A link to a resource representing the person whom the delivered item is for."]
    pub r#patient: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates the type of dispensing event that is performed. Examples include: Trial Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The item that is being delivered or has been supplied."]
    pub r#supplied_item: Option<SupplyDeliverySuppliedItem>,
    #[doc = "The date or time(s) the activity occurred."]
    pub r#occurrence: Option<SupplyDeliveryOccurrence>,
    #[doc = "The individual responsible for dispensing the medication, supplier or device."]
    pub r#supplier: Option<Box<super::super::types::Reference>>,
    #[doc = "Identification of the facility/location where the Supply was shipped to, as part of the dispense event."]
    pub r#destination: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the person who picked up the Supply."]
    pub r#receiver: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for SupplyDelivery {
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
            r#status: Default::default(),
            r#patient: Default::default(),
            r#type: Default::default(),
            r#supplied_item: Default::default(),
            r#occurrence: Default::default(),
            r#supplier: Default::default(),
            r#destination: Default::default(),
            r#receiver: Default::default(),
        }
    }
}
