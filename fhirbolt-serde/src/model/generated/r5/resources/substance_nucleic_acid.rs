// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunitLinkage;
impl serde::ser::Serialize for SerializationContext<&SubstanceNucleicAcidSubunitLinkage> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceNucleicAcid.subunit.linkage", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#connectivity.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("connectivity", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_connectivity", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#connectivity.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("connectivity", ctx)));
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("name", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_name", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("name", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#residue_site.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("residueSite", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_residueSite", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#residue_site.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("residueSite", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceNucleicAcidSubunitLinkage>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceNucleicAcidSubunitLinkage>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<SubstanceNucleicAcidSubunitLinkage>
{
    type Value = SubstanceNucleicAcidSubunitLinkage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceNucleicAcidSubunitLinkage>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceNucleicAcidSubunitLinkage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunitLinkage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunitLinkage, V::Error>
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
                    #[serde(rename = "connectivity")]
                    Connectivity,
                    #[serde(rename = "_connectivity")]
                    ConnectivityPrimitiveElement,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "residueSite")]
                    ResidueSite,
                    #[serde(rename = "_residueSite")]
                    ResidueSitePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "connectivity",
                            "identifier",
                            "name",
                            "residueSite",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#connectivity: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#residue_site: Option<fhirbolt_model::r5::types::String> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Connectivity => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#connectivity.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("connectivity"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#connectivity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("connectivity"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#connectivity =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ConnectivityPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#connectivity.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_connectivity"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("connectivity");
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::ResidueSite => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#residue_site.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("residueSite"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#residue_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("residueSite"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#residue_site =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResidueSitePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#residue_site.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_residueSite"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("residueSite");
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
                Ok(SubstanceNucleicAcidSubunitLinkage {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#connectivity,
                    r#identifier,
                    r#name,
                    r#residue_site,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubstanceNucleicAcidSubunitLinkage>>
{
    type Value = Box<SubstanceNucleicAcidSubunitLinkage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceNucleicAcidSubunitLinkage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubstanceNucleicAcidSubunitLinkage>>
{
    type Value = Vec<SubstanceNucleicAcidSubunitLinkage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceNucleicAcidSubunitLinkage>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceNucleicAcidSubunitLinkage>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceNucleicAcidSubunitLinkage> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunitSugar;
impl serde::ser::Serialize for SerializationContext<&SubstanceNucleicAcidSubunitSugar> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceNucleicAcid.subunit.sugar", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#identifier.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("identifier", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#name.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("name", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_name", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#name.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("name", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#residue_site.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("residueSite", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_residueSite", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#residue_site.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("residueSite", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceNucleicAcidSubunitSugar>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceNucleicAcidSubunitSugar>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<SubstanceNucleicAcidSubunitSugar>
{
    type Value = SubstanceNucleicAcidSubunitSugar;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceNucleicAcidSubunitSugar>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceNucleicAcidSubunitSugar;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunitSugar")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunitSugar, V::Error>
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
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "residueSite")]
                    ResidueSite,
                    #[serde(rename = "_residueSite")]
                    ResidueSitePrimitiveElement,
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
                            "name",
                            "residueSite",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Box<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#name: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#residue_site: Option<fhirbolt_model::r5::types::String> = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Name => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#name = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
                            }
                        }
                        Field::ResidueSite => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#residue_site.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("residueSite"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#residue_site.is_some() {
                                    return Err(serde::de::Error::duplicate_field("residueSite"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#residue_site =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ResidueSitePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#residue_site.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_residueSite"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("residueSite");
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
                Ok(SubstanceNucleicAcidSubunitSugar {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#name,
                    r#residue_site,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubstanceNucleicAcidSubunitSugar>>
{
    type Value = Box<SubstanceNucleicAcidSubunitSugar>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceNucleicAcidSubunitSugar>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubstanceNucleicAcidSubunitSugar>>
{
    type Value = Vec<SubstanceNucleicAcidSubunitSugar>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceNucleicAcidSubunitSugar>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceNucleicAcidSubunitSugar>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceNucleicAcidSubunitSugar> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunit;
impl serde::ser::Serialize for SerializationContext<&SubstanceNucleicAcidSubunit> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceNucleicAcid.subunit", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        if let Some(value) = self.value.r#id.as_ref() {
            tri!(state.serialize_entry("id", value));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#subunit.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("subunit", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_subunit", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#subunit.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("subunit", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#sequence.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("sequence", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_sequence", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#sequence.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sequence", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#length.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("length", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_length", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#length.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("length", ctx)));
        }
        if let Some(some) = self.value.r#sequence_attachment.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sequenceAttachment", ctx)));
        }
        if let Some(some) = self.value.r#five_prime.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("fivePrime", ctx)));
        }
        if let Some(some) = self.value.r#three_prime.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("threePrime", ctx)));
        }
        if !self.value.r#linkage.is_empty() {
            tri!(self.with_context(&self.value.r#linkage, |ctx| state
                .serialize_entry("linkage", ctx)));
        }
        if !self.value.r#sugar.is_empty() {
            tri!(self.with_context(&self.value.r#sugar, |ctx| state
                .serialize_entry("sugar", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceNucleicAcidSubunit>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceNucleicAcidSubunit>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<SubstanceNucleicAcidSubunit>
{
    type Value = SubstanceNucleicAcidSubunit;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceNucleicAcidSubunit>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceNucleicAcidSubunit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcidSubunit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SubstanceNucleicAcidSubunit, V::Error>
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
                    #[serde(rename = "subunit")]
                    Subunit,
                    #[serde(rename = "_subunit")]
                    SubunitPrimitiveElement,
                    #[serde(rename = "sequence")]
                    Sequence,
                    #[serde(rename = "_sequence")]
                    SequencePrimitiveElement,
                    #[serde(rename = "length")]
                    Length,
                    #[serde(rename = "_length")]
                    LengthPrimitiveElement,
                    #[serde(rename = "sequenceAttachment")]
                    SequenceAttachment,
                    #[serde(rename = "fivePrime")]
                    FivePrime,
                    #[serde(rename = "threePrime")]
                    ThreePrime,
                    #[serde(rename = "linkage")]
                    Linkage,
                    #[serde(rename = "sugar")]
                    Sugar,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "subunit",
                            "sequence",
                            "length",
                            "sequenceAttachment",
                            "fivePrime",
                            "threePrime",
                            "linkage",
                            "sugar",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#subunit: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#sequence: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#length: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#sequence_attachment: Option<Box<fhirbolt_model::r5::types::Attachment>> =
                    None;
                let mut r#five_prime: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#three_prime: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#linkage: Option<
                    Vec<fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunitLinkage>,
                > = None;
                let mut r#sugar: Option<
                    Vec<fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunitSugar>,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(tri!(map_access.next_value()));
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Subunit => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#subunit.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#subunit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#subunit = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SubunitPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#subunit.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_subunit"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("subunit");
                            }
                        }
                        Field::Sequence => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#sequence.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sequence"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#sequence = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SequencePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sequence.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sequence"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sequence");
                            }
                        }
                        Field::Length => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#length.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#length.is_some() {
                                    return Err(serde::de::Error::duplicate_field("length"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#length = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LengthPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#length.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_length"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("length");
                            }
                        }
                        Field::SequenceAttachment => {
                            if r#sequence_attachment.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "sequenceAttachment",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Attachment>,
                            > = self.0.transmute();
                            r#sequence_attachment =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::FivePrime => {
                            if r#five_prime.is_some() {
                                return Err(serde::de::Error::duplicate_field("fivePrime"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#five_prime = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ThreePrime => {
                            if r#three_prime.is_some() {
                                return Err(serde::de::Error::duplicate_field("threePrime"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#three_prime = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Linkage => {
                            if self.0.from == crate::context::Format::Json {
                                if r#linkage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("linkage"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: SubstanceNucleicAcidSubunitLinkage >> = self . 0 . transmute () ;
                                r#linkage = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#linkage.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: SubstanceNucleicAcidSubunitLinkage > = self . 0 . transmute () ;
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Sugar => {
                            if self.0.from == crate::context::Format::Json {
                                if r#sugar.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sugar"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: SubstanceNucleicAcidSubunitSugar >> = self . 0 . transmute () ;
                                r#sugar = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#sugar.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunitSugar,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(SubstanceNucleicAcidSubunit {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#subunit,
                    r#sequence,
                    r#length,
                    r#sequence_attachment,
                    r#five_prime,
                    r#three_prime,
                    r#linkage: r#linkage.unwrap_or(vec![]),
                    r#sugar: r#sugar.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubstanceNucleicAcidSubunit>>
{
    type Value = Box<SubstanceNucleicAcidSubunit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceNucleicAcidSubunit>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubstanceNucleicAcidSubunit>>
{
    type Value = Vec<SubstanceNucleicAcidSubunit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceNucleicAcidSubunit>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceNucleicAcidSubunit>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceNucleicAcidSubunit> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r5::resources::SubstanceNucleicAcid;
impl crate::Resource for SubstanceNucleicAcid {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&SubstanceNucleicAcid> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SubstanceNucleicAcid", field
            )))
        }
        let mut state = tri!(serializer.serialize_map(None));
        tri!(state.serialize_entry("resourceType", "SubstanceNucleicAcid"));
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("id", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self
                        .with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("id", ctx)));
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("meta", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("implicitRules", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_implicitRules", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("language", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_language", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("language", ctx)));
        }
        if let Some(some) = self.value.r#text.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("text", ctx)));
        }
        if !self.value.r#contained.is_empty() {
            tri!(self.with_context(&self.value.r#contained, |ctx| state
                .serialize_entry("contained", ctx)));
        }
        if !self.value.r#extension.is_empty() {
            tri!(self.with_context(&self.value.r#extension, |ctx| state
                .serialize_entry("extension", ctx)));
        }
        if !self.value.r#modifier_extension.is_empty() {
            tri!(
                self.with_context(&self.value.r#modifier_extension, |ctx| state
                    .serialize_entry("modifierExtension", ctx))
            );
        }
        if let Some(some) = self.value.r#sequence_type.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("sequenceType", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#number_of_subunits.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("numberOfSubunits", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_numberOfSubunits", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#number_of_subunits.as_ref() {
            tri!(self.with_context(some, |ctx| state.serialize_entry("numberOfSubunits", ctx)));
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#area_of_hybridisation.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    tri!(state.serialize_entry("areaOfHybridisation", &some?));
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    tri!(self.with_context(&primitive_element, |ctx| state
                        .serialize_entry("_areaOfHybridisation", ctx)));
                }
            }
        } else if let Some(some) = self.value.r#area_of_hybridisation.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("areaOfHybridisation", ctx)));
        }
        if let Some(some) = self.value.r#oligo_nucleotide_type.as_ref() {
            tri!(self.with_context(some, |ctx| state
                .serialize_entry("oligoNucleotideType", ctx)));
        }
        if !self.value.r#subunit.is_empty() {
            tri!(self.with_context(&self.value.r#subunit, |ctx| state
                .serialize_entry("subunit", ctx)));
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SubstanceNucleicAcid>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SubstanceNucleicAcid>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq_serializer = tri!(serializer.serialize_seq(Some(self.value.len())));
        for value in self.value {
            tri!(self.with_context(value, |ctx| { seq_serializer.serialize_element(ctx) }))
        }
        seq_serializer.end()
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<SubstanceNucleicAcid> {
    type Value = SubstanceNucleicAcid;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SubstanceNucleicAcid> {
    type Value = SubstanceNucleicAcid;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SubstanceNucleicAcid>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SubstanceNucleicAcid;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SubstanceNucleicAcid")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SubstanceNucleicAcid, V::Error>
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
                    #[serde(rename = "_id")]
                    IdPrimitiveElement,
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
                    #[serde(rename = "sequenceType")]
                    SequenceType,
                    #[serde(rename = "numberOfSubunits")]
                    NumberOfSubunits,
                    #[serde(rename = "_numberOfSubunits")]
                    NumberOfSubunitsPrimitiveElement,
                    #[serde(rename = "areaOfHybridisation")]
                    AreaOfHybridisation,
                    #[serde(rename = "_areaOfHybridisation")]
                    AreaOfHybridisationPrimitiveElement,
                    #[serde(rename = "oligoNucleotideType")]
                    OligoNucleotideType,
                    #[serde(rename = "subunit")]
                    Subunit,
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
                            "sequenceType",
                            "numberOfSubunits",
                            "areaOfHybridisation",
                            "oligoNucleotideType",
                            "subunit",
                        ],
                    ))
                }
                let mut r#id: Option<fhirbolt_model::r5::types::Id> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#sequence_type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#number_of_subunits: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#area_of_hybridisation: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#oligo_nucleotide_type: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#subunit: Option<
                    Vec<fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunit>,
                > = None;
                while let Some(map_access_key) = tri!(map_access.next_key()) {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = tri!(map_access.next_value());
                            if value != "SubstanceNucleicAcid" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SubstanceNucleicAcid",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Id,
                                > = self.0.transmute();
                                r#id = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("id");
                            }
                        }
                        Field::Meta => {
                            if r#meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ImplicitRulesPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_implicitRules",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("implicitRules");
                            }
                        }
                        Field::Language => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
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
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::SequenceType => {
                            if r#sequence_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceType"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#sequence_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::NumberOfSubunits => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#number_of_subunits.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "numberOfSubunits",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#number_of_subunits.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "numberOfSubunits",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#number_of_subunits =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::NumberOfSubunitsPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#number_of_subunits.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_numberOfSubunits",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("numberOfSubunits");
                            }
                        }
                        Field::AreaOfHybridisation => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#area_of_hybridisation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "areaOfHybridisation",
                                    ));
                                }
                                some.value = Some(tri!(map_access.next_value()));
                            } else {
                                if r#area_of_hybridisation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "areaOfHybridisation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#area_of_hybridisation =
                                    Some(tri!(map_access.next_value_seed(&mut *_context)));
                            }
                        }
                        Field::AreaOfHybridisationPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some =
                                    r#area_of_hybridisation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_areaOfHybridisation",
                                    ));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    tri!(map_access.next_value_seed(&mut *_context));
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("areaOfHybridisation");
                            }
                        }
                        Field::OligoNucleotideType => {
                            if r#oligo_nucleotide_type.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "oligoNucleotideType",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#oligo_nucleotide_type =
                                Some(tri!(map_access.next_value_seed(&mut *_context)));
                        }
                        Field::Subunit => {
                            if self.0.from == crate::context::Format::Json {
                                if r#subunit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("subunit"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunit>,
                                > = self.0.transmute();
                                r#subunit = Some(tri!(map_access.next_value_seed(&mut *_context)));
                            } else {
                                let vec = r#subunit.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::resources::SubstanceNucleicAcidSubunit,
                                > = self.0.transmute();
                                vec.push(tri!(map_access.next_value_seed(&mut *_context)));
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
                Ok(SubstanceNucleicAcid {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#sequence_type,
                    r#number_of_subunits,
                    r#area_of_hybridisation,
                    r#oligo_nucleotide_type,
                    r#subunit: r#subunit.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SubstanceNucleicAcid>>
{
    type Value = Box<SubstanceNucleicAcid>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SubstanceNucleicAcid>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SubstanceNucleicAcid>>
{
    type Value = Vec<SubstanceNucleicAcid>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SubstanceNucleicAcid>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SubstanceNucleicAcid>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SubstanceNucleicAcid> =
                    self.0.transmute();
                while let Some(value) = tri!(seq.next_element_seed(&mut *_context)) {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
