// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "The article or artifact that the Citation Resource is related to."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CitationRelatesToTarget {
    Uri(Box<super::super::types::Uri>),
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "The article or artifact that the cited artifact is related to."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum CitationCitedArtifactRelatesToTarget {
    Uri(Box<super::super::types::Uri>),
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "A human-readable display of the citation."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationSummary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Format for display of the citation."]
    pub r#style: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The human-readable display of the citation."]
    pub r#text: super::super::types::Markdown,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationSummary {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#style: Default::default(),
            r#text: super::super::types::Markdown {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "The assignment to an organizing scheme."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of classifier (e.g. publication type, keyword)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific classification value."]
    pub r#classifier: Vec<super::super::types::CodeableConcept>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationClassification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#classifier: Default::default(),
        }
    }
}
#[doc = "An effective date or period for a status of the citation."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationStatusDate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Classification of the status."]
    pub r#activity: Box<super::super::types::CodeableConcept>,
    #[doc = "Either occurred or expected."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "When the status started and/or ended."]
    pub r#period: Box<super::super::types::Period>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationStatusDate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#activity: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#actual: Default::default(),
            r#period: Box::new(super::super::types::Period {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "Artifact related to the Citation Resource."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the Citation resource relates to the target artifact."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The clasification of the related artifact."]
    pub r#target_classifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "The article or artifact that the Citation Resource is related to."]
    pub r#target: CitationRelatesToTarget,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationRelatesTo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relationship_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#target_classifier: Default::default(),
            r#target: Default::default(),
        }
    }
}
#[doc = "The defined version of the cited artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactVersion {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The version number or other version identifier."]
    pub r#value: super::super::types::String,
    #[doc = "Citation for the main version of the cited artifact."]
    pub r#base_citation: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactVersion {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#base_citation: Default::default(),
        }
    }
}
#[doc = "An effective date or period for a status of the cited artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactStatusDate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Classification of the status."]
    pub r#activity: Box<super::super::types::CodeableConcept>,
    #[doc = "Either occurred or expected."]
    pub r#actual: Option<super::super::types::Boolean>,
    #[doc = "When the status started and/or ended."]
    pub r#period: Box<super::super::types::Period>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactStatusDate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#activity: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#actual: Default::default(),
            r#period: Box::new(super::super::types::Period {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
        }
    }
}
#[doc = "The title details of the article or artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactTitle {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Used to express the reason or specific aspect for the title."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "Used to express the specific language."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The title of the article or artifact."]
    pub r#text: super::super::types::Markdown,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactTitle {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#language: Default::default(),
            r#text: super::super::types::Markdown {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Summary of the article or artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactAbstract {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Used to express the reason or specific aspect for the abstract."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to express the specific language."]
    pub r#language: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Abstract content."]
    pub r#text: super::super::types::Markdown,
    #[doc = "Copyright notice for the abstract."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactAbstract {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#language: Default::default(),
            r#text: super::super::types::Markdown {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#copyright: Default::default(),
        }
    }
}
#[doc = "The component of the article or artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactPart {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of component."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specification of the component."]
    pub r#value: Option<super::super::types::String>,
    #[doc = "The citation for the full article or artifact."]
    pub r#base_citation: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactPart {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#value: Default::default(),
            r#base_citation: Default::default(),
        }
    }
}
#[doc = "The artifact related to the cited artifact."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactRelatesTo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "How the cited artifact relates to the target artifact."]
    pub r#relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "The clasification of the related artifact."]
    pub r#target_classifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "The article or artifact that the cited artifact is related to."]
    pub r#target: CitationCitedArtifactRelatesToTarget,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactRelatesTo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#relationship_type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#target_classifier: Default::default(),
            r#target: Default::default(),
        }
    }
}
#[doc = "The collection the cited article or artifact is published in."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactPublicationFormPublishedIn {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Kind of container (e.g. Periodical, database, or book)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID; Book identifiers include ISBN."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Name of the database or title of the book or journal."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "Name of the publisher."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "Geographic location of the publisher."]
    pub r#publisher_location: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactPublicationFormPublishedIn {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#identifier: Default::default(),
            r#title: Default::default(),
            r#publisher: Default::default(),
            r#publisher_location: Default::default(),
        }
    }
}
#[doc = "Defining the date on which the issue of the journal was published."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Date on which the issue of the journal was published."]
    pub r#date: Option<super::super::types::Date>,
    #[doc = "Year on which the issue of the journal was published."]
    pub r#year: Option<super::super::types::String>,
    #[doc = "Month on which the issue of the journal was published."]
    pub r#month: Option<super::super::types::String>,
    #[doc = "Day on which the issue of the journal was published."]
    pub r#day: Option<super::super::types::String>,
    #[doc = "Spring, Summer, Fall/Autumn, Winter."]
    pub r#season: Option<super::super::types::String>,
    #[doc = "Text representation of the date of which the issue of the journal was published."]
    pub r#text: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#date: Default::default(),
            r#year: Default::default(),
            r#month: Default::default(),
            r#day: Default::default(),
            r#season: Default::default(),
            r#text: Default::default(),
        }
    }
}
#[doc = "The specific issue in which the cited article resides."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactPublicationFormPeriodicRelease {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the form of the medium cited. Common codes are \"Internet\" or \"Print\"."]
    pub r#cited_medium: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Volume number of journal in which the article is published."]
    pub r#volume: Option<super::super::types::String>,
    #[doc = "Issue, part or supplement of journal in which the article is published."]
    pub r#issue: Option<super::super::types::String>,
    #[doc = "Defining the date on which the issue of the journal was published."]
    pub r#date_of_publication:
        Option<CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactPublicationFormPeriodicRelease {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#cited_medium: Default::default(),
            r#volume: Default::default(),
            r#issue: Default::default(),
            r#date_of_publication: Default::default(),
        }
    }
}
#[doc = "If multiple, used to represent alternative forms of the article that are not separate citations."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactPublicationForm {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The collection the cited article or artifact is published in."]
    pub r#published_in: Option<CitationCitedArtifactPublicationFormPublishedIn>,
    #[doc = "The specific issue in which the cited article resides."]
    pub r#periodic_release: Option<CitationCitedArtifactPublicationFormPeriodicRelease>,
    #[doc = "The date the article was added to the database, or the date the article was released (which may differ from the journal issue publication date)."]
    pub r#article_date: Option<super::super::types::DateTime>,
    #[doc = "The date the article was last revised or updated in the database."]
    pub r#last_revision_date: Option<super::super::types::DateTime>,
    #[doc = "Language in which this form of the article is published."]
    pub r#language: Vec<super::super::types::CodeableConcept>,
    #[doc = "Entry number or identifier for inclusion in a database."]
    pub r#accession_number: Option<super::super::types::String>,
    #[doc = "Used for full display of pagination."]
    pub r#page_string: Option<super::super::types::String>,
    #[doc = "Used for isolated representation of first page."]
    pub r#first_page: Option<super::super::types::String>,
    #[doc = "Used for isolated representation of last page."]
    pub r#last_page: Option<super::super::types::String>,
    #[doc = "Actual or approximate number of pages or screens."]
    pub r#page_count: Option<super::super::types::String>,
    #[doc = "Copyright notice for the full article or artifact."]
    pub r#copyright: Option<super::super::types::Markdown>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactPublicationForm {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#published_in: Default::default(),
            r#periodic_release: Default::default(),
            r#article_date: Default::default(),
            r#last_revision_date: Default::default(),
            r#language: Default::default(),
            r#accession_number: Default::default(),
            r#page_string: Default::default(),
            r#first_page: Default::default(),
            r#last_page: Default::default(),
            r#page_count: Default::default(),
            r#copyright: Default::default(),
        }
    }
}
#[doc = "Used for any URL for the article or artifact cited."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactWebLocation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Code the reason for different URLs, e.g. abstract and full-text."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific URL."]
    pub r#url: Option<super::super::types::Uri>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactWebLocation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#url: Default::default(),
        }
    }
}
#[doc = "Provenance and copyright of classification."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactClassificationWhoClassified {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Person who created the classification."]
    pub r#person: Option<Box<super::super::types::Reference>>,
    #[doc = "Organization who created the classification."]
    pub r#organization: Option<Box<super::super::types::Reference>>,
    #[doc = "The publisher of the classification, not the publisher of the article or artifact being cited."]
    pub r#publisher: Option<Box<super::super::types::Reference>>,
    #[doc = "Rights management statement for the classification."]
    pub r#classifier_copyright: Option<super::super::types::String>,
    #[doc = "Acceptable to re-use the classification."]
    pub r#free_to_share: Option<super::super::types::Boolean>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactClassificationWhoClassified {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#person: Default::default(),
            r#organization: Default::default(),
            r#publisher: Default::default(),
            r#classifier_copyright: Default::default(),
            r#free_to_share: Default::default(),
        }
    }
}
#[doc = "The assignment to an organizing scheme."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactClassification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The kind of classifier (e.g. publication type, keyword)."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The specific classification value."]
    pub r#classifier: Vec<super::super::types::CodeableConcept>,
    #[doc = "Provenance and copyright of classification."]
    pub r#who_classified: Option<CitationCitedArtifactClassificationWhoClassified>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactClassification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#classifier: Default::default(),
            r#who_classified: Default::default(),
        }
    }
}
#[doc = "Organization affiliated with the entity."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactContributorshipEntryAffiliationInfo {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Display for the organization."]
    pub r#affiliation: Option<super::super::types::String>,
    #[doc = "Role within the organization, such as professional title."]
    pub r#role: Option<super::super::types::String>,
    #[doc = "Identifier for the organization."]
    pub r#identifier: Vec<super::super::types::Identifier>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactContributorshipEntryAffiliationInfo {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#affiliation: Default::default(),
            r#role: Default::default(),
            r#identifier: Default::default(),
        }
    }
}
#[doc = "Contributions with accounting for time or number."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactContributorshipEntryContributionInstance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The specific contribution."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The time that the contribution was made."]
    pub r#time: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactContributorshipEntryContributionInstance {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#time: Default::default(),
        }
    }
}
#[doc = "An individual entity named in the author list or contributor list."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactContributorshipEntry {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A name associated with the individual."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "Initials for forename."]
    pub r#initials: Option<super::super::types::String>,
    #[doc = "Used for collective or corporate name as an author."]
    pub r#collective_name: Option<super::super::types::String>,
    #[doc = "Unique person identifier."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Organization affiliated with the entity."]
    pub r#affiliation_info: Vec<CitationCitedArtifactContributorshipEntryAffiliationInfo>,
    #[doc = "Physical mailing address for the author or contributor."]
    pub r#address: Vec<super::super::types::Address>,
    #[doc = "Email or telephone contact methods for the author or contributor."]
    pub r#telecom: Vec<super::super::types::ContactPoint>,
    #[doc = "This element identifies the specific nature of an individual’s contribution with respect to the cited work."]
    pub r#contribution_type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The role of the contributor (e.g. author, editor, reviewer)."]
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Contributions with accounting for time or number."]
    pub r#contribution_instance: Vec<CitationCitedArtifactContributorshipEntryContributionInstance>,
    #[doc = "Indication of which contributor is the corresponding contributor for the role."]
    pub r#corresponding_contact: Option<super::super::types::Boolean>,
    #[doc = "Used to code order of authors."]
    pub r#list_order: Option<super::super::types::PositiveInt>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactContributorshipEntry {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#name: Default::default(),
            r#initials: Default::default(),
            r#collective_name: Default::default(),
            r#identifier: Default::default(),
            r#affiliation_info: Default::default(),
            r#address: Default::default(),
            r#telecom: Default::default(),
            r#contribution_type: Default::default(),
            r#role: Default::default(),
            r#contribution_instance: Default::default(),
            r#corresponding_contact: Default::default(),
            r#list_order: Default::default(),
        }
    }
}
#[doc = "Used to record a display of the author/contributor list without separate coding for each list member."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactContributorshipSummary {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Used most commonly to express an author list or a contributorship statement."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The format for the display string."]
    pub r#style: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Used to code the producer or rule for creating the display string."]
    pub r#source: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The display string for the author list, contributor list, or contributorship statement."]
    pub r#value: super::super::types::Markdown,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactContributorshipSummary {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#style: Default::default(),
            r#source: Default::default(),
            r#value: super::super::types::Markdown {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "This element is used to list authors and other contributors, their contact information, specific contributions, and summary statements."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifactContributorship {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Indicates if the list includes all authors and/or contributors."]
    pub r#complete: Option<super::super::types::Boolean>,
    #[doc = "An individual entity named in the author list or contributor list."]
    pub r#entry: Vec<CitationCitedArtifactContributorshipEntry>,
    #[doc = "Used to record a display of the author/contributor list without separate coding for each list member."]
    pub r#summary: Vec<CitationCitedArtifactContributorshipSummary>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifactContributorship {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#complete: Default::default(),
            r#entry: Default::default(),
            r#summary: Default::default(),
        }
    }
}
#[doc = "The article or artifact being described."]
#[derive(Debug, Clone, PartialEq)]
pub struct CitationCitedArtifact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A formal identifier that is used to identify this citation when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A formal identifier that is used to identify things closely related to this citation."]
    pub r#related_identifier: Vec<super::super::types::Identifier>,
    #[doc = "When the cited artifact was accessed."]
    pub r#date_accessed: Option<super::super::types::DateTime>,
    #[doc = "The defined version of the cited artifact."]
    pub r#version: Option<CitationCitedArtifactVersion>,
    #[doc = "The status of the cited artifact."]
    pub r#current_state: Vec<super::super::types::CodeableConcept>,
    #[doc = "An effective date or period for a status of the cited artifact."]
    pub r#status_date: Vec<CitationCitedArtifactStatusDate>,
    #[doc = "The title details of the article or artifact."]
    pub r#title: Vec<CitationCitedArtifactTitle>,
    #[doc = "Summary of the article or artifact."]
    pub r#abstract: Vec<CitationCitedArtifactAbstract>,
    #[doc = "The component of the article or artifact."]
    pub r#part: Option<CitationCitedArtifactPart>,
    #[doc = "The artifact related to the cited artifact."]
    pub r#relates_to: Vec<CitationCitedArtifactRelatesTo>,
    #[doc = "If multiple, used to represent alternative forms of the article that are not separate citations."]
    pub r#publication_form: Vec<CitationCitedArtifactPublicationForm>,
    #[doc = "Used for any URL for the article or artifact cited."]
    pub r#web_location: Vec<CitationCitedArtifactWebLocation>,
    #[doc = "The assignment to an organizing scheme."]
    pub r#classification: Vec<CitationCitedArtifactClassification>,
    #[doc = "This element is used to list authors and other contributors, their contact information, specific contributions, and summary statements."]
    pub r#contributorship: Option<CitationCitedArtifactContributorship>,
    #[doc = "Any additional information or content for the article or artifact."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for CitationCitedArtifact {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#related_identifier: Default::default(),
            r#date_accessed: Default::default(),
            r#version: Default::default(),
            r#current_state: Default::default(),
            r#status_date: Default::default(),
            r#title: Default::default(),
            r#abstract: Default::default(),
            r#part: Default::default(),
            r#relates_to: Default::default(),
            r#publication_form: Default::default(),
            r#web_location: Default::default(),
            r#classification: Default::default(),
            r#contributorship: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "The Citation Resource enables reference to any knowledge artifact for purposes of identification and attribution. The Citation Resource supports existing reference structures and developing publication practices such as versioning, expressing complex contributorship roles, and referencing computable resources."]
