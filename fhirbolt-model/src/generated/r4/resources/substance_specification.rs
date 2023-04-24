// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Quantitative value for this moiety."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationPropertyDefiningSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Quantitative value for this property."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationPropertyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationRelationshipSubstance {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceSpecificationRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Moiety, for structural modifications."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationMoiety {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Role that the moiety is playing."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifier by which this moiety substance is known."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Textual name for this moiety substance."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceSpecificationMoietyAmount>,
}
impl Default for SubstanceSpecificationMoiety {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#role: Default::default(),
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#stereochemistry: Default::default(),
            r#optical_activity: Default::default(),
            r#molecular_formula: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "General specifications for this substance, including how it is related to other substances."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A category for this property, e.g. Physical, Chemical, Enzymatic."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Property type e.g. viscosity, pH, isoelectric point."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Parameters that were used in the measurement of a property (e.g. for viscosity: measured at 20C with a pH of 7.1)."]
    pub r#parameters: Option<super::super::types::String>,
    #[doc = "A substance upon which a defining property depends (e.g. for solubility: in water, in alcohol)."]
    pub r#defining_substance: Option<SubstanceSpecificationPropertyDefiningSubstance>,
    #[doc = "Quantitative value for this property."]
    pub r#amount: Option<SubstanceSpecificationPropertyAmount>,
}
impl Default for SubstanceSpecificationProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: Default::default(),
            r#code: Default::default(),
            r#parameters: Default::default(),
            r#defining_substance: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureIsotopeMolecularWeight {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The method by which the molecular weight was determined."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of molecular weight such as exact, average (also known as. number average), weight average."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
impl Default for SubstanceSpecificationStructureIsotopeMolecularWeight {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#method: Default::default(),
            r#type: Default::default(),
            r#amount: Default::default(),
        }
    }
}
#[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureIsotope {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Substance identifier for each non-natural or radioisotope."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Substance name for each non-natural or radioisotope."]
    pub r#name: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of isotopic substitution present in a single substance."]
    pub r#substitution: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Half life - for a non-natural nuclide."]
    pub r#half_life: Option<Box<super::super::types::Quantity>>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
}
impl Default for SubstanceSpecificationStructureIsotope {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#substitution: Default::default(),
            r#half_life: Default::default(),
            r#molecular_weight: Default::default(),
        }
    }
}
#[doc = "Molecular structural representation."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of structure (e.g. Full, Partial, Representative)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation as text string in a format e.g. InChI, SMILES, MOLFILE, CDX."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "An attached file with the structural representation."]
    pub r#attachment: Option<Box<super::super::types::Attachment>>,
}
impl Default for SubstanceSpecificationStructureRepresentation {
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
#[doc = "Structural information."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "Applicable for single substances that contain a radionuclide or a non-natural isotopic ratio."]
    pub r#isotope: Vec<SubstanceSpecificationStructureIsotope>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Molecular structural representation."]
    pub r#representation: Vec<SubstanceSpecificationStructureRepresentation>,
}
impl Default for SubstanceSpecificationStructure {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#stereochemistry: Default::default(),
            r#optical_activity: Default::default(),
            r#molecular_formula: Default::default(),
            r#molecular_formula_by_moiety: Default::default(),
            r#isotope: Default::default(),
            r#molecular_weight: Default::default(),
            r#source: Default::default(),
            r#representation: Default::default(),
        }
    }
}
#[doc = "Codes associated with the substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status is changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl Default for SubstanceSpecificationCode {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#status: Default::default(),
            r#status_date: Default::default(),
            r#comment: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "Details of the official nature of this name."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl Default for SubstanceSpecificationNameOfficial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#authority: Default::default(),
            r#status: Default::default(),
            r#date: Default::default(),
        }
    }
}
#[doc = "Names applicable to this substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Language of the name."]
    pub r#language: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A synonym of this name."]
    pub r#synonym: Vec<SubstanceSpecificationName>,
    #[doc = "A translation for this name."]
    pub r#translation: Vec<SubstanceSpecificationName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceSpecificationNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl Default for SubstanceSpecificationName {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#type: Default::default(),
            r#status: Default::default(),
            r#preferred: Default::default(),
            r#language: Default::default(),
            r#domain: Default::default(),
            r#jurisdiction: Default::default(),
            r#synonym: Default::default(),
            r#translation: Default::default(),
            r#official: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "A link between this substance and another, with details of the relationship."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecificationRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance: Option<SubstanceSpecificationRelationshipSubstance>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\"."]
    pub r#relationship: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceSpecificationRelationshipAmount>,
    #[doc = "For use when the numeric."]
    pub r#amount_ratio_low_limit: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#amount_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
}
impl Default for SubstanceSpecificationRelationship {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#substance: Default::default(),
            r#relationship: Default::default(),
            r#is_defining: Default::default(),
            r#amount: Default::default(),
            r#amount_ratio_low_limit: Default::default(),
            r#amount_type: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSpecification {
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
    #[doc = "Identifier by which this substance is known."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "High level categorization, e.g. polymer or nucleic acid."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of substance within the catalogue e.g. approved."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If the substance applies to only human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<Box<super::super::types::Reference>>,
    #[doc = "Textual comment about this record of a substance."]
    pub r#comment: Option<super::super::types::String>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceSpecificationMoiety>,
    #[doc = "General specifications for this substance, including how it is related to other substances."]
    pub r#property: Vec<SubstanceSpecificationProperty>,
    #[doc = "General information detailing this substance."]
    pub r#reference_information: Option<Box<super::super::types::Reference>>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceSpecificationStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceSpecificationCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceSpecificationName>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceSpecificationStructureIsotopeMolecularWeight>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceSpecificationRelationship>,
    #[doc = "Data items specific to nucleic acids."]
    pub r#nucleic_acid: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to polymers."]
    pub r#polymer: Option<Box<super::super::types::Reference>>,
    #[doc = "Data items specific to proteins."]
    pub r#protein: Option<Box<super::super::types::Reference>>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<Box<super::super::types::Reference>>,
}
impl Default for SubstanceSpecification {
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
            r#type: Default::default(),
            r#status: Default::default(),
            r#domain: Default::default(),
            r#description: Default::default(),
            r#source: Default::default(),
            r#comment: Default::default(),
            r#moiety: Default::default(),
            r#property: Default::default(),
            r#reference_information: Default::default(),
            r#structure: Default::default(),
            r#code: Default::default(),
            r#name: Default::default(),
            r#molecular_weight: Default::default(),
            r#relationship: Default::default(),
            r#nucleic_acid: Default::default(),
            r#polymer: Default::default(),
            r#protein: Default::default(),
            r#source_material: Default::default(),
        }
    }
}
