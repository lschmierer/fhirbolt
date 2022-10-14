// Generated on 2022-10-14 by fhirbolt-codegen v0.1.0
#[doc = "The date or dates when the enclosed suite of services were performed or completed."]
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseServiced {
    Date(Box<super::super::types::Date>),
    Period(Box<super::super::types::Period>),
    Invalid,
}
impl Default for CoverageEligibilityResponseServiced {
    fn default() -> CoverageEligibilityResponseServiced {
        CoverageEligibilityResponseServiced::Invalid
    }
}
#[doc = "The quantity of the benefit which is permitted under the coverage."]
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for CoverageEligibilityResponseInsuranceItemBenefitAllowed {
    fn default() -> CoverageEligibilityResponseInsuranceItemBenefitAllowed {
        CoverageEligibilityResponseInsuranceItemBenefitAllowed::Invalid
    }
}
#[doc = "The quantity of the benefit which have been consumed to date."]
#[derive(Debug, Clone)]
pub enum CoverageEligibilityResponseInsuranceItemBenefitUsed {
    UnsignedInt(Box<super::super::types::UnsignedInt>),
    String(Box<super::super::types::String>),
    Money(Box<super::super::types::Money>),
    Invalid,
}
impl Default for CoverageEligibilityResponseInsuranceItemBenefitUsed {
    fn default() -> CoverageEligibilityResponseInsuranceItemBenefitUsed {
        CoverageEligibilityResponseInsuranceItemBenefitUsed::Invalid
    }
}
#[doc = "Benefits used to date."]
#[derive(Default, Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Classification of benefit being provided."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "The quantity of the benefit which is permitted under the coverage."]
    pub r#allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed>,
    #[doc = "The quantity of the benefit which have been consumed to date."]
    pub r#used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed>,
}
impl crate::AnyResource for CoverageEligibilityResponseInsuranceItemBenefit {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for CoverageEligibilityResponseInsuranceItemBenefit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("type", &self.r#type)?;
        if let Some(some) = self.r#allowed.as_ref() {
            match some {
                CoverageEligibilityResponseInsuranceItemBenefitAllowed::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allowedUnsignedInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_allowedUnsignedInt", &primitive_element)?;
                    }
                }
                CoverageEligibilityResponseInsuranceItemBenefitAllowed::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("allowedString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_allowedString", &primitive_element)?;
                    }
                }
                CoverageEligibilityResponseInsuranceItemBenefitAllowed::Money(ref value) => {
                    state.serialize_entry("allowedMoney", value)?;
                }
                CoverageEligibilityResponseInsuranceItemBenefitAllowed::Invalid => {
                    return Err(serde::ser::Error::custom("allowed is invalid"))
                }
            }
        }
        if let Some(some) = self.r#used.as_ref() {
            match some {
                CoverageEligibilityResponseInsuranceItemBenefitUsed::UnsignedInt(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("usedUnsignedInt", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_usedUnsignedInt", &primitive_element)?;
                    }
                }
                CoverageEligibilityResponseInsuranceItemBenefitUsed::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("usedString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_usedString", &primitive_element)?;
                    }
                }
                CoverageEligibilityResponseInsuranceItemBenefitUsed::Money(ref value) => {
                    state.serialize_entry("usedMoney", value)?;
                }
                CoverageEligibilityResponseInsuranceItemBenefitUsed::Invalid => {
                    return Err(serde::ser::Error::custom("used is invalid"))
                }
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageEligibilityResponseInsuranceItemBenefit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "allowedUnsignedInt")]
            AllowedUnsignedInt,
            #[serde(rename = "_allowedUnsignedInt")]
            AllowedUnsignedIntPrimitiveElement,
            #[serde(rename = "allowedString")]
            AllowedString,
            #[serde(rename = "_allowedString")]
            AllowedStringPrimitiveElement,
            #[serde(rename = "allowedMoney")]
            AllowedMoney,
            #[serde(rename = "usedUnsignedInt")]
            UsedUnsignedInt,
            #[serde(rename = "_usedUnsignedInt")]
            UsedUnsignedIntPrimitiveElement,
            #[serde(rename = "usedString")]
            UsedString,
            #[serde(rename = "_usedString")]
            UsedStringPrimitiveElement,
            #[serde(rename = "usedMoney")]
            UsedMoney,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageEligibilityResponseInsuranceItemBenefit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItemBenefit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsuranceItemBenefit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#allowed: Option<CoverageEligibilityResponseInsuranceItemBenefitAllowed> =
                    None;
                let mut r#used: Option<CoverageEligibilityResponseInsuranceItemBenefitUsed> = None;
                fhirbolt_shared :: serde_config :: de :: DESERIALIZATION_CONFIG . with (| config | { let config = config . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: Type => { if r#type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("type")) ; } r#type = Some (map_access . next_value () ?) ; } , Field :: AllowedUnsignedInt => { let r#enum = r#allowed . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; } } , Field :: AllowedUnsignedIntPrimitiveElement => { let r#enum = r#allowed . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; } } , Field :: AllowedString => { let r#enum = r#allowed . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; } } , Field :: AllowedStringPrimitiveElement => { let r#enum = r#allowed . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; } } , Field :: AllowedMoney => { if r#allowed . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedMoney")) ; } r#allowed = Some (CoverageEligibilityResponseInsuranceItemBenefitAllowed :: Money (map_access . next_value () ?)) ; } , Field :: UsedUnsignedInt => { let r#enum = r#used . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("used[x]")) ; } } , Field :: UsedUnsignedIntPrimitiveElement => { let r#enum = r#used . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_usedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_used[x]")) ; } } , Field :: UsedString => { let r#enum = r#used . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("used[x]")) ; } } , Field :: UsedStringPrimitiveElement => { let r#enum = r#used . get_or_insert (CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (Default :: default ())) ; if let CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_usedString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value () ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_used[x]")) ; } } , Field :: UsedMoney => { if r#used . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedMoney")) ; } r#used = Some (CoverageEligibilityResponseInsuranceItemBenefitUsed :: Money (map_access . next_value () ?)) ; } , Field :: Unknown (key) => if config . mode == fhirbolt_shared :: serde_config :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "type" , "allowedUnsignedInt" , "allowedString" , "allowedMoney" , "usedUnsignedInt" , "usedString" , "usedMoney" ,])) ; } } } Ok (CoverageEligibilityResponseInsuranceItemBenefit { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if config . mode == fhirbolt_shared :: serde_config :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#allowed , r#used , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Benefits and optionally current balances, and authorization details by category or service."]
#[derive(Default, Debug, Clone)]
pub struct CoverageEligibilityResponseInsuranceItem {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Code to identify the general type of benefits under which products and services are provided."]
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "This contains the product, service, drug or other billing code for the item."]
    pub r#product_or_service: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Item typification or modifiers codes to convey additional context for the product or service."]
    pub r#modifier: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "The practitioner who is eligible for the provision of the product or service."]
    pub r#provider: Option<Box<super::super::types::Reference>>,
    #[doc = "True if the indicated class of service is excluded from the plan, missing or False indicates the product or service is included in the coverage."]
    pub r#excluded: Option<super::super::types::Boolean>,
    #[doc = "A short name or tag for the benefit."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A richer description of the benefit or services covered."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Is a flag to indicate whether the benefits refer to in-network providers or out-of-network providers."]
    pub r#network: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates if the benefits apply to an individual or to the family."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The term or period of the values such as 'maximum lifetime benefit' or 'maximum annual visits'."]
    pub r#term: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Benefits used to date."]
    pub r#benefit: Vec<CoverageEligibilityResponseInsuranceItemBenefit>,
    #[doc = "A boolean flag indicating whether a preauthorization is required prior to actual service delivery."]
    pub r#authorization_required: Option<super::super::types::Boolean>,
    #[doc = "Codes or comments regarding information or actions associated with the preauthorization."]
    pub r#authorization_supporting: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "A web location for obtaining requirements or descriptive information regarding the preauthorization."]
    pub r#authorization_url: Option<super::super::types::Uri>,
}
impl serde::ser::Serialize for CoverageEligibilityResponseInsuranceItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if let Some(some) = self.r#category.as_ref() {
            state.serialize_entry("category", some)?;
        }
        if let Some(some) = self.r#product_or_service.as_ref() {
            state.serialize_entry("productOrService", some)?;
        }
        if !self.r#modifier.is_empty() {
            state.serialize_entry("modifier", &self.r#modifier)?;
        }
        if let Some(some) = self.r#provider.as_ref() {
            state.serialize_entry("provider", some)?;
        }
        if let Some(some) = self.r#excluded.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("excluded", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_excluded", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("name", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("description", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#network.as_ref() {
            state.serialize_entry("network", some)?;
        }
        if let Some(some) = self.r#unit.as_ref() {
            state.serialize_entry("unit", some)?;
        }
        if let Some(some) = self.r#term.as_ref() {
            state.serialize_entry("term", some)?;
        }
        if !self.r#benefit.is_empty() {
            state.serialize_entry("benefit", &self.r#benefit)?;
        }
        if let Some(some) = self.r#authorization_required.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("authorizationRequired", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authorizationRequired", &primitive_element)?;
            }
        }
        if !self.r#authorization_supporting.is_empty() {
            state.serialize_entry("authorizationSupporting", &self.r#authorization_supporting)?;
        }
        if let Some(some) = self.r#authorization_url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("authorizationUrl", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_authorizationUrl", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageEligibilityResponseInsuranceItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "productOrService")]
            ProductOrService,
            #[serde(rename = "modifier")]
            Modifier,
            #[serde(rename = "provider")]
            Provider,
            #[serde(rename = "excluded")]
            Excluded,
            #[serde(rename = "_excluded")]
            ExcludedPrimitiveElement,
            #[serde(rename = "name")]
            Name,
            #[serde(rename = "_name")]
            NamePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "network")]
            Network,
            #[serde(rename = "unit")]
            Unit,
            #[serde(rename = "term")]
            Term,
            #[serde(rename = "benefit")]
            Benefit,
            #[serde(rename = "authorizationRequired")]
            AuthorizationRequired,
            #[serde(rename = "_authorizationRequired")]
            AuthorizationRequiredPrimitiveElement,
            #[serde(rename = "authorizationSupporting")]
            AuthorizationSupporting,
            #[serde(rename = "authorizationUrl")]
            AuthorizationUrl,
            #[serde(rename = "_authorizationUrl")]
            AuthorizationUrlPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageEligibilityResponseInsuranceItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsuranceItem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#category: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#modifier: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#excluded: Option<super::super::types::Boolean> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#network: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#term: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#benefit: Option<Vec<CoverageEligibilityResponseInsuranceItemBenefit>> =
                    None;
                let mut r#authorization_required: Option<super::super::types::Boolean> = None;
                let mut r#authorization_supporting: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#authorization_url: Option<super::super::types::Uri> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::ProductOrService => {
                                if r#product_or_service.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productOrService",
                                    ));
                                }
                                r#product_or_service = Some(map_access.next_value()?);
                            }
                            Field::Modifier => {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier = Some(map_access.next_value()?);
                            }
                            Field::Provider => {
                                if r#provider.is_some() {
                                    return Err(serde::de::Error::duplicate_field("provider"));
                                }
                                r#provider = Some(map_access.next_value()?);
                            }
                            Field::Excluded => {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("excluded"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ExcludedPrimitiveElement => {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_excluded"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Name => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::NamePrimitiveElement => {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Description => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DescriptionPrimitiveElement => {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Network => {
                                if r#network.is_some() {
                                    return Err(serde::de::Error::duplicate_field("network"));
                                }
                                r#network = Some(map_access.next_value()?);
                            }
                            Field::Unit => {
                                if r#unit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("unit"));
                                }
                                r#unit = Some(map_access.next_value()?);
                            }
                            Field::Term => {
                                if r#term.is_some() {
                                    return Err(serde::de::Error::duplicate_field("term"));
                                }
                                r#term = Some(map_access.next_value()?);
                            }
                            Field::Benefit => {
                                if r#benefit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("benefit"));
                                }
                                r#benefit = Some(map_access.next_value()?);
                            }
                            Field::AuthorizationRequired => {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationRequired",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AuthorizationRequiredPrimitiveElement => {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationRequired",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::AuthorizationSupporting => {
                                if r#authorization_supporting.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationSupporting",
                                    ));
                                }
                                r#authorization_supporting = Some(map_access.next_value()?);
                            }
                            Field::AuthorizationUrl => {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationUrl",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::AuthorizationUrlPrimitiveElement => {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationUrl",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "category",
                                        "productOrService",
                                        "modifier",
                                        "provider",
                                        "excluded",
                                        "name",
                                        "description",
                                        "network",
                                        "unit",
                                        "term",
                                        "benefit",
                                        "authorizationRequired",
                                        "authorizationSupporting",
                                        "authorizationUrl",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CoverageEligibilityResponseInsuranceItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category,
                        r#product_or_service,
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#provider,
                        r#excluded,
                        r#name,
                        r#description,
                        r#network,
                        r#unit,
                        r#term,
                        r#benefit: r#benefit.unwrap_or(vec![]),
                        r#authorization_required,
                        r#authorization_supporting: r#authorization_supporting.unwrap_or(vec![]),
                        r#authorization_url,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Financial instruments for reimbursement for the health care products and services."]
#[derive(Default, Debug, Clone)]
pub struct CoverageEligibilityResponseInsurance {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reference to the insurance card level information contained in the Coverage resource. The coverage issuing insurer will use these details to locate the patient's actual coverage within the insurer's information system."]
    pub r#coverage: Box<super::super::types::Reference>,
    #[doc = "Flag indicating if the coverage provided is inforce currently if no service date(s) specified or for the whole duration of the service dates."]
    pub r#inforce: Option<super::super::types::Boolean>,
    #[doc = "The term of the benefits documented in this response."]
    pub r#benefit_period: Option<Box<super::super::types::Period>>,
    #[doc = "Benefits and optionally current balances, and authorization details by category or service."]
    pub r#item: Vec<CoverageEligibilityResponseInsuranceItem>,
}
impl serde::ser::Serialize for CoverageEligibilityResponseInsurance {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("coverage", &self.r#coverage)?;
        if let Some(some) = self.r#inforce.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("inforce", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_inforce", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#benefit_period.as_ref() {
            state.serialize_entry("benefitPeriod", some)?;
        }
        if !self.r#item.is_empty() {
            state.serialize_entry("item", &self.r#item)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageEligibilityResponseInsurance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "coverage")]
            Coverage,
            #[serde(rename = "inforce")]
            Inforce,
            #[serde(rename = "_inforce")]
            InforcePrimitiveElement,
            #[serde(rename = "benefitPeriod")]
            BenefitPeriod,
            #[serde(rename = "item")]
            Item,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageEligibilityResponseInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsurance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseInsurance, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#coverage: Option<Box<super::super::types::Reference>> = None;
                let mut r#inforce: Option<super::super::types::Boolean> = None;
                let mut r#benefit_period: Option<Box<super::super::types::Period>> = None;
                let mut r#item: Option<Vec<CoverageEligibilityResponseInsuranceItem>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Coverage => {
                                if r#coverage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("coverage"));
                                }
                                r#coverage = Some(map_access.next_value()?);
                            }
                            Field::Inforce => {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inforce"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::InforcePrimitiveElement => {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_inforce"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::BenefitPeriod => {
                                if r#benefit_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                                }
                                r#benefit_period = Some(map_access.next_value()?);
                            }
                            Field::Item => {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "coverage",
                                        "inforce",
                                        "benefitPeriod",
                                        "item",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CoverageEligibilityResponseInsurance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#coverage: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#coverage.unwrap_or(Default::default())
                        } else {
                            r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                        },
                        r#inforce,
                        r#benefit_period,
                        r#item: r#item.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Errors encountered during the processing of the request."]
#[derive(Default, Debug, Clone)]
pub struct CoverageEligibilityResponseError {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "An error code,from a specified code system, which details why the eligibility check could not be performed."]
    pub r#code: Box<super::super::types::CodeableConcept>,
}
impl serde::ser::Serialize for CoverageEligibilityResponseError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        state.serialize_entry("code", &self.r#code)?;
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageEligibilityResponseError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "code")]
            Code,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageEligibilityResponseError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseError")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponseError, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &["id", "extension", "modifierExtension", "code"],
                                ));
                            },
                        }
                    }
                    Ok(CoverageEligibilityResponseError {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource."]
#[derive(Default, Debug, Clone)]
pub struct CoverageEligibilityResponse {
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
    #[doc = "A unique identifier assigned to this coverage eligiblity request."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The status of the resource instance."]
    pub r#status: super::super::types::Code,
    #[doc = "Code to specify whether requesting: prior authorization requirements for some service categories or billing codes; benefits for coverages specified or discovered; discovery and return of coverages for the patient; and/or validation that the specified coverage is in-force at the date/period specified or 'now' if not specified."]
    pub r#purpose: Vec<super::super::types::Code>,
    #[doc = "The party who is the beneficiary of the supplied coverage and for whom eligibility is sought."]
    pub r#patient: Box<super::super::types::Reference>,
    #[doc = "The date or dates when the enclosed suite of services were performed or completed."]
    pub r#serviced: Option<CoverageEligibilityResponseServiced>,
    #[doc = "The date this resource was created."]
    pub r#created: super::super::types::DateTime,
    #[doc = "The provider which is responsible for the request."]
    pub r#requestor: Option<Box<super::super::types::Reference>>,
    #[doc = "Reference to the original request resource."]
    pub r#request: Box<super::super::types::Reference>,
    #[doc = "The outcome of the request processing."]
    pub r#outcome: super::super::types::Code,
    #[doc = "A human readable description of the status of the adjudication."]
    pub r#disposition: Option<super::super::types::String>,
    #[doc = "The Insurer who issued the coverage in question and is the author of the response."]
    pub r#insurer: Box<super::super::types::Reference>,
    #[doc = "Financial instruments for reimbursement for the health care products and services."]
    pub r#insurance: Vec<CoverageEligibilityResponseInsurance>,
    #[doc = "A reference from the Insurer to which these services pertain to be used on further communication and as proof that the request occurred."]
    pub r#pre_auth_ref: Option<super::super::types::String>,
    #[doc = "A code for the form to be used for printing the content."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Errors encountered during the processing of the request."]
    pub r#error: Vec<CoverageEligibilityResponseError>,
}
impl serde::ser::Serialize for CoverageEligibilityResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "CoverageEligibilityResponse")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("implicitRules", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("language", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_language", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#text.as_ref() {
            state.serialize_entry("text", some)?;
        }
        if !self.r#contained.is_empty() {
            state.serialize_entry("contained", &self.r#contained)?;
        }
        if !self.r#extension.is_empty() {
            state.serialize_entry("extension", &self.r#extension)?;
        }
        if !self.r#modifier_extension.is_empty() {
            state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
        }
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if let Some(some) = self.r#status.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("status", &some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if !self.r#purpose.is_empty() {
            let values = self
                .r#purpose
                .iter()
                .map(|v| &v.value)
                .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                .collect::<Result<Vec<_>, _>>()?;
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("purpose", &values)?;
            }
            let requires_elements = self
                .r#purpose
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#purpose
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(super::super::serde_helpers::PrimitiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_purpose", &primitive_elements)?;
            }
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#serviced.as_ref() {
            match some {
                CoverageEligibilityResponseServiced::Date(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("servicedDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_servicedDate", &primitive_element)?;
                    }
                }
                CoverageEligibilityResponseServiced::Period(ref value) => {
                    state.serialize_entry("servicedPeriod", value)?;
                }
                CoverageEligibilityResponseServiced::Invalid => {
                    return Err(serde::ser::Error::custom("serviced is invalid"))
                }
            }
        }
        if let Some(some) = self.r#created.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("created", &some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        if let Some(some) = self.r#requestor.as_ref() {
            state.serialize_entry("requestor", some)?;
        }
        state.serialize_entry("request", &self.r#request)?;
        if let Some(some) = self.r#outcome.value.as_ref() {
            let some = Ok(some)?;
            state.serialize_entry("outcome", &some)?;
        }
        if self.r#outcome.id.is_some() || !self.r#outcome.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#outcome.id,
                extension: &self.r#outcome.extension,
            };
            state.serialize_entry("_outcome", &primitive_element)?;
        }
        if let Some(some) = self.r#disposition.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("disposition", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_disposition", &primitive_element)?;
            }
        }
        state.serialize_entry("insurer", &self.r#insurer)?;
        if !self.r#insurance.is_empty() {
            state.serialize_entry("insurance", &self.r#insurance)?;
        }
        if let Some(some) = self.r#pre_auth_ref.as_ref() {
            if let Some(some) = some.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("preAuthRef", &some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_preAuthRef", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#form.as_ref() {
            state.serialize_entry("form", some)?;
        }
        if !self.r#error.is_empty() {
            state.serialize_entry("error", &self.r#error)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for CoverageEligibilityResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        #[derive(serde :: Deserialize)]
        #[serde(field_identifier)]
        enum Field {
            #[serde(rename = "resourceType")]
            ResourceType,
            #[serde(rename = "id")]
            Id,
            #[serde(rename = "meta")]
            Meta,
            #[serde(rename = "implicitRules")]
            ImplicitRules,
            #[serde(rename = "_implicitRules")]
            ImplicitRulesPrimitiveElement,
            #[serde(rename = "language")]
            Language,
            #[serde(rename = "_language")]
            LanguagePrimitiveElement,
            #[serde(rename = "text")]
            Text,
            #[serde(rename = "contained")]
            Contained,
            #[serde(rename = "extension")]
            Extension,
            #[serde(rename = "modifierExtension")]
            ModifierExtension,
            #[serde(rename = "identifier")]
            Identifier,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "purpose")]
            Purpose,
            #[serde(rename = "_purpose")]
            PurposePrimitiveElement,
            #[serde(rename = "patient")]
            Patient,
            #[serde(rename = "servicedDate")]
            ServicedDate,
            #[serde(rename = "_servicedDate")]
            ServicedDatePrimitiveElement,
            #[serde(rename = "servicedPeriod")]
            ServicedPeriod,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "requestor")]
            Requestor,
            #[serde(rename = "request")]
            Request,
            #[serde(rename = "outcome")]
            Outcome,
            #[serde(rename = "_outcome")]
            OutcomePrimitiveElement,
            #[serde(rename = "disposition")]
            Disposition,
            #[serde(rename = "_disposition")]
            DispositionPrimitiveElement,
            #[serde(rename = "insurer")]
            Insurer,
            #[serde(rename = "insurance")]
            Insurance,
            #[serde(rename = "preAuthRef")]
            PreAuthRef,
            #[serde(rename = "_preAuthRef")]
            PreAuthRefPrimitiveElement,
            #[serde(rename = "form")]
            Form,
            #[serde(rename = "error")]
            Error,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CoverageEligibilityResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponse")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<CoverageEligibilityResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#purpose: Option<Vec<super::super::types::Code>> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#serviced: Option<CoverageEligibilityResponseServiced> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#requestor: Option<Box<super::super::types::Reference>> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#outcome: Option<super::super::types::Code> = None;
                let mut r#disposition: Option<super::super::types::String> = None;
                let mut r#insurer: Option<Box<super::super::types::Reference>> = None;
                let mut r#insurance: Option<Vec<CoverageEligibilityResponseInsurance>> = None;
                let mut r#pre_auth_ref: Option<super::super::types::String> = None;
                let mut r#form: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#error: Option<Vec<CoverageEligibilityResponseError>> = None;
                fhirbolt_shared::serde_config::de::DESERIALIZATION_CONFIG.with(|config| {
                    let config = config.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CoverageEligibilityResponse" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CoverageEligibilityResponse",
                                    ));
                                }
                            }
                            Field::Id => {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                r#id = Some(map_access.next_value()?);
                            }
                            Field::Meta => {
                                if r#meta.is_some() {
                                    return Err(serde::de::Error::duplicate_field("meta"));
                                }
                                r#meta = Some(map_access.next_value()?);
                            }
                            Field::ImplicitRules => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::ImplicitRulesPrimitiveElement => {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Language => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::LanguagePrimitiveElement => {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Text => {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value()?);
                            }
                            Field::Contained => {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value()?);
                            }
                            Field::Extension => {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(map_access.next_value()?);
                            }
                            Field::ModifierExtension => {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(map_access.next_value()?);
                            }
                            Field::Identifier => {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some(map_access.next_value()?);
                            }
                            Field::Status => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::StatusPrimitiveElement => {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Purpose => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#purpose.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(values.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != values.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        values.len(),
                                        &"primitive elements length",
                                    ));
                                }
                                if vec.iter().any(|v| v.value.is_some()) {
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::PurposePrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#purpose.get_or_insert(
                                    std::iter::repeat(Default::default())
                                        .take(elements.len())
                                        .collect::<Vec<_>>(),
                                );
                                if vec.len() != elements.len() {
                                    return Err(serde::de::Error::invalid_length(
                                        elements.len(),
                                        &"primitive values length",
                                    ));
                                }
                                if vec
                                    .iter()
                                    .any(|e| e.id.is_some() || !e.extension.is_empty())
                                {
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Patient => {
                                if r#patient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("patient"));
                                }
                                r#patient = Some(map_access.next_value()?);
                            }
                            Field::ServicedDate => {
                                let r#enum = r#serviced.get_or_insert(
                                    CoverageEligibilityResponseServiced::Date(Default::default()),
                                );
                                if let CoverageEligibilityResponseServiced::Date(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "servicedDate",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("serviced[x]"));
                                }
                            }
                            Field::ServicedDatePrimitiveElement => {
                                let r#enum = r#serviced.get_or_insert(
                                    CoverageEligibilityResponseServiced::Date(Default::default()),
                                );
                                if let CoverageEligibilityResponseServiced::Date(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_servicedDate",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_serviced[x]"));
                                }
                            }
                            Field::ServicedPeriod => {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "servicedPeriod",
                                    ));
                                }
                                r#serviced = Some(CoverageEligibilityResponseServiced::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::Created => {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::CreatedPrimitiveElement => {
                                let some = r#created.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_created"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Requestor => {
                                if r#requestor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requestor"));
                                }
                                r#requestor = Some(map_access.next_value()?);
                            }
                            Field::Request => {
                                if r#request.is_some() {
                                    return Err(serde::de::Error::duplicate_field("request"));
                                }
                                r#request = Some(map_access.next_value()?);
                            }
                            Field::Outcome => {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::OutcomePrimitiveElement => {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcome"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Disposition => {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disposition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DispositionPrimitiveElement => {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_disposition"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Insurer => {
                                if r#insurer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurer"));
                                }
                                r#insurer = Some(map_access.next_value()?);
                            }
                            Field::Insurance => {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some(map_access.next_value()?);
                            }
                            Field::PreAuthRef => {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::PreAuthRefPrimitiveElement => {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Form => {
                                if r#form.is_some() {
                                    return Err(serde::de::Error::duplicate_field("form"));
                                }
                                r#form = Some(map_access.next_value()?);
                            }
                            Field::Error => {
                                if r#error.is_some() {
                                    return Err(serde::de::Error::duplicate_field("error"));
                                }
                                r#error = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if config.mode
                                == fhirbolt_shared::serde_config::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "meta",
                                        "implicitRules",
                                        "language",
                                        "text",
                                        "contained",
                                        "extension",
                                        "modifierExtension",
                                        "identifier",
                                        "status",
                                        "purpose",
                                        "patient",
                                        "servicedDate",
                                        "servicedPeriod",
                                        "created",
                                        "requestor",
                                        "request",
                                        "outcome",
                                        "disposition",
                                        "insurer",
                                        "insurance",
                                        "preAuthRef",
                                        "form",
                                        "error",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CoverageEligibilityResponse {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#purpose: r#purpose.unwrap_or(vec![]),
                        r#patient: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#serviced,
                        r#created: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#requestor,
                        r#request: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#request.unwrap_or(Default::default())
                        } else {
                            r#request.ok_or(serde::de::Error::missing_field("request"))?
                        },
                        r#outcome: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#outcome.unwrap_or(Default::default())
                        } else {
                            r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                        },
                        r#disposition,
                        r#insurer: if config.mode
                            == fhirbolt_shared::serde_config::de::DeserializationMode::Lax
                        {
                            r#insurer.unwrap_or(Default::default())
                        } else {
                            r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?
                        },
                        r#insurance: r#insurance.unwrap_or(vec![]),
                        r#pre_auth_ref,
                        r#form,
                        r#error: r#error.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
