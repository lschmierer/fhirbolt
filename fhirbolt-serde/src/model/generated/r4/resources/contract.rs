// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractContentDefinition,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if let Some(some) = self.value.r#sub_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subType", ctx))?;
        }
        if let Some(some) = self.value.r#publisher.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("publisher", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#publication_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("publicationDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_publicationDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#publication_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("publicationDate", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#publication_status.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("publicationStatus", &some)?;
            }
            if self.value.r#publication_status.id.is_some()
                || !self.value.r#publication_status.extension.is_empty()
            {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#publication_status.id.as_ref(),
                    extension: &self.value.r#publication_status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_publicationStatus", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#publication_status, |ctx| {
                state.serialize_entry("publicationStatus", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#copyright.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("copyright", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_copyright", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#copyright.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("copyright", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractContentDefinition>,
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
        &Vec<fhirbolt_model::r4::resources::ContractContentDefinition>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>,
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
        fhirbolt_model::r4::resources::ContractContentDefinition,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractContentDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractContentDefinition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractContentDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractContentDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractContentDefinition, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subType")]
                    SubType,
                    #[serde(rename = "publisher")]
                    Publisher,
                    #[serde(rename = "publicationDate")]
                    PublicationDate,
                    #[serde(rename = "_publicationDate")]
                    PublicationDatePrimitiveElement,
                    #[serde(rename = "publicationStatus")]
                    PublicationStatus,
                    #[serde(rename = "_publicationStatus")]
                    PublicationStatusPrimitiveElement,
                    #[serde(rename = "copyright")]
                    Copyright,
                    #[serde(rename = "_copyright")]
                    CopyrightPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "subType",
                            "publisher",
                            "publicationDate",
                            "publicationStatus",
                            "copyright",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#publisher: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#publication_date: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#publication_status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#copyright: Option<fhirbolt_model::r4::types::Markdown> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::SubType => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Publisher => {
                            if r#publisher.is_some() {
                                return Err(serde::de::Error::duplicate_field("publisher"));
                            }
                            r#publisher = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::PublicationDate => {
                            if self.0.from_json {
                                let some = r#publication_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationDate",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#publication_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationDate",
                                    ));
                                }
                                r#publication_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::PublicationDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#publication_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_publicationDate",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publicationDate");
                            }
                        }
                        Field::PublicationStatus => {
                            if self.0.from_json {
                                let some = r#publication_status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationStatus",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#publication_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "publicationStatus",
                                    ));
                                }
                                r#publication_status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::PublicationStatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#publication_status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_publicationStatus",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publicationStatus");
                            }
                        }
                        Field::Copyright => {
                            if self.0.from_json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#copyright.is_some() {
                                    return Err(serde::de::Error::duplicate_field("copyright"));
                                }
                                r#copyright = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::CopyrightPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#copyright.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_copyright"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("copyright");
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
                Ok(fhirbolt_model::r4::resources::ContractContentDefinition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#sub_type,
                    r#publisher,
                    r#publication_date,
                    r#publication_status: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#publication_status.unwrap_or(Default::default())
                    } else {
                        r#publication_status
                            .ok_or(serde::de::Error::missing_field("publicationStatus"))?
                    },
                    r#copyright,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractContentDefinition>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractContentDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractContentDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractContentDefinition>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractContentDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractContentDefinition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractContentDefinition>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractContentDefinition>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractContentDefinition>>(
                        ),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermSecurityLabel,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if self.output_json {
            if !self.value.r#number.is_empty() {
                let values = self
                    .value
                    .r#number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("number", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#number
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
                        state.serialize_entry("_number", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#number.is_empty() {
                self.with_context(&self.value.r#number, |ctx| {
                    state.serialize_entry("number", ctx)
                })?;
            }
        }
        self.with_context(&self.value.r#classification, |ctx| {
            state.serialize_entry("classification", ctx)
        })?;
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        if !self.value.r#control.is_empty() {
            self.with_context(&self.value.r#control, |ctx| {
                state.serialize_entry("control", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>,
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
        fhirbolt_model::r4::resources::ContractTermSecurityLabel,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermSecurityLabel;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermSecurityLabel,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermSecurityLabel;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermSecurityLabel")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermSecurityLabel, V::Error>
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
                    #[serde(rename = "number")]
                    Number,
                    #[serde(rename = "_number")]
                    NumberPrimitiveElement,
                    #[serde(rename = "classification")]
                    Classification,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "control")]
                    Control,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "number",
                            "classification",
                            "category",
                            "control",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#number: Option<Vec<fhirbolt_model::r4::types::UnsignedInt>> = None;
                let mut r#classification: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#category: Option<Vec<Box<fhirbolt_model::r4::types::Coding>>> = None;
                let mut r#control: Option<Vec<Box<fhirbolt_model::r4::types::Coding>>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Number => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("number"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#number.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::UnsignedInt>(),
                                )?);
                            }
                        }
                        Field::NumberPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#number.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_number"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("number");
                            }
                        }
                        Field::Classification => {
                            if r#classification.is_some() {
                                return Err(serde::de::Error::duplicate_field("classification"));
                            }
                            r#classification = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                            )?);
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Coding > >> ()) ?) ;
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                                )?);
                            }
                        }
                        Field::Control => {
                            if self.0.from_json {
                                if r#control.is_some() {
                                    return Err(serde::de::Error::duplicate_field("control"));
                                }
                                r#control = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Coding > >> ()) ?) ;
                            } else {
                                let vec = r#control.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                                )?);
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
                Ok(fhirbolt_model::r4::resources::ContractTermSecurityLabel {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#number: r#number.unwrap_or(vec![]),
                    r#classification: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#classification.unwrap_or(Default::default())
                    } else {
                        r#classification.ok_or(serde::de::Error::missing_field("classification"))?
                    },
                    r#category: r#category.unwrap_or(vec![]),
                    r#control: r#control.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermSecurityLabel>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermSecurityLabel>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermSecurityLabel>>(
                        ),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermOfferParty,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if !self.value.r#reference.is_empty() {
            self.with_context(&self.value.r#reference, |ctx| {
                state.serialize_entry("reference", ctx)
            })?;
        }
        self.with_context(&self.value.r#role, |ctx| state.serialize_entry("role", ctx))?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermOfferParty>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>,
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
        fhirbolt_model::r4::resources::ContractTermOfferParty,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermOfferParty;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermOfferParty,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermOfferParty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferParty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermOfferParty, V::Error>
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
                    #[serde(rename = "reference")]
                    Reference,
                    #[serde(rename = "role")]
                    Role,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "reference", "role"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#reference: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Reference => {
                            if self.0.from_json {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::ContractTermOfferParty {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.unwrap_or(vec![]),
                    r#role: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#role.unwrap_or(Default::default())
                    } else {
                        r#role.ok_or(serde::de::Error::missing_field("role"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermOfferParty>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermOfferParty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermOfferParty>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermOfferParty>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermOfferParty>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermOfferAnswer,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        match self.value.r#value {
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Boolean(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueBoolean", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueBoolean", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueBoolean", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Decimal(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = some
                            .parse::<serde_json::Number>()
                            .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                        state.serialize_entry("valueDecimal", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDecimal", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDecimal", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Integer(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueInteger", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueInteger", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueInteger", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Date(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueDate", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDate", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDate", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::DateTime(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueDateTime", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueDateTime", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Time(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueTime", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueTime", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::String(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueString", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueString", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueString", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Uri(ref value) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("valueUri", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_valueUri", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| state.serialize_entry("valueUri", ctx))?;
                }
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueAttachment", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Coding(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueCoding", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Quantity(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueQuantity", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("valueReference", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Invalid => {
                return Err(serde::ser::Error::custom("value is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>,
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
        fhirbolt_model::r4::resources::ContractTermOfferAnswer,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermOfferAnswer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermOfferAnswer,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermOfferAnswer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOfferAnswer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermOfferAnswer, V::Error>
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
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueDecimal")]
                    ValueDecimal,
                    #[serde(rename = "_valueDecimal")]
                    ValueDecimalPrimitiveElement,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueDateTime")]
                    ValueDateTime,
                    #[serde(rename = "_valueDateTime")]
                    ValueDateTimePrimitiveElement,
                    #[serde(rename = "valueTime")]
                    ValueTime,
                    #[serde(rename = "_valueTime")]
                    ValueTimePrimitiveElement,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueUri")]
                    ValueUri,
                    #[serde(rename = "_valueUri")]
                    ValueUriPrimitiveElement,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    #[serde(rename = "valueCoding")]
                    ValueCoding,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "valueBoolean",
                            "valueDecimal",
                            "valueInteger",
                            "valueDate",
                            "valueDateTime",
                            "valueTime",
                            "valueString",
                            "valueUri",
                            "valueAttachment",
                            "valueCoding",
                            "valueQuantity",
                            "valueReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#value: Option<
                    fhirbolt_model::r4::resources::ContractTermOfferAnswerValue,
                > = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueDecimal => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Decimal (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDecimal")) ; } let value : serde_json :: Number = map_access . next_value () ? ; variant . value = Some (format ! ("{}" , value)) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDecimal"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Decimal (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Decimal > > ()) ?)) ;
                            }
                        }
                        Field::ValueDecimalPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Decimal (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Decimal (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDecimal")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDecimal");
                            }
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueDateTime => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDateTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::ValueDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDateTime");
                            }
                        }
                        Field::ValueTime => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Time (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueTime"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Time (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Time > > ()) ?)) ;
                            }
                        }
                        Field::ValueTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Time (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Time (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueTime");
                            }
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueUri => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Uri (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueUri")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueUri"));
                                }
                                r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Uri (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Uri > > ()) ?)) ;
                            }
                        }
                        Field::ValueUriPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Uri (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Uri (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueUri")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueUri");
                            }
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ValueCoding => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCoding"));
                            }
                            r#value = Some(
                                fhirbolt_model::r4::resources::ContractTermOfferAnswerValue::Coding(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswerValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Reference > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::ContractTermOfferAnswer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#value: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#value.unwrap_or(Default::default())
                    } else {
                        r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermOfferAnswer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermOfferAnswer>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermOfferAnswer>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractTermOffer>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if !self.value.r#party.is_empty() {
            self.with_context(&self.value.r#party, |ctx| {
                state.serialize_entry("party", ctx)
            })?;
        }
        if let Some(some) = self.value.r#topic.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("topic", ctx))?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#decision.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("decision", ctx))?;
        }
        if !self.value.r#decision_mode.is_empty() {
            self.with_context(&self.value.r#decision_mode, |ctx| {
                state.serialize_entry("decisionMode", ctx)
            })?;
        }
        if !self.value.r#answer.is_empty() {
            self.with_context(&self.value.r#answer, |ctx| {
                state.serialize_entry("answer", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#link_id.is_empty() {
                let values = self
                    .value
                    .r#link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("linkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#link_id
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
                        state.serialize_entry("_linkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#link_id.is_empty() {
                self.with_context(&self.value.r#link_id, |ctx| {
                    state.serialize_entry("linkId", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#security_label_number.is_empty() {
                let values = self
                    .value
                    .r#security_label_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("securityLabelNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#security_label_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#security_label_number.is_empty() {
                self.with_context(&self.value.r#security_label_number, |ctx| {
                    state.serialize_entry("securityLabelNumber", ctx)
                })?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermOffer>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermOffer>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermOffer>>,
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
        fhirbolt_model::r4::resources::ContractTermOffer,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermOffer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermOffer,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermOffer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermOffer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermOffer, V::Error>
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
                    #[serde(rename = "party")]
                    Party,
                    #[serde(rename = "topic")]
                    Topic,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "decision")]
                    Decision,
                    #[serde(rename = "decisionMode")]
                    DecisionMode,
                    #[serde(rename = "answer")]
                    Answer,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "linkId")]
                    LinkId,
                    #[serde(rename = "_linkId")]
                    LinkIdPrimitiveElement,
                    #[serde(rename = "securityLabelNumber")]
                    SecurityLabelNumber,
                    #[serde(rename = "_securityLabelNumber")]
                    SecurityLabelNumberPrimitiveElement,
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
                            "party",
                            "topic",
                            "type",
                            "decision",
                            "decisionMode",
                            "answer",
                            "text",
                            "linkId",
                            "securityLabelNumber",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#party: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermOfferParty>,
                > = None;
                let mut r#topic: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#decision: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#decision_mode: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#answer: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
                > = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#security_label_number: Option<
                    Vec<fhirbolt_model::r4::types::UnsignedInt>,
                > = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::Party => {
                            if self.0.from_json {
                                if r#party.is_some() {
                                    return Err(serde::de::Error::duplicate_field("party"));
                                }
                                r#party =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::ContractTermOfferParty,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#party.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermOfferParty > ()) ?) ;
                            }
                        }
                        Field::Topic => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topic"));
                            }
                            r#topic = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Decision => {
                            if r#decision.is_some() {
                                return Err(serde::de::Error::duplicate_field("decision"));
                            }
                            r#decision = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::DecisionMode => {
                            if self.0.from_json {
                                if r#decision_mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("decisionMode"));
                                }
                                r#decision_mode =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#decision_mode.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Answer => {
                            if self.0.from_json {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answer"));
                                }
                                r#answer =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::ContractTermOfferAnswer,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#answer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswer > ()) ?) ;
                            }
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::LinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::LinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("linkId");
                            }
                        }
                        Field::SecurityLabelNumber => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "securityLabelNumber",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#security_label_number.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::UnsignedInt>(),
                                )?);
                            }
                        }
                        Field::SecurityLabelNumberPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "_securityLabelNumber",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("securityLabelNumber");
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
                Ok(fhirbolt_model::r4::resources::ContractTermOffer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#party: r#party.unwrap_or(vec![]),
                    r#topic,
                    r#type,
                    r#decision,
                    r#decision_mode: r#decision_mode.unwrap_or(vec![]),
                    r#answer: r#answer.unwrap_or(vec![]),
                    r#text,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermOffer>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermOffer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermOffer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermOffer>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermOffer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermOffer>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermOffer>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermOffer>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermOffer>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOffer>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermOffer>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermOffer>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermOffer>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermAssetContext,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#reference.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("reference", ctx))?;
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermAssetContext>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>,
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
        fhirbolt_model::r4::resources::ContractTermAssetContext,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermAssetContext;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermAssetContext,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermAssetContext;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetContext")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermAssetContext, V::Error>
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
                    #[serde(rename = "reference")]
                    Reference,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "reference",
                            "code",
                            "text",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#reference: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#code: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Reference => {
                            if r#reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            r#reference = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Code => {
                            if self.0.from_json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
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
                Ok(fhirbolt_model::r4::resources::ContractTermAssetContext {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference,
                    r#code: r#code.unwrap_or(vec![]),
                    r#text,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermAssetContext>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermAssetContext>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermAssetContext>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermAssetContext>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermAssetContext>>(
                        ),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermAssetValuedItem,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#entity.as_ref() {
            match some { fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItemEntity :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("entityCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItemEntity :: Reference (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("entityReference" , ctx)) ? ; } , fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItemEntity :: Invalid => { return Err (serde :: ser :: Error :: custom ("entity is invalid")) } }
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#effective_time.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("effectiveTime", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_effectiveTime", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#effective_time.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("effectiveTime", ctx))?;
            }
        }
        if let Some(some) = self.value.r#quantity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("quantity", ctx))?;
        }
        if let Some(some) = self.value.r#unit_price.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitPrice", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#factor.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("factor", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_factor", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#factor.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("factor", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#points.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = some
                        .parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))?;
                    state.serialize_entry("points", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_points", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#points.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("points", ctx))?;
            }
        }
        if let Some(some) = self.value.r#net.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("net", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#payment.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("payment", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_payment", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#payment.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("payment", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#payment_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("paymentDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_paymentDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#payment_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("paymentDate", ctx))?;
            }
        }
        if let Some(some) = self.value.r#responsible.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("responsible", ctx))?;
        }
        if let Some(some) = self.value.r#recipient.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("recipient", ctx))?;
        }
        if self.output_json {
            if !self.value.r#link_id.is_empty() {
                let values = self
                    .value
                    .r#link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("linkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#link_id
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
                        state.serialize_entry("_linkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#link_id.is_empty() {
                self.with_context(&self.value.r#link_id, |ctx| {
                    state.serialize_entry("linkId", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#security_label_number.is_empty() {
                let values = self
                    .value
                    .r#security_label_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("securityLabelNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#security_label_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#security_label_number.is_empty() {
                self.with_context(&self.value.r#security_label_number, |ctx| {
                    state.serialize_entry("securityLabelNumber", ctx)
                })?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>>,
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
        fhirbolt_model::r4::resources::ContractTermAssetValuedItem,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermAssetValuedItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermAssetValuedItem,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermAssetValuedItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAssetValuedItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermAssetValuedItem, V::Error>
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
                    #[serde(rename = "entityCodeableConcept")]
                    EntityCodeableConcept,
                    #[serde(rename = "entityReference")]
                    EntityReference,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "effectiveTime")]
                    EffectiveTime,
                    #[serde(rename = "_effectiveTime")]
                    EffectiveTimePrimitiveElement,
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "unitPrice")]
                    UnitPrice,
                    #[serde(rename = "factor")]
                    Factor,
                    #[serde(rename = "_factor")]
                    FactorPrimitiveElement,
                    #[serde(rename = "points")]
                    Points,
                    #[serde(rename = "_points")]
                    PointsPrimitiveElement,
                    #[serde(rename = "net")]
                    Net,
                    #[serde(rename = "payment")]
                    Payment,
                    #[serde(rename = "_payment")]
                    PaymentPrimitiveElement,
                    #[serde(rename = "paymentDate")]
                    PaymentDate,
                    #[serde(rename = "_paymentDate")]
                    PaymentDatePrimitiveElement,
                    #[serde(rename = "responsible")]
                    Responsible,
                    #[serde(rename = "recipient")]
                    Recipient,
                    #[serde(rename = "linkId")]
                    LinkId,
                    #[serde(rename = "_linkId")]
                    LinkIdPrimitiveElement,
                    #[serde(rename = "securityLabelNumber")]
                    SecurityLabelNumber,
                    #[serde(rename = "_securityLabelNumber")]
                    SecurityLabelNumberPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "entityCodeableConcept",
                            "entityReference",
                            "identifier",
                            "effectiveTime",
                            "quantity",
                            "unitPrice",
                            "factor",
                            "points",
                            "net",
                            "payment",
                            "paymentDate",
                            "responsible",
                            "recipient",
                            "linkId",
                            "securityLabelNumber",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#entity: Option<
                    fhirbolt_model::r4::resources::ContractTermAssetValuedItemEntity,
                > = None;
                let mut r#identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#effective_time: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#unit_price: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#factor: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#points: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#net: Option<Box<fhirbolt_model::r4::types::Money>> = None;
                let mut r#payment: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#payment_date: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#responsible: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#recipient: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#security_label_number: Option<
                    Vec<fhirbolt_model::r4::types::UnsignedInt>,
                > = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::EntityCodeableConcept => {
                            if r#entity.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "entityCodeableConcept",
                                ));
                            }
                            r#entity = Some (fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItemEntity :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::EntityReference => {
                            if r#entity.is_some() {
                                return Err(serde::de::Error::duplicate_field("entityReference"));
                            }
                            r#entity = Some (fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItemEntity :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Reference > > ()) ?)) ;
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::EffectiveTime => {
                            if self.0.from_json {
                                let some = r#effective_time.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("effectiveTime"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#effective_time.is_some() {
                                    return Err(serde::de::Error::duplicate_field("effectiveTime"));
                                }
                                r#effective_time = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::EffectiveTimePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#effective_time.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_effectiveTime",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("effectiveTime");
                            }
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::UnitPrice => {
                            if r#unit_price.is_some() {
                                return Err(serde::de::Error::duplicate_field("unitPrice"));
                            }
                            r#unit_price = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Money>>(),
                            )?);
                        }
                        Field::Factor => {
                            if self.0.from_json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#factor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("factor"));
                                }
                                r#factor = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::FactorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#factor.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_factor"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("factor");
                            }
                        }
                        Field::Points => {
                            if self.0.from_json {
                                let some = r#points.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("points"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#points.is_some() {
                                    return Err(serde::de::Error::duplicate_field("points"));
                                }
                                r#points = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::PointsPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#points.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_points"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("points");
                            }
                        }
                        Field::Net => {
                            if r#net.is_some() {
                                return Err(serde::de::Error::duplicate_field("net"));
                            }
                            r#net = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Money>>(),
                            )?);
                        }
                        Field::Payment => {
                            if self.0.from_json {
                                let some = r#payment.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payment"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#payment.is_some() {
                                    return Err(serde::de::Error::duplicate_field("payment"));
                                }
                                r#payment = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::PaymentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#payment.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_payment"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("payment");
                            }
                        }
                        Field::PaymentDate => {
                            if self.0.from_json {
                                let some = r#payment_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("paymentDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#payment_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("paymentDate"));
                                }
                                r#payment_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::PaymentDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#payment_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_paymentDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("paymentDate");
                            }
                        }
                        Field::Responsible => {
                            if r#responsible.is_some() {
                                return Err(serde::de::Error::duplicate_field("responsible"));
                            }
                            r#responsible = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Recipient => {
                            if r#recipient.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            r#recipient = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::LinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::LinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("linkId");
                            }
                        }
                        Field::SecurityLabelNumber => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "securityLabelNumber",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#security_label_number.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::UnsignedInt>(),
                                )?);
                            }
                        }
                        Field::SecurityLabelNumberPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "_securityLabelNumber",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("securityLabelNumber");
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
                Ok(fhirbolt_model::r4::resources::ContractTermAssetValuedItem {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#entity,
                    r#identifier,
                    r#effective_time,
                    r#quantity,
                    r#unit_price,
                    r#factor,
                    r#points,
                    r#net,
                    r#payment,
                    r#payment_date,
                    r#responsible,
                    r#recipient,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItem >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractTermAsset>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#scope.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("scope", ctx))?;
        }
        if !self.value.r#type.is_empty() {
            self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#type_reference.is_empty() {
            self.with_context(&self.value.r#type_reference, |ctx| {
                state.serialize_entry("typeReference", ctx)
            })?;
        }
        if !self.value.r#subtype.is_empty() {
            self.with_context(&self.value.r#subtype, |ctx| {
                state.serialize_entry("subtype", ctx)
            })?;
        }
        if let Some(some) = self.value.r#relationship.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("relationship", ctx))?;
        }
        if !self.value.r#context.is_empty() {
            self.with_context(&self.value.r#context, |ctx| {
                state.serialize_entry("context", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#condition.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("condition", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_condition", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#condition.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("condition", ctx))?;
            }
        }
        if !self.value.r#period_type.is_empty() {
            self.with_context(&self.value.r#period_type, |ctx| {
                state.serialize_entry("periodType", ctx)
            })?;
        }
        if !self.value.r#period.is_empty() {
            self.with_context(&self.value.r#period, |ctx| {
                state.serialize_entry("period", ctx)
            })?;
        }
        if !self.value.r#use_period.is_empty() {
            self.with_context(&self.value.r#use_period, |ctx| {
                state.serialize_entry("usePeriod", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        if self.output_json {
            if !self.value.r#link_id.is_empty() {
                let values = self
                    .value
                    .r#link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("linkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#link_id
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
                        state.serialize_entry("_linkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#link_id.is_empty() {
                self.with_context(&self.value.r#link_id, |ctx| {
                    state.serialize_entry("linkId", ctx)
                })?;
            }
        }
        if !self.value.r#answer.is_empty() {
            self.with_context(&self.value.r#answer, |ctx| {
                state.serialize_entry("answer", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#security_label_number.is_empty() {
                let values = self
                    .value
                    .r#security_label_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("securityLabelNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#security_label_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#security_label_number.is_empty() {
                self.with_context(&self.value.r#security_label_number, |ctx| {
                    state.serialize_entry("securityLabelNumber", ctx)
                })?;
            }
        }
        if !self.value.r#valued_item.is_empty() {
            self.with_context(&self.value.r#valued_item, |ctx| {
                state.serialize_entry("valuedItem", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermAsset>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermAsset>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermAsset>>,
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
        fhirbolt_model::r4::resources::ContractTermAsset,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermAsset;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermAsset,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermAsset;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAsset")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermAsset, V::Error>
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
                    #[serde(rename = "scope")]
                    Scope,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "typeReference")]
                    TypeReference,
                    #[serde(rename = "subtype")]
                    Subtype,
                    #[serde(rename = "relationship")]
                    Relationship,
                    #[serde(rename = "context")]
                    Context,
                    #[serde(rename = "condition")]
                    Condition,
                    #[serde(rename = "_condition")]
                    ConditionPrimitiveElement,
                    #[serde(rename = "periodType")]
                    PeriodType,
                    #[serde(rename = "period")]
                    Period,
                    #[serde(rename = "usePeriod")]
                    UsePeriod,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "linkId")]
                    LinkId,
                    #[serde(rename = "_linkId")]
                    LinkIdPrimitiveElement,
                    #[serde(rename = "answer")]
                    Answer,
                    #[serde(rename = "securityLabelNumber")]
                    SecurityLabelNumber,
                    #[serde(rename = "_securityLabelNumber")]
                    SecurityLabelNumberPrimitiveElement,
                    #[serde(rename = "valuedItem")]
                    ValuedItem,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "scope",
                            "type",
                            "typeReference",
                            "subtype",
                            "relationship",
                            "context",
                            "condition",
                            "periodType",
                            "period",
                            "usePeriod",
                            "text",
                            "linkId",
                            "answer",
                            "securityLabelNumber",
                            "valuedItem",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#scope: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#type: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> = None;
                let mut r#type_reference: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#subtype: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> =
                    None;
                let mut r#relationship: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#context: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermAssetContext>,
                > = None;
                let mut r#condition: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#period_type: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#period: Option<Vec<Box<fhirbolt_model::r4::types::Period>>> = None;
                let mut r#use_period: Option<Vec<Box<fhirbolt_model::r4::types::Period>>> = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#answer: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermOfferAnswer>,
                > = None;
                let mut r#security_label_number: Option<
                    Vec<fhirbolt_model::r4::types::UnsignedInt>,
                > = None;
                let mut r#valued_item: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermAssetValuedItem>,
                > = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Scope => {
                            if r#scope.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            r#scope = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Type => {
                            if self.0.from_json {
                                if r#type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("type"));
                                }
                                r#type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::TypeReference => {
                            if self.0.from_json {
                                if r#type_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeReference"));
                                }
                                r#type_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#type_reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Subtype => {
                            if self.0.from_json {
                                if r#subtype.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtype"));
                                }
                                r#subtype =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#subtype.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Relationship => {
                            if r#relationship.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationship"));
                            }
                            r#relationship = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                            )?);
                        }
                        Field::Context => {
                            if self.0.from_json {
                                if r#context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("context"));
                                }
                                r#context = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::ContractTermAssetContext,
                                    >>(),
                                )?);
                            } else {
                                let vec = r#context.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermAssetContext > ()) ?) ;
                            }
                        }
                        Field::Condition => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#condition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("condition"));
                                }
                                r#condition = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ConditionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#condition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_condition"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("condition");
                            }
                        }
                        Field::PeriodType => {
                            if self.0.from_json {
                                if r#period_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("periodType"));
                                }
                                r#period_type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#period_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Period => {
                            if self.0.from_json {
                                if r#period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("period"));
                                }
                                r#period = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Period > >> ()) ?) ;
                            } else {
                                let vec = r#period.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                                )?);
                            }
                        }
                        Field::UsePeriod => {
                            if self.0.from_json {
                                if r#use_period.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usePeriod"));
                                }
                                r#use_period = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Period > >> ()) ?) ;
                            } else {
                                let vec = r#use_period.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                                )?);
                            }
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::LinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::LinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("linkId");
                            }
                        }
                        Field::Answer => {
                            if self.0.from_json {
                                if r#answer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("answer"));
                                }
                                r#answer =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4::resources::ContractTermOfferAnswer,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#answer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermOfferAnswer > ()) ?) ;
                            }
                        }
                        Field::SecurityLabelNumber => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "securityLabelNumber",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#security_label_number.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::UnsignedInt>(),
                                )?);
                            }
                        }
                        Field::SecurityLabelNumberPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "_securityLabelNumber",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("securityLabelNumber");
                            }
                        }
                        Field::ValuedItem => {
                            if self.0.from_json {
                                if r#valued_item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valuedItem"));
                                }
                                r#valued_item =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::ContractTermAssetValuedItem,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#valued_item.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermAssetValuedItem > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::ContractTermAsset {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#scope,
                    r#type: r#type.unwrap_or(vec![]),
                    r#type_reference: r#type_reference.unwrap_or(vec![]),
                    r#subtype: r#subtype.unwrap_or(vec![]),
                    r#relationship,
                    r#context: r#context.unwrap_or(vec![]),
                    r#condition,
                    r#period_type: r#period_type.unwrap_or(vec![]),
                    r#period: r#period.unwrap_or(vec![]),
                    r#use_period: r#use_period.unwrap_or(vec![]),
                    r#text,
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#answer: r#answer.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                    r#valued_item: r#valued_item.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermAsset>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermAsset>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermAsset>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermAsset>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermAsset>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermAsset>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermAsset>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermAsset>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermAsset>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAsset>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermAsset>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAsset>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermAsset>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermActionSubject,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if !self.value.r#reference.is_empty() {
            self.with_context(&self.value.r#reference, |ctx| {
                state.serialize_entry("reference", ctx)
            })?;
        }
        if let Some(some) = self.value.r#role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("role", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermActionSubject>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>,
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
        fhirbolt_model::r4::resources::ContractTermActionSubject,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermActionSubject;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermActionSubject,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermActionSubject;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermActionSubject")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermActionSubject, V::Error>
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
                    #[serde(rename = "reference")]
                    Reference,
                    #[serde(rename = "role")]
                    Role,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "reference", "role"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#reference: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Reference => {
                            if self.0.from_json {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::ContractTermActionSubject {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#reference: r#reference.unwrap_or(vec![]),
                    r#role,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermActionSubject>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermActionSubject>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermActionSubject>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermActionSubject>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermActionSubject>>(
                        ),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::ContractTermAction,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if self.output_json {
            if let Some(some) = self.value.r#do_not_perform.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("doNotPerform", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_doNotPerform", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#do_not_perform.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("doNotPerform", ctx))?;
            }
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if !self.value.r#subject.is_empty() {
            self.with_context(&self.value.r#subject, |ctx| {
                state.serialize_entry("subject", ctx)
            })?;
        }
        self.with_context(&self.value.r#intent, |ctx| {
            state.serialize_entry("intent", ctx)
        })?;
        if self.output_json {
            if !self.value.r#link_id.is_empty() {
                let values = self
                    .value
                    .r#link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("linkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#link_id
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
                        state.serialize_entry("_linkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#link_id.is_empty() {
                self.with_context(&self.value.r#link_id, |ctx| {
                    state.serialize_entry("linkId", ctx)
                })?;
            }
        }
        self.with_context(&self.value.r#status, |ctx| {
            state.serialize_entry("status", ctx)
        })?;
        if let Some(some) = self.value.r#context.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("context", ctx))?;
        }
        if self.output_json {
            if !self.value.r#context_link_id.is_empty() {
                let values = self
                    .value
                    .r#context_link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("contextLinkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#context_link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#context_link_id
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
                        state.serialize_entry("_contextLinkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#context_link_id.is_empty() {
                self.with_context(&self.value.r#context_link_id, |ctx| {
                    state.serialize_entry("contextLinkId", ctx)
                })?;
            }
        }
        if let Some(some) = self.value.r#occurrence.as_ref() {
            match some {
                fhirbolt_model::r4::resources::ContractTermActionOccurrence::DateTime(
                    ref value,
                ) => {
                    if self.output_json {
                        if let Some(some) = value.value.as_ref() {
                            let some = Ok(some)?;
                            state.serialize_entry("occurrenceDateTime", &some)?;
                        }
                        if value.id.is_some() || !value.extension.is_empty() {
                            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                                id: value.id.as_ref(),
                                extension: &value.extension,
                            };
                            self.with_context(&primitive_element, |ctx| {
                                state.serialize_entry("_occurrenceDateTime", ctx)
                            })?;
                        }
                    } else {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("occurrenceDateTime", ctx)
                        })?;
                    }
                }
                fhirbolt_model::r4::resources::ContractTermActionOccurrence::Period(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurrencePeriod", ctx))?;
                }
                fhirbolt_model::r4::resources::ContractTermActionOccurrence::Timing(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("occurrenceTiming", ctx))?;
                }
                fhirbolt_model::r4::resources::ContractTermActionOccurrence::Invalid => {
                    return Err(serde::ser::Error::custom("occurrence is invalid"))
                }
            }
        }
        if !self.value.r#requester.is_empty() {
            self.with_context(&self.value.r#requester, |ctx| {
                state.serialize_entry("requester", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#requester_link_id.is_empty() {
                let values = self
                    .value
                    .r#requester_link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("requesterLinkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#requester_link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#requester_link_id
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
                        state.serialize_entry("_requesterLinkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#requester_link_id.is_empty() {
                self.with_context(&self.value.r#requester_link_id, |ctx| {
                    state.serialize_entry("requesterLinkId", ctx)
                })?;
            }
        }
        if !self.value.r#performer_type.is_empty() {
            self.with_context(&self.value.r#performer_type, |ctx| {
                state.serialize_entry("performerType", ctx)
            })?;
        }
        if let Some(some) = self.value.r#performer_role.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("performerRole", ctx))?;
        }
        if let Some(some) = self.value.r#performer.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("performer", ctx))?;
        }
        if self.output_json {
            if !self.value.r#performer_link_id.is_empty() {
                let values = self
                    .value
                    .r#performer_link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("performerLinkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#performer_link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#performer_link_id
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
                        state.serialize_entry("_performerLinkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#performer_link_id.is_empty() {
                self.with_context(&self.value.r#performer_link_id, |ctx| {
                    state.serialize_entry("performerLinkId", ctx)
                })?;
            }
        }
        if !self.value.r#reason_code.is_empty() {
            self.with_context(&self.value.r#reason_code, |ctx| {
                state.serialize_entry("reasonCode", ctx)
            })?;
        }
        if !self.value.r#reason_reference.is_empty() {
            self.with_context(&self.value.r#reason_reference, |ctx| {
                state.serialize_entry("reasonReference", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#reason.is_empty() {
                let values = self
                    .value
                    .r#reason
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("reason", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#reason
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#reason
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
                        state.serialize_entry("_reason", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#reason.is_empty() {
                self.with_context(&self.value.r#reason, |ctx| {
                    state.serialize_entry("reason", ctx)
                })?;
            }
        }
        if self.output_json {
            if !self.value.r#reason_link_id.is_empty() {
                let values = self
                    .value
                    .r#reason_link_id
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("reasonLinkId", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#reason_link_id
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#reason_link_id
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
                        state.serialize_entry("_reasonLinkId", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#reason_link_id.is_empty() {
                self.with_context(&self.value.r#reason_link_id, |ctx| {
                    state.serialize_entry("reasonLinkId", ctx)
                })?;
            }
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if self.output_json {
            if !self.value.r#security_label_number.is_empty() {
                let values = self
                    .value
                    .r#security_label_number
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(|some| Ok(some)).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("securityLabelNumber", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#security_label_number
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#security_label_number
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
                        state.serialize_entry("_securityLabelNumber", ctx)
                    })?;
                }
            }
        } else {
            if !self.value.r#security_label_number.is_empty() {
                self.with_context(&self.value.r#security_label_number, |ctx| {
                    state.serialize_entry("securityLabelNumber", ctx)
                })?;
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractTermAction>,
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
        &Vec<fhirbolt_model::r4::resources::ContractTermAction>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTermAction>>,
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
        fhirbolt_model::r4::resources::ContractTermAction,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractTermAction;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTermAction,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTermAction;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTermAction")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTermAction, V::Error>
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
                    #[serde(rename = "doNotPerform")]
                    DoNotPerform,
                    #[serde(rename = "_doNotPerform")]
                    DoNotPerformPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "intent")]
                    Intent,
                    #[serde(rename = "linkId")]
                    LinkId,
                    #[serde(rename = "_linkId")]
                    LinkIdPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "context")]
                    Context,
                    #[serde(rename = "contextLinkId")]
                    ContextLinkId,
                    #[serde(rename = "_contextLinkId")]
                    ContextLinkIdPrimitiveElement,
                    #[serde(rename = "occurrenceDateTime")]
                    OccurrenceDateTime,
                    #[serde(rename = "_occurrenceDateTime")]
                    OccurrenceDateTimePrimitiveElement,
                    #[serde(rename = "occurrencePeriod")]
                    OccurrencePeriod,
                    #[serde(rename = "occurrenceTiming")]
                    OccurrenceTiming,
                    #[serde(rename = "requester")]
                    Requester,
                    #[serde(rename = "requesterLinkId")]
                    RequesterLinkId,
                    #[serde(rename = "_requesterLinkId")]
                    RequesterLinkIdPrimitiveElement,
                    #[serde(rename = "performerType")]
                    PerformerType,
                    #[serde(rename = "performerRole")]
                    PerformerRole,
                    #[serde(rename = "performer")]
                    Performer,
                    #[serde(rename = "performerLinkId")]
                    PerformerLinkId,
                    #[serde(rename = "_performerLinkId")]
                    PerformerLinkIdPrimitiveElement,
                    #[serde(rename = "reasonCode")]
                    ReasonCode,
                    #[serde(rename = "reasonReference")]
                    ReasonReference,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "_reason")]
                    ReasonPrimitiveElement,
                    #[serde(rename = "reasonLinkId")]
                    ReasonLinkId,
                    #[serde(rename = "_reasonLinkId")]
                    ReasonLinkIdPrimitiveElement,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "securityLabelNumber")]
                    SecurityLabelNumber,
                    #[serde(rename = "_securityLabelNumber")]
                    SecurityLabelNumberPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "doNotPerform",
                            "type",
                            "subject",
                            "intent",
                            "linkId",
                            "status",
                            "context",
                            "contextLinkId",
                            "occurrenceDateTime",
                            "occurrencePeriod",
                            "occurrenceTiming",
                            "requester",
                            "requesterLinkId",
                            "performerType",
                            "performerRole",
                            "performer",
                            "performerLinkId",
                            "reasonCode",
                            "reasonReference",
                            "reason",
                            "reasonLinkId",
                            "note",
                            "securityLabelNumber",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#do_not_perform: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#subject: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermActionSubject>,
                > = None;
                let mut r#intent: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#status: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#context: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#context_link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#occurrence: Option<
                    fhirbolt_model::r4::resources::ContractTermActionOccurrence,
                > = None;
                let mut r#requester: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#requester_link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#performer_type: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#performer_role: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#performer: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#performer_link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#reason_code: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#reason_reference: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#reason: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#reason_link_id: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4::types::Annotation>>> = None;
                let mut r#security_label_number: Option<
                    Vec<fhirbolt_model::r4::types::UnsignedInt>,
                > = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::DoNotPerform => {
                            if self.0.from_json {
                                let some = r#do_not_perform.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doNotPerform"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#do_not_perform.is_some() {
                                    return Err(serde::de::Error::duplicate_field("doNotPerform"));
                                }
                                r#do_not_perform = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::DoNotPerformPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#do_not_perform.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_doNotPerform"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("doNotPerform");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Subject => {
                            if self.0.from_json {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::ContractTermActionSubject,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#subject.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermActionSubject > ()) ?) ;
                            }
                        }
                        Field::Intent => {
                            if r#intent.is_some() {
                                return Err(serde::de::Error::duplicate_field("intent"));
                            }
                            r#intent = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::LinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("linkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::LinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_linkId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("linkId");
                            }
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Context => {
                            if r#context.is_some() {
                                return Err(serde::de::Error::duplicate_field("context"));
                            }
                            r#context = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ContextLinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#context_link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("contextLinkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#context_link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ContextLinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#context_link_id.get_or_insert(
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
                                        "_contextLinkId",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("contextLinkId");
                            }
                        }
                        Field::OccurrenceDateTime => {
                            if self.0.from_json {
                                let r#enum = r#occurrence . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermActionOccurrence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermActionOccurrence :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("occurrenceDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("occurrence[x]")) ; }
                            } else {
                                if r#occurrence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurrenceDateTime",
                                    ));
                                }
                                r#occurrence = Some (fhirbolt_model :: r4 :: resources :: ContractTermActionOccurrence :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::OccurrenceDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#occurrence . get_or_insert (fhirbolt_model :: r4 :: resources :: ContractTermActionOccurrence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r4 :: resources :: ContractTermActionOccurrence :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_occurrenceDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_occurrence[x]")) ; }
                            } else {
                                return unknown_field_error("occurrenceDateTime");
                            }
                        }
                        Field::OccurrencePeriod => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrencePeriod"));
                            }
                            r#occurrence = Some(
                                fhirbolt_model::r4::resources::ContractTermActionOccurrence::Period(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::OccurrenceTiming => {
                            if r#occurrence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurrenceTiming"));
                            }
                            r#occurrence = Some(
                                fhirbolt_model::r4::resources::ContractTermActionOccurrence::Timing(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Timing>>(),
                                    )?,
                                ),
                            );
                        }
                        Field::Requester => {
                            if self.0.from_json {
                                if r#requester.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requester"));
                                }
                                r#requester = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#requester.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::RequesterLinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#requester_link_id.get_or_insert(
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
                                        "requesterLinkId",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#requester_link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::RequesterLinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#requester_link_id.get_or_insert(
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
                                        "_requesterLinkId",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("requesterLinkId");
                            }
                        }
                        Field::PerformerType => {
                            if self.0.from_json {
                                if r#performer_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performerType"));
                                }
                                r#performer_type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#performer_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::PerformerRole => {
                            if r#performer_role.is_some() {
                                return Err(serde::de::Error::duplicate_field("performerRole"));
                            }
                            r#performer_role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Performer => {
                            if r#performer.is_some() {
                                return Err(serde::de::Error::duplicate_field("performer"));
                            }
                            r#performer = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::PerformerLinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#performer_link_id.get_or_insert(
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
                                        "performerLinkId",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#performer_link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::PerformerLinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#performer_link_id.get_or_insert(
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
                                        "_performerLinkId",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("performerLinkId");
                            }
                        }
                        Field::ReasonCode => {
                            if self.0.from_json {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#reason_code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::ReasonReference => {
                            if self.0.from_json {
                                if r#reason_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "reasonReference",
                                    ));
                                }
                                r#reason_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#reason_reference.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Reason => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#reason.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ReasonPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#reason.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_reason"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("reason");
                            }
                        }
                        Field::ReasonLinkId => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#reason_link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("reasonLinkId"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#reason_link_id.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::ReasonLinkIdPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#reason_link_id.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_reasonLinkId"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("reasonLinkId");
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::SecurityLabelNumber => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "securityLabelNumber",
                                    ));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#security_label_number.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::UnsignedInt>(),
                                )?);
                            }
                        }
                        Field::SecurityLabelNumberPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#security_label_number.get_or_insert(
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
                                        "_securityLabelNumber",
                                    ));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("securityLabelNumber");
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
                Ok(fhirbolt_model::r4::resources::ContractTermAction {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#do_not_perform,
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#intent: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#intent.unwrap_or(Default::default())
                    } else {
                        r#intent.ok_or(serde::de::Error::missing_field("intent"))?
                    },
                    r#link_id: r#link_id.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#context,
                    r#context_link_id: r#context_link_id.unwrap_or(vec![]),
                    r#occurrence,
                    r#requester: r#requester.unwrap_or(vec![]),
                    r#requester_link_id: r#requester_link_id.unwrap_or(vec![]),
                    r#performer_type: r#performer_type.unwrap_or(vec![]),
                    r#performer_role,
                    r#performer,
                    r#performer_link_id: r#performer_link_id.unwrap_or(vec![]),
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#reason_reference: r#reason_reference.unwrap_or(vec![]),
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#reason_link_id: r#reason_link_id.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#security_label_number: r#security_label_number.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTermAction>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTermAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTermAction>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTermAction>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTermAction>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTermAction>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTermAction>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTermAction>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTermAction>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAction>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTermAction>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTermAction>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTermAction>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractTerm>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#issued.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("issued", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_issued", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#issued.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("issued", ctx))?;
            }
        }
        if let Some(some) = self.value.r#applies.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("applies", ctx))?;
        }
        if let Some(some) = self.value.r#topic.as_ref() {
            match some {
                fhirbolt_model::r4::resources::ContractTermTopic::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("topicCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::ContractTermTopic::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("topicReference", ctx))?;
                }
                fhirbolt_model::r4::resources::ContractTermTopic::Invalid => {
                    return Err(serde::ser::Error::custom("topic is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#sub_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("subType", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        if !self.value.r#security_label.is_empty() {
            self.with_context(&self.value.r#security_label, |ctx| {
                state.serialize_entry("securityLabel", ctx)
            })?;
        }
        self.with_context(&self.value.r#offer, |ctx| {
            state.serialize_entry("offer", ctx)
        })?;
        if !self.value.r#asset.is_empty() {
            self.with_context(&self.value.r#asset, |ctx| {
                state.serialize_entry("asset", ctx)
            })?;
        }
        if !self.value.r#action.is_empty() {
            self.with_context(&self.value.r#action, |ctx| {
                state.serialize_entry("action", ctx)
            })?;
        }
        if !self.value.r#group.is_empty() {
            self.with_context(&self.value.r#group, |ctx| {
                state.serialize_entry("group", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::ContractTerm>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::ContractTerm>>
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractTerm>>,
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::ContractTerm>
{
    type Value = fhirbolt_model::r4::resources::ContractTerm;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractTerm,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractTerm;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractTerm")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractTerm, V::Error>
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
                    #[serde(rename = "issued")]
                    Issued,
                    #[serde(rename = "_issued")]
                    IssuedPrimitiveElement,
                    #[serde(rename = "applies")]
                    Applies,
                    #[serde(rename = "topicCodeableConcept")]
                    TopicCodeableConcept,
                    #[serde(rename = "topicReference")]
                    TopicReference,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subType")]
                    SubType,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "securityLabel")]
                    SecurityLabel,
                    #[serde(rename = "offer")]
                    Offer,
                    #[serde(rename = "asset")]
                    Asset,
                    #[serde(rename = "action")]
                    Action,
                    #[serde(rename = "group")]
                    Group,
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
                            "issued",
                            "applies",
                            "topicCodeableConcept",
                            "topicReference",
                            "type",
                            "subType",
                            "text",
                            "securityLabel",
                            "offer",
                            "asset",
                            "action",
                            "group",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#issued: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#applies: Option<Box<fhirbolt_model::r4::types::Period>> = None;
                let mut r#topic: Option<fhirbolt_model::r4::resources::ContractTermTopic> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#text: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#security_label: Option<
                    Vec<fhirbolt_model::r4::resources::ContractTermSecurityLabel>,
                > = None;
                let mut r#offer: Option<fhirbolt_model::r4::resources::ContractTermOffer> = None;
                let mut r#asset: Option<Vec<fhirbolt_model::r4::resources::ContractTermAsset>> =
                    None;
                let mut r#action: Option<Vec<fhirbolt_model::r4::resources::ContractTermAction>> =
                    None;
                let mut r#group: Option<Vec<fhirbolt_model::r4::resources::ContractTerm>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::Issued => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#issued.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                r#issued = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::IssuedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issued"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issued");
                            }
                        }
                        Field::Applies => {
                            if r#applies.is_some() {
                                return Err(serde::de::Error::duplicate_field("applies"));
                            }
                            r#applies = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                            )?);
                        }
                        Field::TopicCodeableConcept => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "topicCodeableConcept",
                                ));
                            }
                            r#topic = Some (fhirbolt_model :: r4 :: resources :: ContractTermTopic :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::TopicReference => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topicReference"));
                            }
                            r#topic =
                                Some(fhirbolt_model::r4::resources::ContractTermTopic::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::SubType => {
                            if r#sub_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("subType"));
                            }
                            r#sub_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::SecurityLabel => {
                            if self.0.from_json {
                                if r#security_label.is_some() {
                                    return Err(serde::de::Error::duplicate_field("securityLabel"));
                                }
                                r#security_label =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4::resources::ContractTermSecurityLabel,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#security_label.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermSecurityLabel > ()) ?) ;
                            }
                        }
                        Field::Offer => {
                            if r#offer.is_some() {
                                return Err(serde::de::Error::duplicate_field("offer"));
                            }
                            r#offer = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermOffer > ()) ?) ;
                        }
                        Field::Asset => {
                            if self.0.from_json {
                                if r#asset.is_some() {
                                    return Err(serde::de::Error::duplicate_field("asset"));
                                }
                                r#asset =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4::resources::ContractTermAsset,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#asset.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermAsset > ()) ?) ;
                            }
                        }
                        Field::Action => {
                            if self.0.from_json {
                                if r#action.is_some() {
                                    return Err(serde::de::Error::duplicate_field("action"));
                                }
                                r#action =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4::resources::ContractTermAction,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#action.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTermAction > ()) ?) ;
                            }
                        }
                        Field::Group => {
                            if self.0.from_json {
                                if r#group.is_some() {
                                    return Err(serde::de::Error::duplicate_field("group"));
                                }
                                r#group = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: ContractTerm >> ()) ?) ;
                            } else {
                                let vec = r#group.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTerm > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::ContractTerm {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#issued,
                    r#applies,
                    r#topic,
                    r#type,
                    r#sub_type,
                    r#text,
                    r#security_label: r#security_label.unwrap_or(vec![]),
                    r#offer: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#offer.unwrap_or(Default::default())
                    } else {
                        r#offer.ok_or(serde::de::Error::missing_field("offer"))?
                    },
                    r#asset: r#asset.unwrap_or(vec![]),
                    r#action: r#action.unwrap_or(vec![]),
                    r#group: r#group.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractTerm>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractTerm>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractTerm>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractTerm>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractTerm>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractTerm>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractTerm>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractTerm>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractTerm>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTerm>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractTerm>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractTerm>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractTerm>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractSigner>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        self.with_context(&self.value.r#party, |ctx| {
            state.serialize_entry("party", ctx)
        })?;
        if !self.value.r#signature.is_empty() {
            self.with_context(&self.value.r#signature, |ctx| {
                state.serialize_entry("signature", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractSigner>,
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
        &Vec<fhirbolt_model::r4::resources::ContractSigner>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractSigner>>,
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
        fhirbolt_model::r4::resources::ContractSigner,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractSigner;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractSigner,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractSigner;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractSigner")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractSigner, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "party")]
                    Party,
                    #[serde(rename = "signature")]
                    Signature,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "party",
                            "signature",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::Coding>> = None;
                let mut r#party: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#signature: Option<Vec<Box<fhirbolt_model::r4::types::Signature>>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Coding>>(),
                            )?);
                        }
                        Field::Party => {
                            if r#party.is_some() {
                                return Err(serde::de::Error::duplicate_field("party"));
                            }
                            r#party = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Signature => {
                            if self.0.from_json {
                                if r#signature.is_some() {
                                    return Err(serde::de::Error::duplicate_field("signature"));
                                }
                                r#signature = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Signature > >> ()) ?) ;
                            } else {
                                let vec = r#signature.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Signature>>(
                                            ),
                                    )?,
                                );
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
                Ok(fhirbolt_model::r4::resources::ContractSigner {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#party: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#party.unwrap_or(Default::default())
                    } else {
                        r#party.ok_or(serde::de::Error::missing_field("party"))?
                    },
                    r#signature: r#signature.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractSigner>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractSigner>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractSigner>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractSigner>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractSigner>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractSigner>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractSigner>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractSigner>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractSigner>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractSigner>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractSigner>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractSigner>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractSigner>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractFriendly>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        match self.value.r#content {
            fhirbolt_model::r4::resources::ContractFriendlyContent::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentAttachment", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractFriendlyContent::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentReference", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractFriendlyContent::Invalid => {
                return Err(serde::ser::Error::custom("content is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractFriendly>,
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
        &Vec<fhirbolt_model::r4::resources::ContractFriendly>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractFriendly>>,
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
        fhirbolt_model::r4::resources::ContractFriendly,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractFriendly;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractFriendly,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractFriendly;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractFriendly")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractFriendly, V::Error>
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
                    #[serde(rename = "contentAttachment")]
                    ContentAttachment,
                    #[serde(rename = "contentReference")]
                    ContentReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "contentAttachment",
                            "contentReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#content: Option<fhirbolt_model::r4::resources::ContractFriendlyContent> =
                    None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ContentAttachment => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content = Some (fhirbolt_model :: r4 :: resources :: ContractFriendlyContent :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ContentReference => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content = Some(
                                fhirbolt_model::r4::resources::ContractFriendlyContent::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ),
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
                Ok(fhirbolt_model::r4::resources::ContractFriendly {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#content.unwrap_or(Default::default())
                    } else {
                        r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractFriendly>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractFriendly>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractFriendly>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractFriendly>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractFriendly>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractFriendly>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractFriendly>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractFriendly>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractFriendly>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractFriendly>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractFriendly>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractFriendly>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractFriendly>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractLegal>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        match self.value.r#content {
            fhirbolt_model::r4::resources::ContractLegalContent::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentAttachment", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractLegalContent::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentReference", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractLegalContent::Invalid => {
                return Err(serde::ser::Error::custom("content is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::ContractLegal>,
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
        &Vec<fhirbolt_model::r4::resources::ContractLegal>,
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractLegal>>,
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
        fhirbolt_model::r4::resources::ContractLegal,
    >
{
    type Value = fhirbolt_model::r4::resources::ContractLegal;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractLegal,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractLegal;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractLegal")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractLegal, V::Error>
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
                    #[serde(rename = "contentAttachment")]
                    ContentAttachment,
                    #[serde(rename = "contentReference")]
                    ContentReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "contentAttachment",
                            "contentReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#content: Option<fhirbolt_model::r4::resources::ContractLegalContent> =
                    None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ContentAttachment => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content = Some (fhirbolt_model :: r4 :: resources :: ContractLegalContent :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ContentReference => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content = Some(
                                fhirbolt_model::r4::resources::ContractLegalContent::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ),
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
                Ok(fhirbolt_model::r4::resources::ContractLegal {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#content.unwrap_or(Default::default())
                    } else {
                        r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractLegal>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractLegal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractLegal>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractLegal>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractLegal>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractLegal>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractLegal>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractLegal>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractLegal>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractLegal>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractLegal>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractLegal>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractLegal>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::ContractRule>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        match self.value.r#content {
            fhirbolt_model::r4::resources::ContractRuleContent::Attachment(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentAttachment", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractRuleContent::Reference(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("contentReference", ctx))?;
            }
            fhirbolt_model::r4::resources::ContractRuleContent::Invalid => {
                return Err(serde::ser::Error::custom("content is a required field"))
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::ContractRule>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::ContractRule>>
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::ContractRule>>,
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
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::ContractRule>
{
    type Value = fhirbolt_model::r4::resources::ContractRule;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::ContractRule,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::ContractRule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ContractRule")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::ContractRule, V::Error>
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
                    #[serde(rename = "contentAttachment")]
                    ContentAttachment,
                    #[serde(rename = "contentReference")]
                    ContentReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "contentAttachment",
                            "contentReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#content: Option<fhirbolt_model::r4::resources::ContractRuleContent> =
                    None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ContentAttachment => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentAttachment"));
                            }
                            r#content = Some (fhirbolt_model :: r4 :: resources :: ContractRuleContent :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ContentReference => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentReference"));
                            }
                            r#content = Some(
                                fhirbolt_model::r4::resources::ContractRuleContent::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ),
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
                Ok(fhirbolt_model::r4::resources::ContractRule {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#content: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#content.unwrap_or(Default::default())
                    } else {
                        r#content.ok_or(serde::de::Error::missing_field("content[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::ContractRule>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::ContractRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::ContractRule>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::ContractRule>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::ContractRule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::ContractRule>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::ContractRule>;
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
                        .transmute::<fhirbolt_model::r4::resources::ContractRule>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::ContractRule>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::ContractRule>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::ContractRule>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::ContractRule>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::ContractRule>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::Contract {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::resources::Contract>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Contract")?;
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
            if let Some(some) = self.value.r#url.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("url", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_url", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#url.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("url", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#version.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("version", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_version", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#version.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("version", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#status.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("status", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_status", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
            }
        }
        if let Some(some) = self.value.r#legal_state.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("legalState", ctx))?;
        }
        if let Some(some) = self.value.r#instantiates_canonical.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("instantiatesCanonical", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#instantiates_uri.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("instantiatesUri", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_instantiatesUri", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#instantiates_uri.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("instantiatesUri", ctx))?;
            }
        }
        if let Some(some) = self.value.r#content_derivative.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("contentDerivative", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#issued.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("issued", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_issued", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#issued.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("issued", ctx))?;
            }
        }
        if let Some(some) = self.value.r#applies.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("applies", ctx))?;
        }
        if let Some(some) = self.value.r#expiration_type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("expirationType", ctx))?;
        }
        if !self.value.r#subject.is_empty() {
            self.with_context(&self.value.r#subject, |ctx| {
                state.serialize_entry("subject", ctx)
            })?;
        }
        if !self.value.r#authority.is_empty() {
            self.with_context(&self.value.r#authority, |ctx| {
                state.serialize_entry("authority", ctx)
            })?;
        }
        if !self.value.r#domain.is_empty() {
            self.with_context(&self.value.r#domain, |ctx| {
                state.serialize_entry("domain", ctx)
            })?;
        }
        if !self.value.r#site.is_empty() {
            self.with_context(&self.value.r#site, |ctx| state.serialize_entry("site", ctx))?;
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
            if let Some(some) = self.value.r#title.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("title", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_title", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#title.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("title", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#subtitle.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("subtitle", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_subtitle", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#subtitle.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("subtitle", ctx))?;
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
        if let Some(some) = self.value.r#author.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("author", ctx))?;
        }
        if let Some(some) = self.value.r#scope.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("scope", ctx))?;
        }
        if let Some(some) = self.value.r#topic.as_ref() {
            match some {
                fhirbolt_model::r4::resources::ContractTopic::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("topicCodeableConcept", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::ContractTopic::Reference(ref value) => {
                    self.with_context(value, |ctx| state.serialize_entry("topicReference", ctx))?;
                }
                fhirbolt_model::r4::resources::ContractTopic::Invalid => {
                    return Err(serde::ser::Error::custom("topic is invalid"))
                }
            }
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#sub_type.is_empty() {
            self.with_context(&self.value.r#sub_type, |ctx| {
                state.serialize_entry("subType", ctx)
            })?;
        }
        if let Some(some) = self.value.r#content_definition.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("contentDefinition", ctx))?;
        }
        if !self.value.r#term.is_empty() {
            self.with_context(&self.value.r#term, |ctx| state.serialize_entry("term", ctx))?;
        }
        if !self.value.r#supporting_info.is_empty() {
            self.with_context(&self.value.r#supporting_info, |ctx| {
                state.serialize_entry("supportingInfo", ctx)
            })?;
        }
        if !self.value.r#relevant_history.is_empty() {
            self.with_context(&self.value.r#relevant_history, |ctx| {
                state.serialize_entry("relevantHistory", ctx)
            })?;
        }
        if !self.value.r#signer.is_empty() {
            self.with_context(&self.value.r#signer, |ctx| {
                state.serialize_entry("signer", ctx)
            })?;
        }
        if !self.value.r#friendly.is_empty() {
            self.with_context(&self.value.r#friendly, |ctx| {
                state.serialize_entry("friendly", ctx)
            })?;
        }
        if !self.value.r#legal.is_empty() {
            self.with_context(&self.value.r#legal, |ctx| {
                state.serialize_entry("legal", ctx)
            })?;
        }
        if !self.value.r#rule.is_empty() {
            self.with_context(&self.value.r#rule, |ctx| state.serialize_entry("rule", ctx))?;
        }
        if let Some(some) = self.value.r#legally_binding.as_ref() {
            match some {
                fhirbolt_model::r4::resources::ContractLegallyBinding::Attachment(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("legallyBindingAttachment", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::ContractLegallyBinding::Reference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("legallyBindingReference", ctx)
                    })?;
                }
                fhirbolt_model::r4::resources::ContractLegallyBinding::Invalid => {
                    return Err(serde::ser::Error::custom("legally_binding is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::resources::Contract>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<fhirbolt_model::r4::resources::Contract>>
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r4::resources::Contract>>,
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Contract>
{
    type Value = fhirbolt_model::r4::resources::Contract;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Contract>
{
    type Value = fhirbolt_model::r4::resources::Contract;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::Contract,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::Contract;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Contract")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::Contract, V::Error>
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
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "_version")]
                    VersionPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "legalState")]
                    LegalState,
                    #[serde(rename = "instantiatesCanonical")]
                    InstantiatesCanonical,
                    #[serde(rename = "instantiatesUri")]
                    InstantiatesUri,
                    #[serde(rename = "_instantiatesUri")]
                    InstantiatesUriPrimitiveElement,
                    #[serde(rename = "contentDerivative")]
                    ContentDerivative,
                    #[serde(rename = "issued")]
                    Issued,
                    #[serde(rename = "_issued")]
                    IssuedPrimitiveElement,
                    #[serde(rename = "applies")]
                    Applies,
                    #[serde(rename = "expirationType")]
                    ExpirationType,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "authority")]
                    Authority,
                    #[serde(rename = "domain")]
                    Domain,
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "subtitle")]
                    Subtitle,
                    #[serde(rename = "_subtitle")]
                    SubtitlePrimitiveElement,
                    #[serde(rename = "alias")]
                    Alias,
                    #[serde(rename = "_alias")]
                    AliasPrimitiveElement,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "scope")]
                    Scope,
                    #[serde(rename = "topicCodeableConcept")]
                    TopicCodeableConcept,
                    #[serde(rename = "topicReference")]
                    TopicReference,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "subType")]
                    SubType,
                    #[serde(rename = "contentDefinition")]
                    ContentDefinition,
                    #[serde(rename = "term")]
                    Term,
                    #[serde(rename = "supportingInfo")]
                    SupportingInfo,
                    #[serde(rename = "relevantHistory")]
                    RelevantHistory,
                    #[serde(rename = "signer")]
                    Signer,
                    #[serde(rename = "friendly")]
                    Friendly,
                    #[serde(rename = "legal")]
                    Legal,
                    #[serde(rename = "rule")]
                    Rule,
                    #[serde(rename = "legallyBindingAttachment")]
                    LegallyBindingAttachment,
                    #[serde(rename = "legallyBindingReference")]
                    LegallyBindingReference,
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
                            "url",
                            "version",
                            "status",
                            "legalState",
                            "instantiatesCanonical",
                            "instantiatesUri",
                            "contentDerivative",
                            "issued",
                            "applies",
                            "expirationType",
                            "subject",
                            "authority",
                            "domain",
                            "site",
                            "name",
                            "title",
                            "subtitle",
                            "alias",
                            "author",
                            "scope",
                            "topicCodeableConcept",
                            "topicReference",
                            "type",
                            "subType",
                            "contentDefinition",
                            "term",
                            "supportingInfo",
                            "relevantHistory",
                            "signer",
                            "friendly",
                            "legal",
                            "rule",
                            "legallyBindingAttachment",
                            "legallyBindingReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#url: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#version: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#legal_state: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#instantiates_canonical: Option<
                    Box<fhirbolt_model::r4::types::Reference>,
                > = None;
                let mut r#instantiates_uri: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#content_derivative: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#issued: Option<fhirbolt_model::r4::types::DateTime> = None;
                let mut r#applies: Option<Box<fhirbolt_model::r4::types::Period>> = None;
                let mut r#expiration_type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#subject: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#authority: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#domain: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#site: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#title: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#subtitle: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#alias: Option<Vec<fhirbolt_model::r4::types::String>> = None;
                let mut r#author: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#scope: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#topic: Option<fhirbolt_model::r4::resources::ContractTopic> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#sub_type: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> =
                    None;
                let mut r#content_definition: Option<
                    fhirbolt_model::r4::resources::ContractContentDefinition,
                > = None;
                let mut r#term: Option<Vec<fhirbolt_model::r4::resources::ContractTerm>> = None;
                let mut r#supporting_info: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#relevant_history: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#signer: Option<Vec<fhirbolt_model::r4::resources::ContractSigner>> = None;
                let mut r#friendly: Option<Vec<fhirbolt_model::r4::resources::ContractFriendly>> =
                    None;
                let mut r#legal: Option<Vec<fhirbolt_model::r4::resources::ContractLegal>> = None;
                let mut r#rule: Option<Vec<fhirbolt_model::r4::resources::ContractRule>> = None;
                let mut r#legally_binding: Option<
                    fhirbolt_model::r4::resources::ContractLegallyBinding,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Contract" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Contract",
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
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r4::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::Url => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                r#url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
                                )?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Version => {
                            if self.0.from_json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#version.is_some() {
                                    return Err(serde::de::Error::duplicate_field("version"));
                                }
                                r#version = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::VersionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#version.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_version"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("version");
                            }
                        }
                        Field::Status => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
                                )?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::LegalState => {
                            if r#legal_state.is_some() {
                                return Err(serde::de::Error::duplicate_field("legalState"));
                            }
                            r#legal_state = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::InstantiatesCanonical => {
                            if r#instantiates_canonical.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "instantiatesCanonical",
                                ));
                            }
                            r#instantiates_canonical = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::InstantiatesUri => {
                            if self.0.from_json {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#instantiates_uri.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "instantiatesUri",
                                    ));
                                }
                                r#instantiates_uri = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
                                )?);
                            }
                        }
                        Field::InstantiatesUriPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#instantiates_uri.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_instantiatesUri",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instantiatesUri");
                            }
                        }
                        Field::ContentDerivative => {
                            if r#content_derivative.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentDerivative"));
                            }
                            r#content_derivative = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Issued => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#issued.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                r#issued = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::IssuedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issued"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issued");
                            }
                        }
                        Field::Applies => {
                            if r#applies.is_some() {
                                return Err(serde::de::Error::duplicate_field("applies"));
                            }
                            r#applies = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                            )?);
                        }
                        Field::ExpirationType => {
                            if r#expiration_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("expirationType"));
                            }
                            r#expiration_type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Subject => {
                            if self.0.from_json {
                                if r#subject.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subject"));
                                }
                                r#subject = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#subject.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Authority => {
                            if self.0.from_json {
                                if r#authority.is_some() {
                                    return Err(serde::de::Error::duplicate_field("authority"));
                                }
                                r#authority = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#authority.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Domain => {
                            if self.0.from_json {
                                if r#domain.is_some() {
                                    return Err(serde::de::Error::duplicate_field("domain"));
                                }
                                r#domain = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#domain.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Site => {
                            if self.0.from_json {
                                if r#site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("site"));
                                }
                                r#site = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#site.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
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
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
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
                        Field::Title => {
                            if self.0.from_json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                r#title = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::TitlePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::Subtitle => {
                            if self.0.from_json {
                                let some = r#subtitle.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtitle"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#subtitle.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subtitle"));
                                }
                                r#subtitle = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::SubtitlePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#subtitle.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_subtitle"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("subtitle");
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
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
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
                        Field::Author => {
                            if r#author.is_some() {
                                return Err(serde::de::Error::duplicate_field("author"));
                            }
                            r#author = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Scope => {
                            if r#scope.is_some() {
                                return Err(serde::de::Error::duplicate_field("scope"));
                            }
                            r#scope = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::TopicCodeableConcept => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "topicCodeableConcept",
                                ));
                            }
                            r#topic = Some (fhirbolt_model :: r4 :: resources :: ContractTopic :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::TopicReference => {
                            if r#topic.is_some() {
                                return Err(serde::de::Error::duplicate_field("topicReference"));
                            }
                            r#topic =
                                Some(fhirbolt_model::r4::resources::ContractTopic::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ));
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::SubType => {
                            if self.0.from_json {
                                if r#sub_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subType"));
                                }
                                r#sub_type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#sub_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::ContentDefinition => {
                            if r#content_definition.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentDefinition"));
                            }
                            r#content_definition = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractContentDefinition > ()) ?) ;
                        }
                        Field::Term => {
                            if self.0.from_json {
                                if r#term.is_some() {
                                    return Err(serde::de::Error::duplicate_field("term"));
                                }
                                r#term = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: ContractTerm >> ()) ?) ;
                            } else {
                                let vec = r#term.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractTerm > ()) ?) ;
                            }
                        }
                        Field::SupportingInfo => {
                            if self.0.from_json {
                                if r#supporting_info.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInfo",
                                    ));
                                }
                                r#supporting_info = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#supporting_info.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::RelevantHistory => {
                            if self.0.from_json {
                                if r#relevant_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relevantHistory",
                                    ));
                                }
                                r#relevant_history = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#relevant_history.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Signer => {
                            if self.0.from_json {
                                if r#signer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("signer"));
                                }
                                r#signer = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: ContractSigner >> ()) ?) ;
                            } else {
                                let vec = r#signer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractSigner > ()) ?) ;
                            }
                        }
                        Field::Friendly => {
                            if self.0.from_json {
                                if r#friendly.is_some() {
                                    return Err(serde::de::Error::duplicate_field("friendly"));
                                }
                                r#friendly =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                fhirbolt_model::r4::resources::ContractFriendly,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#friendly.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractFriendly > ()) ?) ;
                            }
                        }
                        Field::Legal => {
                            if self.0.from_json {
                                if r#legal.is_some() {
                                    return Err(serde::de::Error::duplicate_field("legal"));
                                }
                                r#legal = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: ContractLegal >> ()) ?) ;
                            } else {
                                let vec = r#legal.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractLegal > ()) ?) ;
                            }
                        }
                        Field::Rule => {
                            if self.0.from_json {
                                if r#rule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("rule"));
                                }
                                r#rule = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: ContractRule >> ()) ?) ;
                            } else {
                                let vec = r#rule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: ContractRule > ()) ?) ;
                            }
                        }
                        Field::LegallyBindingAttachment => {
                            if r#legally_binding.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legallyBindingAttachment",
                                ));
                            }
                            r#legally_binding = Some (fhirbolt_model :: r4 :: resources :: ContractLegallyBinding :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::LegallyBindingReference => {
                            if r#legally_binding.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legallyBindingReference",
                                ));
                            }
                            r#legally_binding = Some(
                                fhirbolt_model::r4::resources::ContractLegallyBinding::Reference(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                ),
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
                Ok(fhirbolt_model::r4::resources::Contract {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#url,
                    r#version,
                    r#status,
                    r#legal_state,
                    r#instantiates_canonical,
                    r#instantiates_uri,
                    r#content_derivative,
                    r#issued,
                    r#applies,
                    r#expiration_type,
                    r#subject: r#subject.unwrap_or(vec![]),
                    r#authority: r#authority.unwrap_or(vec![]),
                    r#domain: r#domain.unwrap_or(vec![]),
                    r#site: r#site.unwrap_or(vec![]),
                    r#name,
                    r#title,
                    r#subtitle,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#author,
                    r#scope,
                    r#topic,
                    r#type,
                    r#sub_type: r#sub_type.unwrap_or(vec![]),
                    r#content_definition,
                    r#term: r#term.unwrap_or(vec![]),
                    r#supporting_info: r#supporting_info.unwrap_or(vec![]),
                    r#relevant_history: r#relevant_history.unwrap_or(vec![]),
                    r#signer: r#signer.unwrap_or(vec![]),
                    r#friendly: r#friendly.unwrap_or(vec![]),
                    r#legal: r#legal.unwrap_or(vec![]),
                    r#rule: r#rule.unwrap_or(vec![]),
                    r#legally_binding,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::Contract>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::Contract>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::Contract>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::Contract>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::Contract>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::Contract>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::Contract>;
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
                        .transmute::<fhirbolt_model::r4::resources::Contract>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::Contract>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::Contract>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::Contract>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::Contract>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::Contract>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
