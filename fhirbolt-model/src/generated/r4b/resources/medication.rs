// Generated on 2023-05-03 by fhirbolt-codegen v0.7.0
#[doc = "The actual ingredient - either a substance (simple ingredient) or another medication of a medication."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum MedicationIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    #[default]
    Invalid,
}
#[doc = "Identifies a particular constituent of interest in the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationIngredient {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The actual ingredient - either a substance (simple ingredient) or another medication of a medication."]
    pub r#item: MedicationIngredientItem,
    #[doc = "Indication of whether this ingredient affects the therapeutic action of the drug."]
    pub r#is_active: Option<super::super::types::Boolean>,
    #[doc = "Specifies how many (or how much) of the items there are in this Medication.  For example, 250 mg per tablet.  This is expressed as a ratio where the numerator is 250mg and the denominator is 1 tablet."]
    pub r#strength: Option<Box<super::super::types::Ratio>>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationIngredient {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#item: Default::default(),
            r#is_active: Default::default(),
            r#strength: Default::default(),
        }
    }
}
#[doc = "Information that only applies to packages (not products)."]
#[derive(Debug, Clone, PartialEq)]
pub struct MedicationBatch {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "The assigned lot number of a batch of the specified product."]
    pub r#lot_number: Option<super::super::types::String>,
    #[doc = "When this specific batch of product will expire."]
    pub r#expiration_date: Option<super::super::types::DateTime>,
}
#[allow(clippy::derivable_impls)]
impl Default for MedicationBatch {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#lot_number: Default::default(),
            r#expiration_date: Default::default(),
        }
    }
}
#[doc = "This resource is primarily used for the identification and definition of a medication for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use."]
#[derive(Debug, Clone, PartialEq)]
pub struct Medication {
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
    #[doc = "Business identifier for this medication."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "A code (or set of codes) that specify this medication, or a textual description if no code is available. Usage note: This could be a standard medication code such as a code from RxNorm, SNOMED CT, IDMP etc. It could also be a national or local formulary code, optionally with translations to other code systems."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A code to indicate if the medication is in active use."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "Describes the details of the manufacturer of the medication product.  This is not intended to represent the distributor of a medication product."]
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    #[doc = "Describes the form of the item.  Powder; tablets; capsule."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Specific amount of the drug in the packaged product.  For example, when specifying a product that has the same strength (For example, Insulin glargine 100 unit per mL solution for injection), this attribute provides additional clarification of the package amount (For example, 3 mL, 10mL, etc.)."]
    pub r#amount: Option<Box<super::super::types::Ratio>>,
    #[doc = "Identifies a particular constituent of interest in the product."]
    pub r#ingredient: Vec<MedicationIngredient>,
    #[doc = "Information that only applies to packages (not products)."]
    pub r#batch: Option<MedicationBatch>,
}
#[allow(clippy::derivable_impls)]
impl Default for Medication {
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
            r#code: Default::default(),
            r#status: Default::default(),
            r#manufacturer: Default::default(),
            r#form: Default::default(),
            r#amount: Default::default(),
            r#ingredient: Default::default(),
            r#batch: Default::default(),
        }
    }
}
