// Generated on 2023-05-08 by fhirbolt-codegen v0.8.0
#[doc = "Allows for adjustment on two axis."]
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescriptionLensSpecificationPrism {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Amount of prism to compensate for eye alignment in fractional units."]
    pub r#amount: super::super::types::Decimal,
    #[doc = "The relative base, or reference lens edge, for the prism."]
    pub r#base: super::super::types::Code,
}
#[allow(clippy::derivable_impls)]
impl Default for VisionPrescriptionLensSpecificationPrism {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#amount: super::super::types::Decimal {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#base: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
        }
    }
}
#[doc = "Contain the details of  the individual lens specifications and serves as the authorization for the fullfillment by certified professionals."]
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescriptionLensSpecification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Identifies the type of vision correction product which is required for the patient."]
    pub r#product: Box<super::super::types::CodeableConcept>,
    #[doc = "The eye for which the lens specification applies."]
    pub r#eye: super::super::types::Code,
    #[doc = "Lens power measured in dioptres (0.25 units)."]
    pub r#sphere: Option<super::super::types::Decimal>,
    #[doc = "Power adjustment for astigmatism measured in dioptres (0.25 units)."]
    pub r#cylinder: Option<super::super::types::Decimal>,
    #[doc = "Adjustment for astigmatism measured in integer degrees."]
    pub r#axis: Option<super::super::types::Integer>,
    #[doc = "Allows for adjustment on two axis."]
    pub r#prism: Vec<VisionPrescriptionLensSpecificationPrism>,
    #[doc = "Power adjustment for multifocal lenses measured in dioptres (0.25 units)."]
    pub r#add: Option<super::super::types::Decimal>,
    #[doc = "Contact lens power measured in dioptres (0.25 units)."]
    pub r#power: Option<super::super::types::Decimal>,
    #[doc = "Back curvature measured in millimetres."]
    pub r#back_curve: Option<super::super::types::Decimal>,
    #[doc = "Contact lens diameter measured in millimetres."]
    pub r#diameter: Option<super::super::types::Decimal>,
    #[doc = "The recommended maximum wear period for the lens."]
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    #[doc = "Special color or pattern."]
    pub r#color: Option<super::super::types::String>,
    #[doc = "Brand recommendations or restrictions."]
    pub r#brand: Option<super::super::types::String>,
    #[doc = "Notes for special requirements such as coatings and lens materials."]
    pub r#note: Vec<super::super::types::Annotation>,
}
#[allow(clippy::derivable_impls)]
impl Default for VisionPrescriptionLensSpecification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#product: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#eye: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#sphere: Default::default(),
            r#cylinder: Default::default(),
            r#axis: Default::default(),
            r#prism: Default::default(),
            r#add: Default::default(),
            r#power: Default::default(),
            r#back_curve: Default::default(),
            r#diameter: Default::default(),
            r#duration: Default::default(),
            r#color: Default::default(),
            r#brand: Default::default(),
            r#note: Default::default(),
        }
    }
}
#[doc = "An authorization for the provision of glasses and/or contact lenses to a patient."]
#[derive(Debug, Clone, PartialEq)]
pub struct VisionPrescription {
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
    #[doc = "A unique identifier assigned to this vision prescription."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "A resource reference to the person to whom the vision prescription applies."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "A reference to a resource that identifies the particular occurrence of contact between patient and health care provider during which the prescription was issued."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "The date (and perhaps time) when the prescription was written."]
    pub r#date_written: super::super::types::DateTime,
    #[doc = "The healthcare professional responsible for authorizing the prescription."]
    pub r#prescriber: Box<super::super::types::Reference>,
    #[doc = "Contain the details of  the individual lens specifications and serves as the authorization for the fullfillment by certified professionals."]
    pub r#lens_specification: Vec<VisionPrescriptionLensSpecification>,
}
#[allow(clippy::derivable_impls)]
impl Default for VisionPrescription {
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
            r#created: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#patient: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#encounter: Default::default(),
            r#date_written: super::super::types::DateTime {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#prescriber: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#lens_specification: Default::default(),
        }
    }
}
