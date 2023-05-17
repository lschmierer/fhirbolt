// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::resources::VisionPrescriptionLensSpecificationPrism;
impl serde::ser::Serialize for SerializationContext<&VisionPrescriptionLensSpecificationPrism> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "VisionPrescription.lensSpecification.prism", field
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
        if self.output == crate::context::Format::Json {
            if self.value.r#amount.id.as_deref() == Some("$invalid") {
                return missing_field_error("amount");
            }
            if let Some(some) = self.value.r#amount.value.as_ref().map(|v| {
                v.parse::<serde_json::Number>()
                    .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
            }) {
                state.serialize_entry("amount", &some?)?;
            }
            if self.value.r#amount.id.is_some() || !self.value.r#amount.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#amount.id.as_ref(),
                    extension: &self.value.r#amount.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_amount", ctx)
                })?;
            }
        } else if self.value.r#amount.id.as_deref() == Some("$invalid") {
            return missing_field_error("amount");
        } else {
            self.with_context(&self.value.r#amount, |ctx| {
                state.serialize_entry("amount", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#base.id.as_deref() == Some("$invalid") {
                return missing_field_error("base");
            }
            if let Some(some) = self.value.r#base.value.as_ref().map(Ok) {
                state.serialize_entry("base", &some?)?;
            }
            if self.value.r#base.id.is_some() || !self.value.r#base.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#base.id.as_ref(),
                    extension: &self.value.r#base.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_base", ctx)
                })?;
            }
        } else if self.value.r#base.id.as_deref() == Some("$invalid") {
            return missing_field_error("base");
        } else {
            self.with_context(&self.value.r#base, |ctx| state.serialize_entry("base", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Box<VisionPrescriptionLensSpecificationPrism>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for SerializationContext<&Vec<VisionPrescriptionLensSpecificationPrism>>
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
    for &mut DeserializationContext<VisionPrescriptionLensSpecificationPrism>
{
    type Value = VisionPrescriptionLensSpecificationPrism;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<VisionPrescriptionLensSpecificationPrism>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = VisionPrescriptionLensSpecificationPrism;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescriptionLensSpecificationPrism")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VisionPrescriptionLensSpecificationPrism, V::Error>
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
                    #[serde(rename = "amount")]
                    Amount,
                    #[serde(rename = "_amount")]
                    AmountPrimitiveElement,
                    #[serde(rename = "base")]
                    Base,
                    #[serde(rename = "_base")]
                    BasePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "amount", "base"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#amount: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#base: Option<fhirbolt_model::r5::types::Code> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Amount => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#amount.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#amount.is_some() {
                                    return Err(serde::de::Error::duplicate_field("amount"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#amount = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AmountPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#amount.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_amount"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("amount");
                            }
                        }
                        Field::Base => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#base.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("base"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#base.is_some() {
                                    return Err(serde::de::Error::duplicate_field("base"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#base = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BasePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#base.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_base"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("base");
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
                Ok(VisionPrescriptionLensSpecificationPrism {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#amount: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#amount.unwrap_or(Default::default())
                    } else {
                        r#amount.ok_or(serde::de::Error::missing_field("amount"))?
                    },
                    r#base: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#base.unwrap_or(Default::default())
                    } else {
                        r#base.ok_or(serde::de::Error::missing_field("base"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<VisionPrescriptionLensSpecificationPrism>>
{
    type Value = Box<VisionPrescriptionLensSpecificationPrism>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<VisionPrescriptionLensSpecificationPrism>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<VisionPrescriptionLensSpecificationPrism>>
{
    type Value = Vec<VisionPrescriptionLensSpecificationPrism>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<VisionPrescriptionLensSpecificationPrism>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<VisionPrescriptionLensSpecificationPrism>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<
                    VisionPrescriptionLensSpecificationPrism,
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
use fhirbolt_model::r5::resources::VisionPrescriptionLensSpecification;
impl serde::ser::Serialize for SerializationContext<&VisionPrescriptionLensSpecification> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "VisionPrescription.lensSpecification", field
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
        } else {
            self.with_context(&self.value.r#product, |ctx| {
                state.serialize_entry("product", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#eye.id.as_deref() == Some("$invalid") {
                return missing_field_error("eye");
            }
            if let Some(some) = self.value.r#eye.value.as_ref().map(Ok) {
                state.serialize_entry("eye", &some?)?;
            }
            if self.value.r#eye.id.is_some() || !self.value.r#eye.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#eye.id.as_ref(),
                    extension: &self.value.r#eye.extension,
                };
                self.with_context(&primitive_element, |ctx| state.serialize_entry("_eye", ctx))?;
            }
        } else if self.value.r#eye.id.as_deref() == Some("$invalid") {
            return missing_field_error("eye");
        } else {
            self.with_context(&self.value.r#eye, |ctx| state.serialize_entry("eye", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#sphere.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("sphere", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_sphere", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#sphere.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("sphere", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#cylinder.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("cylinder", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_cylinder", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#cylinder.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("cylinder", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#axis.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("axis", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_axis", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#axis.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("axis", ctx))?;
        }
        if !self.value.r#prism.is_empty() {
            self.with_context(&self.value.r#prism, |ctx| {
                state.serialize_entry("prism", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#add.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("add", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_add", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#add.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("add", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#power.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("power", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_power", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#power.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("power", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#back_curve.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("backCurve", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_backCurve", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#back_curve.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("backCurve", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#diameter.as_ref() {
                if let Some(some) = some.value.as_ref().map(|v| {
                    v.parse::<serde_json::Number>()
                        .map_err(|_| serde::ser::Error::custom("error serializing decimal"))
                }) {
                    state.serialize_entry("diameter", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_diameter", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#diameter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("diameter", ctx))?;
        }
        if let Some(some) = self.value.r#duration.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("duration", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#color.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("color", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_color", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#color.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("color", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if let Some(some) = self.value.r#brand.as_ref() {
                if let Some(some) = some.value.as_ref().map(Ok) {
                    state.serialize_entry("brand", &some?)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    use super::super::serde_helpers::PrimitiveElement;
                    let primitive_element = PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_brand", ctx)
                    })?;
                }
            }
        } else if let Some(some) = self.value.r#brand.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("brand", ctx))?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<VisionPrescriptionLensSpecification>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<VisionPrescriptionLensSpecification>> {
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
    for &mut DeserializationContext<VisionPrescriptionLensSpecification>
{
    type Value = VisionPrescriptionLensSpecification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<VisionPrescriptionLensSpecification>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = VisionPrescriptionLensSpecification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescriptionLensSpecification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<VisionPrescriptionLensSpecification, V::Error>
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
                    #[serde(rename = "eye")]
                    Eye,
                    #[serde(rename = "_eye")]
                    EyePrimitiveElement,
                    #[serde(rename = "sphere")]
                    Sphere,
                    #[serde(rename = "_sphere")]
                    SpherePrimitiveElement,
                    #[serde(rename = "cylinder")]
                    Cylinder,
                    #[serde(rename = "_cylinder")]
                    CylinderPrimitiveElement,
                    #[serde(rename = "axis")]
                    Axis,
                    #[serde(rename = "_axis")]
                    AxisPrimitiveElement,
                    #[serde(rename = "prism")]
                    Prism,
                    #[serde(rename = "add")]
                    Add,
                    #[serde(rename = "_add")]
                    AddPrimitiveElement,
                    #[serde(rename = "power")]
                    Power,
                    #[serde(rename = "_power")]
                    PowerPrimitiveElement,
                    #[serde(rename = "backCurve")]
                    BackCurve,
                    #[serde(rename = "_backCurve")]
                    BackCurvePrimitiveElement,
                    #[serde(rename = "diameter")]
                    Diameter,
                    #[serde(rename = "_diameter")]
                    DiameterPrimitiveElement,
                    #[serde(rename = "duration")]
                    Duration,
                    #[serde(rename = "color")]
                    Color,
                    #[serde(rename = "_color")]
                    ColorPrimitiveElement,
                    #[serde(rename = "brand")]
                    Brand,
                    #[serde(rename = "_brand")]
                    BrandPrimitiveElement,
                    #[serde(rename = "note")]
                    Note,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "product",
                            "eye",
                            "sphere",
                            "cylinder",
                            "axis",
                            "prism",
                            "add",
                            "power",
                            "backCurve",
                            "diameter",
                            "duration",
                            "color",
                            "brand",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#product: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#eye: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#sphere: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#cylinder: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#axis: Option<fhirbolt_model::r5::types::Integer> = None;
                let mut r#prism: Option<
                    Vec<fhirbolt_model::r5::resources::VisionPrescriptionLensSpecificationPrism>,
                > = None;
                let mut r#add: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#power: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#back_curve: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#diameter: Option<fhirbolt_model::r5::types::Decimal> = None;
                let mut r#duration: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#color: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#brand: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#note: Option<Vec<fhirbolt_model::r5::types::Annotation>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::Id => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
                        }
                        Field::Extension => {
                            if self.0.from == crate::context::Format::Json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Extension>,
                                > = self.0.transmute();
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Product => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                            > = self.0.transmute();
                            r#product = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Eye => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#eye.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eye"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#eye.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eye"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#eye = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::EyePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#eye.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_eye"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("eye");
                            }
                        }
                        Field::Sphere => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sphere.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sphere"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#sphere.is_some() {
                                    return Err(serde::de::Error::duplicate_field("sphere"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#sphere = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::SpherePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#sphere.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_sphere"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("sphere");
                            }
                        }
                        Field::Cylinder => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cylinder.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cylinder"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#cylinder.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cylinder"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#cylinder = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CylinderPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#cylinder.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_cylinder"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("cylinder");
                            }
                        }
                        Field::Axis => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#axis.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("axis"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#axis.is_some() {
                                    return Err(serde::de::Error::duplicate_field("axis"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Integer,
                                > = self.0.transmute();
                                r#axis = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AxisPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#axis.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_axis"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("axis");
                            }
                        }
                        Field::Prism => {
                            if self.0.from == crate::context::Format::Json {
                                if r#prism.is_some() {
                                    return Err(serde::de::Error::duplicate_field("prism"));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: VisionPrescriptionLensSpecificationPrism >> = self . 0 . transmute () ;
                                r#prism = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#prism.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: VisionPrescriptionLensSpecificationPrism > = self . 0 . transmute () ;
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Add => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#add.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("add"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#add.is_some() {
                                    return Err(serde::de::Error::duplicate_field("add"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#add = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::AddPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#add.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_add"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("add");
                            }
                        }
                        Field::Power => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#power.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("power"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#power.is_some() {
                                    return Err(serde::de::Error::duplicate_field("power"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#power = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::PowerPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#power.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_power"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("power");
                            }
                        }
                        Field::BackCurve => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#back_curve.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("backCurve"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#back_curve.is_some() {
                                    return Err(serde::de::Error::duplicate_field("backCurve"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#back_curve = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BackCurvePrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#back_curve.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_backCurve"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("backCurve");
                            }
                        }
                        Field::Diameter => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#diameter.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diameter"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#diameter.is_some() {
                                    return Err(serde::de::Error::duplicate_field("diameter"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Decimal,
                                > = self.0.transmute();
                                r#diameter = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DiameterPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#diameter.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_diameter"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("diameter");
                            }
                        }
                        Field::Duration => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Quantity>,
                            > = self.0.transmute();
                            r#duration = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Color => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#color.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("color"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#color.is_some() {
                                    return Err(serde::de::Error::duplicate_field("color"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#color = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::ColorPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#color.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_color"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("color");
                            }
                        }
                        Field::Brand => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#brand.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("brand"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#brand.is_some() {
                                    return Err(serde::de::Error::duplicate_field("brand"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::String,
                                > = self.0.transmute();
                                r#brand = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::BrandPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#brand.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_brand"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("brand");
                            }
                        }
                        Field::Note => {
                            if self.0.from == crate::context::Format::Json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Annotation>,
                                > = self.0.transmute();
                                r#note = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Annotation,
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
                Ok(VisionPrescriptionLensSpecification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#product.unwrap_or(Default::default())
                    } else {
                        r#product.ok_or(serde::de::Error::missing_field("product"))?
                    },
                    r#eye: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#eye.unwrap_or(Default::default())
                    } else {
                        r#eye.ok_or(serde::de::Error::missing_field("eye"))?
                    },
                    r#sphere,
                    r#cylinder,
                    r#axis,
                    r#prism: r#prism.unwrap_or(vec![]),
                    r#add,
                    r#power,
                    r#back_curve,
                    r#diameter,
                    r#duration,
                    r#color,
                    r#brand,
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Box<VisionPrescriptionLensSpecification>>
{
    type Value = Box<VisionPrescriptionLensSpecification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<VisionPrescriptionLensSpecification>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut DeserializationContext<Vec<VisionPrescriptionLensSpecification>>
{
    type Value = Vec<VisionPrescriptionLensSpecification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut DeserializationContext<Vec<VisionPrescriptionLensSpecification>>,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<VisionPrescriptionLensSpecification>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<VisionPrescriptionLensSpecification> =
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
use fhirbolt_model::r5::resources::VisionPrescription;
impl crate::Resource for VisionPrescription {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&VisionPrescription> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "VisionPrescription", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "VisionPrescription")?;
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
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
        if self.output == crate::context::Format::Json {
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
        if !self.value.r#identifier.is_empty() {
            self.with_context(&self.value.r#identifier, |ctx| {
                state.serialize_entry("identifier", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            if let Some(some) = self.value.r#status.value.as_ref().map(Ok) {
                state.serialize_entry("status", &some?)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else if self.value.r#status.id.as_deref() == Some("$invalid") {
            return missing_field_error("status");
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#created.id.as_deref() == Some("$invalid") {
                return missing_field_error("created");
            }
            if let Some(some) = self.value.r#created.value.as_ref().map(Ok) {
                state.serialize_entry("created", &some?)?;
            }
            if self.value.r#created.id.is_some() || !self.value.r#created.extension.is_empty() {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#created.id.as_ref(),
                    extension: &self.value.r#created.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_created", ctx)
                })?;
            }
        } else if self.value.r#created.id.as_deref() == Some("$invalid") {
            return missing_field_error("created");
        } else {
            self.with_context(&self.value.r#created, |ctx| {
                state.serialize_entry("created", ctx)
            })?;
        }
        if self.value.r#patient.id.as_deref() == Some("$invalid") {
            return missing_field_error("patient");
        } else {
            self.with_context(&self.value.r#patient, |ctx| {
                state.serialize_entry("patient", ctx)
            })?;
        }
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if self.output == crate::context::Format::Json {
            if self.value.r#date_written.id.as_deref() == Some("$invalid") {
                return missing_field_error("dateWritten");
            }
            if let Some(some) = self.value.r#date_written.value.as_ref().map(Ok) {
                state.serialize_entry("dateWritten", &some?)?;
            }
            if self.value.r#date_written.id.is_some()
                || !self.value.r#date_written.extension.is_empty()
            {
                use super::super::serde_helpers::PrimitiveElement;
                let primitive_element = PrimitiveElement {
                    id: self.value.r#date_written.id.as_ref(),
                    extension: &self.value.r#date_written.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_dateWritten", ctx)
                })?;
            }
        } else if self.value.r#date_written.id.as_deref() == Some("$invalid") {
            return missing_field_error("dateWritten");
        } else {
            self.with_context(&self.value.r#date_written, |ctx| {
                state.serialize_entry("dateWritten", ctx)
            })?;
        }
        if self.value.r#prescriber.id.as_deref() == Some("$invalid") {
            return missing_field_error("prescriber");
        } else {
            self.with_context(&self.value.r#prescriber, |ctx| {
                state.serialize_entry("prescriber", ctx)
            })?;
        }
        if !self.value.r#lens_specification.is_empty() {
            self.with_context(&self.value.r#lens_specification, |ctx| {
                state.serialize_entry("lensSpecification", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize for SerializationContext<&Box<VisionPrescription>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<VisionPrescription>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<VisionPrescription> {
    type Value = VisionPrescription;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<VisionPrescription> {
    type Value = VisionPrescription;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<VisionPrescription>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = VisionPrescription;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescription")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<VisionPrescription, V::Error>
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
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "dateWritten")]
                    DateWritten,
                    #[serde(rename = "_dateWritten")]
                    DateWrittenPrimitiveElement,
                    #[serde(rename = "prescriber")]
                    Prescriber,
                    #[serde(rename = "lensSpecification")]
                    LensSpecification,
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
                            "status",
                            "created",
                            "patient",
                            "encounter",
                            "dateWritten",
                            "prescriber",
                            "lensSpecification",
                        ],
                    ))
                }
                let mut r#id: Option<Box<fhirbolt_model::r5::types::Id>> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<fhirbolt_model::r5::Resource>> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#identifier: Option<Vec<fhirbolt_model::r5::types::Identifier>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#created: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#date_written: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#prescriber: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#lens_specification: Option<
                    Vec<fhirbolt_model::r5::resources::VisionPrescriptionLensSpecification>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "VisionPrescription" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"VisionPrescription",
                                ));
                            }
                        }
                        Field::Id => {
                            if self.0.from == crate::context::Format::Json {
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
                                    Box<fhirbolt_model::r5::types::Id>,
                                > = self.0.transmute();
                                r#id = Some(map_access.next_value_seed(&mut *_context)?);
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
                                Box<fhirbolt_model::r5::types::Meta>,
                            > = self.0.transmute();
                            r#meta = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::ImplicitRules => {
                            if self.0.from == crate::context::Format::Json {
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
                                    fhirbolt_model::r5::types::Uri,
                                > = self.0.transmute();
                                r#implicit_rules =
                                    Some(map_access.next_value_seed(&mut *_context)?);
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
                                    map_access.next_value_seed(&mut *_context)?;
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
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#language.is_some() {
                                    return Err(serde::de::Error::duplicate_field("language"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#language = Some(map_access.next_value_seed(&mut *_context)?);
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
                                Box<fhirbolt_model::r5::types::Narrative>,
                            > = self.0.transmute();
                            r#text = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Contained => {
                            if self.0.from == crate::context::Format::Json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::Resource>,
                                > = self.0.transmute();
                                r#contained = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::Resource,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                r#extension = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
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
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Extension,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from == crate::context::Format::Json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                let _context: &mut DeserializationContext<
                                    Vec<fhirbolt_model::r5::types::Identifier>,
                                > = self.0.transmute();
                                r#identifier = Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Identifier,
                                > = self.0.transmute();
                                vec.push(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::Status => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::Code,
                                > = self.0.transmute();
                                r#status = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::Created => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#created.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#created = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::CreatedPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_created"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("created");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#patient = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#encounter = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::DateWritten => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date_written.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateWritten"));
                                }
                                some.value = Some(map_access.next_value()?);
                            } else {
                                if r#date_written.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dateWritten"));
                                }
                                let _context: &mut DeserializationContext<
                                    fhirbolt_model::r5::types::DateTime,
                                > = self.0.transmute();
                                r#date_written = Some(map_access.next_value_seed(&mut *_context)?);
                            }
                        }
                        Field::DateWrittenPrimitiveElement => {
                            if self.0.from == crate::context::Format::Json {
                                let some = r#date_written.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_dateWritten"));
                                }
                                use super::super::serde_helpers::PrimitiveElementOwned;
                                let _context: &mut DeserializationContext<PrimitiveElementOwned> =
                                    self.0.transmute();
                                let PrimitiveElementOwned { id, extension } =
                                    map_access.next_value_seed(&mut *_context)?;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("dateWritten");
                            }
                        }
                        Field::Prescriber => {
                            if r#prescriber.is_some() {
                                return Err(serde::de::Error::duplicate_field("prescriber"));
                            }
                            let _context: &mut DeserializationContext<
                                Box<fhirbolt_model::r5::types::Reference>,
                            > = self.0.transmute();
                            r#prescriber = Some(map_access.next_value_seed(&mut *_context)?);
                        }
                        Field::LensSpecification => {
                            if self.0.from == crate::context::Format::Json {
                                if r#lens_specification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "lensSpecification",
                                    ));
                                }
                                let _context : & mut DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: VisionPrescriptionLensSpecification >> = self . 0 . transmute () ;
                                r#lens_specification =
                                    Some(map_access.next_value_seed(&mut *_context)?);
                            } else {
                                let vec = r#lens_specification.get_or_insert(Default::default());
                                let _context : & mut DeserializationContext < fhirbolt_model :: r5 :: resources :: VisionPrescriptionLensSpecification > = self . 0 . transmute () ;
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
                Ok(VisionPrescription {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#created: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#created.unwrap_or(Default::default())
                    } else {
                        r#created.ok_or(serde::de::Error::missing_field("created"))?
                    },
                    r#patient: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#patient.unwrap_or(Default::default())
                    } else {
                        r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                    },
                    r#encounter,
                    r#date_written: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#date_written.unwrap_or(Default::default())
                    } else {
                        r#date_written.ok_or(serde::de::Error::missing_field("dateWritten"))?
                    },
                    r#prescriber: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#prescriber.unwrap_or(Default::default())
                    } else {
                        r#prescriber.ok_or(serde::de::Error::missing_field("prescriber"))?
                    },
                    r#lens_specification: r#lens_specification.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Box<VisionPrescription>> {
    type Value = Box<VisionPrescription>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<VisionPrescription>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<VisionPrescription>> {
    type Value = Vec<VisionPrescription>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<VisionPrescription>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<VisionPrescription>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                let _context: &mut DeserializationContext<VisionPrescription> = self.0.transmute();
                while let Some(value) = seq.next_element_seed(&mut *_context)? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
