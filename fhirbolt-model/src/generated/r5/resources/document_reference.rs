// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "Code|uri|canonical."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DocumentReferenceContentProfileValue {
    Coding(Box<super::super::types::Coding>),
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "A participant who has authenticated the accuracy of the document."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceAttester {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of attestation the authenticator offers."]
    pub r#mode: Box<super::super::types::CodeableConcept>,
    #[doc = "When the document was attested by the party."]
    pub r#time: Option<super::super::types::DateTime>,
    #[doc = "Who attested the document in the specified way."]
    pub r#party: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentReferenceAttester {
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
#[doc = "Relationships that this document has with other document references that already exist."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of relationship that this document has with anther document."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The target document of this relationship."]
    pub r#target: Box<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentReferenceRelatesTo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#target: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "An identifier of the document constraints, encoding, structure, and template that the document conforms to beyond the base format indicated in the mimeType."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceContentProfile {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code|uri|canonical."]
    pub r#value: DocumentReferenceContentProfileValue,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentReferenceContentProfile {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "The document and format referenced.  If there are multiple content element repetitions, these must all represent the same document in different format, or attachment metadata."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReferenceContent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The document or URL of the document along with critical metadata to prove content has integrity."]
    pub r#attachment: Box<super::super::types::Attachment>,
    #[doc = "An identifier of the document constraints, encoding, structure, and template that the document conforms to beyond the base format indicated in the mimeType."]
    pub r#profile: Vec<DocumentReferenceContentProfile>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentReferenceContent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#attachment: Box::new(super::super::types::Attachment {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#profile: Default::default(),
        }
    }
}
#[doc = "A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference."]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentReference {
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
    #[doc = "Other business identifiers associated with the document, including version independent identifiers."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "An explicitly assigned identifer of a variation of the content in the DocumentReference."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A procedure that is fulfilled in whole or in part by the creation of this media."]
    pub r#based_on: Vec<super::super::types::Reference>,
    #[doc = "The status of this document reference."]
    pub r#status: super::super::types::Code,
    #[doc = "The status of the underlying document."]
    pub r#doc_status: Option<super::super::types::Code>,
    #[doc = "Imaging modality used. This may include both acquisition and non-acquisition modalities."]
    pub r#modality: Vec<super::super::types::CodeableConcept>,
    #[doc = "Specifies the particular kind of document referenced  (e.g. History and Physical, Discharge Summary, Progress Note). This usually equates to the purpose of making the document referenced."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A categorization for the type of document referenced - helps for indexing and searching. This may be implied by or derived from the code specified in the DocumentReference.type."]
    pub r#category: Vec<super::super::types::CodeableConcept>,
    #[doc = "Who or what the document is about. The document can be about a person, (patient or healthcare practitioner), a device (e.g. a machine) or even a group of subjects (such as a document about a herd of farm animals, or a set of patients that share a common exposure)."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the clinical encounter or type of care that the document content is associated with."]
    pub r#context: Vec<super::super::types::Reference>,
    #[doc = "This list of codes represents the main clinical acts, such as a colonoscopy or an appendectomy, being documented. In some cases, the event is inherent in the type Code, such as a \"History and Physical Report\" in which the procedure being documented is necessarily a \"History and Physical\" act."]
    pub r#event: Vec<super::super::types::CodeableReference>,
    #[doc = "The anatomic structures included in the document."]
    pub r#body_site: Vec<super::super::types::CodeableReference>,
    #[doc = "The kind of facility where the patient was seen."]
    pub r#facility_type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This property may convey specifics about the practice setting where the content was created, often reflecting the clinical specialty."]
    pub r#practice_setting: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The time period over which the service that is described by the document was provided."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "When the document reference was created."]
    pub r#date: Option<super::super::types::Instant>,
    #[doc = "Identifies who is responsible for adding the information to the document."]
    pub r#author: Vec<super::super::types::Reference>,
    #[doc = "A participant who has authenticated the accuracy of the document."]
    pub r#attester: Vec<DocumentReferenceAttester>,
    #[doc = "Identifies the organization or group who is responsible for ongoing maintenance of and access to the document."]
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    #[doc = "Relationships that this document has with other document references that already exist."]
    pub r#relates_to: Vec<DocumentReferenceRelatesTo>,
    #[doc = "Human-readable description of the source document."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "A set of Security-Tag codes specifying the level of privacy/security of the Document found at DocumentReference.content.attachment.url. Note that DocumentReference.meta.security contains the security labels of the data elements in DocumentReference, while DocumentReference.securityLabel contains the security labels for the document the reference refers to. The distinction recognizes that the document may contain sensitive information, while the DocumentReference is metadata about the document and thus might not be as sensitive as the document. For example: a psychotherapy episode may contain highly sensitive information, while the metadata may simply indicate that some episode happened."]
    pub r#security_label: Vec<super::super::types::CodeableConcept>,
    #[doc = "The document and format referenced.  If there are multiple content element repetitions, these must all represent the same document in different format, or attachment metadata."]
    pub r#content: Vec<DocumentReferenceContent>,
}
#[allow(clippy::derivable_impls)]
impl Default for DocumentReference {
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
            r#based_on: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#doc_status: Default::default(),
            r#modality: Default::default(),
            r#type: Default::default(),
            r#category: Default::default(),
            r#subject: Default::default(),
            r#context: Default::default(),
            r#event: Default::default(),
            r#body_site: Default::default(),
            r#facility_type: Default::default(),
            r#practice_setting: Default::default(),
            r#period: Default::default(),
            r#date: Default::default(),
            r#author: Default::default(),
            r#attester: Default::default(),
            r#custodian: Default::default(),
            r#relates_to: Default::default(),
            r#description: Default::default(),
            r#security_label: Default::default(),
            r#content: Default::default(),
        }
    }
}
