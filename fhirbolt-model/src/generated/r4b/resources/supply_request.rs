// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The item that is requested to be supplied. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SupplyRequestItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The value of the device detail."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SupplyRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
    #[default]
    Invalid,
}
#[doc = "When the request should be fulfilled."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SupplyRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    #[default]
    Invalid,
}
#[doc = "Specific parameters for the ordered item.  For example, the size of the indicated item."]
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyRequestParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A code or string that identifies the device detail being asserted."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The value of the device detail."]
    pub r#value: Option<SupplyRequestParameterValue>,
}
impl Default for SupplyRequestParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "A record of a request for a medication, substance or device used in the healthcare setting."]
#[derive(Debug, Clone, PartialEq)]
pub struct SupplyRequest {
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
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifiers assigned to this SupplyRequest by the author and/or other systems. These identifiers remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Status of the supply request."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Category of supply, e.g.  central, non-stock, etc. This is used to support work flows associated with the supply process."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates how quickly this SupplyRequest should be addressed with respect to other requests."]
    pub r#priority: Option<super::super::types::Code>,
    #[doc = "The item that is requested to be supplied. This is either a link to a resource representing the details of the item or a code that identifies the item from a known list."]
    pub r#item: SupplyRequestItem,
    #[doc = "The amount that is being ordered of the indicated item."]
    pub r#quantity: Box<super::super::types::Quantity>,
    #[doc = "Specific parameters for the ordered item.  For example, the size of the indicated item."]
    pub r#parameter: Vec<SupplyRequestParameter>,
    #[doc = "When the request should be fulfilled."]
    pub r#occurrence: Option<SupplyRequestOccurrence>,
    #[doc = "When the request was made."]
    pub r#authored_on: Option<super::super::types::DateTime>,
    #[doc = "The device, practitioner, etc. who initiated the request."]
    pub r#requester: Option<Box<super::super::types::Reference>>,
    #[doc = "Who is intended to fulfill the request."]
    pub r#supplier: Vec<Box<super::super::types::Reference>>,
    #[doc = "The reason why the supply item was requested."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reason why the supply item was requested."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Where the supply is expected to come from."]
    pub r#deliver_from: Option<Box<super::super::types::Reference>>,
    #[doc = "Where the supply is destined to go."]
    pub r#deliver_to: Option<Box<super::super::types::Reference>>,
}
impl Default for SupplyRequest {
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
            r#status: Default::default(),
            r#category: Default::default(),
            r#priority: Default::default(),
            r#item: Default::default(),
            r#quantity: {
                let mut default: Box<super::super::types::Quantity> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#parameter: Default::default(),
            r#occurrence: Default::default(),
            r#authored_on: Default::default(),
            r#requester: Default::default(),
            r#supplier: Default::default(),
            r#reason_code: Default::default(),
            r#reason_reference: Default::default(),
            r#deliver_from: Default::default(),
            r#deliver_to: Default::default(),
        }
    }
}
