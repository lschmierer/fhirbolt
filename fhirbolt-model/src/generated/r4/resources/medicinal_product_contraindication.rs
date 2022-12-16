// Generated on 2022-12-16 by fhirbolt-codegen v0.1.0
#[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
#[derive(Debug, Clone)]
pub enum MedicinalProductContraindicationOtherTherapyMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for MedicinalProductContraindicationOtherTherapyMedication {
    fn default() -> MedicinalProductContraindicationOtherTherapyMedication {
        MedicinalProductContraindicationOtherTherapyMedication::Invalid
    }
}
#[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductContraindicationOtherTherapy {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The type of relationship between the medicinal product indication or contraindication and another therapy."]
    pub r#therapy_relationship_type: Box<super::super::types::CodeableConcept>,
    #[doc = "Reference to a specific medication (active substance, medicinal product or class of products) as part of an indication or contraindication."]
    pub r#medication: MedicinalProductContraindicationOtherTherapyMedication,
}
impl serde::ser::Serialize for MedicinalProductContraindicationOtherTherapy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
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
            state.serialize_entry("therapyRelationshipType", &self.r#therapy_relationship_type)?;
            match self.r#medication {
                MedicinalProductContraindicationOtherTherapyMedication::CodeableConcept(
                    ref value,
                ) => {
                    state.serialize_entry("medicationCodeableConcept", value)?;
                }
                MedicinalProductContraindicationOtherTherapyMedication::Reference(ref value) => {
                    state.serialize_entry("medicationReference", value)?;
                }
                MedicinalProductContraindicationOtherTherapyMedication::Invalid => {
                    return Err(serde::ser::Error::custom("medication is a required field"))
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductContraindicationOtherTherapy {
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
            #[serde(rename = "therapyRelationshipType")]
            TherapyRelationshipType,
            #[serde(rename = "medicationCodeableConcept")]
            MedicationCodeableConcept,
            #[serde(rename = "medicationReference")]
            MedicationReference,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductContraindicationOtherTherapy;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductContraindicationOtherTherapy")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductContraindicationOtherTherapy, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#therapy_relationship_type: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#medication: Option<
                    MedicinalProductContraindicationOtherTherapyMedication,
                > = None;
                fhirbolt_shared :: serde_context :: de :: DESERIALIZATION_CONTEXT . with (| _ctx | { let _ctx = _ctx . get () ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value () ?) ; } , Field :: ModifierExtension => { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value () ?) ; } , Field :: TherapyRelationshipType => { if r#therapy_relationship_type . is_some () { return Err (serde :: de :: Error :: duplicate_field ("therapyRelationshipType")) ; } r#therapy_relationship_type = Some (map_access . next_value () ?) ; } , Field :: MedicationCodeableConcept => { if r#medication . is_some () { return Err (serde :: de :: Error :: duplicate_field ("medicationCodeableConcept")) ; } r#medication = Some (MedicinalProductContraindicationOtherTherapyMedication :: CodeableConcept (map_access . next_value () ?)) ; } , Field :: MedicationReference => { if r#medication . is_some () { return Err (serde :: de :: Error :: duplicate_field ("medicationReference")) ; } r#medication = Some (MedicinalProductContraindicationOtherTherapyMedication :: Reference (map_access . next_value () ?)) ; } , Field :: Unknown (key) => if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Strict { return Err (serde :: de :: Error :: unknown_field (& key , & ["id" , "extension" , "modifierExtension" , "therapyRelationshipType" , "medicationCodeableConcept" , "medicationReference" ,])) ; } } } Ok (MedicinalProductContraindicationOtherTherapy { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#therapy_relationship_type : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#therapy_relationship_type . unwrap_or (Default :: default ()) } else { r#therapy_relationship_type . ok_or (serde :: de :: Error :: missing_field ("therapyRelationshipType")) ? } , r#medication : if _ctx . config . mode == fhirbolt_shared :: serde_context :: de :: DeserializationMode :: Lax { r#medication . unwrap_or (Default :: default ()) } else { r#medication . ok_or (serde :: de :: Error :: missing_field ("medication[x]")) ? } , }) })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "The clinical particulars - indications, contraindications etc. of a medicinal product, including for regulatory purposes."]
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductContraindication {
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
    #[doc = "The medication for which this is an indication."]
    pub r#subject: Vec<Box<super::super::types::Reference>>,
    #[doc = "The disease, symptom or procedure for the contraindication."]
    pub r#disease: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The status of the disease or symptom for the contraindication."]
    pub r#disease_status: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "A comorbidity (concurrent condition) or coinfection."]
    pub r#comorbidity: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies as part of the indication."]
    pub r#therapeutic_indication: Vec<Box<super::super::types::Reference>>,
    #[doc = "Information about the use of the medicinal product in relation to other therapies described as part of the indication."]
    pub r#other_therapy: Vec<MedicinalProductContraindicationOtherTherapy>,
    #[doc = "The population group to which this applies."]
    pub r#population: Vec<Box<super::super::types::Population>>,
}
impl crate::AnyResource for MedicinalProductContraindication {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for MedicinalProductContraindication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "MedicinalProductContraindication")?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if let Some(some) = self.r#meta.as_ref() {
                state.serialize_entry("meta", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("implicitRules", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_implicitRules", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#implicit_rules.as_ref() {
                    state.serialize_entry("implicitRules", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#language.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("language", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_language", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#language.as_ref() {
                    state.serialize_entry("language", some)?;
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
            if !self.r#subject.is_empty() {
                state.serialize_entry("subject", &self.r#subject)?;
            }
            if let Some(some) = self.r#disease.as_ref() {
                state.serialize_entry("disease", some)?;
            }
            if let Some(some) = self.r#disease_status.as_ref() {
                state.serialize_entry("diseaseStatus", some)?;
            }
            if !self.r#comorbidity.is_empty() {
                state.serialize_entry("comorbidity", &self.r#comorbidity)?;
            }
            if !self.r#therapeutic_indication.is_empty() {
                state.serialize_entry("therapeuticIndication", &self.r#therapeutic_indication)?;
            }
            if !self.r#other_therapy.is_empty() {
                state.serialize_entry("otherTherapy", &self.r#other_therapy)?;
            }
            if !self.r#population.is_empty() {
                state.serialize_entry("population", &self.r#population)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductContraindication {
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
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "disease")]
            Disease,
            #[serde(rename = "diseaseStatus")]
            DiseaseStatus,
            #[serde(rename = "comorbidity")]
            Comorbidity,
            #[serde(rename = "therapeuticIndication")]
            TherapeuticIndication,
            #[serde(rename = "otherTherapy")]
            OtherTherapy,
            #[serde(rename = "population")]
            Population,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductContraindication;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductContraindication")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductContraindication, V::Error>
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
                let mut r#subject: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#disease: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#disease_status: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#comorbidity: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#therapeutic_indication: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#other_therapy: Option<Vec<MedicinalProductContraindicationOtherTherapy>> =
                    None;
                let mut r#population: Option<Vec<Box<super::super::types::Population>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "MedicinalProductContraindication" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"MedicinalProductContraindication",
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
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Disease => {
                                if r#disease.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disease"));
                                }
                                r#disease = Some(map_access.next_value()?);
                            }
                            Field::DiseaseStatus => {
                                if r#disease_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diseaseStatus"));
                                }
                                r#disease_status = Some(map_access.next_value()?);
                            }
                            Field::Comorbidity => {
                                if r#comorbidity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comorbidity"));
                                }
                                r#comorbidity = Some(map_access.next_value()?);
                            }
                            Field::TherapeuticIndication => {
                                if r#therapeutic_indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "therapeuticIndication",
                                    ));
                                }
                                r#therapeutic_indication = Some(map_access.next_value()?);
                            }
                            Field::OtherTherapy => {
                                if r#other_therapy.is_some() {
                                    return Err(serde::de::Error::duplicate_field("otherTherapy"));
                                }
                                r#other_therapy = Some(map_access.next_value()?);
                            }
                            Field::Population => {
                                if r#population.is_some() {
                                    return Err(serde::de::Error::duplicate_field("population"));
                                }
                                r#population = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
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
                                        "subject",
                                        "disease",
                                        "diseaseStatus",
                                        "comorbidity",
                                        "therapeuticIndication",
                                        "otherTherapy",
                                        "population",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(MedicinalProductContraindication {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#subject: r#subject.unwrap_or(vec![]),
                        r#disease,
                        r#disease_status,
                        r#comorbidity: r#comorbidity.unwrap_or(vec![]),
                        r#therapeutic_indication: r#therapeutic_indication.unwrap_or(vec![]),
                        r#other_therapy: r#other_therapy.unwrap_or(vec![]),
                        r#population: r#population.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
