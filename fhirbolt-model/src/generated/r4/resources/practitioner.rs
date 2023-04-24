// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
#[doc = "The official certifications, training, and licenses that authorize or otherwise pertain to the provision of care by the practitioner.  For example, a medical license issued by a medical board authorizing the practitioner to practice medicine within a certian locality."]
#[derive(Debug, Clone, PartialEq)]
pub struct PractitionerQualification {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An identifier that applies to this person's qualification in this role."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Coded representation of the qualification."]
    pub r#code: Box<super::super::types::CodeableConcept>,
    #[doc = "Period during which the qualification is valid."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Organization that regulates and issues the qualification."]
    pub r#issuer: Option<Box<super::super::types::Reference>>,
}
impl Default for PractitionerQualification {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#code: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#period: Default::default(),
            r#issuer: Default::default(),
        }
    }
}
#[doc = "A person who is directly or indirectly involved in the provisioning of healthcare.\n\nNeed to track doctors, staff, locums etc. for both healthcare practitioners, funders, etc."]
#[derive(Debug, Clone, PartialEq)]
pub struct Practitioner {
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
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An identifier that applies to this person in this role."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Whether this practitioner's record is in active use."]
    pub r#active: Option<super::super::types::Boolean>,
    #[doc = "The name(s) associated with the practitioner."]
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail for the practitioner, e.g. a telephone number or an email address."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Address(es) of the practitioner that are not role specific (typically home address). \rWork addresses are not typically entered in this property as they are usually role dependent."]
    pub r#address: Vec<Box<super::super::types::Address>>,
    #[doc = "Administrative Gender - the gender that the person is considered to have for administration and record keeping purposes."]
    pub r#gender: Option<super::super::types::Code>,
    #[doc = "The date of birth for the practitioner."]
    pub r#birth_date: Option<super::super::types::Date>,
    #[doc = "Image of the person."]
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    #[doc = "The official certifications, training, and licenses that authorize or otherwise pertain to the provision of care by the practitioner.  For example, a medical license issued by a medical board authorizing the practitioner to practice medicine within a certian locality."]
    pub r#qualification: Vec<PractitionerQualification>,
    #[doc = "A language the practitioner can use in patient communication."]
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
}
impl Default for Practitioner {
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
            r#active: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#address: Default::default(),
            r#gender: Default::default(),
            r#birth_date: Default::default(),
            r#photo: Default::default(),
            r#qualification: Default::default(),
            r#communication: Default::default(),
        }
    }
}
