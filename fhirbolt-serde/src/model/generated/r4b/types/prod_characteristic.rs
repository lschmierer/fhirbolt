// Generated on 2023-05-07 by fhirbolt-codegen v0.8.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r4b::types::ProdCharacteristic;
impl serde::ser::Serialize for SerializationContext<&ProdCharacteristic> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "ProdCharacteristic", field
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
        if let Some(some) = self.value.r#height.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("height", ctx))?;
        }
        if let Some(some) = self.value.r#width.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("width", ctx))?;
        }
        if let Some(some) = self.value.r#depth.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("depth", ctx))?;
        }
        if let Some(some) = self.value.r#weight.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("weight", ctx))?;
        }
        if let Some(some) = self.value.r#nominal_volume.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("nominalVolume", ctx))?;
        }
        if let Some(some) = self.value.r#external_diameter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("externalDiameter", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#shape.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("shape", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_shape", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#shape.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("shape", ctx))?;
        }
        if self.output_json {
            if !self.value.r#color.is_empty() {
                let values = self
                    .value
                    .r#color
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("color", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#color
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#color
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_color", ctx)
                    })?;
                }
            }
        } else if !self.value.r#color.is_empty() {
            self.with_context(&self.value.r#color, |ctx| {
                state.serialize_entry("color", ctx)
            })?;
        }
        if self.output_json {
            if !self.value.r#imprint.is_empty() {
                let values = self
                    .value
                    .r#imprint
                    .iter()
                    .map(|v| &v.value)
                    .map(|v| v.as_ref().map(Ok).transpose())
                    .collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.is_some()) {
                    state.serialize_entry("imprint", &values)?;
                }
                let requires_elements = self
                    .value
                    .r#imprint
                    .iter()
                    .any(|e| e.id.is_some() || !e.extension.is_empty());
                if requires_elements {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_elements: Vec<_> = self
                        .value
                        .r#imprint
                        .iter()
                        .map(|e| {
                            if e.id.is_some() || !e.extension.is_empty() {
                                Some(PrimitiveElement {
                                    id: e.id.as_ref(),
                                    extension: &e.extension,
                                })
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.with_context(&primitive_elements, |ctx| {
                        state.serialize_entry("_imprint", ctx)
                    })?;
                }
            }
        } else if !self.value.r#imprint.is_empty() {
            self.with_context(&self.value.r#imprint, |ctx| {
                state.serialize_entry("imprint", ctx)
            })?;
        }
        if !self.value.r#image.is_empty() {
            self.with_context(&self.value.r#image, |ctx| {
                state.serialize_entry("image", ctx)
            })?;
        }
        if let Some(some) = self.value.r#scoring.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("scoring", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<ProdCharacteristic>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<ProdCharacteristic>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<ProdCharacteristic> {
    type Value = ProdCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<ProdCharacteristic>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = ProdCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProdCharacteristic")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProdCharacteristic, V::Error>
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
                    #[serde(rename = "height")]
                    Height,
                    #[serde(rename = "width")]
                    Width,
                    #[serde(rename = "depth")]
                    Depth,
                    #[serde(rename = "weight")]
                    Weight,
                    #[serde(rename = "nominalVolume")]
                    NominalVolume,
                    #[serde(rename = "externalDiameter")]
                    ExternalDiameter,
                    #[serde(rename = "shape")]
                    Shape,
                    #[serde(rename = "_shape")]
                    ShapePrimitiveElement,
                    #[serde(rename = "color")]
                    Color,
                    #[serde(rename = "_color")]
                    ColorPrimitiveElement,
                    #[serde(rename = "imprint")]
                    Imprint,
                    #[serde(rename = "_imprint")]
                    ImprintPrimitiveElement,
                    #[serde(rename = "image")]
                    Image,
                    #[serde(rename = "scoring")]
                    Scoring,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "height",
                            "width",
                            "depth",
                            "weight",
                            "nominalVolume",
                            "externalDiameter",
                            "shape",
                            "color",
                            "imprint",
                            "image",
                            "scoring",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r4b::types::Extension>> =
                    None;
                let mut r#height: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#width: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#depth: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#weight: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#nominal_volume: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#external_diameter: Option<Box<fhirbolt_model::r4b::types::Quantity>> =
                    None;
                let mut r#shape: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#color: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
                let mut r#imprint: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
                let mut r#image: Option<Vec<fhirbolt_model::r4b::types::Attachment>> = None;
                let mut r#scoring: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
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
                                    Vec<fhirbolt_model::r4b::types::Extension>,
                                > = self.0.transmute();
                                r#modifier_extension =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Height => {
                            if r#height.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#height = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Width => {
                            if r#width.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#width = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Depth => {
                            if r#depth.is_some() {
                                return Err(serde::de::Error::duplicate_field("depth"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#depth = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Weight => {
                            if r#weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#weight = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::NominalVolume => {
                            if r#nominal_volume.is_some() {
                                return Err(serde::de::Error::duplicate_field("nominalVolume"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#nominal_volume = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ExternalDiameter => {
                            if r#external_diameter.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalDiameter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::Quantity>,
                            > = self.0.transmute();
                            r#external_diameter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Shape => {
                            if self.0.from_json {
                                let some = r#shape.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("shape"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#shape.is_some() {
                                    return Err(serde::de::Error::duplicate_field("shape"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                r#shape = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ShapePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#shape.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_shape"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("shape");
                            }
                        }
                        Field::Color => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#color.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("color"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#color.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ColorPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#color.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_color"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("color");
                            }
                        }
                        Field::Imprint => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#imprint.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("imprint"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#imprint.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::String,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ImprintPrimitiveElement => {
                            if self.0.from_json {
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<
                                    Vec<Option<PrimitiveElementOwned>>,
                                > = self.0.transmute();
                                let elements: Vec<Option<PrimitiveElementOwned>> =
                                    map_access.next_value_seed(&mut *_context)?;
                                let vec = r#imprint.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_imprint"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("imprint");
                            }
                        }
                        Field::Image => {
                            if self.0.from_json {
                                if r#image.is_some() {
                                    return Err(serde::de::Error::duplicate_field("image"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r4b::types::Attachment>,
                                > = self.0.transmute();
                                r#image = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#image.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r4b::types::Attachment,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Scoring => {
                            if r#scoring.is_some() {
                                return Err(serde::de::Error::duplicate_field("scoring"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#scoring = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(ProdCharacteristic {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#height,
                    r#width,
                    r#depth,
                    r#weight,
                    r#nominal_volume,
                    r#external_diameter,
                    r#shape,
                    r#color: r#color.unwrap_or(vec![]),
                    r#imprint: r#imprint.unwrap_or(vec![]),
                    r#image: r#image.unwrap_or(vec![]),
                    r#scoring,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<ProdCharacteristic>> {
    type Value = Box<ProdCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<ProdCharacteristic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<ProdCharacteristic>> {
    type Value = Vec<ProdCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<ProdCharacteristic>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<ProdCharacteristic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<ProdCharacteristic> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
