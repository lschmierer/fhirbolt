// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct VisionPrescriptionLensSpecificationPrism {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#amount: super::super::types::Decimal,
    pub r#base: super::super::types::Code,
}
impl serde::ser::Serialize for VisionPrescriptionLensSpecificationPrism {
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
        if let Some(some) = self.r#amount.value.as_ref() {
            state.serialize_entry("amount", some)?;
        }
        if self.r#amount.id.is_some() || !self.r#amount.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#amount.id,
                extension: &self.r#amount.extension,
            };
            state.serialize_entry("_amount", &primitive_element)?;
        }
        if let Some(some) = self.r#base.value.as_ref() {
            state.serialize_entry("base", some)?;
        }
        if self.r#base.id.is_some() || !self.r#base.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#base.id,
                extension: &self.r#base.extension,
            };
            state.serialize_entry("_base", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescriptionLensSpecificationPrism {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#amount: Option<super::super::types::Decimal> = None;
                let mut r#base: Option<super::super::types::Code> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
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
                        "amount" => {
                            let some = r#amount.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_amount" => {
                            let some = r#amount.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_amount"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "base" => {
                            let some = r#base.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("base"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_base" => {
                            let some = r#base.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_base"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "amount", "base"],
                            ))
                        }
                    }
                }
                Ok(VisionPrescriptionLensSpecificationPrism {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#amount: r#amount.ok_or(serde::de::Error::missing_field("amount"))?,
                    r#base: r#base.ok_or(serde::de::Error::missing_field("base"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct VisionPrescriptionLensSpecification {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#product: Box<super::super::types::CodeableConcept>,
    pub r#eye: super::super::types::Code,
    pub r#sphere: Option<super::super::types::Decimal>,
    pub r#cylinder: Option<super::super::types::Decimal>,
    pub r#axis: Option<super::super::types::Integer>,
    pub r#prism: Vec<VisionPrescriptionLensSpecificationPrism>,
    pub r#add: Option<super::super::types::Decimal>,
    pub r#power: Option<super::super::types::Decimal>,
    pub r#back_curve: Option<super::super::types::Decimal>,
    pub r#diameter: Option<super::super::types::Decimal>,
    pub r#duration: Option<Box<super::super::types::Quantity>>,
    pub r#color: Option<super::super::types::String>,
    pub r#brand: Option<super::super::types::String>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
impl serde::ser::Serialize for VisionPrescriptionLensSpecification {
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
        state.serialize_entry("product", &self.r#product)?;
        if let Some(some) = self.r#eye.value.as_ref() {
            state.serialize_entry("eye", some)?;
        }
        if self.r#eye.id.is_some() || !self.r#eye.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#eye.id,
                extension: &self.r#eye.extension,
            };
            state.serialize_entry("_eye", &primitive_element)?;
        }
        if let Some(some) = self.r#sphere.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("sphere", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_sphere", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#cylinder.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("cylinder", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_cylinder", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#axis.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("axis", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_axis", &primitive_element)?;
            }
        }
        if !self.r#prism.is_empty() {
            state.serialize_entry("prism", &self.r#prism)?;
        }
        if let Some(some) = self.r#add.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("add", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_add", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#power.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("power", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_power", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#back_curve.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("backCurve", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_backCurve", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#diameter.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("diameter", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_diameter", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#duration.as_ref() {
            state.serialize_entry("duration", some)?;
        }
        if let Some(some) = self.r#color.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("color", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_color", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#brand.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("brand", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_brand", &primitive_element)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescriptionLensSpecification {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
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
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#product: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#eye: Option<super::super::types::Code> = None;
                let mut r#sphere: Option<super::super::types::Decimal> = None;
                let mut r#cylinder: Option<super::super::types::Decimal> = None;
                let mut r#axis: Option<super::super::types::Integer> = None;
                let mut r#prism: Option<Vec<VisionPrescriptionLensSpecificationPrism>> = None;
                let mut r#add: Option<super::super::types::Decimal> = None;
                let mut r#power: Option<super::super::types::Decimal> = None;
                let mut r#back_curve: Option<super::super::types::Decimal> = None;
                let mut r#diameter: Option<super::super::types::Decimal> = None;
                let mut r#duration: Option<Box<super::super::types::Quantity>> = None;
                let mut r#color: Option<super::super::types::String> = None;
                let mut r#brand: Option<super::super::types::String> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        "id" => {
                            if r#id.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            r#id = Some(map_access.next_value()?);
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
                        "product" => {
                            if r#product.is_some() {
                                return Err(serde::de::Error::duplicate_field("product"));
                            }
                            r#product = Some(map_access.next_value()?);
                        }
                        "eye" => {
                            let some = r#eye.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("eye"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_eye" => {
                            let some = r#eye.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_eye"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "sphere" => {
                            let some = r#sphere.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("sphere"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_sphere" => {
                            let some = r#sphere.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_sphere"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "cylinder" => {
                            let some = r#cylinder.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("cylinder"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_cylinder" => {
                            let some = r#cylinder.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_cylinder"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "axis" => {
                            let some = r#axis.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("axis"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_axis" => {
                            let some = r#axis.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_axis"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "prism" => {
                            if r#prism.is_some() {
                                return Err(serde::de::Error::duplicate_field("prism"));
                            }
                            r#prism = Some(map_access.next_value()?);
                        }
                        "add" => {
                            let some = r#add.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("add"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_add" => {
                            let some = r#add.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_add"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "power" => {
                            let some = r#power.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("power"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_power" => {
                            let some = r#power.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_power"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "backCurve" => {
                            let some = r#back_curve.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("backCurve"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_backCurve" => {
                            let some = r#back_curve.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_backCurve"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "diameter" => {
                            let some = r#diameter.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("diameter"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_diameter" => {
                            let some = r#diameter.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_diameter"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "duration" => {
                            if r#duration.is_some() {
                                return Err(serde::de::Error::duplicate_field("duration"));
                            }
                            r#duration = Some(map_access.next_value()?);
                        }
                        "color" => {
                            let some = r#color.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("color"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_color" => {
                            let some = r#color.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_color"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "brand" => {
                            let some = r#brand.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("brand"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_brand" => {
                            let some = r#brand.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_brand"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "product",
                                    "eye",
                                    "sphere",
                                    "cylinder",
                                    "axis",
                                    "prism",
                                    "add",
                                    "power",
                                    "back_curve",
                                    "diameter",
                                    "duration",
                                    "color",
                                    "brand",
                                    "note",
                                ],
                            ))
                        }
                    }
                }
                Ok(VisionPrescriptionLensSpecification {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#product: r#product.ok_or(serde::de::Error::missing_field("product"))?,
                    r#eye: r#eye.ok_or(serde::de::Error::missing_field("eye"))?,
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
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct VisionPrescription {
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
    pub r#created: super::super::types::DateTime,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#date_written: super::super::types::DateTime,
    pub r#prescriber: Box<super::super::types::Reference>,
    pub r#lens_specification: Vec<VisionPrescriptionLensSpecification>,
}
impl serde::ser::Serialize for VisionPrescription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "VisionPrescription")?;
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
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
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#status.id,
                extension: &self.r#status.extension,
            };
            state.serialize_entry("_status", &primitive_element)?;
        }
        if let Some(some) = self.r#created.value.as_ref() {
            state.serialize_entry("created", some)?;
        }
        if self.r#created.id.is_some() || !self.r#created.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#created.id,
                extension: &self.r#created.extension,
            };
            state.serialize_entry("_created", &primitive_element)?;
        }
        state.serialize_entry("patient", &self.r#patient)?;
        if let Some(some) = self.r#encounter.as_ref() {
            state.serialize_entry("encounter", some)?;
        }
        if let Some(some) = self.r#date_written.value.as_ref() {
            state.serialize_entry("dateWritten", some)?;
        }
        if self.r#date_written.id.is_some() || !self.r#date_written.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#date_written.id,
                extension: &self.r#date_written.extension,
            };
            state.serialize_entry("_dateWritten", &primitive_element)?;
        }
        state.serialize_entry("prescriber", &self.r#prescriber)?;
        if !self.r#lens_specification.is_empty() {
            state.serialize_entry("lensSpecification", &self.r#lens_specification)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for VisionPrescription {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = VisionPrescription;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("VisionPrescription")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<VisionPrescription, V::Error>
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
                let mut r#created: Option<super::super::types::DateTime> = None;
                let mut r#patient: Option<Box<super::super::types::Reference>> = None;
                let mut r#encounter: Option<Box<super::super::types::Reference>> = None;
                let mut r#date_written: Option<super::super::types::DateTime> = None;
                let mut r#prescriber: Option<Box<super::super::types::Reference>> = None;
                let mut r#lens_specification: Option<Vec<VisionPrescriptionLensSpecification>> =
                    None;
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
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_implicitRules"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
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
                        "created" => {
                            let some = r#created.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("created"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_created" => {
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
                        "patient" => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(map_access.next_value()?);
                        }
                        "encounter" => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(map_access.next_value()?);
                        }
                        "dateWritten" => {
                            let some = r#date_written.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateWritten"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_dateWritten" => {
                            let some = r#date_written.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_dateWritten"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "prescriber" => {
                            if r#prescriber.is_some() {
                                return Err(serde::de::Error::duplicate_field("prescriber"));
                            }
                            r#prescriber = Some(map_access.next_value()?);
                        }
                        "lensSpecification" => {
                            if r#lens_specification.is_some() {
                                return Err(serde::de::Error::duplicate_field("lensSpecification"));
                            }
                            r#lens_specification = Some(map_access.next_value()?);
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
                                    "created",
                                    "patient",
                                    "encounter",
                                    "date_written",
                                    "prescriber",
                                    "lens_specification",
                                ],
                            ))
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
                    r#status: r#status.ok_or(serde::de::Error::missing_field("status"))?,
                    r#created: r#created.ok_or(serde::de::Error::missing_field("created"))?,
                    r#patient: r#patient.ok_or(serde::de::Error::missing_field("patient"))?,
                    r#encounter,
                    r#date_written: r#date_written
                        .ok_or(serde::de::Error::missing_field("date_written"))?,
                    r#prescriber: r#prescriber
                        .ok_or(serde::de::Error::missing_field("prescriber"))?,
                    r#lens_specification: r#lens_specification.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
