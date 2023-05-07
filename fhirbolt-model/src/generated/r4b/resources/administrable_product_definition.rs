// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
#[doc = "A value for the characteristic."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum AdministrableProductDefinitionPropertyValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Date(Box<super::super::types::Date>),
    Boolean(Box<super::super::types::Boolean>),
    Attachment(Box<super::super::types::Attachment>),
    #[default]
    Invalid,
}
#[doc = "Characteristics e.g. a product's onset of action."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionProperty {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "A code expressing the type of characteristic."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the characteristic."]
    pub r#value: Option<AdministrableProductDefinitionPropertyValue>,
    #[doc = "The status of characteristic e.g. assigned or pending."]
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for AdministrableProductDefinitionProperty {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#value: Default::default(),
            r#status: Default::default(),
        }
    }
}
#[doc = "A species specific time during which consumption of animal product is not appropriate."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded expression for the type of tissue for which the withdrawal period applies, e.g. meat, milk."]
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    #[doc = "A value for the time."]
    pub r#value: Box<super::super::types::Quantity>,
    #[doc = "Extra information about the withdrawal period."]
    pub r#supporting_information: Option<super::super::types::String>,
}
#[allow(clippy::derivable_impls)]
impl Default for AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
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
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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
        Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
}
#[allow(clippy::derivable_impls)]
impl Default for AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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
#[doc = "The path by which the product is taken into or makes contact with the body. In some regions this is referred to as the licenced or approved route. RouteOfAdministration cannot be used when the 'formOf' product already uses MedicinalProductDefinition.route (and vice versa)."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinitionRouteOfAdministration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Coded expression for the route."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "The first dose (dose quantity) administered can be specified for the product, using a numerical value and its unit of measurement."]
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum single dose that can be administered, specified using a numerical value and its unit of measurement."]
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered."]
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    #[doc = "The maximum dose per treatment period that can be administered."]
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    #[doc = "The maximum treatment period during which the product can be administered."]
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    #[doc = "A species for which this route applies."]
    pub r#target_species: Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>,
}
#[allow(clippy::derivable_impls)]
impl Default for AdministrableProductDefinitionRouteOfAdministration {
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
#[doc = "A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed)."]
#[derive(Debug, Clone, PartialEq)]
pub struct AdministrableProductDefinition {
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
    #[doc = "An identifier for the administrable product."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of this administrable product. Enables tracking the life-cycle of the content."]
    pub r#status: super::super::types::Code,
    #[doc = "References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product.  If this administrable product describes the administration of a crushed tablet, the 'formOf' would be the product representing a distribution containing tablets and possibly also a cream.  This is distinct from the 'producedFrom' which refers to the specific components of the product that are used in this preparation, rather than the product as a whole."]
    pub r#form_of: Vec<super::super::types::Reference>,
    #[doc = "The dose form of the final product after necessary reconstitution or processing. Contrasts to the manufactured dose form (see ManufacturedItemDefinition). If the manufactured form was 'powder for solution for injection', the administrable dose form could be 'solution for injection' (once mixed with another item having manufactured form 'solvent for solution for injection')."]
    pub r#administrable_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The presentation type in which this item is given to a patient. e.g. for a spray - 'puff' (as in 'contains 100 mcg per puff'), or for a liquid - 'vial' (as in 'contains 5 ml per vial')."]
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form.  In some cases, an administrable form might use all of the items from the overall product (or there might only be one item), while in other cases, an administrable form might use only a subset of the items available in the overall product.  For example, an administrable form might involve combining a liquid and a powder available as part of an overall product, but not involve applying the also supplied cream."]
    pub r#produced_from: Vec<super::super::types::Reference>,
    #[doc = "The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton (via AdministrableProductDefinition.producedFrom) to state which component items are used to make this, or using by incoming references from the Ingredient resource, to state in detail which substances exist within this. This element allows a basic coded ingredient to be used."]
    pub r#ingredient: Vec<super::super::types::CodeableConcept>,
    #[doc = "A device that is integral to the medicinal product, in effect being considered as an \"ingredient\" of the medicinal product. This is not intended for devices that are just co-packaged."]
    pub r#device: Option<Box<super::super::types::Reference>>,
    #[doc = "Characteristics e.g. a product's onset of action."]
    pub r#property: Vec<AdministrableProductDefinitionProperty>,
    #[doc = "The path by which the product is taken into or makes contact with the body. In some regions this is referred to as the licenced or approved route. RouteOfAdministration cannot be used when the 'formOf' product already uses MedicinalProductDefinition.route (and vice versa)."]
    pub r#route_of_administration: Vec<AdministrableProductDefinitionRouteOfAdministration>,
}
#[allow(clippy::derivable_impls)]
impl Default for AdministrableProductDefinition {
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
            r#form_of: Default::default(),
            r#administrable_dose_form: Default::default(),
            r#unit_of_presentation: Default::default(),
            r#produced_from: Default::default(),
            r#ingredient: Default::default(),
            r#device: Default::default(),
            r#property: Default::default(),
            r#route_of_administration: Default::default(),
        }
    }
}
