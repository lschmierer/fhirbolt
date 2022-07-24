// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct ProdCharacteristic {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#height: Option<Box<super::super::types::Quantity>>,
    pub r#width: Option<Box<super::super::types::Quantity>>,
    pub r#depth: Option<Box<super::super::types::Quantity>>,
    pub r#weight: Option<Box<super::super::types::Quantity>>,
    pub r#nominal_volume: Option<Box<super::super::types::Quantity>>,
    pub r#external_diameter: Option<Box<super::super::types::Quantity>>,
    pub r#shape: Option<super::super::types::String>,
    pub r#color: Vec<super::super::types::String>,
    pub r#imprint: Vec<super::super::types::String>,
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for ProdCharacteristic {
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
        if let Some(some) = self.r#height.as_ref() {
            state.serialize_entry("height", some)?;
        }
        if let Some(some) = self.r#width.as_ref() {
            state.serialize_entry("width", some)?;
        }
        if let Some(some) = self.r#depth.as_ref() {
            state.serialize_entry("depth", some)?;
        }
        if let Some(some) = self.r#weight.as_ref() {
            state.serialize_entry("weight", some)?;
        }
        if let Some(some) = self.r#nominal_volume.as_ref() {
            state.serialize_entry("nominalVolume", some)?;
        }
        if let Some(some) = self.r#external_diameter.as_ref() {
            state.serialize_entry("externalDiameter", some)?;
        }
        if let Some(some) = self.r#shape.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("shape", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_shape", &primitive_element)?;
            }
        }
        if !self.r#color.is_empty() {
            let values: Vec<_> = self.r#color.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("color", &values)?;
            }
            let requires_elements = self
                .r#color
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#color
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
                state.serialize_entry("_color", &primitive_elements)?;
            }
        }
        if !self.r#imprint.is_empty() {
            let values: Vec<_> = self.r#imprint.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("imprint", &values)?;
            }
            let requires_elements = self
                .r#imprint
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#imprint
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
                state.serialize_entry("_imprint", &primitive_elements)?;
            }
        }
        if !self.r#image.is_empty() {
            state.serialize_entry("image", &self.r#image)?;
        }
        if let Some(some) = self.r#scoring.as_ref() {
            state.serialize_entry("scoring", some)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for ProdCharacteristic {
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
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProdCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("ProdCharacteristic")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<ProdCharacteristic, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#height: Option<Box<super::super::types::Quantity>> = None;
                let mut r#width: Option<Box<super::super::types::Quantity>> = None;
                let mut r#depth: Option<Box<super::super::types::Quantity>> = None;
                let mut r#weight: Option<Box<super::super::types::Quantity>> = None;
                let mut r#nominal_volume: Option<Box<super::super::types::Quantity>> = None;
                let mut r#external_diameter: Option<Box<super::super::types::Quantity>> = None;
                let mut r#shape: Option<super::super::types::String> = None;
                let mut r#color: Option<Vec<super::super::types::String>> = None;
                let mut r#imprint: Option<Vec<super::super::types::String>> = None;
                let mut r#image: Option<Vec<Box<super::super::types::Attachment>>> = None;
                let mut r#scoring: Option<Box<super::super::types::CodeableConcept>> = None;
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
                                return Err(serde::de::Error::duplicate_field("modifierExtension"));
                            }
                            r#modifier_extension = Some(map_access.next_value()?);
                        }
                        Field::Height => {
                            if r#height.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            r#height = Some(map_access.next_value()?);
                        }
                        Field::Width => {
                            if r#width.is_some() {
                                return Err(serde::de::Error::duplicate_field("width"));
                            }
                            r#width = Some(map_access.next_value()?);
                        }
                        Field::Depth => {
                            if r#depth.is_some() {
                                return Err(serde::de::Error::duplicate_field("depth"));
                            }
                            r#depth = Some(map_access.next_value()?);
                        }
                        Field::Weight => {
                            if r#weight.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            r#weight = Some(map_access.next_value()?);
                        }
                        Field::NominalVolume => {
                            if r#nominal_volume.is_some() {
                                return Err(serde::de::Error::duplicate_field("nominalVolume"));
                            }
                            r#nominal_volume = Some(map_access.next_value()?);
                        }
                        Field::ExternalDiameter => {
                            if r#external_diameter.is_some() {
                                return Err(serde::de::Error::duplicate_field("externalDiameter"));
                            }
                            r#external_diameter = Some(map_access.next_value()?);
                        }
                        Field::Shape => {
                            let some = r#shape.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("shape"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ShapePrimitiveElement => {
                            let some = r#shape.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_shape"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Color => {
                            let values: Vec<_> = map_access.next_value()?;
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
                                vec[i].value = value;
                            }
                        }
                        Field::ColorPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Imprint => {
                            let values: Vec<_> = map_access.next_value()?;
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
                                vec[i].value = value;
                            }
                        }
                        Field::ImprintPrimitiveElement => {
                            let elements: Vec<super::super::serde_helpers::PrimitiveElementOwned> =
                                map_access.next_value()?;
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
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        Field::Image => {
                            if r#image.is_some() {
                                return Err(serde::de::Error::duplicate_field("image"));
                            }
                            r#image = Some(map_access.next_value()?);
                        }
                        Field::Scoring => {
                            if r#scoring.is_some() {
                                return Err(serde::de::Error::duplicate_field("scoring"));
                            }
                            r#scoring = Some(map_access.next_value()?);
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
        deserializer.deserialize_map(Visitor)
    }
}
