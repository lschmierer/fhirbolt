// Generated on 2023-04-19 by fhirbolt-codegen v0.3.0
#[doc = "The date and time(s) at which the media was collected."]
#[derive(Debug, Clone, PartialEq)]
pub enum MediaCreated {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for MediaCreated {
    fn default() -> MediaCreated {
        MediaCreated::Invalid
    }
}
#[doc = "A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Media {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifiers associated with the image - these may include identifiers for the image itself, identifiers for the context of its collection (e.g. series ids) and context ids such as accession numbers or other workflow identifiers."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "A procedure that is fulfilled in whole or in part by the creation of this media."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger event of which this particular event is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "The current state of the {{title}}."]
    pub r#status: super::super::types::Code,
    #[doc = "A code that classifies whether the media is an image, video or audio recording or some other media category."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of the type of the media - usually, how it was acquired (what type of device). If images sourced from a DICOM system, are wrapped in a Media resource, then this is the modality."]
    pub r#modality: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name of the imaging view e.g. Lateral or Antero-posterior (AP)."]
    pub r#view: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Who/What this Media is a record of."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter that establishes the context for this media."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date and time(s) at which the media was collected."]
    pub r#created: Option<MediaCreated>,
    #[doc = "The date and time this version of the media was made available to providers, typically after having been reviewed."]
    pub r#issued: Option<super::super::types::Instant>,
    #[doc = "The person who administered the collection of the image."]
    pub r#operator: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes why the event occurred in coded or textual form."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the site on the subject's body where the observation was made (i.e. the target site)."]
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The name of the device / manufacturer of the device  that was used to make the recording."]
    pub r#device_name: Option<super::super::types::String>,
    #[doc = "The device used to collect the media."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Height of the image in pixels (photo/video)."]
    pub r#height: Option<super::super::types::PositiveInt>,
    #[doc = "Width of the image in pixels (photo/video)."]
    pub r#width: Option<super::super::types::PositiveInt>,
    #[doc = "The number of frames in a photo. This is used with a multi-page fax, or an imaging acquisition context that takes multiple slices in a single image, or an animated gif. If there is more than one frame, this SHALL have a value in order to alert interface software that a multi-frame capable rendering widget is required."]
    pub r#frames: Option<super::super::types::PositiveInt>,
    #[doc = "The duration of the recording in seconds - for audio and video."]
    pub r#duration: Option<super::super::types::Decimal>,
    #[doc = "The actual content of the media - inline or by direct reference to the media source file."]
    pub r#content: Box<super::super::types::Attachment>,
    #[doc = "Comments made about the media by the performer, subject or other participants."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
