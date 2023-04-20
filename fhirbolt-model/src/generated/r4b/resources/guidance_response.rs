// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "An identifier, CodeableConcept or canonical reference to the guidance that was requested."]
#[derive(Debug, Clone, PartialEq)]
pub enum GuidanceResponseModule {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for GuidanceResponseModule {
    fn default() -> GuidanceResponseModule {
        GuidanceResponseModule::Invalid
    }
}
#[doc = "A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.\n\nThe GuidanceResponse resource supports recording the results of decision support interactions, reportability determination for public health, as well as the communication of additional data requirements for subsequent interactions."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GuidanceResponse {
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
    #[doc = "The identifier of the request associated with this response. If an identifier was given as part of the request, it will be reproduced here to enable the requester to more easily identify the response in a multi-request scenario."]
    pub r#request_identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "Allows a service to provide  unique, business identifiers for the response."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "An identifier, CodeableConcept or canonical reference to the guidance that was requested."]
    pub r#module: GuidanceResponseModule,
    #[doc = "The status of the response. If the evaluation is completed successfully, the status will indicate success. However, in order to complete the evaluation, the engine may require more information. In this case, the status will be data-required, and the response will contain a description of the additional required information. If the evaluation completed successfully, but the engine determines that a potentially more accurate response could be provided if more data was available, the status will be data-requested, and the response will contain a description of the additional requested information."]
    pub r#status: super::super::types::Code,
    #[doc = "The patient for which the request was processed."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The encounter during which this response was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the guidance response was processed."]
    pub r#occurrence_date_time: Option<super::super::types::DateTime>,
    #[doc = "Provides a reference to the device that performed the guidance."]
    pub r#performer: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the reason for the guidance response in coded or textual form."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the reason the request was initiated. This is typically provided as a parameter to the evaluation and echoed by the service, although for some use cases, such as subscription- or event-based scenarios, it may provide an indication of the cause for the response."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Provides a mechanism to communicate additional information about the response."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    #[doc = "Messages resulting from the evaluation of the artifact or artifacts. As part of evaluating the request, the engine may produce informational or warning messages. These messages will be provided by this element."]
    pub r#evaluation_message: Vec<Box<super::super::types::Reference>>,
    #[doc = "The output parameters of the evaluation, if any. Many modules will result in the return of specific resources such as procedure or communication requests that are returned as part of the operation result. However, modules may define specific outputs that would be returned as the result of the evaluation, and these would be returned in this element."]
    pub r#output_parameters: Option<Box<super::super::types::Reference>>,
    #[doc = "The actions, if any, produced by the evaluation of the artifact."]
    pub r#result: Option<Box<super::super::types::Reference>>,
    #[doc = "If the evaluation could not be completed due to lack of information, or additional information would potentially result in a more accurate response, this element will a description of the data required in order to proceed with the evaluation. A subsequent request to the service should include this data."]
    pub r#data_requirement: Vec<Box<super::super::types::DataRequirement>>,
}
