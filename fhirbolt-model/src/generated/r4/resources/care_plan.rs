// Generated on 2022-12-15 by fhirbolt-codegen v0.1.0
#[doc = "The period, timing or frequency upon which the described activity is to occur."]
#[derive(Debug, Clone)]
pub enum CarePlanActivityDetailScheduled {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Invalid,
}
impl Default for CarePlanActivityDetailScheduled {
    fn default() -> CarePlanActivityDetailScheduled {
        CarePlanActivityDetailScheduled::Invalid
    }
}
#[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
#[derive(Debug, Clone)]
pub enum CarePlanActivityDetailProduct {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
    Invalid,
}
impl Default for CarePlanActivityDetailProduct {
    fn default() -> CarePlanActivityDetailProduct {
        CarePlanActivityDetailProduct::Invalid
    }
}
#[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
#[derive(Default, Debug, Clone)]
pub struct CarePlanActivityDetail {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "A description of the kind of resource the in-line definition of a care plan activity is representing.  The CarePlan.activity.detail is an in-line definition when a resource is not referenced using CarePlan.activity.reference.  For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest."]
    pub r#kind: Option<super::super::types::Code>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan activity."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "Detailed description of the type of planned activity; e.g. what lab test, what procedure, what kind of encounter."]
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Provides the rationale that drove the inclusion of this particular activity as part of the plan or the reason why the activity was prohibited."]
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Indicates another resource, such as the health condition(s), whose existence justifies this request and drove the inclusion of this particular activity as part of the plan."]
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Internal reference that identifies the goals that this activity is intended to contribute towards meeting."]
    pub r#goal: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies what progress is being made for the specific activity."]
    pub r#status: super::super::types::Code,
    #[doc = "Provides reason why the activity isn't yet started, is on hold, was cancelled, etc."]
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "If true, indicates that the described activity is one that must NOT be engaged in when following the plan.  If false, or missing, indicates that the described activity is one that should be engaged in when following the plan."]
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    #[doc = "The period, timing or frequency upon which the described activity is to occur."]
    pub r#scheduled: Option<CarePlanActivityDetailScheduled>,
    #[doc = "Identifies the facility where the activity will occur; e.g. home, hospital, specific clinic, etc."]
    pub r#location: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies who's expected to be involved in the activity."]
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies the food, drug or other product to be consumed or supplied in the activity."]
    pub r#product: Option<CarePlanActivityDetailProduct>,
    #[doc = "Identifies the quantity expected to be consumed in a given day."]
    pub r#daily_amount: Option<Box<super::super::types::Quantity>>,
    #[doc = "Identifies the quantity expected to be supplied, administered or consumed by the subject."]
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    #[doc = "This provides a textual description of constraints on the intended activity occurrence, including relation to other activities.  It may also include objectives, pre-conditions and end-conditions.  Finally, it may convey specifics about the activity such as body site, method, route, etc."]
    pub r#description: Option<super::super::types::String>,
}
impl crate::AnyResource for CarePlanActivityDetail {
    fn fhir_release() -> crate::FhirRelease {
        crate::FhirRelease::R4
    }
}
impl serde::ser::Serialize for CarePlanActivityDetail {
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
            if _ctx.output_json {
                if let Some(some) = self.r#kind.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("kind", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_kind", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#kind.as_ref() {
                    state.serialize_entry("kind", some)?;
                }
            }
            if _ctx.output_json {
                if !self.r#instantiates_canonical.is_empty() {
                    let values = self
                        .r#instantiates_canonical
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesCanonical", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_canonical
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_canonical
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_canonical.is_empty() {
                    state
                        .serialize_entry("instantiatesCanonical", &self.r#instantiates_canonical)?;
                }
            }
            if _ctx.output_json {
                if !self.r#instantiates_uri.is_empty() {
                    let values = self
                        .r#instantiates_uri
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesUri", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_uri
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_uri
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesUri", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_uri.is_empty() {
                    state.serialize_entry("instantiatesUri", &self.r#instantiates_uri)?;
                }
            }
            if let Some(some) = self.r#code.as_ref() {
                state.serialize_entry("code", some)?;
            }
            if !self.r#reason_code.is_empty() {
                state.serialize_entry("reasonCode", &self.r#reason_code)?;
            }
            if !self.r#reason_reference.is_empty() {
                state.serialize_entry("reasonReference", &self.r#reason_reference)?;
            }
            if !self.r#goal.is_empty() {
                state.serialize_entry("goal", &self.r#goal)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if let Some(some) = self.r#status_reason.as_ref() {
                state.serialize_entry("statusReason", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("doNotPerform", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_doNotPerform", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#do_not_perform.as_ref() {
                    state.serialize_entry("doNotPerform", some)?;
                }
            }
            if let Some(some) = self.r#scheduled.as_ref() {
                match some {
                    CarePlanActivityDetailScheduled::Timing(ref value) => {
                        state.serialize_entry("scheduledTiming", value)?;
                    }
                    CarePlanActivityDetailScheduled::Period(ref value) => {
                        state.serialize_entry("scheduledPeriod", value)?;
                    }
                    CarePlanActivityDetailScheduled::String(ref value) => {
                        if _ctx.output_json {
                            if let Some(some) = value.value.as_ref() {
                                let some = Ok(some)?;
                                state.serialize_entry("scheduledString", &some)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                let primitive_element =
                                    super::super::serde_helpers::PrimitiveElement {
                                        id: value.id.as_ref(),
                                        extension: &value.extension,
                                    };
                                state.serialize_entry("_scheduledString", &primitive_element)?;
                            }
                        } else {
                            state.serialize_entry("scheduledString", value)?;
                        }
                    }
                    CarePlanActivityDetailScheduled::Invalid => {
                        return Err(serde::ser::Error::custom("scheduled is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#location.as_ref() {
                state.serialize_entry("location", some)?;
            }
            if !self.r#performer.is_empty() {
                state.serialize_entry("performer", &self.r#performer)?;
            }
            if let Some(some) = self.r#product.as_ref() {
                match some {
                    CarePlanActivityDetailProduct::CodeableConcept(ref value) => {
                        state.serialize_entry("productCodeableConcept", value)?;
                    }
                    CarePlanActivityDetailProduct::Reference(ref value) => {
                        state.serialize_entry("productReference", value)?;
                    }
                    CarePlanActivityDetailProduct::Invalid => {
                        return Err(serde::ser::Error::custom("product is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#daily_amount.as_ref() {
                state.serialize_entry("dailyAmount", some)?;
            }
            if let Some(some) = self.r#quantity.as_ref() {
                state.serialize_entry("quantity", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CarePlanActivityDetail {
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
            #[serde(rename = "kind")]
            Kind,
            #[serde(rename = "_kind")]
            KindPrimitiveElement,
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "code")]
            Code,
            #[serde(rename = "reasonCode")]
            ReasonCode,
            #[serde(rename = "reasonReference")]
            ReasonReference,
            #[serde(rename = "goal")]
            Goal,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "statusReason")]
            StatusReason,
            #[serde(rename = "doNotPerform")]
            DoNotPerform,
            #[serde(rename = "_doNotPerform")]
            DoNotPerformPrimitiveElement,
            #[serde(rename = "scheduledTiming")]
            ScheduledTiming,
            #[serde(rename = "scheduledPeriod")]
            ScheduledPeriod,
            #[serde(rename = "scheduledString")]
            ScheduledString,
            #[serde(rename = "_scheduledString")]
            ScheduledStringPrimitiveElement,
            #[serde(rename = "location")]
            Location,
            #[serde(rename = "performer")]
            Performer,
            #[serde(rename = "productCodeableConcept")]
            ProductCodeableConcept,
            #[serde(rename = "productReference")]
            ProductReference,
            #[serde(rename = "dailyAmount")]
            DailyAmount,
            #[serde(rename = "quantity")]
            Quantity,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CarePlanActivityDetail;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CarePlanActivityDetail")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CarePlanActivityDetail, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#kind: Option<super::super::types::Code> = None;
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#code: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#reason_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#reason_reference: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#goal: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#status_reason: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#do_not_perform: Option<super::super::types::Boolean> = None;
                let mut r#scheduled: Option<CarePlanActivityDetailScheduled> = None;
                let mut r#location: Option<Box<super::super::types::Reference>> = None;
                let mut r#performer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#product: Option<CarePlanActivityDetailProduct> = None;
                let mut r#daily_amount: Option<Box<super::super::types::Quantity>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::Kind => {
                                let some = r#kind.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("kind"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::KindPrimitiveElement => {
                                let some = r#kind.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_kind"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::InstantiatesCanonical => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesCanonical",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::InstantiatesUri => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::Code => {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value()?);
                            }
                            Field::ReasonCode => {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code = Some(map_access.next_value()?);
                            }
                            Field::ReasonReference => {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some(map_access.next_value()?);
                            }
                            Field::Goal => {
                                if r#goal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("goal"));
                                }
                                r#goal = Some(map_access.next_value()?);
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
                            Field::StatusReason => {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason = Some(map_access.next_value()?);
                            }
                            Field::DoNotPerform => {
                                let some = r#do_not_perform.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doNotPerform"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::DoNotPerformPrimitiveElement => {
                                let some = r#do_not_perform.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::ScheduledTiming => {
                                if r#scheduled.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "scheduledTiming",
                                    ));
                                }
                                r#scheduled = Some(CarePlanActivityDetailScheduled::Timing(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ScheduledPeriod => {
                                if r#scheduled.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "scheduledPeriod",
                                    ));
                                }
                                r#scheduled = Some(CarePlanActivityDetailScheduled::Period(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ScheduledString => {
                                let r#enum = r#scheduled.get_or_insert(
                                    CarePlanActivityDetailScheduled::String(Default::default()),
                                );
                                if let CarePlanActivityDetailScheduled::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "scheduledString",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("scheduled[x]"));
                                }
                            }
                            Field::ScheduledStringPrimitiveElement => {
                                let r#enum = r#scheduled.get_or_insert(
                                    CarePlanActivityDetailScheduled::String(Default::default()),
                                );
                                if let CarePlanActivityDetailScheduled::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_scheduledString",
                                        ));
                                    }
                                    let super::super::serde_helpers::PrimitiveElementOwned {
                                        id,
                                        extension,
                                    } = map_access.next_value()?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_scheduled[x]"));
                                }
                            }
                            Field::Location => {
                                if r#location.is_some() {
                                    return Err(serde::de::Error::duplicate_field("location"));
                                }
                                r#location = Some(map_access.next_value()?);
                            }
                            Field::Performer => {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some(map_access.next_value()?);
                            }
                            Field::ProductCodeableConcept => {
                                if r#product.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productCodeableConcept",
                                    ));
                                }
                                r#product = Some(CarePlanActivityDetailProduct::CodeableConcept(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::ProductReference => {
                                if r#product.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "productReference",
                                    ));
                                }
                                r#product = Some(CarePlanActivityDetailProduct::Reference(
                                    map_access.next_value()?,
                                ));
                            }
                            Field::DailyAmount => {
                                if r#daily_amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dailyAmount"));
                                }
                                r#daily_amount = Some(map_access.next_value()?);
                            }
                            Field::Quantity => {
                                if r#quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("quantity"));
                                }
                                r#quantity = Some(map_access.next_value()?);
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
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "kind",
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "code",
                                        "reasonCode",
                                        "reasonReference",
                                        "goal",
                                        "status",
                                        "statusReason",
                                        "doNotPerform",
                                        "scheduledTiming",
                                        "scheduledPeriod",
                                        "scheduledString",
                                        "location",
                                        "performer",
                                        "productCodeableConcept",
                                        "productReference",
                                        "dailyAmount",
                                        "quantity",
                                        "description",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CarePlanActivityDetail {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#kind,
                        r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                        r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                        r#code,
                        r#reason_code: r#reason_code.unwrap_or(vec![]),
                        r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                        r#goal: r#goal.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#status_reason,
                        r#do_not_perform,
                        r#scheduled,
                        r#location,
                        r#performer: r#performer.unwrap_or(vec![]),
                        r#product,
                        r#daily_amount,
                        r#quantity,
                        r#description,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
#[derive(Default, Debug, Clone)]
pub struct CarePlanActivity {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Identifies the outcome at the point when the status of the activity is assessed.  For example, the outcome of an education activity could be patient understands (or not)."]
    pub r#outcome_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Details of the outcome or action resulting from the activity.  The reference to an \"event\" resource, such as Procedure or Encounter or Observation, is the result/outcome of the activity itself.  The activity can be conveyed using CarePlan.activity.detail OR using the CarePlan.activity.reference (a reference to a “request” resource)."]
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
    #[doc = "Notes about the adherence/status/progress of the activity."]
    pub r#progress: Vec<Box<super::super::types::Annotation>>,
    #[doc = "The details of the proposed activity represented in a specific resource."]
    pub r#reference: Option<Box<super::super::types::Reference>>,
    #[doc = "A simple summary of a planned activity suitable for a general care plan system (e.g. form driven) that doesn't know about specific resources such as procedure etc."]
    pub r#detail: Option<CarePlanActivityDetail>,
}
impl serde::ser::Serialize for CarePlanActivity {
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
            if !self.r#outcome_codeable_concept.is_empty() {
                state
                    .serialize_entry("outcomeCodeableConcept", &self.r#outcome_codeable_concept)?;
            }
            if !self.r#outcome_reference.is_empty() {
                state.serialize_entry("outcomeReference", &self.r#outcome_reference)?;
            }
            if !self.r#progress.is_empty() {
                state.serialize_entry("progress", &self.r#progress)?;
            }
            if let Some(some) = self.r#reference.as_ref() {
                state.serialize_entry("reference", some)?;
            }
            if let Some(some) = self.r#detail.as_ref() {
                state.serialize_entry("detail", some)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CarePlanActivity {
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
            #[serde(rename = "outcomeCodeableConcept")]
            OutcomeCodeableConcept,
            #[serde(rename = "outcomeReference")]
            OutcomeReference,
            #[serde(rename = "progress")]
            Progress,
            #[serde(rename = "reference")]
            Reference,
            #[serde(rename = "detail")]
            Detail,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CarePlanActivity;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CarePlanActivity")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CarePlanActivity, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#outcome_codeable_concept: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
                let mut r#outcome_reference: Option<Vec<Box<super::super::types::Reference>>> =
                    None;
                let mut r#progress: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#reference: Option<Box<super::super::types::Reference>> = None;
                let mut r#detail: Option<CarePlanActivityDetail> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
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
                            Field::OutcomeCodeableConcept => {
                                if r#outcome_codeable_concept.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outcomeCodeableConcept",
                                    ));
                                }
                                r#outcome_codeable_concept = Some(map_access.next_value()?);
                            }
                            Field::OutcomeReference => {
                                if r#outcome_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "outcomeReference",
                                    ));
                                }
                                r#outcome_reference = Some(map_access.next_value()?);
                            }
                            Field::Progress => {
                                if r#progress.is_some() {
                                    return Err(serde::de::Error::duplicate_field("progress"));
                                }
                                r#progress = Some(map_access.next_value()?);
                            }
                            Field::Reference => {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some(map_access.next_value()?);
                            }
                            Field::Detail => {
                                if r#detail.is_some() {
                                    return Err(serde::de::Error::duplicate_field("detail"));
                                }
                                r#detail = Some(map_access.next_value()?);
                            }
                            Field::Unknown(key) => if _ctx.config.mode
                                == fhirbolt_shared::serde_context::de::DeserializationMode::Strict
                            {
                                return Err(serde::de::Error::unknown_field(
                                    &key,
                                    &[
                                        "id",
                                        "extension",
                                        "modifierExtension",
                                        "outcomeCodeableConcept",
                                        "outcomeReference",
                                        "progress",
                                        "reference",
                                        "detail",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CarePlanActivity {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#outcome_codeable_concept: r#outcome_codeable_concept.unwrap_or(vec![]),
                        r#outcome_reference: r#outcome_reference.unwrap_or(vec![]),
                        r#progress: r#progress.unwrap_or(vec![]),
                        r#reference,
                        r#detail,
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[doc = "Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions."]
#[derive(Default, Debug, Clone)]
pub struct CarePlan {
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
    #[doc = "Business identifiers assigned to this care plan by the performer or other systems which remain constant as the resource is updated and propagates from server to server."]
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    #[doc = "The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    #[doc = "The URL pointing to an externally maintained protocol, guideline, questionnaire or other definition that is adhered to in whole or in part by this CarePlan."]
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    #[doc = "A care plan that is fulfilled in whole or in part by this care plan."]
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    #[doc = "Completed or terminated care plan whose function is taken by this new care plan."]
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    #[doc = "A larger care plan of which this particular care plan is a component or step."]
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    #[doc = "Indicates whether the plan is currently being acted upon, represents future intentions or is now a historical record."]
    pub r#status: super::super::types::Code,
    #[doc = "Indicates the level of authority/intentionality associated with the care plan and where the care plan fits into the workflow chain."]
    pub r#intent: super::super::types::Code,
    #[doc = "Identifies what \"kind\" of plan this is to support differentiation between multiple co-existing plans; e.g. \"Home health\", \"psychiatric\", \"asthma\", \"disease management\", \"wellness plan\", etc."]
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    #[doc = "Human-friendly name for the care plan."]
    pub r#title: Option<super::super::types::String>,
    #[doc = "A description of the scope and nature of the plan."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Identifies the patient or group whose intended care is described by the plan."]
    pub r#subject: Box<super::super::types::Reference>,
    #[doc = "The Encounter during which this CarePlan was created or to which the creation of this record is tightly associated."]
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    #[doc = "Indicates when the plan did (or is intended to) come into effect and end."]
    pub r#period: Option<Box<super::super::types::Period>>,
    #[doc = "Represents when this particular CarePlan record was created in the system, which is often a system-generated date."]
    pub r#created: Option<super::super::types::DateTime>,
    #[doc = "When populated, the author is responsible for the care plan.  The care plan is attributed to the author."]
    pub r#author: Option<Box<super::super::types::Reference>>,
    #[doc = "Identifies the individual(s) or organization who provided the contents of the care plan."]
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies all people and organizations who are expected to be involved in the care envisioned by this plan."]
    pub r#care_team: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies the conditions/problems/concerns/diagnoses/etc. whose management and/or mitigation are handled by this plan."]
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies portions of the patient's record that specifically influenced the formation of the plan.  These might include comorbidities, recent procedures, limitations, recent assessments, etc."]
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    #[doc = "Describes the intended objective(s) of carrying out the care plan."]
    pub r#goal: Vec<Box<super::super::types::Reference>>,
    #[doc = "Identifies a planned action to occur as part of the plan.  For example, a medication to be used, lab tests to perform, self-monitoring, education, etc."]
    pub r#activity: Vec<CarePlanActivity>,
    #[doc = "General notes about the care plan not covered elsewhere."]
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for CarePlan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.get();
            let mut state = serializer.serialize_map(None)?;
            state.serialize_entry("resourceType", "CarePlan")?;
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
            if !self.r#identifier.is_empty() {
                state.serialize_entry("identifier", &self.r#identifier)?;
            }
            if _ctx.output_json {
                if !self.r#instantiates_canonical.is_empty() {
                    let values = self
                        .r#instantiates_canonical
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesCanonical", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_canonical
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_canonical
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesCanonical", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_canonical.is_empty() {
                    state
                        .serialize_entry("instantiatesCanonical", &self.r#instantiates_canonical)?;
                }
            }
            if _ctx.output_json {
                if !self.r#instantiates_uri.is_empty() {
                    let values = self
                        .r#instantiates_uri
                        .iter()
                        .map(|v| &v.value)
                        .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                        .collect::<Result<Vec<_>, _>>()?;
                    if values.iter().any(|v| v.is_some()) {
                        state.serialize_entry("instantiatesUri", &values)?;
                    }
                    let requires_elements = self
                        .r#instantiates_uri
                        .iter()
                        .any(|e| e.id.is_some() || !e.extension.is_empty());
                    if requires_elements {
                        let primitive_elements: Vec<_> = self
                            .r#instantiates_uri
                            .iter()
                            .map(|e| {
                                if e.id.is_some() || !e.extension.is_empty() {
                                    Some(super::super::serde_helpers::PrimitiveElement {
                                        id: e.id.as_ref(),
                                        extension: &e.extension,
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect();
                        state.serialize_entry("_instantiatesUri", &primitive_elements)?;
                    }
                }
            } else {
                if !self.r#instantiates_uri.is_empty() {
                    state.serialize_entry("instantiatesUri", &self.r#instantiates_uri)?;
                }
            }
            if !self.r#based_on.is_empty() {
                state.serialize_entry("basedOn", &self.r#based_on)?;
            }
            if !self.r#replaces.is_empty() {
                state.serialize_entry("replaces", &self.r#replaces)?;
            }
            if !self.r#part_of.is_empty() {
                state.serialize_entry("partOf", &self.r#part_of)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#status.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#status.id.as_ref(),
                        extension: &self.r#status.extension,
                    };
                    state.serialize_entry("_status", &primitive_element)?;
                }
            } else {
                state.serialize_entry("status", &self.r#status)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#intent.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("intent", &some)?;
                }
                if self.r#intent.id.is_some() || !self.r#intent.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: self.r#intent.id.as_ref(),
                        extension: &self.r#intent.extension,
                    };
                    state.serialize_entry("_intent", &primitive_element)?;
                }
            } else {
                state.serialize_entry("intent", &self.r#intent)?;
            }
            if !self.r#category.is_empty() {
                state.serialize_entry("category", &self.r#category)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#title.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("title", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_title", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#title.as_ref() {
                    state.serialize_entry("title", some)?;
                }
            }
            if _ctx.output_json {
                if let Some(some) = self.r#description.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("description", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_description", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#description.as_ref() {
                    state.serialize_entry("description", some)?;
                }
            }
            state.serialize_entry("subject", &self.r#subject)?;
            if let Some(some) = self.r#encounter.as_ref() {
                state.serialize_entry("encounter", some)?;
            }
            if let Some(some) = self.r#period.as_ref() {
                state.serialize_entry("period", some)?;
            }
            if _ctx.output_json {
                if let Some(some) = self.r#created.as_ref() {
                    if let Some(some) = some.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("created", &some)?;
                    }
                    if some.id.is_some() || !some.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: some.id.as_ref(),
                            extension: &some.extension,
                        };
                        state.serialize_entry("_created", &primitive_element)?;
                    }
                }
            } else {
                if let Some(some) = self.r#created.as_ref() {
                    state.serialize_entry("created", some)?;
                }
            }
            if let Some(some) = self.r#author.as_ref() {
                state.serialize_entry("author", some)?;
            }
            if !self.r#contributor.is_empty() {
                state.serialize_entry("contributor", &self.r#contributor)?;
            }
            if !self.r#care_team.is_empty() {
                state.serialize_entry("careTeam", &self.r#care_team)?;
            }
            if !self.r#addresses.is_empty() {
                state.serialize_entry("addresses", &self.r#addresses)?;
            }
            if !self.r#supporting_info.is_empty() {
                state.serialize_entry("supportingInfo", &self.r#supporting_info)?;
            }
            if !self.r#goal.is_empty() {
                state.serialize_entry("goal", &self.r#goal)?;
            }
            if !self.r#activity.is_empty() {
                state.serialize_entry("activity", &self.r#activity)?;
            }
            if !self.r#note.is_empty() {
                state.serialize_entry("note", &self.r#note)?;
            }
            state.end()
        })
    }
}
impl<'de> serde::de::Deserialize<'de> for CarePlan {
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
            #[serde(rename = "instantiatesCanonical")]
            InstantiatesCanonical,
            #[serde(rename = "_instantiatesCanonical")]
            InstantiatesCanonicalPrimitiveElement,
            #[serde(rename = "instantiatesUri")]
            InstantiatesUri,
            #[serde(rename = "_instantiatesUri")]
            InstantiatesUriPrimitiveElement,
            #[serde(rename = "basedOn")]
            BasedOn,
            #[serde(rename = "replaces")]
            Replaces,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "intent")]
            Intent,
            #[serde(rename = "_intent")]
            IntentPrimitiveElement,
            #[serde(rename = "category")]
            Category,
            #[serde(rename = "title")]
            Title,
            #[serde(rename = "_title")]
            TitlePrimitiveElement,
            #[serde(rename = "description")]
            Description,
            #[serde(rename = "_description")]
            DescriptionPrimitiveElement,
            #[serde(rename = "subject")]
            Subject,
            #[serde(rename = "encounter")]
            Encounter,
            #[serde(rename = "period")]
            Period,
            #[serde(rename = "created")]
            Created,
            #[serde(rename = "_created")]
            CreatedPrimitiveElement,
            #[serde(rename = "author")]
            Author,
            #[serde(rename = "contributor")]
            Contributor,
            #[serde(rename = "careTeam")]
            CareTeam,
            #[serde(rename = "addresses")]
            Addresses,
            #[serde(rename = "supportingInfo")]
            SupportingInfo,
            #[serde(rename = "goal")]
            Goal,
            #[serde(rename = "activity")]
            Activity,
            #[serde(rename = "note")]
            Note,
            Unknown(String),
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = CarePlan;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CarePlan")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<CarePlan, V::Error>
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
                let mut r#instantiates_canonical: Option<Vec<super::super::types::Canonical>> =
                    None;
                let mut r#instantiates_uri: Option<Vec<super::super::types::Uri>> = None;
                let mut r#based_on: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#replaces: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#intent: Option<super::super::types::Code> = None;
                let mut r#category: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#title: Option<super::super::types::String> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#subject: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#period: Option<Box<super::super::types::Period>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#author: Option<Box<super::super::types::Reference>> = None;
                let mut r#contributor: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#care_team: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#addresses: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#supporting_info: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#goal: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#activity: Option<Vec<CarePlanActivity>> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                fhirbolt_shared::serde_context::de::DESERIALIZATION_CONTEXT.with(|_ctx| {
                    let _ctx = _ctx.get();
                    while let Some(map_access_key) = map_access.next_key()? {
                        match map_access_key {
                            Field::ResourceType => {
                                let value: std::borrow::Cow<str> = map_access.next_value()?;
                                if value != "CarePlan" {
                                    return Err(serde::de::Error::invalid_value(
                                        serde::de::Unexpected::Str(&value),
                                        &"CarePlan",
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
                            Field::InstantiatesCanonical => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesCanonical",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesCanonicalPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_canonical.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesCanonical",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::InstantiatesUri => {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            }
                            Field::InstantiatesUriPrimitiveElement => {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > = map_access.next_value()?;
                                let vec = r#instantiates_uri.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            }
                            Field::BasedOn => {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some(map_access.next_value()?);
                            }
                            Field::Replaces => {
                                if r#replaces.is_some() {
                                    return Err(serde::de::Error::duplicate_field("replaces"));
                                }
                                r#replaces = Some(map_access.next_value()?);
                            }
                            Field::PartOf => {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some(map_access.next_value()?);
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
                            Field::Intent => {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intent"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::IntentPrimitiveElement => {
                                let some = r#intent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_intent"));
                                }
                                let super::super::serde_helpers::PrimitiveElementOwned {
                                    id,
                                    extension,
                                } = map_access.next_value()?;
                                some.id = id;
                                some.extension = extension;
                            }
                            Field::Category => {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some(map_access.next_value()?);
                            }
                            Field::Title => {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            }
                            Field::TitlePrimitiveElement => {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
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
                            Field::Subject => {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some(map_access.next_value()?);
                            }
                            Field::Encounter => {
                                if r#encounter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("encounter"));
                                }
                                r#encounter = Some(map_access.next_value()?);
                            }
                            Field::Period => {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some(map_access.next_value()?);
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
                            Field::Author => {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some(map_access.next_value()?);
                            }
                            Field::Contributor => {
                                if r#contributor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contributor"));
                                }
                                r#contributor = Some(map_access.next_value()?);
                            }
                            Field::CareTeam => {
                                if r#care_team.is_some() {
                                    return Err(serde::de::Error::duplicate_field("careTeam"));
                                }
                                r#care_team = Some(map_access.next_value()?);
                            }
                            Field::Addresses => {
                                if r#addresses.is_some() {
                                    return Err(serde::de::Error::duplicate_field("addresses"));
                                }
                                r#addresses = Some(map_access.next_value()?);
                            }
                            Field::SupportingInfo => {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                r#supporting_info = Some(map_access.next_value()?);
                            }
                            Field::Goal => {
                                if r#goal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("goal"));
                                }
                                r#goal = Some(map_access.next_value()?);
                            }
                            Field::Activity => {
                                if r#activity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("activity"));
                                }
                                r#activity = Some(map_access.next_value()?);
                            }
                            Field::Note => {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some(map_access.next_value()?);
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
                                        "identifier",
                                        "instantiatesCanonical",
                                        "instantiatesUri",
                                        "basedOn",
                                        "replaces",
                                        "partOf",
                                        "status",
                                        "intent",
                                        "category",
                                        "title",
                                        "description",
                                        "subject",
                                        "encounter",
                                        "period",
                                        "created",
                                        "author",
                                        "contributor",
                                        "careTeam",
                                        "addresses",
                                        "supportingInfo",
                                        "goal",
                                        "activity",
                                        "note",
                                    ],
                                ));
                            },
                        }
                    }
                    Ok(CarePlan {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#instantiates_canonical: r#instantiates_canonical.unwrap_or(vec![]),
                        r#instantiates_uri: r#instantiates_uri.unwrap_or(vec![]),
                        r#based_on: r#based_on.unwrap_or(vec![]),
                        r#replaces: r#replaces.unwrap_or(vec![]),
                        r#part_of: r#part_of.unwrap_or(vec![]),
                        r#status: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#intent: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#intent.unwrap_or(Default::default())
                        } else {
                            r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                        },
                        r#category: r#category.unwrap_or(vec![]),
                        r#title,
                        r#description,
                        r#subject: if _ctx.config.mode
                            == fhirbolt_shared::serde_context::de::DeserializationMode::Lax
                        {
                            r#subject.unwrap_or(Default::default())
                        } else {
                            r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                        },
                        r#encounter,
                        r#period,
                        r#created,
                        r#author,
                        r#contributor: r#contributor.unwrap_or(vec![]),
                        r#care_team: r#care_team.unwrap_or(vec![]),
                        r#addresses: r#addresses.unwrap_or(vec![]),
                        r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                        r#goal: r#goal.unwrap_or(vec![]),
                        r#activity: r#activity.unwrap_or(vec![]),
                        r#note: r#note.unwrap_or(vec![]),
                    })
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
