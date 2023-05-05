// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenCollectionFastingStatus {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Duration(Box<super::super::types::Duration>),
    #[default]
    Invalid,
}
#[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    #[default]
    Invalid,
}
#[doc = "A physical feature or landmark on a specimen, highlighted for context by the collector of the specimen (e.g. surgeon), that identifies the type of feature as well as its meaning (e.g. the red ink indicating the resection margin of the right lobe of the excised prostate tissue or wire loop at radiologically suspected tumor location)."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenFeature {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The landmark or feature being highlighted."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Description of the feature of the specimen."]
    pub r#description: super::super::types::String,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenFeature {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#description: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Details concerning the specimen collection."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenCollection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Person who collected the specimen."]
    pub r#collector: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen was collected from subject - the physiologically relevant time."]
    pub r#collected: Option<SpecimenCollectionCollected>,
    #[doc = "The span of time over which the collection of a specimen occurred."]
    pub r#duration: Option<Box<super::super::types::Duration>>,
    #[doc = "The quantity of specimen collected; for instance the volume of a blood sample, or the physical measurement of an anatomic pathology sample."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "A coded value specifying the technique that is used to perform the procedure."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A coded value specifying the technique that is used to perform the procedure."]
    pub r#device: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "The procedure event during which the specimen was collected (e.g. the surgery leading to the collection of a pathology sample)."]
    pub r#procedure: Option<Box<super::super::types::Reference>>,
    #[doc = "Anatomical location from which the specimen was collected (if subject is a patient). This is the target site.  This element is not used for environmental specimens."]
    pub r#body_site: Option<Box<super::super::types::CodeableReference>>,
    #[doc = "Abstinence or reduction from some or all food, drink, or both, for a period of time prior to sample collection."]
    pub r#fasting_status: Option<SpecimenCollectionFastingStatus>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenCollection {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#collector: Default::default(),
            r#collected: Default::default(),
            r#duration: Default::default(),
            r#quantity: Default::default(),
            r#method: Default::default(),
            r#device: Default::default(),
            r#procedure: Default::default(),
            r#body_site: Default::default(),
            r#fasting_status: Default::default(),
        }
    }
}
#[doc = "Details concerning processing and processing steps for the specimen."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenProcessing {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Textual description of procedure."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "A coded value specifying the method used to process the specimen."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Material used in the processing step."]
    pub r#additive: Vec<super::super::types::Reference>,
    #[doc = "A record of the time or period when the specimen processing occurred.  For example the time of sample fixation or the period of time the sample was in formalin."]
    pub r#time: Option<SpecimenProcessingTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenProcessing {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#description: Default::default(),
            r#method: Default::default(),
            r#additive: Default::default(),
            r#time: Default::default(),
        }
    }
}
#[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The device resource for the the container holding the specimen. If the container is in a holder then the referenced device will point to a parent device."]
    pub r#device: Box<super::super::types::Reference>,
    #[doc = "The location of the container holding the specimen."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The quantity of specimen in the container; may be volume, dimensions, or other appropriate measurements, depending on the specimen type."]
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenContainer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#device: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#location: Default::default(),
            r#specimen_quantity: Default::default(),
        }
    }
}
#[doc = "A sample to be used for analysis."]
#[derive(Debug, Clone, PartialEq)]
pub struct Specimen {
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
    #[doc = "Id for specimen."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier assigned by the lab when accessioning specimen(s). This is not necessarily the same as the specimen identifier, depending on local lab procedures."]
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The availability of the specimen."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The kind of material that forms the specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance, a biologically-derived product, or a device."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Time when specimen is received by the testing laboratory for processing or testing."]
    pub r#received_time: Option<super::super::types::DateTime>,
    #[doc = "Reference to the parent (source) specimen which is used when the specimen was either derived from or a component of another specimen."]
    pub r#parent: Vec<super::super::types::Reference>,
    #[doc = "Details concerning a service request that required a specimen to be collected."]
    pub r#request: Vec<super::super::types::Reference>,
    #[doc = "This element signifies if the specimen is part of a group or pooled."]
    pub r#combined: Option<super::super::types::Code>,
    #[doc = "The role or reason for the specimen in the testing workflow."]
    pub r#role: Vec<super::super::types::CodeableConcept>,
    #[doc = "A physical feature or landmark on a specimen, highlighted for context by the collector of the specimen (e.g. surgeon), that identifies the type of feature as well as its meaning (e.g. the red ink indicating the resection margin of the right lobe of the excised prostate tissue or wire loop at radiologically suspected tumor location)."]
    pub r#feature: Vec<SpecimenFeature>,
    #[doc = "Details concerning the specimen collection."]
    pub r#collection: Option<SpecimenCollection>,
    #[doc = "Details concerning processing and processing steps for the specimen."]
    pub r#processing: Vec<SpecimenProcessing>,
    #[doc = "The container holding the specimen.  The recursive nature of containers; i.e. blood in tube in tray in rack is not addressed here."]
    pub r#container: Vec<SpecimenContainer>,
    #[doc = "A mode or state of being that describes the nature of the specimen."]
    pub r#condition: Vec<super::super::types::CodeableConcept>,
    #[doc = "To communicate any details or issues about the specimen or during the specimen collection. (for example: broken vial, sent with patient, frozen)."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for Specimen {
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
            r#accession_identifier: Default::default(),
            r#status: Default::default(),
            r#type: Default::default(),
            r#subject: Default::default(),
            r#received_time: Default::default(),
            r#parent: Default::default(),
            r#request: Default::default(),
            r#combined: Default::default(),
            r#role: Default::default(),
            r#feature: Default::default(),
            r#collection: Default::default(),
            r#processing: Default::default(),
            r#container: Default::default(),
            r#condition: Default::default(),
            r#note: Default::default(),
        }
    }
}
