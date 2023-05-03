// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "The starting materials - monomer(s) used in the synthesis of the polymer."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerMonomerSetStartingMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of substance for this starting material."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Substance high level category, e.g. chemical substance."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to specify whether the attribute described is a defining element for the unique identification of the polymer."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A percentage."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerMonomerSetStartingMaterial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#category: Default::default(),
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
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer ratio, SRU/Polymer Ratio."]
    pub r#ratio_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The starting materials - monomer(s) used in the synthesis of the polymer."]
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
#[doc = "Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of the degree of polymerisation shall be described, e.g. SRU/Polymer Ratio."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An average amount of polymerisation."]
    pub r#average: Option<super::super::types::Integer>,
    #[doc = "A low expected limit of the amount."]
    pub r#low: Option<super::super::types::Integer>,
    #[doc = "A high expected limit of the amount."]
    pub r#high: Option<super::super::types::Integer>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#average: Default::default(),
            r#low: Default::default(),
            r#high: Default::default(),
        }
    }
}
#[doc = "A graphical structure for this SRU."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of structure (e.g. Full, Partial, Representative)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation as text string in a standard format e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF."]
    pub r#format: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An attached file with the structural representation."]
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
            r#format: Default::default(),
            r#attachment: Default::default(),
        }
    }
}
#[doc = "An SRU - Structural Repeat Unit."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeatRepeatUnit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Structural repeat units are essential elements for defining polymers."]
    pub r#unit: Option<super::super::types::String>,
    #[doc = "The orientation of the polymerisation, e.g. head-tail, head-head, random."]
    pub r#orientation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Number of repeats of this unit."]
    pub r#amount: Option<super::super::types::Integer>,
    #[doc = "Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described."]
    pub r#degree_of_polymerisation: Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisation>,
    #[doc = "A graphical structure for this SRU."]
    pub r#structural_representation: Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentation>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeatRepeatUnit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#unit: Default::default(),
            r#orientation: Default::default(),
            r#amount: Default::default(),
            r#degree_of_polymerisation: Default::default(),
            r#structural_representation: Default::default(),
        }
    }
}
#[doc = "Specifies and quantifies the repeated units and their configuration."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymerRepeat {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A representation of an (average) molecular formula from a polymer."]
    pub r#average_molecular_formula: Option<super::super::types::String>,
    #[doc = "How the quantitative amount of Structural Repeat Units is captured (e.g. Exact, Numeric, Average)."]
    pub r#repeat_unit_amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An SRU - Structural Repeat Unit."]
    pub r#repeat_unit: Vec<SubstancePolymerRepeatRepeatUnit>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstancePolymerRepeat {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#average_molecular_formula: Default::default(),
            r#repeat_unit_amount_type: Default::default(),
            r#repeat_unit: Default::default(),
        }
    }
}
#[doc = "Properties of a substance specific to it being a polymer."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstancePolymer {
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
    #[doc = "A business idenfier for this polymer, but typically this is handled by a SubstanceDefinition identifier."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Overall type of the polymer."]
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Polymer geometry, e.g. linear, branched, cross-linked, network or dendritic."]
    pub r#geometry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Descrtibes the copolymer sequence type (polymer connectivity)."]
    pub r#copolymer_connectivity: Vec<super::super::types::CodeableConcept>,
    #[doc = "Todo - this is intended to connect to a repeating full modification structure, also used by Protein and Nucleic Acid . String is just a placeholder."]
    pub r#modification: Option<super::super::types::String>,
    #[doc = "Todo."]
    pub r#monomer_set: Vec<SubstancePolymerMonomerSet>,
    #[doc = "Specifies and quantifies the repeated units and their configuration."]
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
            r#identifier: Default::default(),
            r#class: Default::default(),
            r#geometry: Default::default(),
            r#copolymer_connectivity: Default::default(),
            r#modification: Default::default(),
            r#monomer_set: Default::default(),
            r#repeat: Default::default(),
        }
    }
}
