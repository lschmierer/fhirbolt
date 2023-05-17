// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DataRequirementSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration before now."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DataRequirementDateFilterValue {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    #[default]
    Invalid,
}
#[doc = "Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data. Each code filter defines an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed."]
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementCodeFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The code-valued attribute of the filter. The specified path SHALL be a FHIRPath resolveable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type code, Coding, or CodeableConcept."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "A token parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type code, Coding, or CodeableConcept."]
    pub r#search_param: Option<super::super::types::String>,
    #[doc = "The valueset for the code filter. The valueSet and code elements are additive. If valueSet is specified, the filter will return only those data items for which the value of the code-valued element specified in the path is a member of the specified valueset."]
    pub r#value_set: Option<super::super::types::Canonical>,
    #[doc = "The codes for the code filter. If values are given, the filter will return only those data items for which the code-valued attribute specified by the path has a value that is one of the specified codes. If codes are specified in addition to a value set, the filter returns items matching a code in the value set or one of the specified codes."]
    pub r#code: Vec<super::super::types::Coding>,
}
#[allow(clippy::derivable_impls)]
impl Default for DataRequirementCodeFilter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#path: Default::default(),
            r#search_param: Default::default(),
            r#value_set: Default::default(),
            r#code: Default::default(),
        }
    }
}
#[doc = "Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed."]
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementDateFilter {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The date-valued attribute of the filter. The specified path SHALL be a FHIRPath resolveable on the specified type of the DataRequirement, and SHALL consist only of identifiers, constant indexers, and .resolve(). The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details). Note that the index must be an integer constant. The path must resolve to an element of type date, dateTime, Period, Schedule, or Timing."]
    pub r#path: Option<super::super::types::String>,
    #[doc = "A date parameter that refers to a search parameter defined on the specified type of the DataRequirement, and which searches on elements of type date, dateTime, Period, Schedule, or Timing."]
    pub r#search_param: Option<super::super::types::String>,
    #[doc = "The value of the filter. If period is specified, the filter will return only those data items that fall within the bounds determined by the Period, inclusive of the period boundaries. If dateTime is specified, the filter will return only those data items that are equal to the specified dateTime. If a Duration is specified, the filter will return only those data items that fall within Duration before now."]
    pub r#value: Option<DataRequirementDateFilterValue>,
}
#[allow(clippy::derivable_impls)]
impl Default for DataRequirementDateFilter {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#path: Default::default(),
            r#search_param: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "Specifies the order of the results to be returned."]
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirementSort {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The attribute of the sort. The specified path must be resolvable from the type of the required data. The path is allowed to contain qualifiers (.) to traverse sub-elements, as well as indexers (\\[x\\]) to traverse multiple-cardinality sub-elements. Note that the index must be an integer constant."]
    pub r#path: super::super::types::String,
    #[doc = "The direction of the sort, ascending or descending."]
    pub r#direction: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for DataRequirementSort {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#path: super::super::types::String {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#direction: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Base StructureDefinition for DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data."]
#[derive(Debug, Clone, PartialEq)]
pub struct DataRequirement {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The type of the required data, specified as the type name of a resource. For profiles, this value is set to the type of the base resource of the profile."]
    pub r#type: super::super::types::Code,
    #[doc = "The profile of the required data, specified as the uri of the profile definition."]
    pub r#profile: Vec<super::super::types::Canonical>,
    #[doc = "The intended subjects of the data requirement. If this element is not provided, a Patient subject is assumed."]
    pub r#subject: Option<DataRequirementSubject>,
    #[doc = "Indicates that specific elements of the type are referenced by the knowledge module and must be supported by the consumer in order to obtain an effective evaluation. This does not mean that a value is required for this element, only that the consuming system must understand the element and be able to provide values for it if they are available. \n\nThe value of mustSupport SHALL be a FHIRPath resolveable on the type of the DataRequirement. The path SHALL consist only of identifiers, constant indexers, and .resolve() (see the [Simple FHIRPath Profile](fhirpath.html#simple) for full details)."]
    pub r#must_support: Vec<super::super::types::String>,
    #[doc = "Code filters specify additional constraints on the data, specifying the value set of interest for a particular element of the data. Each code filter defines an additional constraint on the data, i.e. code filters are AND'ed, not OR'ed."]
    pub r#code_filter: Vec<DataRequirementCodeFilter>,
    #[doc = "Date filters specify additional constraints on the data in terms of the applicable date range for specific elements. Each date filter specifies an additional constraint on the data, i.e. date filters are AND'ed, not OR'ed."]
    pub r#date_filter: Vec<DataRequirementDateFilter>,
    #[doc = "Specifies a maximum number of results that are required (uses the _count search parameter)."]
    pub r#limit: Option<super::super::types::PositiveInt>,
    #[doc = "Specifies the order of the results to be returned."]
    pub r#sort: Vec<DataRequirementSort>,
}
#[allow(clippy::derivable_impls)]
impl Default for DataRequirement {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#profile: Default::default(),
            r#subject: Default::default(),
            r#must_support: Default::default(),
            r#code_filter: Default::default(),
            r#date_filter: Default::default(),
            r#limit: Default::default(),
            r#sort: Default::default(),
        }
    }
}
