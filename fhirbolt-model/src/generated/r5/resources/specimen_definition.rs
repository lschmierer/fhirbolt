// Generated on 2023-05-17 by fhirbolt-codegen v0.9.0
#[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionVersionAlgorithm {
    String(Box<super::super::types::String>),
    Coding(Box<super::super::types::Coding>),
    #[default]
    Invalid,
}
#[doc = "A code or group definition that describes the intended subject  from which this kind of specimen is to be collected."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The minimum volume to be conditioned in the container."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
    #[default]
    Invalid,
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenDefinitionTypeTestedContainerAdditive {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#additive: Default::default(),
        }
    }
}
#[doc = "The specimen's container."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedContainer {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The type of material of the container."]
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The type of container used to contain this kind of specimen."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Color of container cap."]
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The textual description of the kind of container."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The capacity (volume or other measure) of this kind of container."]
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    #[doc = "The minimum volume to be conditioned in the container."]
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    #[doc = "Substance introduced in the kind of container to preserve, maintain or enhance the specimen. Examples: Formalin, Citrate, EDTA."]
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    #[doc = "Special processing that should be applied to the container for this kind of specimen."]
    pub r#preparation: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenDefinitionTypeTestedContainer {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#material: Default::default(),
            r#type: Default::default(),
            r#cap: Default::default(),
            r#description: Default::default(),
            r#capacity: Default::default(),
            r#minimum_volume: Default::default(),
            r#additive: Default::default(),
            r#preparation: Default::default(),
        }
    }
}
#[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTestedHandling {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "It qualifies the interval of temperature, which characterizes an occurrence of handling. Conditions that are not related to temperature may be handled in the instruction element."]
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The temperature interval for this set of handling instructions."]
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
    #[doc = "The maximum time interval of preservation of the specimen with these conditions."]
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    #[doc = "Additional textual instructions for the preservation or transport of the specimen. For instance, 'Protect from light exposure'."]
    pub r#instruction: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenDefinitionTypeTestedHandling {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#temperature_qualifier: Default::default(),
            r#temperature_range: Default::default(),
            r#max_duration: Default::default(),
            r#instruction: Default::default(),
        }
    }
}
#[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinitionTypeTested {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Primary of secondary specimen."]
    pub r#is_derived: Option<super::super::types::Boolean>,
    #[doc = "The kind of specimen conditioned for testing expected by lab."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The preference for this type of conditioned specimen."]
    pub r#preference: super::super::types::Code,
    #[doc = "The specimen's container."]
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
    #[doc = "Requirements for delivery and special handling of this kind of conditioned specimen."]
    pub r#requirement: Option<super::super::types::Markdown>,
    #[doc = "The usual time that a specimen of this kind is retained after the ordered tests are completed, for the purpose of additional testing."]
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    #[doc = "Specimen can be used by only one test or panel if the value is \"true\"."]
    pub r#single_use: Option<super::super::types::Boolean>,
    #[doc = "Criterion for rejection of the specimen in its container by the laboratory."]
    pub r#rejection_criterion: Vec<super::super::types::CodeableConcept>,
    #[doc = "Set of instructions for preservation/transport of the specimen at a defined temperature interval, prior the testing process."]
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
    #[doc = "Where the specimen will be tested: e.g., lab, sector, device or any combination of these."]
    pub r#testing_destination: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenDefinitionTypeTested {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#is_derived: Default::default(),
            r#type: Default::default(),
            r#preference: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#container: Default::default(),
            r#requirement: Default::default(),
            r#retention_time: Default::default(),
            r#single_use: Default::default(),
            r#rejection_criterion: Default::default(),
            r#handling: Default::default(),
            r#testing_destination: Default::default(),
        }
    }
}
#[doc = "A kind of specimen with associated set of requirements."]
#[derive(Debug, Clone, PartialEq)]
pub struct SpecimenDefinition {
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
    #[doc = "An absolute URL that is used to identify this SpecimenDefinition when it is referenced in a specification, model, design or an instance. This SHALL be a URL, SHOULD be globally unique, and SHOULD be an address at which this SpecimenDefinition is (or will be) published. The URL SHOULD include the major version of the SpecimenDefinition. For more information see Technical and Business Versions."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A business identifier assigned to this SpecimenDefinition."]
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    #[doc = "The identifier that is used to identify this version of the SpecimenDefinition when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the SpecimenDefinition author and is not expected to be globally unique."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "Indicates the mechanism used to compare versions to determine which is more current."]
    pub r#version_algorithm: Option<SpecimenDefinitionVersionAlgorithm>,
    #[doc = "A natural language name identifying the {{title}}. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the SpecimenDefinition."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The canonical URL pointing to another FHIR-defined SpecimenDefinition that is adhered to in whole or in part by this definition."]
    pub r#derived_from_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally-defined type of specimen, guideline or other definition that is adhered to in whole or in part by this definition."]
    pub r#derived_from_uri: Vec<super::super::types::Uri>,
    #[doc = "The current state of theSpecimenDefinition."]
    pub r#status: super::super::types::Code,
    #[doc = "A flag to indicate that this SpecimenDefinition is not authored for  genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "A code or group definition that describes the intended subject  from which this kind of specimen is to be collected."]
    pub r#subject: Option<SpecimenDefinitionSubject>,
    #[doc = "For draft definitions, indicates the date of initial creation. For active definitions, represents the date of activation. For withdrawn definitions, indicates the date of withdrawal."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "Helps establish the \"authority/credibility\" of the SpecimenDefinition. May also allow for contact."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the SpecimenDefinition from the consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These terms may be used to assist with indexing and searching of specimen definitions."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A jurisdiction in which the SpecimenDefinition is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explains why this SpecimeDefinition is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "Copyright statement relating to the SpecimenDefinition and/or its contents. Copyright statements are generally legal restrictions on the use and publishing of the SpecimenDefinition."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "A short string (<50 characters), suitable for inclusion in a page footer that identifies the copyright holder, effective period, and optionally whether rights are resctricted. (e.g. 'All rights reserved', 'Some rights reserved')."]
    pub r#copyright_label: Option<super::super::types::String>,
    #[doc = "The date on which the asset content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the asset content was last reviewed. Review happens periodically after that, but doesn't change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the SpecimenDefinition content was or is planned to be effective."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "The kind of material to be collected."]
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Preparation of the patient for specimen collection."]
    pub r#patient_preparation: Vec<super::super::types::CodeableConcept>,
    #[doc = "Time aspect of specimen collection (duration or offset)."]
    pub r#time_aspect: Option<super::super::types::String>,
    #[doc = "The action to be performed for collecting the specimen."]
    pub r#collection: Vec<super::super::types::CodeableConcept>,
    #[doc = "Specimen conditioned in a container as expected by the testing laboratory."]
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
}
#[allow(clippy::derivable_impls)]
impl Default for SpecimenDefinition {
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
            r#version_algorithm: Default::default(),
            r#name: Default::default(),
            r#title: Default::default(),
            r#derived_from_canonical: Default::default(),
            r#derived_from_uri: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#subject: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#copyright_label: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#type_collected: Default::default(),
            r#patient_preparation: Default::default(),
            r#time_aspect: Default::default(),
            r#collection: Default::default(),
            r#type_tested: Default::default(),
        }
    }
}
