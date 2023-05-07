// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerAdditive;
impl serde::ser::Serialize
    for SerializationContext<&SpecimenDefinitionTypeTestedContainerAdditive>
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
                "SpecimenDefinition.typeTested.container.additive", field
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
        {
            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerAdditiveAdditive as _Enum;
            match self.value.r#additive {
                _Enum::CodeableConcept(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("additiveCodeableConcept", ctx)
                    })?;
                }
                _Enum::Reference(ref value) => {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("additiveReference", ctx)
                    })?;
                }
                _Enum::Invalid => {
                    return Err(serde::ser::Error::custom("additive is a required field"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<SpecimenDefinitionTypeTestedContainerAdditive>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<SpecimenDefinitionTypeTestedContainerAdditive>>
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
    for &mut DeserializationContext<SpecimenDefinitionTypeTestedContainerAdditive>
{
    type Value = SpecimenDefinitionTypeTestedContainerAdditive;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<SpecimenDefinitionTypeTestedContainerAdditive>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenDefinitionTypeTestedContainerAdditive;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainerAdditive")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainerAdditive, V::Error>
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
                    #[serde(rename = "additiveCodeableConcept")]
                    AdditiveCodeableConcept,
                    #[serde(rename = "additiveReference")]
                    AdditiveReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "additiveCodeableConcept",
                            "additiveReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#additive : Option < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainerAdditiveAdditive > = None ;
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AdditiveCodeableConcept => {
                            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerAdditiveAdditive as _Enum;
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "additiveCodeableConcept",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#additive = Some(_Enum::CodeableConcept(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::AdditiveReference => {
                            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerAdditiveAdditive as _Enum;
                            if r#additive.is_some() {
                                return Err(serde::de::Error::duplicate_field("additiveReference"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Reference>,
                            > = self.0.transmute();
                            r#additive = Some(_Enum::Reference(
                                map_access.next_value_seed(&mut *_context)?,
                            ));
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(SpecimenDefinitionTypeTestedContainerAdditive {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#additive: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#additive.unwrap_or(Default::default())
                    } else {
                        r#additive.ok_or(serde::de::Error::missing_field("additive[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SpecimenDefinitionTypeTestedContainerAdditive>>
{
    type Value = Box<SpecimenDefinitionTypeTestedContainerAdditive>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenDefinitionTypeTestedContainerAdditive>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedContainerAdditive>>
{
    type Value = Vec<SpecimenDefinitionTypeTestedContainerAdditive>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedContainerAdditive>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenDefinitionTypeTestedContainerAdditive>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    SpecimenDefinitionTypeTestedContainerAdditive,
                > = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainer;
impl serde::ser::Serialize for SerializationContext<&SpecimenDefinitionTypeTestedContainer> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SpecimenDefinition.typeTested.container", field
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
        if let Some(some) = self.value.r#material.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("material", ctx))?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if let Some(some) = self.value.r#cap.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("cap", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("description", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#description.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
        }
        if let Some(some) = self.value.r#capacity.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("capacity", ctx))?;
        }
        {
            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerMinimumVolume as _Enum;
            if let Some(some) = self.value.r#minimum_volume.as_ref() {
                match some {
                    _Enum::Quantity(ref value) => {
                        self.with_context(value, |ctx| {
                            state.serialize_entry("minimumVolumeQuantity", ctx)
                        })?;
                    }
                    _Enum::String(ref value) => {
                        if self.output_json {
                            if let Some(some) = value.value.as_ref().map(Ok) {
                                state.serialize_entry("minimumVolumeString", &some?)?;
                            }
                            if value.id.is_some() || !value.extension.is_empty() {
                                use super::super::serde_helpers::PrimitiveElement;
                                let primitive_element = PrimitiveElement {
                                    id: value.id.as_ref(),
                                    extension: &value.extension,
                                };
                                self.with_context(&primitive_element, |ctx| {
                                    state.serialize_entry("_minimumVolumeString", ctx)
                                })?;
                            }
                        } else {
                            self.with_context(value, |ctx| {
                                state.serialize_entry("minimumVolumeString", ctx)
                            })?;
                        }
                    }
                    _Enum::Invalid => {
                        return Err(serde::ser::Error::custom("minimum_volume is invalid"))
                    }
                }
            }
        }
        if !self.value.r#additive.is_empty() {
            self.with_context(&self.value.r#additive, |ctx| {
                state.serialize_entry("additive", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#preparation.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("preparation", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_preparation", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#preparation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("preparation", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenDefinitionTypeTestedContainer>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenDefinitionTypeTestedContainer>> {
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
    for &mut DeserializationContext<SpecimenDefinitionTypeTestedContainer>
{
    type Value = SpecimenDefinitionTypeTestedContainer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenDefinitionTypeTestedContainer>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenDefinitionTypeTestedContainer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedContainer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedContainer, V::Error>
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
                    #[serde(rename = "material")]
                    Material,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "cap")]
                    Cap,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "capacity")]
                    Capacity,
                    #[serde(rename = "minimumVolumeQuantity")]
                    MinimumVolumeQuantity,
                    #[serde(rename = "minimumVolumeString")]
                    MinimumVolumeString,
                    #[serde(rename = "_minimumVolumeString")]
                    MinimumVolumeStringPrimitiveElement,
                    #[serde(rename = "additive")]
                    Additive,
                    #[serde(rename = "preparation")]
                    Preparation,
                    #[serde(rename = "_preparation")]
                    PreparationPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "material",
                            "type",
                            "cap",
                            "description",
                            "capacity",
                            "minimumVolumeQuantity",
                            "minimumVolumeString",
                            "additive",
                            "preparation",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#material: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#cap: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#description: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#capacity: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#minimum_volume : Option < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainerMinimumVolume > = None ;
                let mut r#additive : Option < Vec < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainerAdditive > > = None ;
                let mut r#preparation: Option<fhirbolt_model::r4::types::String> = None;
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Material => {
                            if r#material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#material = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Cap => {
                            if r#cap.is_some() {
                                return Err(serde::de::Error::duplicate_field("cap"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#cap = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#description = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Capacity => {
                            if r#capacity.is_some() {
                                return Err(serde::de::Error::duplicate_field("capacity"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#capacity = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::MinimumVolumeQuantity => {
                            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerMinimumVolume as _Enum;
                            if r#minimum_volume.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "minimumVolumeQuantity",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Quantity>,
                            > = self.0.transmute();
                            r#minimum_volume =
                                Some(_Enum::Quantity(map_access.next_value_seed(&mut *_context)?));
                        }
                        Field::MinimumVolumeString => {
                            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerMinimumVolume as _Enum;
                            if self.0.from_json {
                                let r#enum = r#minimum_volume
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "minimumVolumeString",
                                        ));
                                    }
                                    variant.value = Some(map_access.next_value()?)
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minimumVolume[x]",
                                    ));
                                }
                            } else {
                                if r#minimum_volume.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "minimumVolumeString",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4::types::String>,
                                > = self.0.transmute();
                                r#minimum_volume = Some(_Enum::String(
                                    map_access.next_value_seed(&mut *_context)?,
                                ));
                            }
                        }
                        Field::MinimumVolumeStringPrimitiveElement => {
                            use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainerMinimumVolume as _Enum;
                            if self.0.from_json {
                                let r#enum = r#minimum_volume
                                    .get_or_insert(_Enum::String(Default::default()));
                                if let _Enum::String(variant) = r#enum {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_minimumVolumeString",
                                        ));
                                    }
                                    use super::super::serde_helpers::PrimitiveElementOwned;
                                    let _context: &mut DeserializationContext<
                                        PrimitiveElementOwned,
                                    > = self.0.transmute();
                                    let PrimitiveElementOwned { id, extension } =
                                        map_access.next_value_seed(&mut *_context)?;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_minimumVolume[x]",
                                    ));
                                }
                            } else {
                                return unknown_field_error("minimumVolumeString");
                            }
                        }
                        Field::Additive => {
                            if self.0.from_json {
                                if r#additive.is_some() {
                                    return Err(serde::de::Error::duplicate_field("additive"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainerAdditive >> = self . 0 . transmute () ;
                                r#additive = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#additive.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainerAdditive > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Preparation => {
                            if self.0.from_json {
                                let some = r#preparation.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preparation"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#preparation.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preparation"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#preparation = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreparationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#preparation.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preparation"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preparation");
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
                Ok(SpecimenDefinitionTypeTestedContainer {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#material,
                    r#type,
                    r#cap,
                    r#description,
                    r#capacity,
                    r#minimum_volume,
                    r#additive: r#additive.unwrap_or(vec![]),
                    r#preparation,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SpecimenDefinitionTypeTestedContainer>>
{
    type Value = Box<SpecimenDefinitionTypeTestedContainer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenDefinitionTypeTestedContainer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedContainer>>
{
    type Value = Vec<SpecimenDefinitionTypeTestedContainer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedContainer>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenDefinitionTypeTestedContainer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenDefinitionTypeTestedContainer> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedHandling;
impl serde::ser::Serialize for SerializationContext<&SpecimenDefinitionTypeTestedHandling> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SpecimenDefinition.typeTested.handling", field
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
        if let Some(some) = self.value.r#temperature_qualifier.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("temperatureQualifier", ctx)
            })?;
        }
        if let Some(some) = self.value.r#temperature_range.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("temperatureRange", ctx))?;
        }
        if let Some(some) = self.value.r#max_duration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxDuration", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#instruction.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("instruction", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_instruction", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#instruction.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("instruction", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenDefinitionTypeTestedHandling>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenDefinitionTypeTestedHandling>> {
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
    for &mut DeserializationContext<SpecimenDefinitionTypeTestedHandling>
{
    type Value = SpecimenDefinitionTypeTestedHandling;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenDefinitionTypeTestedHandling>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenDefinitionTypeTestedHandling;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTestedHandling")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTestedHandling, V::Error>
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
                    #[serde(rename = "temperatureQualifier")]
                    TemperatureQualifier,
                    #[serde(rename = "temperatureRange")]
                    TemperatureRange,
                    #[serde(rename = "maxDuration")]
                    MaxDuration,
                    #[serde(rename = "instruction")]
                    Instruction,
                    #[serde(rename = "_instruction")]
                    InstructionPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "temperatureQualifier",
                            "temperatureRange",
                            "maxDuration",
                            "instruction",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#temperature_qualifier: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#temperature_range: Option<Box<fhirbolt_model::r4::types::Range>> = None;
                let mut r#max_duration: Option<Box<fhirbolt_model::r4::types::Duration>> = None;
                let mut r#instruction: Option<fhirbolt_model::r4::types::String> = None;
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TemperatureQualifier => {
                            if r#temperature_qualifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "temperatureQualifier",
                                ));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#temperature_qualifier =
                                Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TemperatureRange => {
                            if r#temperature_range.is_some() {
                                return Err(serde::de::Error::duplicate_field("temperatureRange"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Range>,
                            > = self.0.transmute();
                            r#temperature_range = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::MaxDuration => {
                            if r#max_duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDuration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Duration>,
                            > = self.0.transmute();
                            r#max_duration = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Instruction => {
                            if self.0.from_json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field("instruction"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#instruction = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::InstructionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_instruction"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("instruction");
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
                Ok(SpecimenDefinitionTypeTestedHandling {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#temperature_qualifier,
                    r#temperature_range,
                    r#max_duration,
                    r#instruction,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SpecimenDefinitionTypeTestedHandling>>
{
    type Value = Box<SpecimenDefinitionTypeTestedHandling>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenDefinitionTypeTestedHandling>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedHandling>>
{
    type Value = Vec<SpecimenDefinitionTypeTestedHandling>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<SpecimenDefinitionTypeTestedHandling>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenDefinitionTypeTestedHandling>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenDefinitionTypeTestedHandling> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenDefinitionTypeTested;
impl serde::ser::Serialize for SerializationContext<&SpecimenDefinitionTypeTested> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SpecimenDefinition.typeTested", field
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
            if let Some(some) = self.value.r#is_derived.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("isDerived", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isDerived", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#is_derived.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("isDerived", ctx))?;
        }
        if let Some(some) = self.value.r#type.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("type", ctx))?;
        }
        if self.output_json {
            if self.value.r#preference.id.as_deref() == Some("$invalid") {
                return missing_field_error("preference");
            }
            if let Some(some) = self.value.r#preference.value.as_ref().map(Ok) {
                state.serialize_entry("preference", &some?)?;
            }
            if self.value.r#preference.id.is_some() || !self.value.r#preference.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#preference.id.as_ref(),
                    extension: &self.value.r#preference.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_preference", ctx)
                })?;
            }
        } else if self.value.r#preference.id.as_deref() == Some("$invalid") {
            return missing_field_error("preference");
        } else {
            self.with_context(&self.value.r#preference, |ctx| {
                state.serialize_entry("preference", ctx)
            })?;
        }
        if let Some(some) = self.value.r#container.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("container", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#requirement.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("requirement", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_requirement", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#requirement.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("requirement", ctx))?;
        }
        if let Some(some) = self.value.r#retention_time.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("retentionTime", ctx))?;
        }
        if !self.value.r#rejection_criterion.is_empty() {
            self.with_context(&self.value.r#rejection_criterion, |ctx| {
                state.serialize_entry("rejectionCriterion", ctx)
            })?;
        }
        if !self.value.r#handling.is_empty() {
            self.with_context(&self.value.r#handling, |ctx| {
                state.serialize_entry("handling", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenDefinitionTypeTested>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenDefinitionTypeTested>> {
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
    for &mut DeserializationContext<SpecimenDefinitionTypeTested>
{
    type Value = SpecimenDefinitionTypeTested;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenDefinitionTypeTested>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenDefinitionTypeTested;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinitionTypeTested")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<SpecimenDefinitionTypeTested, V::Error>
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
                    #[serde(rename = "isDerived")]
                    IsDerived,
                    #[serde(rename = "_isDerived")]
                    IsDerivedPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "preference")]
                    Preference,
                    #[serde(rename = "_preference")]
                    PreferencePrimitiveElement,
                    #[serde(rename = "container")]
                    Container,
                    #[serde(rename = "requirement")]
                    Requirement,
                    #[serde(rename = "_requirement")]
                    RequirementPrimitiveElement,
                    #[serde(rename = "retentionTime")]
                    RetentionTime,
                    #[serde(rename = "rejectionCriterion")]
                    RejectionCriterion,
                    #[serde(rename = "handling")]
                    Handling,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "isDerived",
                            "type",
                            "preference",
                            "container",
                            "requirement",
                            "retentionTime",
                            "rejectionCriterion",
                            "handling",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#is_derived: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#preference: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#container: Option<
                    fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedContainer,
                > = None;
                let mut r#requirement: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#retention_time: Option<Box<fhirbolt_model::r4::types::Duration>> = None;
                let mut r#rejection_criterion: Option<
                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#handling: Option<
                    Vec<fhirbolt_model::r4::resources::SpecimenDefinitionTypeTestedHandling>,
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
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsDerived => {
                            if self.0.from_json {
                                let some = r#is_derived.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isDerived"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#is_derived.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isDerived"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Boolean,
                                > = self.0.transmute();
                                r#is_derived = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IsDerivedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_derived.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isDerived"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isDerived");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Preference => {
                            if self.0.from_json {
                                let some = r#preference.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preference"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#preference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preference"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#preference = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PreferencePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#preference.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preference"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preference");
                            }
                        }
                        Field::Container => {
                            if r#container.is_some() {
                                return Err(serde::de::Error::duplicate_field("container"));
                            }
                            let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedContainer > = self . 0 . transmute () ;
                            r#container = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Requirement => {
                            if self.0.from_json {
                                let some = r#requirement.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirement"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#requirement.is_some() {
                                    return Err(serde::de::Error::duplicate_field("requirement"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#requirement = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::RequirementPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#requirement.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_requirement"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("requirement");
                            }
                        }
                        Field::RetentionTime => {
                            if r#retention_time.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionTime"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Duration>,
                            > = self.0.transmute();
                            r#retention_time = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::RejectionCriterion => {
                            if self.0.from_json {
                                if r#rejection_criterion.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "rejectionCriterion",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#rejection_criterion =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#rejection_criterion.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Handling => {
                            if self.0.from_json {
                                if r#handling.is_some() {
                                    return Err(serde::de::Error::duplicate_field("handling"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedHandling >> = self . 0 . transmute () ;
                                r#handling = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#handling.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r4 :: resources :: SpecimenDefinitionTypeTestedHandling > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(SpecimenDefinitionTypeTested {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#is_derived,
                    r#type,
                    r#preference: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#preference.unwrap_or(Default::default())
                    } else {
                        r#preference.ok_or(serde::de::Error::missing_field("preference"))?
                    },
                    r#container,
                    r#requirement,
                    r#retention_time,
                    r#rejection_criterion: r#rejection_criterion.unwrap_or(vec![]),
                    r#handling: r#handling.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<SpecimenDefinitionTypeTested>>
{
    type Value = Box<SpecimenDefinitionTypeTested>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenDefinitionTypeTested>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<SpecimenDefinitionTypeTested>>
{
    type Value = Vec<SpecimenDefinitionTypeTested>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SpecimenDefinitionTypeTested>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenDefinitionTypeTested>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenDefinitionTypeTested> =
                    self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
use fhirbolt_model::r4::resources::SpecimenDefinition;
impl crate::Resource for SpecimenDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize for SerializationContext<&SpecimenDefinition> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "SpecimenDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "SpecimenDefinition")?;
        if self.output_json {
            if let Some(some) = self.value.r#id.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("id", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| state.serialize_entry("_id", ctx))?;
                }
            }
        } else if let Some(some) = self.value.r#id.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("id", ctx))?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("implicitRules", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#implicit_rules.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("language", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#language.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
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
        if let Some(some) = self.value.r#identifier.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("identifier", ctx))?;
        }
        if let Some(some) = self.value.r#type_collected.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("typeCollected", ctx))?;
        }
        if !self.value.r#patient_preparation.is_empty() {
            self.with_context(&self.value.r#patient_preparation, |ctx| {
                state.serialize_entry("patientPreparation", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#time_aspect.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("timeAspect", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_timeAspect", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#time_aspect.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("timeAspect", ctx))?;
        }
        if !self.value.r#collection.is_empty() {
            self.with_context(&self.value.r#collection, |ctx| {
                state.serialize_entry("collection", ctx)
            })?;
        }
        if !self.value.r#type_tested.is_empty() {
            self.with_context(&self.value.r#type_tested, |ctx| {
                state.serialize_entry("typeTested", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<SpecimenDefinition>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<SpecimenDefinition>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<SpecimenDefinition> {
    type Value = SpecimenDefinition;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<SpecimenDefinition> {
    type Value = SpecimenDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<SpecimenDefinition>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = SpecimenDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("SpecimenDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<SpecimenDefinition, V::Error>
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "typeCollected")]
                    TypeCollected,
                    #[serde(rename = "patientPreparation")]
                    PatientPreparation,
                    #[serde(rename = "timeAspect")]
                    TimeAspect,
                    #[serde(rename = "_timeAspect")]
                    TimeAspectPrimitiveElement,
                    #[serde(rename = "collection")]
                    Collection,
                    #[serde(rename = "typeTested")]
                    TypeTested,
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
                            "typeCollected",
                            "patientPreparation",
                            "timeAspect",
                            "collection",
                            "typeTested",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r4::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r4::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4::types::Extension>> =
                    None;
                let mut r#identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#type_collected: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#patient_preparation: Option<
                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#time_aspect: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#collection: Option<Vec<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#type_tested: Option<
                    Vec<fhirbolt_model::r4::resources::SpecimenDefinitionTypeTested>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "SpecimenDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"SpecimenDefinition",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from_json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#id.is_some() {
                                    return Err(serde::de::Error::duplicate_field("id"));
                                }
                                let _context: &mut DeserializationContext<
                                    Box<fhirbolt_model::r4::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::IdPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#id.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_id"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                                Box<fhirbolt_model::r4::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from_json {
                                let some = r#implicit_rules.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#implicit_rules.is_some() {
                                    return Err(serde::de::Error::duplicate_field("implicitRules"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::LanguagePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#language.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_language"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
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
                                Box<fhirbolt_model::r4::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::Identifier>,
                            > = self.0.transmute();
                            r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::TypeCollected => {
                            if r#type_collected.is_some() {
                                return Err(serde::de::Error::duplicate_field("typeCollected"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#type_collected = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::PatientPreparation => {
                            if self.0.from_json {
                                if r#patient_preparation.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientPreparation",
                                    ));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#patient_preparation =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#patient_preparation.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TimeAspect => {
                            if self.0.from_json {
                                let some = r#time_aspect.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timeAspect"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#time_aspect.is_some() {
                                    return Err(serde::de::Error::duplicate_field("timeAspect"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::String,
                                > = self.0.transmute();
                                r#time_aspect = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TimeAspectPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#time_aspect.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_timeAspect"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("timeAspect");
                            }
                        }
                        Field::Collection => {
                            if self.0.from_json {
                                if r#collection.is_some() {
                                    return Err(serde::de::Error::duplicate_field("collection"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4::types::CodeableConcept>,
                                > = self.0.transmute();
                                r#collection = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#collection.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::types::CodeableConcept,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::TypeTested => {
                            if self.0.from_json {
                                if r#type_tested.is_some() {
                                    return Err(serde::de::Error::duplicate_field("typeTested"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<
                                        fhirbolt_model::r4::resources::SpecimenDefinitionTypeTested,
                                    >,
                                > = self.0.transmute();
                                r#type_tested = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#type_tested.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4::resources::SpecimenDefinitionTypeTested,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                Ok(SpecimenDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#type_collected,
                    r#patient_preparation: r#patient_preparation.unwrap_or(vec![]),
                    r#time_aspect,
                    r#collection: r#collection.unwrap_or(vec![]),
                    r#type_tested: r#type_tested.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<SpecimenDefinition>> {
    type Value = Box<SpecimenDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<SpecimenDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<SpecimenDefinition>> {
    type Value = Vec<SpecimenDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<SpecimenDefinition>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<SpecimenDefinition>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<SpecimenDefinition> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
