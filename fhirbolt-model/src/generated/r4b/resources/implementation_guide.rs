// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "If true or a reference, indicates the resource is an example instance.  If a reference is present, indicates that the example is an example of the specified profile."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImplementationGuideDefinitionResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "The source address for the page."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImplementationGuideDefinitionPageName {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "If true or a reference, indicates the resource is an example instance.  If a reference is present, indicates that the example is an example of the specified profile."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum ImplementationGuideManifestResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
    #[default]
    Invalid,
}
#[doc = "Another implementation guide that this implementation depends on. Typically, an implementation guide uses value sets, profiles etc.defined in other implementation guides."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDependsOn {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A canonical reference to the Implementation guide for the dependency."]
    pub r#uri: super::super::types::Canonical,
    #[doc = "The NPM package name for the Implementation Guide that this IG depends on."]
    pub r#package_id: Option<super::super::types::Id>,
    #[doc = "The version of the IG that is depended on, when the correct version is required to understand the IG correctly."]
    pub r#version: Option<super::super::types::String>,
}
impl Default for ImplementationGuideDependsOn {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#uri: {
                let mut default: super::super::types::Canonical = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#package_id: Default::default(),
            r#version: Default::default(),
        }
    }
}
#[doc = "A set of profiles that all resources covered by this implementation guide must conform to."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideGlobal {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of resource that all instances must conform to."]
    pub r#type: super::super::types::Code,
    #[doc = "A reference to the profile that all instances must conform to."]
    pub r#profile: super::super::types::Canonical,
}
impl Default for ImplementationGuideGlobal {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#profile: {
                let mut default: super::super::types::Canonical = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "A logical group of resources. Logical groups can be used when building pages."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinitionGrouping {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The human-readable title to display for the package of resources when rendering the implementation guide."]
    pub r#name: super::super::types::String,
    #[doc = "Human readable text describing the package."]
    pub r#description: Option<super::super::types::String>,
}
impl Default for ImplementationGuideDefinitionGrouping {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#description: Default::default(),
        }
    }
}
#[doc = "A resource that is part of the implementation guide. Conformance resources (value set, structure definition, capability statements etc.) are obvious candidates for inclusion, but any kind of resource can be included as an example resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinitionResource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Where this resource is found."]
    pub r#reference: Box<super::super::types::Reference>,
    #[doc = "Indicates the FHIR Version(s) this artifact is intended to apply to. If no versions are specified, the resource is assumed to apply to all the versions stated in ImplementationGuide.fhirVersion."]
    pub r#fhir_version: Vec<super::super::types::Code>,
    #[doc = "A human assigned name for the resource. All resources SHOULD have a name, but the name may be extracted from the resource (e.g. ValueSet.name)."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A description of the reason that a resource has been included in the implementation guide."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "If true or a reference, indicates the resource is an example instance.  If a reference is present, indicates that the example is an example of the specified profile."]
    pub r#example: Option<ImplementationGuideDefinitionResourceExample>,
    #[doc = "Reference to the id of the grouping this resource appears in."]
    pub r#grouping_id: Option<super::super::types::Id>,
}
impl Default for ImplementationGuideDefinitionResource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#fhir_version: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#example: Default::default(),
            r#grouping_id: Default::default(),
        }
    }
}
#[doc = "A page / section in the implementation guide. The root page is the implementation guide home page."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinitionPage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The source address for the page."]
    pub r#name: ImplementationGuideDefinitionPageName,
    #[doc = "A short title used to represent this page in navigational structures such as table of contents, bread crumbs, etc."]
    pub r#title: super::super::types::String,
    #[doc = "A code that indicates how the page is generated."]
    pub r#generation: super::super::types::Code,
    #[doc = "Nested Pages/Sections under this page."]
    pub r#page: Vec<ImplementationGuideDefinitionPage>,
}
impl Default for ImplementationGuideDefinitionPage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#title: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#generation: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#page: Default::default(),
        }
    }
}
#[doc = "Defines how IG is built by tools."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinitionParameter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "apply | path-resource | path-pages | path-tx-cache | expansion-parameter | rule-broken-links | generate-xml | generate-json | generate-turtle | html-template."]
    pub r#code: super::super::types::Code,
    #[doc = "Value for named type."]
    pub r#value: super::super::types::String,
}
impl Default for ImplementationGuideDefinitionParameter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#value: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
        }
    }
}
#[doc = "A template for building resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinitionTemplate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Type of template specified."]
    pub r#code: super::super::types::Code,
    #[doc = "The source location for the template."]
    pub r#source: super::super::types::String,
    #[doc = "The scope in which the template applies."]
    pub r#scope: Option<super::super::types::String>,
}
impl Default for ImplementationGuideDefinitionTemplate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#source: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#scope: Default::default(),
        }
    }
}
#[doc = "The information needed by an IG publisher tool to publish the whole implementation guide."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideDefinition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A logical group of resources. Logical groups can be used when building pages."]
    pub r#grouping: Vec<ImplementationGuideDefinitionGrouping>,
    #[doc = "A resource that is part of the implementation guide. Conformance resources (value set, structure definition, capability statements etc.) are obvious candidates for inclusion, but any kind of resource can be included as an example resource."]
    pub r#resource: Vec<ImplementationGuideDefinitionResource>,
    #[doc = "A page / section in the implementation guide. The root page is the implementation guide home page."]
    pub r#page: Option<ImplementationGuideDefinitionPage>,
    #[doc = "Defines how IG is built by tools."]
    pub r#parameter: Vec<ImplementationGuideDefinitionParameter>,
    #[doc = "A template for building resources."]
    pub r#template: Vec<ImplementationGuideDefinitionTemplate>,
}
impl Default for ImplementationGuideDefinition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#grouping: Default::default(),
            r#resource: Default::default(),
            r#page: Default::default(),
            r#parameter: Default::default(),
            r#template: Default::default(),
        }
    }
}
#[doc = "A resource that is part of the implementation guide. Conformance resources (value set, structure definition, capability statements etc.) are obvious candidates for inclusion, but any kind of resource can be included as an example resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideManifestResource {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Where this resource is found."]
    pub r#reference: Box<super::super::types::Reference>,
    #[doc = "If true or a reference, indicates the resource is an example instance.  If a reference is present, indicates that the example is an example of the specified profile."]
    pub r#example: Option<ImplementationGuideManifestResourceExample>,
    #[doc = "The relative path for primary page for this resource within the IG."]
    pub r#relative_path: Option<super::super::types::Url>,
}
impl Default for ImplementationGuideManifestResource {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#reference: {
                let mut default: Box<super::super::types::Reference> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#example: Default::default(),
            r#relative_path: Default::default(),
        }
    }
}
#[doc = "Information about a page within the IG."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideManifestPage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Relative path to the page."]
    pub r#name: super::super::types::String,
    #[doc = "Label for the page intended for human display."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The name of an anchor available on the page."]
    pub r#anchor: Vec<super::super::types::String>,
}
impl Default for ImplementationGuideManifestPage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#title: Default::default(),
            r#anchor: Default::default(),
        }
    }
}
#[doc = "Information about an assembled implementation guide, created by the publication tooling."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuideManifest {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A pointer to official web page, PDF or other rendering of the implementation guide."]
    pub r#rendering: Option<super::super::types::Url>,
    #[doc = "A resource that is part of the implementation guide. Conformance resources (value set, structure definition, capability statements etc.) are obvious candidates for inclusion, but any kind of resource can be included as an example resource."]
    pub r#resource: Vec<ImplementationGuideManifestResource>,
    #[doc = "Information about a page within the IG."]
    pub r#page: Vec<ImplementationGuideManifestPage>,
    #[doc = "Indicates a relative path to an image that exists within the IG."]
    pub r#image: Vec<super::super::types::String>,
    #[doc = "Indicates the relative path of an additional non-page, non-image file that is part of the IG - e.g. zip, jar and similar files that could be the target of a hyperlink in a derived IG."]
    pub r#other: Vec<super::super::types::String>,
}
impl Default for ImplementationGuideManifest {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#rendering: Default::default(),
            r#resource: Default::default(),
            r#page: Default::default(),
            r#image: Default::default(),
            r#other: Default::default(),
        }
    }
}
#[doc = "A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.\n\nAn implementation guide is able to define default profiles that must apply to any use of a resource, so validation services may need to take one or more implementation guide resources when validating."]
#[derive(Debug, Clone, PartialEq)]
pub struct ImplementationGuide {
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
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this implementation guide when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this implementation guide is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the implementation guide is stored on different servers."]
    pub r#url: super::super::types::Uri,
    #[doc = "The identifier that is used to identify this version of the implementation guide when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the implementation guide author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the implementation guide. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: super::super::types::String,
    #[doc = "A short, descriptive, user-friendly title for the implementation guide."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this implementation guide. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this implementation guide is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the implementation guide was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the implementation guide changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the implementation guide."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the implementation guide from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate implementation guide instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the implementation guide is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "A copyright statement relating to the implementation guide and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the implementation guide."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The NPM package name for this Implementation Guide, used in the NPM package distribution, which is the primary mechanism by which FHIR based tooling manages IG dependencies. This value must be globally unique, and should be assigned with care."]
    pub r#package_id: super::super::types::Id,
    #[doc = "The license that applies to this Implementation Guide, using an SPDX license code, or 'not-open-source'."]
    pub r#license: Option<super::super::types::Code>,
    #[doc = "The version(s) of the FHIR specification that this ImplementationGuide targets - e.g. describes how to use. The value of this element is the formal version of the specification, without the revision number, e.g. `publication`.`major`.`minor`, which is 4.3.0 for this version."]
    pub r#fhir_version: Vec<super::super::types::Code>,
    #[doc = "Another implementation guide that this implementation depends on. Typically, an implementation guide uses value sets, profiles etc.defined in other implementation guides."]
    pub r#depends_on: Vec<ImplementationGuideDependsOn>,
    #[doc = "A set of profiles that all resources covered by this implementation guide must conform to."]
    pub r#global: Vec<ImplementationGuideGlobal>,
    #[doc = "The information needed by an IG publisher tool to publish the whole implementation guide."]
    pub r#definition: Option<ImplementationGuideDefinition>,
    #[doc = "Information about an assembled implementation guide, created by the publication tooling."]
    pub r#manifest: Option<ImplementationGuideManifest>,
}
impl Default for ImplementationGuide {
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
            r#url: {
                let mut default: super::super::types::Uri = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#version: Default::default(),
            r#name: {
                let mut default: super::super::types::String = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#title: Default::default(),
            r#status: {
                let mut default: super::super::types::Code = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#copyright: Default::default(),
            r#package_id: {
                let mut default: super::super::types::Id = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#license: Default::default(),
            r#fhir_version: Default::default(),
            r#depends_on: Default::default(),
            r#global: Default::default(),
            r#definition: Default::default(),
            r#manifest: Default::default(),
        }
    }
}
