// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "The value of the trait that holds (or does not hold - see 'exclude') for members of the group."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum GroupCharacteristicValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Identifies traits whose presence r absence is shared by members of the group."]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that identifies the kind of trait being asserted."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The value of the trait that holds (or does not hold - see 'exclude') for members of the group."]
    pub r#value: GroupCharacteristicValue,
    #[doc = "If true, indicates the characteristic is one that is NOT held by members of the group."]
    pub r#exclude: super::super::types::Boolean,
    #[doc = "The period over which the characteristic is tested; e.g. the patient had an operation during the month of June."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for GroupCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
            r#exclude: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#period: Default::default(),
        }
    }
}
#[doc = "Identifies the resource instances that are members of the group."]
#[derive(Debug, Clone, PartialEq)]
pub struct GroupMember {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A reference to the entity that is a member of the group. Must be consistent with Group.type. If the entity is another group, then the type must be the same."]
    pub r#entity: Box<super::super::types::Reference>,
    #[doc = "The period that the member was in the group, if known."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "A flag to indicate that the member is no longer in the group, but previously may have been a member."]
    pub r#inactive: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for GroupMember {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#entity: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#period: Default::default(),
            r#inactive: Default::default(),
        }
    }
}
#[doc = "Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization."]
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
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
    #[doc = "A unique business identifier for this group."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Indicates whether the record for the group is available for use or is merely being retained for historical purposes."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "Identifies the broad classification of the kind of resources the group includes."]
    pub r#type: super::super::types::Code,
    #[doc = "If true, indicates that the resource refers to a specific group of real individuals.  If false, the group defines a set of intended individuals."]
    pub r#actual: super::super::types::Boolean,
    #[doc = "Provides a specific type of resource the group includes; e.g. \"cow\", \"syringe\", etc."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A label assigned to the group for human identification and communication."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A count of the number of resource instances that are part of the group."]
    pub r#quantity: Option<super::super::types::UnsignedInt>,
    #[doc = "Entity responsible for defining and maintaining Group characteristics and/or registered members."]
    pub r#managing_entity: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies traits whose presence r absence is shared by members of the group."]
    pub r#characteristic: Vec<GroupCharacteristic>,
    #[doc = "Identifies the resource instances that are members of the group."]
    pub r#member: Vec<GroupMember>,
}
#[allow(clippy::derivable_impls)]
impl Default for Group {
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
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#actual: super::super::types::Boolean {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#code: Default::default(),
            r#name: Default::default(),
            r#quantity: Default::default(),
            r#managing_entity: Default::default(),
            r#characteristic: Default::default(),
            r#member: Default::default(),
        }
    }
}
