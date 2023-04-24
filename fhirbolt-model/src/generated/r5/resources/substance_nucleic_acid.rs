// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The linkages between sugar residues will also be captured."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidSubunitLinkage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The entity that links the sugar residues together should also be captured for nearly all naturally occurring nucleic acid the linkage is a phosphate group. For many synthetic oligonucleotides phosphorothioate linkages are often seen. Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or 5’-5’ this should be specified."]
    pub r#connectivity: Option<super::super::types::String>,
    #[doc = "Each linkage will be registered as a fragment and have an ID."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Each linkage will be registered as a fragment and have at least one name. A single name shall be assigned to each linkage."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Residues shall be captured as described in 5.3.6.8.3."]
    pub r#residue_site: Option<super::super::types::String>,
}
impl Default for SubstanceNucleicAcidSubunitLinkage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#connectivity: Default::default(),
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#residue_site: Default::default(),
        }
    }
}
#[doc = "5.3.6.8.1 Sugar ID (Mandatory)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidSubunitSugar {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The Substance ID of the sugar or sugar-like component that make up the nucleotide."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The name of the sugar or sugar-like component that make up the nucleotide."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "The residues that contain a given sugar will be captured. The order of given residues will be captured in the 5‘-3‘direction consistent with the base sequences listed above."]
    pub r#residue_site: Option<super::super::types::String>,
}
impl Default for SubstanceNucleicAcidSubunitSugar {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#name: Default::default(),
            r#residue_site: Default::default(),
        }
    }
}
#[doc = "Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcidSubunit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Index of linear sequences of nucleic acids in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts."]
    pub r#subunit: Option<super::super::types::Integer>,
    #[doc = "Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes. In addition to the base sequence, sugar and type of phosphate or non-phosphate linkage should also be captured."]
    pub r#sequence: Option<super::super::types::String>,
    #[doc = "The length of the sequence shall be captured."]
    pub r#length: Option<super::super::types::Integer>,
    #[doc = "(TBC)."]
    pub r#sequence_attachment: Option<Box<super::super::types::Attachment>>,
    #[doc = "The nucleotide present at the 5’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the first position in the sequence. A separate representation would be redundant."]
    pub r#five_prime: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The nucleotide present at the 3’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the last position in the sequence. A separate representation would be redundant."]
    pub r#three_prime: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The linkages between sugar residues will also be captured."]
    pub r#linkage: Vec<SubstanceNucleicAcidSubunitLinkage>,
    #[doc = "5.3.6.8.1 Sugar ID (Mandatory)."]
    pub r#sugar: Vec<SubstanceNucleicAcidSubunitSugar>,
}
impl Default for SubstanceNucleicAcidSubunit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#subunit: Default::default(),
            r#sequence: Default::default(),
            r#length: Default::default(),
            r#sequence_attachment: Default::default(),
            r#five_prime: Default::default(),
            r#three_prime: Default::default(),
            r#linkage: Default::default(),
            r#sugar: Default::default(),
        }
    }
}
#[doc = "Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction."]
#[derive(Debug, Clone, PartialEq)]
pub struct SubstanceNucleicAcid {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of the sequence shall be specified based on a controlled vocabulary."]
    pub r#sequence_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of linear sequences of nucleotides linked through phosphodiester bonds shall be described. Subunits would be strands of nucleic acids that are tightly associated typically through Watson-Crick base pairing. NOTE: If not specified in the reference source, the assumption is that there is 1 subunit."]
    pub r#number_of_subunits: Option<super::super::types::Integer>,
    #[doc = "The area of hybridisation shall be described if applicable for double stranded RNA or DNA. The number associated with the subunit followed by the number associated to the residue shall be specified in increasing order. The underscore “” shall be used as separator as follows: “Subunitnumber Residue”."]
    pub r#area_of_hybridisation: Option<super::super::types::String>,
    #[doc = "(TBC)."]
    pub r#oligo_nucleotide_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times."]
    pub r#subunit: Vec<SubstanceNucleicAcidSubunit>,
}
impl Default for SubstanceNucleicAcid {
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
            r#sequence_type: Default::default(),
            r#number_of_subunits: Default::default(),
            r#area_of_hybridisation: Default::default(),
            r#oligo_nucleotide_type: Default::default(),
            r#subunit: Default::default(),
        }
    }
}
