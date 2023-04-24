// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionContact,
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
                "MedicinalProductDefinition.contact", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.value.r#contact.id.as_deref() == Some("$invalid") {
            return missing_field_error("contact");
        }
        self.with_context(&self.value.r#contact, |ctx| {
            state.serialize_entry("contact", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionContact,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionContact;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionContact,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionContact;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionContact")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact, V::Error>
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
                    #[serde(rename = "contact")]
                    Contact,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "contact"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#contact: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Contact => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(
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
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionContact {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#contact: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#contact.unwrap_or(Default::default())
                        } else {
                            r#contact.ok_or(serde::de::Error::missing_field("contact"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionContact > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart,
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
                "MedicinalProductDefinition.name.part", field
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
        if self.output_json {
            if self.value.r#part.id.as_deref() == Some("$invalid") {
                return missing_field_error("part");
            }
            if let Some(some) = self.value.r#part.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("part", &some)?;
            }
            if self.value.r#part.id.is_some() || !self.value.r#part.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#part.id.as_ref(),
                    extension: &self.value.r#part.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_part", ctx)
                })?;
            }
        } else {
            if self.value.r#part.id.as_deref() == Some("$invalid") {
                return missing_field_error("part");
            }
            self.with_context(&self.value.r#part, |ctx| state.serialize_entry("part", ctx))?;
        }
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionNamePart")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart, V::Error>
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
                    #[serde(rename = "part")]
                    Part,
                    #[serde(rename = "_part")]
                    PartPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "part", "type"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#part: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Part => {
                            if self.0.from_json {
                                let some = r#part.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                r#part = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::PartPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#part.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_part"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("part");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#part: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#part.unwrap_or(Default::default())
                        } else {
                            r#part.ok_or(serde::de::Error::missing_field("part"))?
                        },
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNamePart > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage,
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
                "MedicinalProductDefinition.name.usage", field
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
        if self.value.r#country.id.as_deref() == Some("$invalid") {
            return missing_field_error("country");
        }
        self.with_context(&self.value.r#country, |ctx| {
            state.serialize_entry("country", ctx)
        })?;
        if let Some(some) = self.value.r#jurisdiction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("jurisdiction", ctx))?;
        }
        if self.value.r#language.id.as_deref() == Some("$invalid") {
            return missing_field_error("language");
        }
        self.with_context(&self.value.r#language, |ctx| {
            state.serialize_entry("language", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionNameUsage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage, V::Error>
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
                    #[serde(rename = "country")]
                    Country,
                    #[serde(rename = "jurisdiction")]
                    Jurisdiction,
                    #[serde(rename = "language")]
                    Language,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "country",
                            "jurisdiction",
                            "language",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#country: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#jurisdiction: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#language: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Country => {
                            if r#country.is_some() {
                                return Err(serde::de::Error::duplicate_field("country"));
                            }
                            r#country = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Jurisdiction => {
                            if r#jurisdiction.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            r#jurisdiction = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Language => {
                            if r#language.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            r#language = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#country: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#country.unwrap_or(Default::default())
                        } else {
                            r#country.ok_or(serde::de::Error::missing_field("country"))?
                        },
                        r#jurisdiction,
                        r#language: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#language.unwrap_or(Default::default())
                        } else {
                            r#language.ok_or(serde::de::Error::missing_field("language"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNameUsage > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionName,
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
                "MedicinalProductDefinition.name", field
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
        if self.output_json {
            if self.value.r#product_name.id.as_deref() == Some("$invalid") {
                return missing_field_error("productName");
            }
            if let Some(some) = self.value.r#product_name.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("productName", &some)?;
            }
            if self.value.r#product_name.id.is_some()
                || !self.value.r#product_name.extension.is_empty()
            {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#product_name.id.as_ref(),
                    extension: &self.value.r#product_name.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_productName", ctx)
                })?;
            }
        } else {
            if self.value.r#product_name.id.as_deref() == Some("$invalid") {
                return missing_field_error("productName");
            }
            self.with_context(&self.value.r#product_name, |ctx| {
                state.serialize_entry("productName", ctx)
            })?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if !self.value.r#part.is_empty() {
            self.with_context(&self.value.r#part, |ctx| state.serialize_entry("part", ctx))?;
        }
        if !self.value.r#usage.is_empty() {
            self.with_context(&self.value.r#usage, |ctx| {
                state.serialize_entry("usage", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionName,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionName;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionName,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionName")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinitionName, V::Error>
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
                    #[serde(rename = "productName")]
                    ProductName,
                    #[serde(rename = "_productName")]
                    ProductNamePrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "part")]
                    Part,
                    #[serde(rename = "usage")]
                    Usage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "productName",
                            "type",
                            "part",
                            "usage",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#product_name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#part: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNamePart>,
                > = None;
                let mut r#usage: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionNameUsage>,
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
                        Field::ProductName => {
                            if self.0.from_json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#product_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productName"));
                                }
                                r#product_name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::ProductNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#product_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_productName"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("productName");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Part => {
                            if self.0.from_json {
                                if r#part.is_some() {
                                    return Err(serde::de::Error::duplicate_field("part"));
                                }
                                r#part = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNamePart >> ()) ?) ;
                            } else {
                                let vec = r#part.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNamePart > ()) ?) ;
                            }
                        }
                        Field::Usage => {
                            if self.0.from_json {
                                if r#usage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usage"));
                                }
                                r#usage = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNameUsage >> ()) ?) ;
                            } else {
                                let vec = r#usage.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionNameUsage > ()) ?) ;
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
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionName {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#product_name: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#product_name.unwrap_or(Default::default())
                        } else {
                            r#product_name.ok_or(serde::de::Error::missing_field("productName"))?
                        },
                        r#type,
                        r#part: r#part.unwrap_or(vec![]),
                        r#usage: r#usage.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>(
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
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference,
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
                "MedicinalProductDefinition.crossReference", field
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
        if self.value.r#product.id.as_deref() == Some("$invalid") {
            return missing_field_error("product");
        }
        self.with_context(&self.value.r#product, |ctx| {
            state.serialize_entry("product", ctx)
        })?;
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionCrossReference")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference,
                V::Error,
            >
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
                    #[serde(rename = "product")]
                    Product,
                    #[serde(rename = "type")]
                    Type,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "product", "type"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#product: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                        Field::Product => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            r#product = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#product: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#product.unwrap_or(Default::default())
                        } else {
                            r#product.ok_or(serde::de::Error::missing_field("product"))?
                        },
                        r#type,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCrossReference > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation,
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
                "MedicinalProductDefinition.operation", field
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#effective_date.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("effectiveDate", ctx))?;
        }
        if !self.value.r#organization.is_empty() {
            self.with_context(&self.value.r#organization, |ctx| {
                state.serialize_entry("organization", ctx)
            })?;
        }
        if let Some(some) = self.value.r#confidentiality_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("confidentialityIndicator", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionOperation")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation, V::Error>
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
                    #[serde(rename = "effectiveDate")]
                    EffectiveDate,
                    #[serde(rename = "organization")]
                    Organization,
                    #[serde(rename = "confidentialityIndicator")]
                    ConfidentialityIndicator,
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
                            "effectiveDate",
                            "organization",
                            "confidentialityIndicator",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
                let mut r#effective_date: Option<Box<fhirbolt_model::r5::types::Period>> = None;
                let mut r#organization: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#confidentiality_indicator: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::EffectiveDate => {
                            if r#effective_date.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveDate"));
                            }
                            r#effective_date = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Period>>(),
                            )?);
                        }
                        Field::Organization => {
                            if self.0.from_json {
                                if r#organization.is_some() {
                                    return Err(serde::de::Error::duplicate_field("organization"));
                                }
                                r#organization = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#organization.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::ConfidentialityIndicator => {
                            if r#confidentiality_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "confidentialityIndicator",
                                ));
                            }
                            r#confidentiality_indicator = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#effective_date,
                        r#organization: r#organization.unwrap_or(vec![]),
                        r#confidentiality_indicator,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionOperation > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic,
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
                "MedicinalProductDefinition.characteristic", field
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if let Some(some) = self.value.r#value.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueMarkdown" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueMarkdown" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueMarkdown" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueInteger" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueInteger" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueInteger" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueDate" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueDate" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueDate" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueBoolean" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueBoolean" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueBoolean" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Attachment (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueAttachment" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("value is invalid")) } }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
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
        fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinitionCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic,
                V::Error,
            >
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
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueMarkdown")]
                    ValueMarkdown,
                    #[serde(rename = "_valueMarkdown")]
                    ValueMarkdownPrimitiveElement,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueInteger")]
                    ValueInteger,
                    #[serde(rename = "_valueInteger")]
                    ValueIntegerPrimitiveElement,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
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
                            "valueCodeableConcept",
                            "valueMarkdown",
                            "valueQuantity",
                            "valueInteger",
                            "valueDate",
                            "valueBoolean",
                            "valueAttachment",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristicValue,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueInteger => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueInteger")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueInteger"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Integer > > ()) ?)) ;
                            }
                        }
                        Field::ValueIntegerPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Integer (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueInteger")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueInteger");
                            }
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristicValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#value,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristic > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::MedicinalProductDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicinalProductDefinition,
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
                "MedicinalProductDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductDefinition")?;
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
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#domain.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("domain", ctx))?;
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
        if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status_date.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("statusDate", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_statusDate", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#status_date.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("statusDate", ctx))?;
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
        if let Some(some) = self.value.r#combined_pharmaceutical_dose_form.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("combinedPharmaceuticalDoseForm", ctx)
            })?;
        }
        if !self.value.r#route.is_empty() {
            self.with_context(&self.value.r#route, |ctx| {
                state.serialize_entry("route", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#indication.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("indication", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_indication", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#indication.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("indication", ctx))?;
            }
        }
        if let Some(some) = self.value.r#legal_status_of_supply.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("legalStatusOfSupply", ctx)
            })?;
        }
        if let Some(some) = self.value.r#additional_monitoring_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("additionalMonitoringIndicator", ctx)
            })?;
        }
        if !self.value.r#special_measures.is_empty() {
            self.with_context(&self.value.r#special_measures, |ctx| {
                state.serialize_entry("specialMeasures", ctx)
            })?;
        }
        if let Some(some) = self.value.r#pediatric_use_indicator.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("pediatricUseIndicator", ctx)
            })?;
        }
        if !self.value.r#classification.is_empty() {
            self.with_context(&self.value.r#classification, |ctx| {
                state.serialize_entry("classification", ctx)
            })?;
        }
        if !self.value.r#marketing_status.is_empty() {
            self.with_context(&self.value.r#marketing_status, |ctx| {
                state.serialize_entry("marketingStatus", ctx)
            })?;
        }
        if !self.value.r#packaged_medicinal_product.is_empty() {
            self.with_context(&self.value.r#packaged_medicinal_product, |ctx| {
                state.serialize_entry("packagedMedicinalProduct", ctx)
            })?;
        }
        if !self.value.r#comprised_of.is_empty() {
            self.with_context(&self.value.r#comprised_of, |ctx| {
                state.serialize_entry("comprisedOf", ctx)
            })?;
        }
        if !self.value.r#ingredient.is_empty() {
            self.with_context(&self.value.r#ingredient, |ctx| {
                state.serialize_entry("ingredient", ctx)
            })?;
        }
        if !self.value.r#impurity.is_empty() {
            self.with_context(&self.value.r#impurity, |ctx| {
                state.serialize_entry("impurity", ctx)
            })?;
        }
        if !self.value.r#attached_document.is_empty() {
            self.with_context(&self.value.r#attached_document, |ctx| {
                state.serialize_entry("attachedDocument", ctx)
            })?;
        }
        if !self.value.r#master_file.is_empty() {
            self.with_context(&self.value.r#master_file, |ctx| {
                state.serialize_entry("masterFile", ctx)
            })?;
        }
        if !self.value.r#contact.is_empty() {
            self.with_context(&self.value.r#contact, |ctx| {
                state.serialize_entry("contact", ctx)
            })?;
        }
        if !self.value.r#clinical_trial.is_empty() {
            self.with_context(&self.value.r#clinical_trial, |ctx| {
                state.serialize_entry("clinicalTrial", ctx)
            })?;
        }
        if !self.value.r#code.is_empty() {
            self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        }
        if !self.value.r#name.is_empty() {
            self.with_context(&self.value.r#name, |ctx| state.serialize_entry("name", ctx))?;
        }
        if !self.value.r#cross_reference.is_empty() {
            self.with_context(&self.value.r#cross_reference, |ctx| {
                state.serialize_entry("crossReference", ctx)
            })?;
        }
        if !self.value.r#operation.is_empty() {
            self.with_context(&self.value.r#operation, |ctx| {
                state.serialize_entry("operation", ctx)
            })?;
        }
        if !self.value.r#characteristic.is_empty() {
            self.with_context(&self.value.r#characteristic, |ctx| {
                state.serialize_entry("characteristic", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
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
        &Vec<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
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
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicinalProductDefinition,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinition;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicinalProductDefinition,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicinalProductDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicinalProductDefinition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicinalProductDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicinalProductDefinition, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "domain")]
                    Domain,
                    #[serde(rename = "version")]
                    Version,
                    #[serde(rename = "_version")]
                    VersionPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "statusDate")]
                    StatusDate,
                    #[serde(rename = "_statusDate")]
                    StatusDatePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "combinedPharmaceuticalDoseForm")]
                    CombinedPharmaceuticalDoseForm,
                    #[serde(rename = "route")]
                    Route,
                    #[serde(rename = "indication")]
                    Indication,
                    #[serde(rename = "_indication")]
                    IndicationPrimitiveElement,
                    #[serde(rename = "legalStatusOfSupply")]
                    LegalStatusOfSupply,
                    #[serde(rename = "additionalMonitoringIndicator")]
                    AdditionalMonitoringIndicator,
                    #[serde(rename = "specialMeasures")]
                    SpecialMeasures,
                    #[serde(rename = "pediatricUseIndicator")]
                    PediatricUseIndicator,
                    #[serde(rename = "classification")]
                    Classification,
                    #[serde(rename = "marketingStatus")]
                    MarketingStatus,
                    #[serde(rename = "packagedMedicinalProduct")]
                    PackagedMedicinalProduct,
                    #[serde(rename = "comprisedOf")]
                    ComprisedOf,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "impurity")]
                    Impurity,
                    #[serde(rename = "attachedDocument")]
                    AttachedDocument,
                    #[serde(rename = "masterFile")]
                    MasterFile,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "clinicalTrial")]
                    ClinicalTrial,
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "crossReference")]
                    CrossReference,
                    #[serde(rename = "operation")]
                    Operation,
                    #[serde(rename = "characteristic")]
                    Characteristic,
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
                            "type",
                            "domain",
                            "version",
                            "status",
                            "statusDate",
                            "description",
                            "combinedPharmaceuticalDoseForm",
                            "route",
                            "indication",
                            "legalStatusOfSupply",
                            "additionalMonitoringIndicator",
                            "specialMeasures",
                            "pediatricUseIndicator",
                            "classification",
                            "marketingStatus",
                            "packagedMedicinalProduct",
                            "comprisedOf",
                            "ingredient",
                            "impurity",
                            "attachedDocument",
                            "masterFile",
                            "contact",
                            "clinicalTrial",
                            "code",
                            "name",
                            "crossReference",
                            "operation",
                            "characteristic",
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
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#domain: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#version: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#status: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#status_date: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#combined_pharmaceutical_dose_form: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#route: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#indication: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#legal_status_of_supply: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#additional_monitoring_indicator: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#special_measures: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#pediatric_use_indicator: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#classification: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#marketing_status: Option<
                    Vec<fhirbolt_model::r5::types::MarketingStatus>,
                > = None;
                let mut r#packaged_medicinal_product: Option<
                    Vec<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#comprised_of: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#ingredient: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#impurity: Option<Vec<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#attached_document: Option<Vec<fhirbolt_model::r5::types::Reference>> =
                    None;
                let mut r#master_file: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#contact: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionContact>,
                > = None;
                let mut r#clinical_trial: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#code: Option<Vec<fhirbolt_model::r5::types::Coding>> = None;
                let mut r#name: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionName>,
                > = None;
                let mut r#cross_reference: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCrossReference>,
                > = None;
                let mut r#operation: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionOperation>,
                > = None;
                let mut r#characteristic: Option<
                    Vec<fhirbolt_model::r5::resources::MedicinalProductDefinitionCharacteristic>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductDefinition",
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Domain => {
                            if r#domain.is_some() {
                                return Err(serde::de::Error::duplicate_field("domain"));
                            }
                            r#domain = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
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
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
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
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::StatusDate => {
                            if self.0.from_json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status_date.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusDate"));
                                }
                                r#status_date = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::StatusDatePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status_date.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_statusDate"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("statusDate");
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
                        Field::CombinedPharmaceuticalDoseForm => {
                            if r#combined_pharmaceutical_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "combinedPharmaceuticalDoseForm",
                                ));
                            }
                            r#combined_pharmaceutical_dose_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Route => {
                            if self.0.from_json {
                                if r#route.is_some() {
                                    return Err(serde::de::Error::duplicate_field("route"));
                                }
                                r#route = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#route.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Indication => {
                            if self.0.from_json {
                                let some = r#indication.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#indication.is_some() {
                                    return Err(serde::de::Error::duplicate_field("indication"));
                                }
                                r#indication = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::IndicationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#indication.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_indication"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("indication");
                            }
                        }
                        Field::LegalStatusOfSupply => {
                            if r#legal_status_of_supply.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "legalStatusOfSupply",
                                ));
                            }
                            r#legal_status_of_supply = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::AdditionalMonitoringIndicator => {
                            if r#additional_monitoring_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additionalMonitoringIndicator",
                                ));
                            }
                            r#additional_monitoring_indicator = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::SpecialMeasures => {
                            if self.0.from_json {
                                if r#special_measures.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specialMeasures",
                                    ));
                                }
                                r#special_measures = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#special_measures.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::PediatricUseIndicator => {
                            if r#pediatric_use_indicator.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "pediatricUseIndicator",
                                ));
                            }
                            r#pediatric_use_indicator = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Classification => {
                            if self.0.from_json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::MarketingStatus => {
                            if self.0.from_json {
                                if r#marketing_status.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "marketingStatus",
                                    ));
                                }
                                r#marketing_status = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: MarketingStatus >> ()) ?) ;
                            } else {
                                let vec = r#marketing_status.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: MarketingStatus > ()) ?) ;
                            }
                        }
                        Field::PackagedMedicinalProduct => {
                            if self.0.from_json {
                                if r#packaged_medicinal_product.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "packagedMedicinalProduct",
                                    ));
                                }
                                r#packaged_medicinal_product = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec =
                                    r#packaged_medicinal_product.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::ComprisedOf => {
                            if self.0.from_json {
                                if r#comprised_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("comprisedOf"));
                                }
                                r#comprised_of = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#comprised_of.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Ingredient => {
                            if self.0.from_json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Impurity => {
                            if self.0.from_json {
                                if r#impurity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("impurity"));
                                }
                                r#impurity = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableReference >> ()) ?) ;
                            } else {
                                let vec = r#impurity.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableReference > ()) ?) ;
                            }
                        }
                        Field::AttachedDocument => {
                            if self.0.from_json {
                                if r#attached_document.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "attachedDocument",
                                    ));
                                }
                                r#attached_document = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#attached_document.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::MasterFile => {
                            if self.0.from_json {
                                if r#master_file.is_some() {
                                    return Err(serde::de::Error::duplicate_field("masterFile"));
                                }
                                r#master_file = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#master_file.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Contact => {
                            if self.0.from_json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionContact >> ()) ?) ;
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionContact > ()) ?) ;
                            }
                        }
                        Field::ClinicalTrial => {
                            if self.0.from_json {
                                if r#clinical_trial.is_some() {
                                    return Err(serde::de::Error::duplicate_field("clinicalTrial"));
                                }
                                r#clinical_trial = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#clinical_trial.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<fhirbolt_model::r5::types::Coding>>(),
                                )?);
                            } else {
                                let vec = r#code.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Coding>(),
                                )?);
                            }
                        }
                        Field::Name => {
                            if self.0.from_json {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionName >> ()) ?) ;
                            } else {
                                let vec = r#name.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionName > ()) ?) ;
                            }
                        }
                        Field::CrossReference => {
                            if self.0.from_json {
                                if r#cross_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "crossReference",
                                    ));
                                }
                                r#cross_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCrossReference >> ()) ?) ;
                            } else {
                                let vec = r#cross_reference.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCrossReference > ()) ?) ;
                            }
                        }
                        Field::Operation => {
                            if self.0.from_json {
                                if r#operation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("operation"));
                                }
                                r#operation = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionOperation >> ()) ?) ;
                            } else {
                                let vec = r#operation.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionOperation > ()) ?) ;
                            }
                        }
                        Field::Characteristic => {
                            if self.0.from_json {
                                if r#characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristic",
                                    ));
                                }
                                r#characteristic = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristic >> ()) ?) ;
                            } else {
                                let vec = r#characteristic.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinitionCharacteristic > ()) ?) ;
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
                Ok(fhirbolt_model::r5::resources::MedicinalProductDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#type,
                    r#domain,
                    r#version,
                    r#status,
                    r#status_date,
                    r#description,
                    r#combined_pharmaceutical_dose_form,
                    r#route: r#route.unwrap_or(vec![]),
                    r#indication,
                    r#legal_status_of_supply,
                    r#additional_monitoring_indicator,
                    r#special_measures: r#special_measures.unwrap_or(vec![]),
                    r#pediatric_use_indicator,
                    r#classification: r#classification.unwrap_or(vec![]),
                    r#marketing_status: r#marketing_status.unwrap_or(vec![]),
                    r#packaged_medicinal_product: r#packaged_medicinal_product.unwrap_or(vec![]),
                    r#comprised_of: r#comprised_of.unwrap_or(vec![]),
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#impurity: r#impurity.unwrap_or(vec![]),
                    r#attached_document: r#attached_document.unwrap_or(vec![]),
                    r#master_file: r#master_file.unwrap_or(vec![]),
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#clinical_trial: r#clinical_trial.unwrap_or(vec![]),
                    r#code: r#code.unwrap_or(vec![]),
                    r#name: r#name.unwrap_or(vec![]),
                    r#cross_reference: r#cross_reference.unwrap_or(vec![]),
                    r#operation: r#operation.unwrap_or(vec![]),
                    r#characteristic: r#characteristic.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicinalProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicinalProductDefinition>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicinalProductDefinition>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
