// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "Amount of medication per dose."]
#[derive(Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for DosageDoseAndRateDose {
    fn default() -> DosageDoseAndRateDose {
        DosageDoseAndRateDose::Invalid
    }
}
#[doc = "Amount of medication per unit of time."]
#[derive(Debug, Clone, PartialEq)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
    Invalid,
}
impl Default for DosageDoseAndRateRate {
    fn default() -> DosageDoseAndRateRate {
        DosageDoseAndRateRate::Invalid
    }
}
#[doc = "Depending on the resource,this is the amount of medication administered, to  be administered or typical amount to be administered."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DosageDoseAndRate {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The kind of dose or rate specified, for example, ordered or calculated."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Amount of medication per dose."]
    pub r#dose: Option<DosageDoseAndRateDose>,
    #[doc = "Amount of medication per unit of time."]
    pub r#rate: Option<DosageDoseAndRateRate>,
}
#[doc = "Dosage Type: Indicates how the medication is/was taken or should be taken by the patient."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dosage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
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
    #[doc = "Indicates whether the Medication is only taken when needed within a specific dosing schedule (Boolean option)."]
    pub r#as_needed: Option<super::super::types::Boolean>,
    #[doc = "Indicates whether the Medication is only taken based on a precondition for taking the Medication (CodeableConcept)."]
    pub r#as_needed_for: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Body site to administer to."]
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "How drug should enter body."]
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Technique for administering medication."]
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Depending on the resource,this is the amount of medication administered, to  be administered or typical amount to be administered."]
    pub r#dose_and_rate: Vec<DosageDoseAndRate>,
    #[doc = "Upper limit on medication per unit of time."]
    pub r#max_dose_per_period: Vec<Box<super::super::types::Ratio>>,
    #[doc = "Upper limit on medication per administration."]
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Upper limit on medication per lifetime of the patient."]
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
}
