// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "Characteristics e.g. a products onset of action."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A coded characteristic."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The status of characteristic e.g. assigned or pending."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPharmaceuticalCharacteristics {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#status: Default::default(),
        }
    }
}
#[doc = "A species specific time during which consumption of animal product is not appropriate."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded expression for the type of tissue for which the withdrawal period applues, e.g. meat, milk."]
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the time."]
    pub r#value: Box<super::super::types::Quantity>,
    #[doc = "Extra information about the withdrawal period."]
    pub r#supporting_information: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#tissue: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Box::new(super::super::types::Quantity {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#supporting_information: Default::default(),
        }
    }
}
#[doc = "A species for which this route applies."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded expression for the species."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "A species specific time during which consumption of animal product is not appropriate."]
    pub r#withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#withdrawal_period: Default::default(),
        }
    }
}
#[doc = "The path by which the pharmaceutical product is taken into or makes contact with the body."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded expression for the route."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The first dose (dose quantity) administered in humans can be specified, for a product under investigation, using a numerical value and its unit of measurement."]
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum single dose that can be administered as per the protocol of a clinical trial can be specified using a numerical value and its unit of measurement."]
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per treatment period that can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "The maximum treatment period during which an Investigational Medicinal Product can be administered as per the protocol referenced in the clinical trial authorisation."]
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    #[doc = "A species for which this route applies."]
    pub r#target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPharmaceuticalRouteOfAdministration {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#code: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#first_dose: Default::default(),
            r#max_single_dose: Default::default(),
            r#max_dose_per_day: Default::default(),
            r#max_dose_per_treatment_period: Default::default(),
            r#max_treatment_period: Default::default(),
            r#target_species: Default::default(),
        }
    }
}
#[doc = "A pharmaceutical product described in terms of its composition and dose form."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicinalProductPharmaceutical {
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
    #[doc = "An identifier for the pharmaceutical medicinal product."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The administrable dose form, after necessary reconstitution."]
    pub r#administrable_dose_form: Box<super::super::types::CodeableConcept>,
    #[doc = "Todo."]
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Ingredient."]
    pub r#ingredient: Vec<super::super::types::Reference>,
    #[doc = "Accompanying device."]
    pub r#device: Vec<super::super::types::Reference>,
    #[doc = "Characteristics e.g. a products onset of action."]
    pub r#characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    #[doc = "The path by which the pharmaceutical product is taken into or makes contact with the body."]
    pub r#route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicinalProductPharmaceutical {
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
            r#administrable_dose_form: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#unit_of_presentation: Default::default(),
            r#ingredient: Default::default(),
            r#device: Default::default(),
            r#characteristics: Default::default(),
            r#route_of_administration: Default::default(),
        }
    }
}
