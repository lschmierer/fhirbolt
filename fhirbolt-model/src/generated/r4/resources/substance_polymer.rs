// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "Todo."]
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerMonomerSetStartingMaterial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#material: Default::default(),
            r#type: Default::default(),
            r#is_defining: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerMonomerSet {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#ratio_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#starting_material: Vec<SubstancePolymerMonomerSetStartingMaterial>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerMonomerSet {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#ratio_type: Default::default(),
            r#starting_material: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#degree: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#degree: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "Todo."]
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#representation: Default::default(),
            r#attachment: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#orientation_of_polymerisation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#repeat_unit: Option<super::super::types::String>,
    #[doc = "Todo."]
    pub r#amount: Option<Box<super::super::types::SubstanceAmount>>,
    #[doc = "Todo."]
    pub r#degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
    #[doc = "Todo."]
    pub r#structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeatRepeatUnit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#orientation_of_polymerisation: Default::default(),
            r#repeat_unit: Default::default(),
            r#amount: Default::default(),
            r#degree_of_polymerisation: Default::default(),
            r#structural_representation: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeat {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Todo."]
    pub r#number_of_units: Option<super::super::types::Integer>,
    #[doc = "Todo."]
    pub r#average_molecular_formula: Option<super::super::types::String>,
    #[doc = "Todo."]
    pub r#repeat_unit_amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeat {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#number_of_units: Default::default(),
            r#average_molecular_formula: Default::default(),
            r#repeat_unit_amount_type: Default::default(),
            r#repeat_unit: Default::default(),
        }
    }
}
#[doc = "Todo."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymer {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
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
    #[doc = "Todo."]
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#geometry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Todo."]
    pub r#copolymer_connectivity: Vec<super::super::types::CodeableConcept>,
    #[doc = "Todo."]
    pub r#modification: Vec<super::super::types::String>,
    #[doc = "Todo."]
    pub r#monomer_set: Vec<SubstancePolymerMonomerSet>,
    #[doc = "Todo."]
    pub r#repeat: Vec<SubstancePolymerRepeat>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymer {
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
            r#class: Default::default(),
            r#geometry: Default::default(),
            r#copolymer_connectivity: Default::default(),
            r#modification: Default::default(),
            r#monomer_set: Default::default(),
            r#repeat: Default::default(),
        }
    }
}
