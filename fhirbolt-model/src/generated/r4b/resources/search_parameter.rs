// Generated on 2023-04-13 by fhirbolt-codegen v0.1.0
#[doc = "Used to define the parts of a composite search parameter."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchParameterComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The definition of the search parameter that describes this part."]
    pub r#definition: super::super::types::Canonical,
    #[doc = "A sub-expression that defines how to extract values for this component from the output of the main SearchParameter.expression."]
    pub r#expression: super::super::types::String,
}
#[doc = "A search parameter that defines a named search item that can be used to search/filter on a resource."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchParameter {
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
    #[doc = "An absolute URI that is used to identify this search parameter when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this search parameter is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the search parameter is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "The identifier that is used to identify this version of the search parameter when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the search parameter author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the search parameter. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "Where this search parameter is originally defined. If a derivedFrom is provided, then the details in the search parameter must be consistent with the definition from which it is defined. i.e. the parameter should have the same meaning, and (usually) the functionality should be a proper subset of the underlying search parameter."]
    pub r#derived_from: Option<super::super::types::Canonical>,
    #[doc = "The status of this search parameter. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this search parameter is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the search parameter was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the search parameter changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the search parameter."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    #[doc = "And how it used."]
    pub r#description: super::super::types::Markdown,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate search parameter instances."]
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    #[doc = "A legal or geographic region in which the search parameter is intended to be used."]
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Explanation of why this search parameter is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "The code used in the URL or the parameter name in a parameters resource for this search parameter."]
    pub r#code: super::super::types::Code,
    #[doc = "The base resource type(s) that this search parameter can be used against."]
    pub r#base: Vec<super::super::types::Code>,
    #[doc = "The type of value that a search parameter may contain, and how the content is interpreted."]
    pub r#type: super::super::types::Code,
    #[doc = "A FHIRPath expression that returns a set of elements for the search parameter."]
    pub r#expression: Option<super::super::types::String>,
    #[doc = "An XPath expression that returns a set of elements for the search parameter."]
    pub r#xpath: Option<super::super::types::String>,
    #[doc = "How the search parameter relates to the set of elements returned by evaluating the xpath query."]
    pub r#xpath_usage: Option<super::super::types::Code>,
    #[doc = "Types of resource (if a resource is referenced)."]
    pub r#target: Vec<super::super::types::Code>,
    #[doc = "Whether multiple values are allowed for each time the parameter exists. Values are separated by commas, and the parameter matches if any of the values match."]
    pub r#multiple_or: Option<super::super::types::Boolean>,
    #[doc = "Whether multiple parameters are allowed - e.g. more than one parameter with the same name. The search matches if all the parameters match."]
    pub r#multiple_and: Option<super::super::types::Boolean>,
    #[doc = "Comparators supported for the search parameter."]
    pub r#comparator: Vec<super::super::types::Code>,
    #[doc = "A modifier supported for the search parameter."]
    pub r#modifier: Vec<super::super::types::Code>,
    #[doc = "Contains the names of any search parameters which may be chained to the containing search parameter. Chained parameters may be added to search parameters of type reference and specify that resources will only be returned if they contain a reference to a resource which matches the chained parameter value. Values for this field should be drawn from SearchParameter.code for a parameter on the target resource type."]
    pub r#chain: Vec<super::super::types::String>,
    #[doc = "Used to define the parts of a composite search parameter."]
    pub r#component: Vec<SearchParameterComponent>,
}
