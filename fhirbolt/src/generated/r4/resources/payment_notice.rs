// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct PaymentNotice {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: super::super::types::Code,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#response: Option<Box<super::super::types::Reference>>,
    pub r#created: super::super::types::DateTime,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#payment: Box<super::super::types::Reference>,
    pub r#payment_date: Option<super::super::types::Date>,
    pub r#payee: Option<Box<super::super::types::Reference>>,
    pub r#recipient: Box<super::super::types::Reference>,
    pub r#amount: Box<super::super::types::Money>,
    pub r#payment_status: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for PaymentNotice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "PaymentNotice")?;
        if let Some(some) = self.r#id.as_ref() {
            state.serialize_entry("id", some)?;
        }
        if let Some(some) = self.r#meta.as_ref() {
            state.serialize_entry("meta", some)?;
        }
        if let Some(some) = self.r#implicit_rules.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("implicitRules", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_implicitRules", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#language.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("language", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
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
            state.serialize_entry("status", some)?;
        }
        if self.r#status.id.is_some() || !self.r#status.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#request.as_ref() {
            state.serialize_entry("request", some)?;
        }
        if let Some(some) = self.r#response.as_ref() {
            state.serialize_entry("response", some)?;
        }
        if let Some(some) = self.r#created.value.as_ref() {
            state.serialize_entry("created", some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        if let Some(some) = self.r#provider.as_ref() {
            state.serialize_entry("provider", some)?;
        }
        state.serialize_entry("payment", &self.r#payment)?;
        if let Some(some) = self.r#payment_date.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("paymentDate", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_element = PrimtiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_paymentDate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#payee.as_ref() {
            state.serialize_entry("payee", some)?;
        }
        state.serialize_entry("recipient", &self.r#recipient)?;
        state.serialize_entry("amount", &self.r#amount)?;
        if let Some(some) = self.r#payment_status.as_ref() {
            state.serialize_entry("paymentStatus", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for PaymentNotice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = PaymentNotice;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("PaymentNotice")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<PaymentNotice, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<super::super::types::Meta>> = None;
                let mut r#implicit_rules: Option<super::super::types::Uri> = None;
                let mut r#language: Option<super::super::types::Code> = None;
                let mut r#text: Option<Box<super::super::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<super::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<super::super::types::Identifier>>> = None;
                let mut r#status: Option<super::super::types::Code> = None;
                let mut r#request: Option<Box<super::super::types::Reference>> = None;
                let mut r#response: Option<Box<super::super::types::Reference>> = None;
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#provider: Option<Box<super::super::types::Reference>> = None;
                let mut r#payment: Option<Box<super::super::types::Reference>> = None;
                let mut r#payment_date: Option<super::super::types::Date> = None;
                let mut r#payee: Option<Box<super::super::types::Reference>> = None;
                let mut r#recipient: Option<Box<super::super::types::Reference>> = None;
                let mut r#amount: Option<Box<super::super::types::Money>> = None;
                let mut r#payment_status: Option<Box<super::super::types::CodeableConcept>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        "meta" => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            r#meta = Some(map_access.next_value()?);
                        }
                        "implicitRules" => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_implicitRules" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "language" => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_language" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#language.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_language"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "text" => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        "contained" => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
                        }
                        "extension" => {
                            if r#extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("extension"));
                            }
                            r#extension = Some(map_access.next_value()?);
                        }
                        "modifierExtension" => {
                            if r#modifier_extension.is_some() {
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "status" => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_status" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#status.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_status"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "request" => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(map_access.next_value()?);
                        }
                        "response" => {
                            if r#response.is_some() {
                                return Err(serde::de::Error::duplicate_field("response"));
                            }
                            r#response = Some(map_access.next_value()?);
                        }
                        "created" => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_created" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#created.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_created"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "provider" => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(map_access.next_value()?);
                        }
                        "payment" => {
                            if r#payment.is_some() {
                                return Err(serde::de::Error::duplicate_field("payment"));
                            }
                            r#payment = Some(map_access.next_value()?);
                        }
                        "paymentDate" => {
                            let some = r#payment_date.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentDate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_paymentDate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#payment_date.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_paymentDate"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "payee" => {
                            if r#payee.is_some() {
                                return Err(serde::de::Error::duplicate_field("payee"));
                            }
                            r#payee = Some(map_access.next_value()?);
                        }
                        "recipient" => {
                            if r#recipient.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            r#recipient = Some(map_access.next_value()?);
                        }
                        "amount" => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(map_access.next_value()?);
                        }
                        "paymentStatus" => {
                            if r#payment_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("paymentStatus"));
                            }
                            r#payment_status = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "meta",
                                    "implicit_rules",
                                    "language",
                                    "text",
                                    "contained",
                                    "extension",
                                    "modifier_extension",
                                    "identifier",
                                    "status",
                                    "request",
                                    "response",
                                    "created",
                                    "provider",
                                    "payment",
                                    "payment_date",
                                    "payee",
                                    "recipient",
                                    "amount",
                                    "payment_status",
                                ],
                            ))
                        }
                    }
                }
                Ok(PaymentNotice {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#request,
                    r#response,
                    r#created: r#created.ok_or(serde::de::Error::missing_field("created"))?,
                    r#provider,
                    r#payment: r#payment.ok_or(serde::de::Error::missing_field("payment"))?,
                    r#payment_date,
                    r#payee,
                    r#recipient: r#recipient.ok_or(serde::de::Error::missing_field("recipient"))?,
                    r#amount: r#amount.ok_or(serde::de::Error::missing_field("amount"))?,
                    r#payment_status,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
