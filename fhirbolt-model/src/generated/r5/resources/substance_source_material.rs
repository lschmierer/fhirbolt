// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialFractionDescription {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "This element is capturing information about the fraction of a plant part, or human plasma for fractionation."]
    pub r#fraction: Option<super::super::types::String>,
    #[doc = "The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1."]
    pub r#material_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialFractionDescription {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#fraction: Default::default(),
            r#material_type: Default::default(),
        }
    }
}
#[doc = "4.9.13.6.1 Author type (Conditional)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganismAuthor {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of author of an organism species shall be specified. The parenthetical author of an organism species refers to the first author who published the plant/animal name (of any rank). The primary author of an organism species refers to the first author(s), who validly published the plant/animal name."]
    pub r#author_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable; refers to the year in which the first author(s) published the infraspecific plant/animal name (of any rank)."]
    pub r#author_description: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialOrganismAuthor {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#author_type: Default::default(),
            r#author_description: Default::default(),
        }
    }
}
#[doc = "4.9.13.8.1 Hybrid species maternal organism ID (Optional)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganismHybrid {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal."]
    pub r#maternal_organism_id: Option<super::super::types::String>,
    #[doc = "The name of the maternal species constituting the hybrid organism shall be specified. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal."]
    pub r#maternal_organism_name: Option<super::super::types::String>,
    #[doc = "The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary."]
    pub r#paternal_organism_id: Option<super::super::types::String>,
    #[doc = "The name of the paternal species constituting the hybrid organism shall be specified."]
    pub r#paternal_organism_name: Option<super::super::types::String>,
    #[doc = "The hybrid type of an organism shall be specified."]
    pub r#hybrid_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialOrganismHybrid {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#maternal_organism_id: Default::default(),
            r#maternal_organism_name: Default::default(),
            r#paternal_organism_id: Default::default(),
            r#paternal_organism_name: Default::default(),
            r#hybrid_type: Default::default(),
        }
    }
}
#[doc = "4.9.13.7.1 Kingdom (Conditional)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kingdom of an organism shall be specified."]
    pub r#kingdom: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The phylum of an organism shall be specified."]
    pub r#phylum: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The class of an organism shall be specified."]
    pub r#class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The order of an organism shall be specified,."]
    pub r#order: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialOrganismOrganismGeneral {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#kingdom: Default::default(),
            r#phylum: Default::default(),
            r#class: Default::default(),
            r#order: Default::default(),
        }
    }
}
#[doc = "This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialOrganism {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The family of an organism shall be specified."]
    pub r#family: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name; it is present in names for genera, species and infraspecies."]
    pub r#genus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal; it is present in names for species and infraspecies."]
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The Intraspecific type of an organism shall be specified."]
    pub r#intraspecific_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The intraspecific description of an organism shall be specified based on a controlled vocabulary. For Influenza Vaccine, the intraspecific description shall contain the syntax of the antigen in line with the WHO convention."]
    pub r#intraspecific_description: Option<super::super::types::String>,
    #[doc = "4.9.13.6.1 Author type (Conditional)."]
    pub r#author: Vec<SubstanceSourceMaterialOrganismAuthor>,
    #[doc = "4.9.13.8.1 Hybrid species maternal organism ID (Optional)."]
    pub r#hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,
    #[doc = "4.9.13.7.1 Kingdom (Conditional)."]
    pub r#organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialOrganism {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#family: Default::default(),
            r#genus: Default::default(),
            r#species: Default::default(),
            r#intraspecific_type: Default::default(),
            r#intraspecific_description: Default::default(),
            r#author: Default::default(),
            r#hybrid: Default::default(),
            r#organism_general: Default::default(),
        }
    }
}
#[doc = "To do."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterialPartDescription {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Entity of anatomical origin of source material within an organism."]
    pub r#part: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply."]
    pub r#part_location: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterialPartDescription {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#part: Default::default(),
            r#part_location: Default::default(),
        }
    }
}
#[doc = "Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceSourceMaterial {
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
    #[doc = "General high level classification of the source material specific to the origin of the material."]
    pub r#source_material_class: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent."]
    pub r#source_material_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The state of the source material when extracted."]
    pub r#source_material_state: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The unique identifier associated with the source material parent organism shall be specified."]
    pub r#organism_id: Option<Box<super::super::types::Identifier>>,
    #[doc = "The organism accepted Scientific name shall be provided based on the organism taxonomy."]
    pub r#organism_name: Option<super::super::types::String>,
    #[doc = "The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)."]
    pub r#parent_substance_id: Vec<super::super::types::Identifier>,
    #[doc = "The parent substance of the Herbal Drug, or Herbal preparation."]
    pub r#parent_substance_name: Vec<super::super::types::String>,
    #[doc = "The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File. For “Plasma-derived substances” the attribute country of origin provides information about the countries used for the manufacturing of the Cryopoor plama or Crioprecipitate."]
    pub r#country_of_origin: Vec<super::super::types::CodeableConcept>,
    #[doc = "The place/region where the plant is harvested or the places/regions where the animal source material has its habitat."]
    pub r#geographical_location: Vec<super::super::types::String>,
    #[doc = "Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)."]
    pub r#development_stage: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels."]
    pub r#fraction_description: Vec<SubstanceSourceMaterialFractionDescription>,
    #[doc = "This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf."]
    pub r#organism: Option<SubstanceSourceMaterialOrganism>,
    #[doc = "To do."]
    pub r#part_description: Vec<SubstanceSourceMaterialPartDescription>,
}
#[allow(clippy::derivable_impls)]
impl Default for SubstanceSourceMaterial {
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
            r#source_material_class: Default::default(),
            r#source_material_type: Default::default(),
            r#source_material_state: Default::default(),
            r#organism_id: Default::default(),
            r#organism_name: Default::default(),
            r#parent_substance_id: Default::default(),
            r#parent_substance_name: Default::default(),
            r#country_of_origin: Default::default(),
            r#geographical_location: Default::default(),
            r#development_stage: Default::default(),
            r#fraction_description: Default::default(),
            r#organism: Default::default(),
            r#part_description: Default::default(),
        }
    }
}
