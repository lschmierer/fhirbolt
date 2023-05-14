// Generated on 2023-05-14 by fhirbolt-codegen v0.8.0
#[doc = "Indicates the mechanism used to compare versions to determine which ConceptMap is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConceptMapVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "Identifier for the source value set that contains the concepts that are being mapped and provides context for the mappings.  Limits the scope of the map to source codes (ConceptMap.group.element code or valueSet) that are members of this value set."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConceptMapSourceScope {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "Identifier for the target value set that provides important context about how the mapping choices are made.  Limits the scope of the map to target codes (ConceptMap.group.element.target code or valueSet) that are members of this value set."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConceptMapTargetScope {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "The value of this property. If the type chosen for this element is 'code', then the property SHALL be defined in a ConceptMap.property element."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConceptMapGroupElementTargetPropertyValue {
    Coding(Box<super::super::types::Coding>),
    String(Box<super::super::types::String>),
    Integer(Box<super::super::types::Integer>),
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
    Code(Box<super::super::types::Code>),
    #[default]
    Invalid,
}
#[doc = "Data element value that the map depends on / produces."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ConceptMapGroupElementTargetDependsOnValue {
    Code(Box<super::super::types::Code>),
    Coding(Box<super::super::types::Coding>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    #[default]
    Invalid,
}
#[doc = "A property defines a slot through which additional information can be provided about a map from source -> target."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that is used to identify the property. The code is used internally (in ConceptMap.group.element.target.property.code) and also in the $translate operation."]
    pub r#code: super::super::types::Code,
    #[doc = "Reference to the formal meaning of the property."]
    pub r#uri: Option<super::super::types::Uri>,
    #[doc = "A description of the property - why it is defined, and how its value might be used."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of the property value."]
    pub r#type: super::super::types::Code,
    #[doc = "The CodeSystem that defines the codes from which values of type ```code``` in property values."]
    pub r#system: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#uri: Default::default(),
            r#description: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#system: Default::default(),
        }
    }
}
#[doc = "An additionalAttribute defines an additional data element found in the source or target data model where the data will come from or be mapped to. Some mappings are based on data in addition to the source data element, where codes in multiple fields are combined to a single field (or vice versa)."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapAdditionalAttribute {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code that is used to identify this additional data attribute. The code is used internally in ConceptMap.group.element.target.dependsOn.attribute and ConceptMap.group.element.target.product.attribute."]
    pub r#code: super::super::types::Code,
    #[doc = "Reference to the formal definition of the source/target data element. For elements defined by the FHIR specification, or using a FHIR logical model, the correct format is {canonical-url}#{element-id}."]
    pub r#uri: Option<super::super::types::Uri>,
    #[doc = "A description of the additional attribute and/or the data element it refers to - why it is defined, and how the value might be used in mappings, and a discussion of issues associated with the use of the data element."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "The type of the source data contained in this concept map for this data element."]
    pub r#type: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapAdditionalAttribute {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#uri: Default::default(),
            r#description: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "A property value for this source -> target mapping."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroupElementTargetProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A reference to a mapping property defined in ConceptMap.property."]
    pub r#code: super::super::types::Code,
    #[doc = "The value of this property. If the type chosen for this element is 'code', then the property SHALL be defined in a ConceptMap.property element."]
    pub r#value: ConceptMapGroupElementTargetPropertyValue,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroupElementTargetProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: Default::default(),
        }
    }
}
#[doc = "A set of additional dependencies for this mapping to hold. This mapping is only applicable if the specified data attribute can be resolved, and it has the specified value."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroupElementTargetDependsOn {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A reference to the additional attribute that holds a value the map depends on."]
    pub r#attribute: super::super::types::Code,
    #[doc = "Data element value that the map depends on / produces."]
    pub r#value: Option<ConceptMapGroupElementTargetDependsOnValue>,
    #[doc = "This mapping applies if the data element value is a code from this value set."]
    pub r#value_set: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroupElementTargetDependsOn {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#attribute: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#value: Default::default(),
            r#value_set: Default::default(),
        }
    }
}
#[doc = "A concept from the target value set that this concept maps to."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroupElementTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identity (code or path) or the element/item that the map refers to."]
    pub r#code: Option<super::super::types::Code>,
    #[doc = "The display for the code. The display is only provided to help editors when editing the concept map."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "The set of concepts from the ConceptMap.group.target code system which are all being mapped to as part of this mapping rule. The effect of using this data element is the same as having multiple ConceptMap.group.element.target elements with one for each concept in the ConceptMap.group.element.target.valueSet value set."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The relationship between the source and target concepts. The relationship is read from source to target (e.g. source-is-narrower-than-target)."]
    pub r#relationship: super::super::types::Code,
    #[doc = "A description of status/issues in mapping that conveys additional information not represented in  the structured data."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "A property value for this source -> target mapping."]
    pub r#property: Vec<ConceptMapGroupElementTargetProperty>,
    #[doc = "A set of additional dependencies for this mapping to hold. This mapping is only applicable if the specified data attribute can be resolved, and it has the specified value."]
    pub r#depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,
    #[doc = "Product is the output of a ConceptMap that provides additional values that go in other attributes / data elemnts of the target data."]
    pub r#product: Vec<ConceptMapGroupElementTargetDependsOn>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroupElementTarget {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#display: Default::default(),
            r#value_set: Default::default(),
            r#relationship: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#comment: Default::default(),
            r#property: Default::default(),
            r#depends_on: Default::default(),
            r#product: Default::default(),
        }
    }
}
#[doc = "Mappings for an individual concept in the source to one or more concepts in the target."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroupElement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identity (code or path) or the element/item being mapped."]
    pub r#code: Option<super::super::types::Code>,
    #[doc = "The display for the code. The display is only provided to help editors when editing the concept map."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "The set of concepts from the ConceptMap.group.source code system which are all being mapped to the target as part of this mapping rule."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "If noMap = true this indicates that no mapping to a target concept exists for this source concept."]
    pub r#no_map: Option<super::super::types::Boolean>,
    #[doc = "A concept from the target value set that this concept maps to."]
    pub r#target: Vec<ConceptMapGroupElementTarget>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroupElement {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#display: Default::default(),
            r#value_set: Default::default(),
            r#no_map: Default::default(),
            r#target: Default::default(),
        }
    }
}
#[doc = "What to do when there is no mapping to a target concept from the source concept and ConceptMap.group.element.noMap is not true. This provides the \"default\" to be applied when there is no target concept mapping specified or the expansion of ConceptMap.group.element.target.valueSet is empty."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroupUnmapped {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Defines which action to take if there is no match for the source concept in the target system designated for the group. One of 3 actions are possible: use the unmapped source code (this is useful when doing a mapping between versions, and only a few codes have changed), use a fixed code (a default code), or alternatively, a reference to a different concept map can be provided (by canonical URL)."]
    pub r#mode: super::super::types::Code,
    #[doc = "The fixed code to use when the mode = 'fixed'  - all unmapped codes are mapped to a single fixed code."]
    pub r#code: Option<super::super::types::Code>,
    #[doc = "The display for the code. The display is only provided to help editors when editing the concept map."]
    pub r#display: Option<super::super::types::String>,
    #[doc = "The set of fixed codes to use when the mode = 'fixed'  - all unmapped codes are mapped to each of the fixed codes."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The default relationship value to apply between the source and target concepts when the source code is unmapped and the mode is 'fixed' or 'use-source-code'."]
    pub r#relationship: Option<super::super::types::Code>,
    #[doc = "The canonical reference to an additional ConceptMap resource instance to use for mapping if this ConceptMap resource contains no matching mapping for the source concept."]
    pub r#other_map: Option<super::super::types::Canonical>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroupUnmapped {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#mode: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#code: Default::default(),
            r#display: Default::default(),
            r#value_set: Default::default(),
            r#relationship: Default::default(),
            r#other_map: Default::default(),
        }
    }
}
#[doc = "A group of mappings that all have the same source and target system."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMapGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that identifies the source system where the concepts to be mapped are defined."]
    pub r#source: Option<super::super::types::Canonical>,
    #[doc = "An absolute URI that identifies the target system that the concepts will be mapped to."]
    pub r#target: Option<super::super::types::Canonical>,
    #[doc = "Mappings for an individual concept in the source to one or more concepts in the target."]
    pub r#element: Vec<ConceptMapGroupElement>,
    #[doc = "What to do when there is no mapping to a target concept from the source concept and ConceptMap.group.element.noMap is not true. This provides the \"default\" to be applied when there is no target concept mapping specified or the expansion of ConceptMap.group.element.target.valueSet is empty."]
    pub r#unmapped: Option<ConceptMapGroupUnmapped>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMapGroup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#source: Default::default(),
            r#target: Default::default(),
            r#element: Default::default(),
            r#unmapped: Default::default(),
        }
    }
}
#[doc = "A statement of relationships from one set of concepts to one or more other concepts - either concepts in code systems, or data element/data element concepts, or classes in class models."]
#[derive(Debug, Clone, PartialEq)]
pub struct ConceptMap {
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
    #[doc = "An absolute URI that is used to identify this concept map when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this concept map is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the concept map is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this concept map when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the concept map when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the concept map author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which ConceptMap is more current."]
    pub r#version_algorithm: Option<ConceptMapVersionAlgorithm>,
    #[doc = "A natural language name identifying the concept map. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the concept map."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this concept map. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this concept map is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the concept map was last significantly changed. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the concept map changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the concept map."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the concept map from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate concept map instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the concept map is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this concept map is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "A copyright statement relating to the concept map and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the concept map."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the ConceptMap content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Descriptions related to the content of the ConceptMap. Topics provide a high-level categorization as well as keywords for the ConceptMap that can be useful for filtering and searching."]
    pub r#topic: Vec<super::super::types::CodeableConcept>,
    #[doc = "An individiual or organization primarily involved in the creation and maintenance of the ConceptMap."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization primarily responsible for internal coherence of the ConceptMap."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be primarily responsible for review of some aspect of the ConceptMap."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individual or organization asserted by the publisher to be responsible for officially endorsing the ConceptMap for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Related artifacts such as additional documentation, justification, dependencies, bibliographic references, and predecessor and successor artifacts."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "A property defines a slot through which additional information can be provided about a map from source -> target."]
    pub r#property: Vec<ConceptMapProperty>,
    #[doc = "An additionalAttribute defines an additional data element found in the source or target data model where the data will come from or be mapped to. Some mappings are based on data in addition to the source data element, where codes in multiple fields are combined to a single field (or vice versa)."]
    pub r#additional_attribute: Vec<ConceptMapAdditionalAttribute>,
    #[doc = "Identifier for the source value set that contains the concepts that are being mapped and provides context for the mappings.  Limits the scope of the map to source codes (ConceptMap.group.element code or valueSet) that are members of this value set."]
    pub r#source_scope: Option<ConceptMapSourceScope>,
    #[doc = "Identifier for the target value set that provides important context about how the mapping choices are made.  Limits the scope of the map to target codes (ConceptMap.group.element.target code or valueSet) that are members of this value set."]
    pub r#target_scope: Option<ConceptMapTargetScope>,
    #[doc = "A group of mappings that all have the same source and target system."]
    pub r#group: Vec<ConceptMapGroup>,
}
#[allow(clippy::derivable_impls)]
impl Default for ConceptMap {
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
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#version: Default::default(),
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#topic: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#related_artifact: Default::default(),
            r#property: Default::default(),
            r#additional_attribute: Default::default(),
            r#source_scope: Default::default(),
            r#target_scope: Default::default(),
            r#group: Default::default(),
        }
    }
}
