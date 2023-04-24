// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Quantitative value for this moiety."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionMoietyAmount {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "A value for the property."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "A pointer to another substance, as a resource or just a representational code."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipSubstanceDefinition {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SubstanceDefinitionRelationshipAmount {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Moiety, for structural modifications."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMoiety {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
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
    #[doc = "Molecular formula for this moiety of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Quantitative value for this moiety."]
    pub r#amount: Option<SubstanceDefinitionMoietyAmount>,
    #[doc = "The measurement type of the quantitative value. In capturing the actual relative amounts of substances or molecular fragments it may be necessary to indicate whether the amount refers to, for example, a mole ratio or weight ratio."]
    pub r#measurement_type: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for SubstanceDefinitionMoiety {
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
            r#measurement_type: Default::default(),
        }
    }
}
#[doc = "General specifications for this substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code expressing the type of property."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the property."]
    pub r#value: Option<SubstanceDefinitionPropertyValue>,
}
impl Default for SubstanceDefinitionProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#value: Default::default(),
        }
    }
}
#[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionMolecularWeight {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The method by which the molecular weight was determined."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Type of molecular weight such as exact, average (also known as. number average), weight average."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to capture quantitative values for a variety of elements. If only limits are given, the arithmetic mean would be the average. If only a single definite value for a given element is given, it would be captured in this field."]
    pub r#amount: Box<super::super::types::Quantity>,
}
impl Default for SubstanceDefinitionMolecularWeight {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#method: Default::default(),
            r#type: Default::default(),
            r#amount: {
                let mut default: Box<super::super::types::Quantity> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "A depiction of the structure or characterization of the substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructureRepresentation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of structural representation (e.g. full, partial)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The structural representation or characterization as a text string in a standard format."]
    pub r#representation: Option<super::super::types::String>,
    #[doc = "The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF. The logical content type rather than the physical file format of a document."]
    pub r#format: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An attached file with the structural representation or characterization e.g. a molecular structure graphic of the substance, a JCAMP or AnIML file."]
    pub r#document: Option<Box<super::super::types::Reference>>,
}
impl Default for SubstanceDefinitionStructureRepresentation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#representation: Default::default(),
            r#format: Default::default(),
            r#document: Default::default(),
        }
    }
}
#[doc = "Structural information."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionStructure {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Stereochemistry type."]
    pub r#stereochemistry: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optical activity type."]
    pub r#optical_activity: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Molecular formula of this substance, typically using the Hill system."]
    pub r#molecular_formula: Option<super::super::types::String>,
    #[doc = "Specified per moiety according to the Hill system, i.e. first C, then H, then alphabetical, each moiety separated by a dot."]
    pub r#molecular_formula_by_moiety: Option<super::super::types::String>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Option<SubstanceDefinitionMolecularWeight>,
    #[doc = "The method used to elucidate the structure or characterization of the drug substance. Examples: X-ray, HPLC, NMR, Peptide mapping, Ligand binding assay."]
    pub r#technique: Vec<super::super::types::CodeableConcept>,
    #[doc = "The source of information about the structure."]
    pub r#source_document: Vec<super::super::types::Reference>,
    #[doc = "A depiction of the structure or characterization of the substance."]
    pub r#representation: Vec<SubstanceDefinitionStructureRepresentation>,
}
impl Default for SubstanceDefinitionStructure {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#stereochemistry: Default::default(),
            r#optical_activity: Default::default(),
            r#molecular_formula: Default::default(),
            r#molecular_formula_by_moiety: Default::default(),
            r#molecular_weight: Default::default(),
            r#technique: Default::default(),
            r#source_document: Default::default(),
            r#representation: Default::default(),
        }
    }
}
#[doc = "Codes associated with the substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionCode {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The specific code."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Status of the code assignment, for example 'provisional', 'approved'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The date at which the code status was changed as part of the terminology maintenance."]
    pub r#status_date: Option<super::super::types::DateTime>,
    #[doc = "Any comment can be provided in this field, if necessary."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<super::super::types::Reference>,
}
impl Default for SubstanceDefinitionCode {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Default::default(),
            r#status: Default::default(),
            r#status_date: Default::default(),
            r#note: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "Details of the official nature of this name."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionNameOfficial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Which authority uses this official name."]
    pub r#authority: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the official name, for example 'draft', 'active', 'retired'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Date of the official name change."]
    pub r#date: Option<super::super::types::DateTime>,
}
impl Default for SubstanceDefinitionNameOfficial {
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
pub struct SubstanceDefinitionName {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual name."]
    pub r#name: super::super::types::String,
    #[doc = "Name type, for example 'systematic',  'scientific, 'brand'."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the name, for example 'current', 'proposed'."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If this is the preferred name for this substance."]
    pub r#preferred: Option<super::super::types::Boolean>,
    #[doc = "Human language that the name is written in."]
    pub r#language: Vec<super::super::types::CodeableConcept>,
    #[doc = "The use context of this name for example if there is a different name a drug active ingredient as opposed to a food colour additive."]
    pub r#domain: Vec<super::super::types::CodeableConcept>,
    #[doc = "The jurisdiction where this name applies."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "A synonym of this particular name, by which the substance is also known."]
    pub r#synonym: Vec<SubstanceDefinitionName>,
    #[doc = "A translation for this name into another human language."]
    pub r#translation: Vec<SubstanceDefinitionName>,
    #[doc = "Details of the official nature of this name."]
    pub r#official: Vec<SubstanceDefinitionNameOfficial>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<super::super::types::Reference>,
}
impl Default for SubstanceDefinitionName {
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
pub struct SubstanceDefinitionRelationship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A pointer to another substance, as a resource or just a representational code."]
    pub r#substance_definition: Option<SubstanceDefinitionRelationshipSubstanceDefinition>,
    #[doc = "For example \"salt to parent\", \"active moiety\", \"starting material\", \"polymorph\", \"impurity of\"."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible substance relationships."]
    pub r#is_defining: Option<super::super::types::Boolean>,
    #[doc = "A numeric factor for the relationship, for instance to express that the salt of a substance has some percentage of the active substance in relation to some other."]
    pub r#amount: Option<SubstanceDefinitionRelationshipAmount>,
    #[doc = "For use when the numeric has an uncertain range."]
    pub r#ratio_high_limit_amount: Option<Box<super::super::types::Ratio>>,
    #[doc = "An operator for the amount, for example \"average\", \"approximately\", \"less than\"."]
    pub r#comparator: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Supporting literature."]
    pub r#source: Vec<super::super::types::Reference>,
}
impl Default for SubstanceDefinitionRelationship {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#substance_definition: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#is_defining: Default::default(),
            r#amount: Default::default(),
            r#ratio_high_limit_amount: Default::default(),
            r#comparator: Default::default(),
            r#source: Default::default(),
        }
    }
}
#[doc = "Material or taxonomic/anatomical source for the substance."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinitionSourceMaterial {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A classification that provides the origin of the raw material. Example: cat hair would be an Animal source type."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The genus of an organism, typically referring to the Latin epithet of the genus element of the plant/animal scientific name."]
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The species of an organism, typically referring to the Latin epithet of the species of the plant/animal."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "An anatomical origin of the source material within an organism."]
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The country or countries where the material is harvested."]
    pub r#country_of_origin: Vec<super::super::types::CodeableConcept>,
}
impl Default for SubstanceDefinitionSourceMaterial {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#genus: Default::default(),
            r#species: Default::default(),
            r#part: Default::default(),
            r#country_of_origin: Default::default(),
        }
    }
}
#[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceDefinition {
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
    #[doc = "Identifier by which this substance is known."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A business level version identifier of the substance."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Status of substance within the catalogue e.g. active, retired."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A high level categorization, e.g. polymer or nucleic acid, or food, chemical, biological, or a lower level such as the general types of polymer (linear or branch chain) or type of impurity (process related or contaminant)."]
    pub r#classification: Vec<super::super::types::CodeableConcept>,
    #[doc = "If the substance applies to human or veterinary use."]
    pub r#domain: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The quality standard, established benchmark, to which substance complies (e.g. USP/NF, Ph. Eur, JP, BP, Company Standard)."]
    pub r#grade: Vec<super::super::types::CodeableConcept>,
    #[doc = "Textual description of the substance."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Supporting literature."]
    pub r#information_source: Vec<super::super::types::Reference>,
    #[doc = "Textual comment about the substance's catalogue or registry record."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The entity that creates, makes, produces or fabricates the substance. This is a set of potential manufacturers but is not necessarily comprehensive."]
    pub r#manufacturer: Vec<super::super::types::Reference>,
    #[doc = "An entity that is the source for the substance. It may be different from the manufacturer. Supplier is synonymous to a distributor."]
    pub r#supplier: Vec<super::super::types::Reference>,
    #[doc = "Moiety, for structural modifications."]
    pub r#moiety: Vec<SubstanceDefinitionMoiety>,
    #[doc = "General specifications for this substance."]
    pub r#property: Vec<SubstanceDefinitionProperty>,
    #[doc = "The molecular weight or weight range (for proteins, polymers or nucleic acids)."]
    pub r#molecular_weight: Vec<SubstanceDefinitionMolecularWeight>,
    #[doc = "Structural information."]
    pub r#structure: Option<SubstanceDefinitionStructure>,
    #[doc = "Codes associated with the substance."]
    pub r#code: Vec<SubstanceDefinitionCode>,
    #[doc = "Names applicable to this substance."]
    pub r#name: Vec<SubstanceDefinitionName>,
    #[doc = "A link between this substance and another, with details of the relationship."]
    pub r#relationship: Vec<SubstanceDefinitionRelationship>,
    #[doc = "Material or taxonomic/anatomical source for the substance."]
    pub r#source_material: Option<SubstanceDefinitionSourceMaterial>,
}
impl Default for SubstanceDefinition {
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
            r#version: Default::default(),
            r#status: Default::default(),
            r#classification: Default::default(),
            r#domain: Default::default(),
            r#grade: Default::default(),
            r#description: Default::default(),
            r#information_source: Default::default(),
            r#note: Default::default(),
            r#manufacturer: Default::default(),
            r#supplier: Default::default(),
            r#moiety: Default::default(),
            r#property: Default::default(),
            r#molecular_weight: Default::default(),
            r#structure: Default::default(),
            r#code: Default::default(),
            r#name: Default::default(),
            r#relationship: Default::default(),
            r#source_material: Default::default(),
        }
    }
}