#[derive(Debug, Clone, PartialEq)]
pub struct Citation {
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
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, and nor can they have their own independent transaction scope."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "An absolute URI that is used to identify this citation when it is referenced in a specification, model, design or an instance; also called its canonical identifier. This SHOULD be globally unique and SHOULD be a literal address at which at which an authoritative instance of this summary is (or will be) published. This URL can be the target of a canonical reference. It SHALL remain the same when the summary is stored on different servers."]
    pub r#url: Option<super::super::types::Uri>,
    #[doc = "A formal identifier that is used to identify this citation when it is represented in other formats, or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The identifier that is used to identify this version of the citation when it is referenced in a specification, model, design or instance. This is an arbitrary value managed by the citation author and is not expected to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a managed version is not available. There is also no expectation that versions can be placed in a lexicographical sequence."]
    pub r#version: Option<super::super::types::String>,
    #[doc = "A natural language name identifying the citation. This name should be usable as an identifier for the module by machine processing applications such as code generation."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A short, descriptive, user-friendly title for the citation."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "The status of this summary. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "A Boolean value to indicate that this citation is authored for testing purposes (or education/evaluation/marketing) and is not intended to be used for genuine usage."]
    pub r#experimental: Option<super::super::types::Boolean>,
    #[doc = "The date  (and optionally time) when the citation was published. The date must change when the business version changes and it must change if the status code changes. In addition, it should change when the substantive content of the citation changes."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The name of the organization or individual that published the citation."]
    pub r#publisher: Option<super::super::types::String>,
    #[doc = "Contact details to assist a user in finding and communicating with the publisher."]
    pub r#contact: Vec<super::super::types::ContactDetail>,
    #[doc = "A free text natural language description of the citation from a consumer's perspective."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "The content was developed with a focus and intent of supporting the contexts that are listed. These contexts may be general categories (gender, age, ...) or may be references to specific programs (insurance plans, studies, ...) and may be used to assist with indexing and searching for appropriate citation instances."]
    pub r#use_context: Vec<super::super::types::UsageContext>,
    #[doc = "A legal or geographic region in which the citation is intended to be used."]
    pub r#jurisdiction: Vec<super::super::types::CodeableConcept>,
    #[doc = "Explanation of why this citation is needed and why it has been designed as it has."]
    pub r#purpose: Option<super::super::types::Markdown>,
    #[doc = "Use and/or publishing restrictions for the Citation, not for the cited artifact."]
    pub r#copyright: Option<super::super::types::Markdown>,
    #[doc = "The date on which the resource content was approved by the publisher. Approval happens once when the content is officially approved for usage."]
    pub r#approval_date: Option<super::super::types::Date>,
    #[doc = "The date on which the resource content was last reviewed. Review happens periodically after approval but does not change the original approval date."]
    pub r#last_review_date: Option<super::super::types::Date>,
    #[doc = "The period during which the citation content was or is planned to be in active use."]
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    #[doc = "Who authored the Citation."]
    pub r#author: Vec<super::super::types::ContactDetail>,
    #[doc = "Who edited the Citation."]
    pub r#editor: Vec<super::super::types::ContactDetail>,
    #[doc = "Who reviewed the Citation."]
    pub r#reviewer: Vec<super::super::types::ContactDetail>,
    #[doc = "Who endorsed the Citation."]
    pub r#endorser: Vec<super::super::types::ContactDetail>,
    #[doc = "A human-readable display of the citation."]
    pub r#summary: Vec<CitationSummary>,
    #[doc = "The assignment to an organizing scheme."]
    pub r#classification: Vec<CitationClassification>,
    #[doc = "Used for general notes and annotations not coded elsewhere."]
    pub r#note: Vec<super::super::types::Annotation>,
    #[doc = "The status of the citation."]
    pub r#current_state: Vec<super::super::types::CodeableConcept>,
    #[doc = "An effective date or period for a status of the citation."]
    pub r#status_date: Vec<CitationStatusDate>,
    #[doc = "Artifact related to the Citation Resource."]
    pub r#relates_to: Vec<CitationRelatesTo>,
    #[doc = "The article or artifact being described."]
    pub r#cited_artifact: Option<CitationCitedArtifact>,
}
#[allow(clippy::derivable_impls)]
impl Default for Citation {
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
            r#name: Default::default(),
            r#title: Default::default(),
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#experimental: Default::default(),
            r#date: Default::default(),
            r#publisher: Default::default(),
            r#contact: Default::default(),
            r#description: Default::default(),
            r#use_context: Default::default(),
            r#jurisdiction: Default::default(),
            r#purpose: Default::default(),
            r#copyright: Default::default(),
            r#approval_date: Default::default(),
            r#last_review_date: Default::default(),
            r#effective_period: Default::default(),
            r#author: Default::default(),
            r#editor: Default::default(),
            r#reviewer: Default::default(),
            r#endorser: Default::default(),
            r#summary: Default::default(),
            r#classification: Default::default(),
            r#note: Default::default(),
            r#current_state: Default::default(),
            r#status_date: Default::default(),
            r#relates_to: Default::default(),
            r#cited_artifact: Default::default(),
        }
    }
}
