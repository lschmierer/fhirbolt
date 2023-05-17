// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "The reference sequence that represents the starting sequence."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MolecularSequenceRelativeStartingSequenceSequence {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "A sequence that is used as a starting sequence to describe variants that are present in a sequence analyzed."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceRelativeStartingSequence {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The genome assembly used for starting sequence, e.g. GRCh38."]
    pub r#genome_assembly: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Structural unit composed of a nucleic acid molecule which controls its own replication through the interaction of specific proteins at one or more origins of replication ([SO:0000340](<http://www.sequenceontology.org/browser/current_svn/term/SO:0000340>))."]
    pub r#chromosome: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The reference sequence that represents the starting sequence."]
    pub r#sequence: Option<MolecularSequenceRelativeStartingSequenceSequence>,
    #[doc = "Start position of the window on the starting sequence. This value should honor the rules of the coordinateSystem."]
    pub r#window_start: Option<super::super::types::Integer>,
    #[doc = "End position of the window on the starting sequence. This value should honor the rules of the  coordinateSystem."]
    pub r#window_end: Option<super::super::types::Integer>,
    #[doc = "A relative reference to a DNA strand based on gene orientation. The strand that contains the open reading frame of the gene is the \"sense\" strand, and the opposite complementary strand is the \"antisense\" strand."]
    pub r#orientation: Option<super::super::types::Code>,
    #[doc = "An absolute reference to a strand. The Watson strand is the strand whose 5'-end is on the short arm of the chromosome, and the Crick strand as the one whose 5'-end is on the long arm."]
    pub r#strand: Option<super::super::types::Code>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceRelativeStartingSequence {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#genome_assembly: Default::default(),
            r#chromosome: Default::default(),
            r#sequence: Default::default(),
            r#window_start: Default::default(),
            r#window_end: Default::default(),
            r#orientation: Default::default(),
            r#strand: Default::default(),
        }
    }
}
#[doc = "Changes in sequence from the starting sequence."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceRelativeEdit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Start position of the edit on the starting sequence. If the coordinate system is either 0-based or 1-based, then start position is inclusive."]
    pub r#start: Option<super::super::types::Integer>,
    #[doc = "End position of the edit on the starting sequence. If the coordinate system is 0-based then end is exclusive and does not include the last position. If the coordinate system is 1-base, then end is inclusive and includes the last position."]
    pub r#end: Option<super::super::types::Integer>,
    #[doc = "Allele that was observed. Nucleotide(s)/amino acids from start position of sequence to stop position of sequence on the positive (+) strand of the observed sequence. When the sequence type is DNA, it should be the sequence on the positive (+) strand. This will lay in the range between variant.start and variant.end."]
    pub r#replacement_sequence: Option<super::super::types::String>,
    #[doc = "Allele in the starting sequence. Nucleotide(s)/amino acids from start position of sequence to stop position of sequence on the positive (+) strand of the starting sequence. When the sequence  type is DNA, it should be the sequence on the positive (+) strand. This will lay in the range between variant.start and variant.end."]
    pub r#replaced_sequence: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceRelativeEdit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#start: Default::default(),
            r#end: Default::default(),
            r#replacement_sequence: Default::default(),
            r#replaced_sequence: Default::default(),
        }
    }
}
#[doc = "A sequence defined relative to another sequence."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequenceRelative {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "These are different ways of identifying nucleotides or amino acids within a sequence. Different databases and file types may use different systems. For detail definitions, see <https://loinc.org/92822>-6/ for more detail."]
    pub r#coordinate_system: Box<super::super::types::CodeableConcept>,
    #[doc = "Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together."]
    pub r#ordinal_position: Option<super::super::types::Integer>,
    #[doc = "Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together."]
    pub r#sequence_range: Option<Box<super::super::types::Range>>,
    #[doc = "A sequence that is used as a starting sequence to describe variants that are present in a sequence analyzed."]
    pub r#starting_sequence: Option<MolecularSequenceRelativeStartingSequence>,
    #[doc = "Changes in sequence from the starting sequence."]
    pub r#edit: Vec<MolecularSequenceRelativeEdit>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequenceRelative {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#coordinate_system: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#ordinal_position: Default::default(),
            r#sequence_range: Default::default(),
            r#starting_sequence: Default::default(),
            r#edit: Default::default(),
        }
    }
}
#[doc = "Representation of a molecular sequence."]
#[derive(Debug, Clone, PartialEq)]
pub struct MolecularSequence {
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
    #[doc = "A unique identifier for this particular sequence instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Amino Acid Sequence/ DNA Sequence / RNA Sequence."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Indicates the subject this sequence is associated too."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The actual focus of a molecular sequence when it is not the patient of record representing something or someone associated with the patient such as a spouse, parent, child, or sibling. For example, in trio testing, the subject would be the child (proband) and the focus would be the parent."]
    pub r#focus: Vec<super::super::types::Reference>,
    #[doc = "Specimen used for sequencing."]
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    #[doc = "The method for sequencing, for example, chip information."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "The organization or lab that should be responsible for this result."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Sequence that was observed."]
    pub r#literal: Option<super::super::types::String>,
    #[doc = "Sequence that was observed as file content. Can be an actual file contents, or referenced by a URL to an external system."]
    pub r#formatted: Vec<super::super::types::Attachment>,
    #[doc = "A sequence defined relative to another sequence."]
    pub r#relative: Vec<MolecularSequenceRelative>,
}
#[allow(clippy::derivable_impls)]
impl Default for MolecularSequence {
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
            r#subject: Default::default(),
            r#focus: Default::default(),
            r#specimen: Default::default(),
            r#device: Default::default(),
            r#performer: Default::default(),
            r#literal: Default::default(),
            r#formatted: Default::default(),
            r#relative: Default::default(),
        }
    }
}
