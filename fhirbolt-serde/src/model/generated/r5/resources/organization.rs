// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::OrganizationQualification,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Organization.qualification", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        }
        self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        if let Some(some) = self.value.r#period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("period", ctx))?;
        }
        if let Some(some) = self.value.r#issuer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("issuer", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::OrganizationQualification>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::OrganizationQualification>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::OrganizationQualification,
    >
{
    type Value = fhirbolt_model::r5::resources::OrganizationQualification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::OrganizationQualification,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::OrganizationQualification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("OrganizationQualification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::OrganizationQualification, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "issuer")]
                    Issuer,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "identifier",
                            "code",
                            "period",
                            "issuer",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#issuer: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Identifier >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Identifier>(),
                                )?);
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Period>>(),
                            )?);
                        }
                        Field::Issuer => {
                            if r#issuer.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            r#issuer = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::OrganizationQualification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#period,
                    r#issuer,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::OrganizationQualification>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::OrganizationQualification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::OrganizationQualification>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::OrganizationQualification>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::OrganizationQualification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::OrganizationQualification>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::OrganizationQualification>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::OrganizationQualification>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::Organization {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::resources::Organization>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "Organization", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Organization")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
        }
        if let Some(some) = self.value.r#text.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
        }
        if !self.value.r#contained.is_empty() {
            self.with_context(&self.value.r#contained, |ctx| {
                state.serialize_entry("contained", ctx)
            })?;
        }
        if !self.value.r#extension.is_empty() {
            self.with_context(&self.value.r#extension, |ctx| {
                state.serialize_entry("extension", ctx)
            })?;
        }
        if !self.value.r#modifier_extension.is_empty() {
            self.with_context(&self.value.r#modifier_extension, |ctx| {
                state.serialize_entry("modifierExtension", ctx)
            })?;
        }
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#active.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("active", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_active", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#active.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("active", ctx))?;
            }
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("name", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_name", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#name.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("name", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#alias.is_empty() {
                let values = self
                    .value
                    .r#alias
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("alias", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#alias
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#alias
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
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_alias", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#alias.is_empty() {
                self.with_context(&self.value.r#alias, |ctx| {
                    state.serialize_entry("alias", ctx)
                })?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#description.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
            }
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if let Some(some) = self.value.r#part_of.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("partOf", ctx))?;
        }
        if !self.value.r#endpoint.is_empty() {
            self.with_context(&self.value.r#endpoint, |ctx| {
                state.serialize_entry("endpoint", ctx)
            })?;
        }
        if !self.value.r#qualification.is_empty() {
            self.with_context(&self.value.r#qualification, |ctx| {
                state.serialize_entry("qualification", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r5::resources::Organization>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r5::resources::Organization>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = serializer.serialize_seq(Some(self.value.len()))?;
        for value in self.value {
            self.with_context(value, |ctx| seq_serializer.serialize_element(ctx))?
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r5::resources::Organization>
{
    type Value = fhirbolt_model::r5::resources::Organization;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r5::resources::Organization>
{
    type Value = fhirbolt_model::r5::resources::Organization;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::Organization,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::Organization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Organization")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::Organization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
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
                    #[serde(rename = "active")]
                    Active,
                    #[serde(rename = "_active")]
                    ActivePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "alias")]
                    Alias,
                    #[serde(rename = "_alias")]
                    AliasPrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "endpoint")]
                    Endpoint,
                    #[serde(rename = "qualification")]
                    Qualification,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
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
                            "active",
                            "type",
                            "name",
                            "alias",
                            "description",
                            "contact",
                            "partOf",
                            "endpoint",
                            "qualification",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#active: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#type: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#alias: Option<Vec<fhirbolt_model::r5::types::String>> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#contact: Option<Vec<fhirbolt_model::r5::types::ExtendedContactDetail>> =
                    None;
                let mut r#part_of: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#endpoint: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#qualification: Option<
                    Vec<fhirbolt_model::r5::resources::OrganizationQualification>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Organization" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Organization",
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
                            r#meta = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Meta>>(),
                            )?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                r#implicit_rules = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
                                )?);
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                r#language = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
                                )?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("language");
                            }
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r5::Resource>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::Resource>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Identifier >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Identifier>(),
                                )?);
                            }
                        }
                        Field::Active => {
                            if self.0.from_json {
                                let some = r#active.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("active"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#active.is_some() {
                                    return Err(serde::de::Error::duplicate_field("active"));
                                }
                                r#active = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ActivePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#active.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_active"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("active");
                            }
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::Alias => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("alias"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#alias.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::AliasPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#alias.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_alias"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("alias");
                            }
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Contact => {
                            if self.0.from_json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r5::types::ExtendedContactDetail,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: ExtendedContactDetail > ()) ?) ;
                            }
                        }
                        Field::PartOf => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Endpoint => {
                            if self.0.from_json {
                                if r#endpoint.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endpoint"));
                                }
                                r#endpoint = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#endpoint.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Qualification => {
                            if self.0.from_json {
                                if r#qualification.is_some() {
                                    return Err(serde::de::Error::duplicate_field("qualification"));
                                }
                                r#qualification =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r5::resources::OrganizationQualification,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#qualification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: OrganizationQualification > ()) ?) ;
                            }
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r5::resources::Organization {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#active,
                    r#type: r#type.unwrap_or(vec![]),
                    r#name,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#description,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#part_of,
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                    r#qualification: r#qualification.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::Organization>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::Organization>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::Organization>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::Organization>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::Organization>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::Organization>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::Organization>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(
                    self.0
                        .transmute::<fhirbolt_model::r5::resources::Organization>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
