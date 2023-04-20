// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
#[doc = "The contact for the health insurance product for a certain purpose."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanContact {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Indicates a purpose for which the contact can be reached."]
    pub r#purpose: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A name associated with the contact."]
    pub r#name: Option<Box<super::super::types::HumanName>>,
    #[doc = "A contact detail (e.g. a telephone number or an email address) by which the party may be contacted."]
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    #[doc = "Visiting or postal addresses for the contact."]
    pub r#address: Option<Box<super::super::types::Address>>,
}
impl Default for InsurancePlanContact {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#purpose: Default::default(),
            r#name: Default::default(),
            r#telecom: Default::default(),
            r#address: Default::default(),
        }
    }
}
#[doc = "The specific limits on the benefit."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanCoverageBenefitLimit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The maximum amount of a service item a plan will pay for a covered benefit.  For examples. wellness visits, or eyeglasses."]
    pub r#value: Option<Box<super::super::types::Quantity>>,
    #[doc = "The specific limit on the benefit."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
impl Default for InsurancePlanCoverageBenefitLimit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#value: Default::default(),
            r#code: Default::default(),
        }
    }
}
#[doc = "Specific benefits under this type of coverage."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanCoverageBenefit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of benefit (primary care; speciality care; inpatient; outpatient)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The referral requirements to have access/coverage for this benefit."]
    pub r#requirement: Option<super::super::types::String>,
    #[doc = "The specific limits on the benefit."]
    pub r#limit: Vec<InsurancePlanCoverageBenefitLimit>,
}
impl Default for InsurancePlanCoverageBenefit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#requirement: Default::default(),
            r#limit: Default::default(),
        }
    }
}
#[doc = "Details about the coverage offered by the insurance product."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanCoverage {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of coverage  (Medical; Dental; Mental Health; Substance Abuse; Vision; Drug; Short Term; Long Term Care; Hospice; Home Health)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to the network that providing the type of coverage."]
    pub r#network: Vec<Box<super::super::types::Reference>>,
    #[doc = "Specific benefits under this type of coverage."]
    pub r#benefit: Vec<InsurancePlanCoverageBenefit>,
}
impl Default for InsurancePlanCoverage {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#network: Default::default(),
            r#benefit: Default::default(),
        }
    }
}
#[doc = "Overall costs associated with the plan."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlanGeneralCost {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of cost."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Number of participants enrolled in the plan."]
    pub r#group_size: Option<super::super::types::PositiveInt>,
    #[doc = "Value of the cost."]
    pub r#cost: Option<Box<super::super::types::Money>>,
    #[doc = "Additional information about the general costs associated with this plan."]
    pub r#comment: Option<super::super::types::String>,
}
impl Default for InsurancePlanPlanGeneralCost {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#group_size: Default::default(),
            r#cost: Default::default(),
            r#comment: Default::default(),
        }
    }
}
#[doc = "List of the costs associated with a specific benefit."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of cost (copay; individual cap; family cap; coinsurance; deductible)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Whether the cost applies to in-network or out-of-network providers (in-network; out-of-network; other)."]
    pub r#applicability: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Additional information about the cost, such as information about funding sources (e.g. HSA, HRA, FSA, RRA)."]
    pub r#qualifiers: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The actual cost value. (some of the costs may be represented as percentages rather than currency, e.g. 10% coinsurance)."]
    pub r#value: Option<Box<super::super::types::Quantity>>,
}
impl Default for InsurancePlanPlanSpecificCostBenefitCost {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#applicability: Default::default(),
            r#qualifiers: Default::default(),
            r#value: Default::default(),
        }
    }
}
#[doc = "List of the specific benefits under this category of benefit."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlanSpecificCostBenefit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Type of specific benefit (preventative; primary care office visit; speciality office visit; hospitalization; emergency room; urgent care)."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "List of the costs associated with a specific benefit."]
    pub r#cost: Vec<InsurancePlanPlanSpecificCostBenefitCost>,
}
impl Default for InsurancePlanPlanSpecificCostBenefit {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#cost: Default::default(),
        }
    }
}
#[doc = "Costs associated with the coverage provided by the product."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlanSpecificCost {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "General category of benefit (Medical; Dental; Vision; Drug; Mental Health; Substance Abuse; Hospice, Home Health)."]
    pub r#category: Box<super::super::types::CodeableConcept>,
    #[doc = "List of the specific benefits under this category of benefit."]
    pub r#benefit: Vec<InsurancePlanPlanSpecificCostBenefit>,
}
impl Default for InsurancePlanPlanSpecificCost {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#category: {
                let mut default: Box<super::super::types::CodeableConcept> = Default::default();
                default.id = Some("$invalid".to_string());
                default
            },
            r#benefit: Default::default(),
        }
    }
}
#[doc = "Details about an insurance plan."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlanPlan {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifiers assigned to this health insurance plan which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "Type of plan. For example, \"Platinum\" or \"High Deductable\"."]
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The geographic region in which a health insurance plan's benefits apply."]
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    #[doc = "Reference to the network that providing the type of coverage."]
    pub r#network: Vec<Box<super::super::types::Reference>>,
    #[doc = "Overall costs associated with the plan."]
    pub r#general_cost: Vec<InsurancePlanPlanGeneralCost>,
    #[doc = "Costs associated with the coverage provided by the product."]
    pub r#specific_cost: Vec<InsurancePlanPlanSpecificCost>,
}
impl Default for InsurancePlanPlan {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#type: Default::default(),
            r#coverage_area: Default::default(),
            r#network: Default::default(),
            r#general_cost: Default::default(),
            r#specific_cost: Default::default(),
        }
    }
}
#[doc = "Details of a Health Insurance product/plan provided by an organization."]
#[derive(Debug, Clone, PartialEq)]
pub struct InsurancePlan {
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
    pub r#contained: Vec<Box<super::super::Resource>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Business identifiers assigned to this health insurance product which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The current state of the health insurance product."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The kind of health insurance product."]
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Official name of the health insurance product as designated by the owner."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A list of alternate names that the product is known as, or was known as in the past."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "The period of time that the health insurance product is available."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "The entity that is providing  the health insurance product and underwriting the risk.  This is typically an insurance carriers, other third-party payers, or health plan sponsors comonly referred to as 'payers'."]
    pub r#owned_by: Option<Box<super::super::types::Reference>>,
    #[doc = "An organization which administer other services such as underwriting, customer service and/or claims processing on behalf of the health insurance product owner."]
    pub r#administered_by: Option<Box<super::super::types::Reference>>,
    #[doc = "The geographic region in which a health insurance product's benefits apply."]
    pub r#coverage_area: Vec<Box<super::super::types::Reference>>,
    #[doc = "The contact for the health insurance product for a certain purpose."]
    pub r#contact: Vec<InsurancePlanContact>,
    #[doc = "The technical endpoints providing access to services operated for the health insurance product."]
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    #[doc = "Reference to the network included in the health insurance product."]
    pub r#network: Vec<Box<super::super::types::Reference>>,
    #[doc = "Details about the coverage offered by the insurance product."]
    pub r#coverage: Vec<InsurancePlanCoverage>,
    #[doc = "Details about an insurance plan."]
    pub r#plan: Vec<InsurancePlanPlan>,
}
impl Default for InsurancePlan {
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
            r#status: Default::default(),
            r#type: Default::default(),
            r#name: Default::default(),
            r#alias: Default::default(),
            r#period: Default::default(),
            r#owned_by: Default::default(),
            r#administered_by: Default::default(),
            r#coverage_area: Default::default(),
            r#contact: Default::default(),
            r#endpoint: Default::default(),
            r#network: Default::default(),
            r#coverage: Default::default(),
            r#plan: Default::default(),
        }
    }
}
