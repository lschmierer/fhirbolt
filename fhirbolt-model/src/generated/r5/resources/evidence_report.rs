// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Citation Resource or display of suggested citation for this report."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceReportCiteAs {
    Reference(Box<super::super::types::Reference>),
    Markdown(Box<super::super::types::Markdown>),
    #[default]
    Invalid,
}
#[doc = "Characteristic value."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum EvidenceReportSubjectCharacteristicValue {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    #[default]
    Invalid,
}
#[doc = "Characteristic."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportSubjectCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Characteristic code."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Characteristic value."]
    pub r#value: EvidenceReportSubjectCharacteristicValue,
    #[doc = "Is used to express not the characteristic."]
    pub r#exclude: Option<super::super::types::Boolean>,
    #[doc = "Timeframe for the characteristic."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReportSubjectCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
            r#exclude: Default::default(),
            r#period: Default::default(),
        }
    }
}
#[doc = "Specifies the subject or focus of the report. Answers \"What is this report about?\"."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportSubject {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Characteristic."]
    pub r#characteristic: Vec<EvidenceReportSubjectCharacteristic>,
    #[doc = "Used for general notes and annotations not coded elsewhere."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReportSubject {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#characteristic: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "The target composition/document of this relationship."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportRelatesToTarget {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Target of the relationship URL."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "Target of the relationship Identifier."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Target of the relationship Display."]
    pub r#display: Option<super::super::types::Markdown>,
    #[doc = "Target of the relationship Resource reference."]
    pub r#resource: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReportRelatesToTarget {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#url: Default::default(),
            r#identifier: Default::default(),
            r#display: Default::default(),
            r#resource: Default::default(),
        }
    }
}
#[doc = "Relationships that this composition has with other compositions or documents that already exist."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of relationship that this composition has with anther composition or document."]
    pub r#code: super::super::types::Code,
    #[doc = "The target composition/document of this relationship."]
    pub r#target: EvidenceReportRelatesToTarget,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReportRelatesTo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#target: EvidenceReportRelatesToTarget {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The root of the sections that make up the composition."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReportSection {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The label for this particular section.  This will be part of the rendered content for the document, and is often used to build a table of contents."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A code identifying the kind of content contained within the section. This should be consistent with the section title."]
    pub r#focus: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A definitional Resource identifying the kind of content contained within the section. This should be consistent with the section title."]
    pub r#focus_reference: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies who is responsible for the information in this section, not necessarily who typed it in."]
    pub r#author: Vec<super::super::types::Reference>,
    #[doc = "A human-readable narrative that contains the attested content of the section, used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is peferred to contain sufficient detail to make it acceptable for a human to just read the narrative."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "How the entry list was prepared - whether it is a working list that is suitable for being maintained on an ongoing basis, or if it represents a snapshot of a list of items from another source, or whether it is a prepared list where items may be marked as added, modified or deleted."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "Specifies the order applied to the items in the section entries."]
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specifies any type of classification of the evidence report."]
    pub r#entry_classifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "A reference to the actual resource from which the narrative in the section is derived."]
    pub r#entry_reference: Vec<super::super::types::Reference>,
    #[doc = "Quantity as content."]
    pub r#entry_quantity: Vec<super::super::types::Quantity>,
    #[doc = "If the section is empty, why the list is empty. An empty section typically has some text explaining the empty reason."]
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A nested sub-section within this section."]
    pub r#section: Vec<EvidenceReportSection>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReportSection {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#title: Default::default(),
            r#focus: Default::default(),
            r#focus_reference: Default::default(),
            r#author: Default::default(),
            r#text: Default::default(),
            r#mode: Default::default(),
            r#ordered_by: Default::default(),
            r#entry_classifier: Default::default(),
            r#entry_reference: Default::default(),
            r#entry_quantity: Default::default(),
            r#empty_reason: Default::default(),
            r#section: Default::default(),
        }
    }
}
#[doc = "The EvidenceReport Resource is a specialized container for a collection of resources and codeable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts."]
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceReport {
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
    #[doc = "An absolute URI that is used to identify this EvidenceReport when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate evidence report instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A formal identifier that is used to identify this EvidenceReport when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A formal identifier that is used to identify things closely related to this EvidenceReport."]
    pub r#related_identifier: Vec<super::super::types::Identifier>,
    #[doc = "Citation Resource or display of suggested citation for this report."]
    pub r#cite_as: Option<EvidenceReportCiteAs>,
    #[doc = "Specifies the kind of report, such as grouping of classifiers, search results, or human-compiled expression."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used for footnotes and annotations."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "Link, description or reference to artifact associated with the report."]
    pub r#related_artifact: Vec<super::super::types::RelatedArtifact>,
    #[doc = "Specifies the subject or focus of the report. Answers \"What is this report about?\"."]
    pub r#subject: EvidenceReportSubject,
    #[doc = "The name of the organization or individual responsible for the release and ongoing maintenance of the evidence report."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily involved in the creation and maintenance of the content."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily responsible for internal coherence of the content."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device primarily responsible for review of some aspect of the content."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "An individiual, organization, or device responsible for officially endorsing the content for use in some setting."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "Relationships that this composition has with other compositions or documents that already exist."]
    pub r#relates_to: Vec<EvidenceReportRelatesTo>,
    #[doc = "The root of the sections that make up the composition."]
    pub r#section: Vec<EvidenceReportSection>,
}
#[allow(clippy::derivable_impls)]
impl Default for EvidenceReport {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#use_context: Default::default(),
            r#identifier: Default::default(),
            r#related_identifier: Default::default(),
            r#cite_as: Default::default(),
            r#type: Default::default(),
            r#note: Default::default(),
            r#related_artifact: Default::default(),
            r#subject: EvidenceReportSubject {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#relates_to: Default::default(),
            r#section: Default::default(),
        }
    }
}
