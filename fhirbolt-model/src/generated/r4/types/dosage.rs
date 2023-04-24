// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept)."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DosageAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    #[default]
    Invalid,
}
#[doc = "Amount of medication per dose."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    #[default]
    Invalid,
}
#[doc = "Amount of medication per unit of time."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    #[default]
    Invalid,
}
#[doc = "The amount of medication administered."]
#[derive(Debug, Clone, PartialEq)]
pub struct DosageDoseAndRate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of dose or rate specified, for example, ordered or calculated."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Amount of medication per dose."]
    pub r#dose: Option<DosageDoseAndRateDose>,
    #[doc = "Amount of medication per unit of time."]
    pub r#rate: Option<DosageDoseAndRateRate>,
}
impl Default for DosageDoseAndRate {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#type: Default::default(),
            r#dose: Default::default(),
            r#rate: Default::default(),
        }
    }
}
#[doc = "Base StructureDefinition for Dosage Type: Indicates how the medication is/was taken or should be taken by the patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct Dosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates the order in which the dosage instructions should be applied or interpreted."]
    pub r#sequence: Option<super::super::types::Integer>,
    #[doc = "Free text dosage instructions e.g. SIG."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "Supplemental instructions to the patient on how to take the medication  (e.g. \"with meals\" or\"take half to one hour before food\") or warnings for the patient about the medication (e.g. \"may cause drowsiness\" or \"avoid exposure of skin to direct sunlight or sunlamps\")."]
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Instructions in terms that are understood by the patient or consumer."]
    pub r#patient_instruction: Option<super::super::types::String>,
    #[doc = "When medication should be administered."]
    pub r#timing: Option<Box<super::super::types::Timing>>,
    #[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option), or it indicates the precondition for taking the Medication (CodeableConcept)."]
    pub r#as_needed: Option<DosageAsNeeded>,
    #[doc = "Body site to administer to."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How drug should enter body."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Technique for administering medication."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The amount of medication administered."]
    pub r#dose_and_rate: Vec<DosageDoseAndRate>,
    #[doc = "Upper limit on medication per unit of time."]
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "Upper limit on medication per administration."]
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Upper limit on medication per lifetime of the patient."]
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
}
impl Default for Dosage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#sequence: Default::default(),
            r#text: Default::default(),
            r#additional_instruction: Default::default(),
            r#patient_instruction: Default::default(),
            r#timing: Default::default(),
            r#as_needed: Default::default(),
            r#site: Default::default(),
            r#route: Default::default(),
            r#method: Default::default(),
            r#dose_and_rate: Default::default(),
            r#max_dose_per_period: Default::default(),
            r#max_dose_per_administration: Default::default(),
            r#max_dose_per_lifetime: Default::default(),
        }
    }
}
