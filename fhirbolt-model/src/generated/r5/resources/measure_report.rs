// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The measure score for this population group, calculated as appropriate for the measure type and scoring method, and based on the contents of the populations defined in the group."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureReportGroupMeasureScore {
    Quantity(Box<super::super::types::Quantity>),
    DateTime(Box<super::super::types::DateTime>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Duration(Box<super::super::types::Duration>),
    #[default]
    Invalid,
}
#[doc = "The value for this stratum, expressed as a CodeableConcept. When defining stratifiers on complex values, the value must be rendered such that the value for each stratum within the stratifier is unique."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureReportGroupStratifierStratumValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The stratum component value."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureReportGroupStratifierStratumComponentValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Boolean(Box<super::super::types::Boolean>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "The measure score for this stratum, calculated as appropriate for the measure type and scoring method, and based on only the members of this stratum."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MeasureReportGroupStratifierStratumMeasureScore {
    Quantity(Box<super::super::types::Quantity>),
    DateTime(Box<super::super::types::DateTime>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Duration(Box<super::super::types::Duration>),
    #[default]
    Invalid,
}
#[doc = "The populations that make up the population group, one for each type of population appropriate for the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroupPopulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The population from the Measure that corresponds to this population in the MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The type of the population."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of members of the population."]
    pub r#count: Option<super::super::types::Integer>,
    #[doc = "This element refers to a List of individual level MeasureReport resources, one for each subject in this population."]
    pub r#subject_results: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to an individual level MeasureReport resource for a member of the population."]
    pub r#subject_report: Vec<super::super::types::Reference>,
    #[doc = "Optional Group identifying the individuals that make up the population."]
    pub r#subjects: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroupPopulation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#count: Default::default(),
            r#subject_results: Default::default(),
            r#subject_report: Default::default(),
            r#subjects: Default::default(),
        }
    }
}
#[doc = "A stratifier component value."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroupStratifierStratumComponent {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The stratifier component from the Measure that corresponds to this stratifier component in the MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The code for the stratum component value."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The stratum component value."]
    pub r#value: MeasureReportGroupStratifierStratumComponentValue,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroupStratifierStratumComponent {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
        }
    }
}
#[doc = "The populations that make up the stratum, one for each type of population appropriate to the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroupStratifierStratumPopulation {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The population from the Measure that corresponds to this population in the MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The type of the population."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The number of members of the population in this stratum."]
    pub r#count: Option<super::super::types::Integer>,
    #[doc = "This element refers to a List of individual level MeasureReport resources, one for each subject in this population in this stratum."]
    pub r#subject_results: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to an individual level MeasureReport resource for a member of the population."]
    pub r#subject_report: Vec<super::super::types::Reference>,
    #[doc = "Optional Group identifying the individuals that make up the population."]
    pub r#subjects: Option<Box<super::super::types::Reference>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroupStratifierStratumPopulation {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#count: Default::default(),
            r#subject_results: Default::default(),
            r#subject_report: Default::default(),
            r#subjects: Default::default(),
        }
    }
}
#[doc = "This element contains the results for a single stratum within the stratifier. For example, when stratifying on administrative gender, there will be four strata, one for each possible gender value."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroupStratifierStratum {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The value for this stratum, expressed as a CodeableConcept. When defining stratifiers on complex values, the value must be rendered such that the value for each stratum within the stratifier is unique."]
    pub r#value: Option<MeasureReportGroupStratifierStratumValue>,
    #[doc = "A stratifier component value."]
    pub r#component: Vec<MeasureReportGroupStratifierStratumComponent>,
    #[doc = "The populations that make up the stratum, one for each type of population appropriate to the measure."]
    pub r#population: Vec<MeasureReportGroupStratifierStratumPopulation>,
    #[doc = "The measure score for this stratum, calculated as appropriate for the measure type and scoring method, and based on only the members of this stratum."]
    pub r#measure_score: Option<MeasureReportGroupStratifierStratumMeasureScore>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroupStratifierStratum {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
            r#component: Default::default(),
            r#population: Default::default(),
            r#measure_score: Default::default(),
        }
    }
}
#[doc = "When a measure includes multiple stratifiers, there will be a stratifier group for each stratifier defined by the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroupStratifier {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The stratifier from the Measure that corresponds to this stratifier in the MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The meaning of this stratifier, as defined in the measure definition."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This element contains the results for a single stratum within the stratifier. For example, when stratifying on administrative gender, there will be four strata, one for each possible gender value."]
    pub r#stratum: Vec<MeasureReportGroupStratifierStratum>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroupStratifier {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#stratum: Default::default(),
        }
    }
}
#[doc = "The results of the calculation, one for each population group in the measure."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReportGroup {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The group from the Measure that corresponds to this group in the MeasureReport resource."]
    pub r#link_id: Option<super::super::types::String>,
    #[doc = "The meaning of the population group as defined in the measure definition."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Optional subject identifying the individual or individuals the report is for."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The populations that make up the population group, one for each type of population appropriate for the measure."]
    pub r#population: Vec<MeasureReportGroupPopulation>,
    #[doc = "The measure score for this population group, calculated as appropriate for the measure type and scoring method, and based on the contents of the populations defined in the group."]
    pub r#measure_score: Option<MeasureReportGroupMeasureScore>,
    #[doc = "When a measure includes multiple stratifiers, there will be a stratifier group for each stratifier defined by the measure."]
    pub r#stratifier: Vec<MeasureReportGroupStratifier>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReportGroup {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#link_id: Default::default(),
            r#code: Default::default(),
            r#subject: Default::default(),
            r#population: Default::default(),
            r#measure_score: Default::default(),
            r#stratifier: Default::default(),
        }
    }
}
#[doc = "The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation."]
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureReport {
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
    #[doc = "A formal identifier that is used to identify this MeasureReport when it is represented in other formats or referenced in a specification, model, design or an instance."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The MeasureReport status. No data will be available until the MeasureReport status is complete."]
    pub r#status: super::super::types::Code,
    #[doc = "The type of measure report. This may be an individual report, which provides the score for the measure for an individual member of the population; a subject-listing, which returns the list of members that meet the various criteria in the measure; a summary report, which returns a population count for each of the criteria in the measure; or a data-collection, which enables the MeasureReport to be used to exchange the data-of-interest for a quality measure."]
    pub r#type: super::super::types::Code,
    #[doc = "Indicates whether the data submitted in a data-exchange report represents a snapshot or incremental update. A snapshot update replaces all previously submitted data for the receiver, whereas an incremental update represents only updated and/or changed data and should be applied as a differential update to the existing submitted data for the receiver."]
    pub r#data_update_type: Option<super::super::types::Code>,
    #[doc = "A reference to the Measure that was calculated to produce this report."]
    pub r#measure: Option<super::super::types::Canonical>,
    #[doc = "Optional subject identifying the individual or individuals the report is for."]
    pub r#subject: Option<Box<super::super::types::Reference>>,
    #[doc = "The date this measure was calculated."]
    pub r#date: Option<super::super::types::DateTime>,
    #[doc = "The individual or organization that is reporting the data."]
    pub r#reporter: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to the vendor who queried the data, calculated results and/or generated the report. The ‘reporting vendor’ is intended to represent the submitting entity when it is not the same as the reporting entity. This extension is used when the Receiver is interested in getting vendor information in the report."]
    pub r#reporting_vendor: Option<Box<super::super::types::Reference>>,
    #[doc = "A reference to the location for which the data is being reported."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "The reporting period for which the report was calculated."]
    pub r#period: Box<super::super::types::Period>,
    #[doc = "A reference to a Parameters resource (typically represented using a contained resource) that represents any input parameters that were provided to the operation that generated the report."]
    pub r#input_parameters: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates how the calculation is performed for the measure, including proportion, ratio, continuous-variable, and cohort. The value set is extensible, allowing additional measure scoring types to be represented. It is expected to be the same as the scoring element on the referenced Measure."]
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Whether improvement in the measure is noted by an increase or decrease in the measure score."]
    pub r#improvement_notation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The results of the calculation, one for each population group in the measure."]
    pub r#group: Vec<MeasureReportGroup>,
    #[doc = "A reference to a Resource that represents additional information collected for the report. If the value of the supplemental data is not a Resource (i.e. evaluating the supplementalData expression for this case in the measure results in a value that is not a FHIR Resource), it is reported as a reference to a contained Observation resource."]
    pub r#supplemental_data: Vec<super::super::types::Reference>,
    #[doc = "Evaluated resources are used to capture what data was involved in the calculation of a measure. This usage is only allowed for individual reports to ensure that the size of the MeasureReport resource is bounded."]
    pub r#evaluated_resource: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for MeasureReport {
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
            r#status: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#type: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#data_update_type: Default::default(),
            r#measure: Default::default(),
            r#subject: Default::default(),
            r#date: Default::default(),
            r#reporter: Default::default(),
            r#reporting_vendor: Default::default(),
            r#location: Default::default(),
            r#period: Box::new(super::super::types::Period {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#input_parameters: Default::default(),
            r#scoring: Default::default(),
            r#improvement_notation: Default::default(),
            r#group: Default::default(),
            r#supplemental_data: Default::default(),
            r#evaluated_resource: Default::default(),
        }
    }
}
