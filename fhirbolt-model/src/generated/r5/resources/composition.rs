// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "A participant who has attested to the accuracy of the composition/document."]
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionAttester {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of attestation the authenticator offers."]
    pub r#mode: Box<super::super::types::CodeableConcept>,
    #[doc = "When the composition was attested by the party."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "Who attested the composition in the specified way."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for CompositionAttester {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#mode: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#time: Default::default(),
            r#party: Default::default(),
        }
    }
}
#[doc = "The clinical service, such as a colonoscopy or an appendectomy, being documented."]
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionEvent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The period of time covered by the documentation. There is no assertion that the documentation is a complete representation for this period, only that it documents events during this time."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Represents the main clinical acts, such as a colonoscopy or an appendectomy, being documented. In some cases, the event is inherent in the typeCode, such as a \"History and Physical Report\" in which case the procedure being documented is necessarily a \"History and Physical\" act. The events may be included as a code or as a reference to an other resource."]
    pub r#detail: Vec<super::super::types::CodeableReference>,
}
#[allow(clippy::derivable_impls)]
impl Default for CompositionEvent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#period: Default::default(),
            r#detail: Default::default(),
        }
    }
}
#[doc = "The root of the sections that make up the composition."]
#[derive(Debug, Clone, PartialEq)]
pub struct CompositionSection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The label for this particular section.  This will be part of the rendered content for the document, and is often used to build a table of contents."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A code identifying the kind of content contained within the section. This must be consistent with the section title."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Identifies who is responsible for the information in this section, not necessarily who typed it in."]
    pub r#author: Vec<super::super::types::Reference>,
    #[doc = "The actual focus of the section when it is not the subject of the composition, but instead represents something or someone associated with the subject such as (for a patient subject) a spouse, parent, fetus, or donor. If not focus is specified, the focus is assumed to be focus of the parent section, or, for a section in the Composition itself, the subject of the composition. Sections with a focus SHALL only include resources where the logical subject (patient, subject, focus, etc.) matches the section focus, or the resources have no logical subject (few resources)."]
    pub r#focus: Option<Box<super::super::types::Reference>>,
    #[doc = "A human-readable narrative that contains the attested content of the section, used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "Specifies the order applied to the items in the section entries."]
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A reference to the actual resource from which the narrative in the section is derived."]
    pub r#entry: Vec<super::super::types::Reference>,
    #[doc = "If the section is empty, why the list is empty. An empty section typically has some text explaining the empty reason."]
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A nested sub-section within this section."]
    pub r#section: Vec<CompositionSection>,
}
#[allow(clippy::derivable_impls)]
impl Default for CompositionSection {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: Default::default(),
            r#code: Default::default(),
            r#author: Default::default(),
            r#focus: Default::default(),
            r#text: Default::default(),
            r#ordered_by: Default::default(),
            r#entry: Default::default(),
            r#empty_reason: Default::default(),
            r#section: Default::default(),
        }
    }
}
#[doc = "A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).\n\nTo support documents, and also to capture the EN13606 notion of an attested commit to the patient EHR, and to allow a set of disparate resources at the information/engineering level to be gathered into a clinical statement."]
#[derive(Debug, Clone, PartialEq)]
pub struct Composition {
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
    #[doc = "An absolute URI that is used to identify this Composition when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this Composition is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the Composition is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A version-independent identifier for the Composition. This identifier stays constant as the composition is changed over time."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "An explicitly assigned identifer of a variation of the content in the Composition."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "The workflow/clinical status of this composition. The status is a marker for the clinical standing of the document."]
    pub r#status: super::super::types::Code,
    #[doc = "Specifies the particular kind of composition (e.g. History and Physical, Discharge Summary, Progress Note). This usually equates to the purpose of making the composition."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A categorization for the type of the composition - helps for indexing and searching. This may be implied by or derived from the code specified in the Composition Type."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Who or what the composition is about. The composition can be about a person, (patient or healthcare practitioner), a device (e.g. a machine) or even a group of subjects (such as a document about a herd of livestock, or a set of patients that share a common exposure)."]
    pub r#subject: Vec<super::super::types::Reference>,
    #[doc = "Describes the clinical encounter or type of care this documentation is associated with."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The composition editing time, when the composition was last logically changed by the author."]
    pub r#date: super::super::types::DateTime,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate Composition instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "Identifies who is responsible for the information in the composition, not necessarily who typed it in."]
    pub r#author: Vec<super::super::types::Reference>,
    #[doc = "A natural language name identifying the {{title}}. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "Official human-readable label for the composition."]
    pub r#title: super::super::types::String,
    #[doc = "For any additional notes."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "A participant who has attested to the accuracy of the composition/document."]
    pub r#attester: Vec<CompositionAttester>,
    #[doc = "Identifies the organization or group who is responsible for ongoing maintenance of and access to the composition/document information."]
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    #[doc = "Relationships that this composition has with other compositions or documents that already exist."]
    pub r#relates_to: Vec<super::super::types::RelatedArtifact>,
    #[doc = "The clinical service, such as a colonoscopy or an appendectomy, being documented."]
    pub r#event: Vec<CompositionEvent>,
    #[doc = "The root of the sections that make up the composition."]
    pub r#section: Vec<CompositionSection>,
}
#[allow(clippy::derivable_impls)]
impl Default for Composition {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#category: Default::default(),
            r#subject: Default::default(),
            r#encounter: Default::default(),
            r#date: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#use_context: Default::default(),
            r#author: Default::default(),
            r#name: Default::default(),
            r#title: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#note: Default::default(),
            r#attester: Default::default(),
            r#custodian: Default::default(),
            r#relates_to: Default::default(),
            r#event: Default::default(),
            r#section: Default::default(),
        }
    }
}
