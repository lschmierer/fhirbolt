// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct MedicinalProductManufactured {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#manufactured_dose_form: Box<super::super::types::CodeableConcept>,
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for MedicinalProductManufactured {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductManufactured")?;
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
        state.serialize_entry("manufacturedDoseForm", &self.r#manufactured_dose_form)?;
        if let Some(some) = self.r#unit_of_presentation.as_ref() {
            state.serialize_entry("unitOfPresentation", some)?;
        }
        state.serialize_entry("quantity", &self.r#quantity)?;
        if !self.r#manufacturer.is_empty() {
            state.serialize_entry("manufacturer", &self.r#manufacturer)?;
        }
        if !self.r#ingredient.is_empty() {
            state.serialize_entry("ingredient", &self.r#ingredient)?;
        }
        if let Some(some) = self.r#physical_characteristics.as_ref() {
            state.serialize_entry("physicalCharacteristics", some)?;
        }
        if !self.r#other_characteristics.is_empty() {
            state.serialize_entry("otherCharacteristics", &self.r#other_characteristics)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for MedicinalProductManufactured {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = MedicinalProductManufactured;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductManufactured")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<MedicinalProductManufactured, V::Error>
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
                let mut r#manufactured_dose_form: Option<
                    Box<super::super::types::CodeableConcept>,
                > = None;
                let mut r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>> =
                    None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#manufacturer: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#ingredient: Option<Vec<Box<super::super::types::Reference>>> = None;
                let mut r#physical_characteristics: Option<
                    Box<super::super::types::ProdCharacteristic>,
                > = None;
                let mut r#other_characteristics: Option<
                    Vec<Box<super::super::types::CodeableConcept>>,
                > = None;
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
                        "manufacturedDoseForm" => {
                            if r#manufactured_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "manufacturedDoseForm",
                                ));
                            }
                            r#manufactured_dose_form = Some(map_access.next_value()?);
                        }
                        "unitOfPresentation" => {
                            if r#unit_of_presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unitOfPresentation",
                                ));
                            }
                            r#unit_of_presentation = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "manufacturer" => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(map_access.next_value()?);
                        }
                        "ingredient" => {
                            if r#ingredient.is_some() {
                                return Err(serde::de::Error::duplicate_field("ingredient"));
                            }
                            r#ingredient = Some(map_access.next_value()?);
                        }
                        "physicalCharacteristics" => {
                            if r#physical_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "physicalCharacteristics",
                                ));
                            }
                            r#physical_characteristics = Some(map_access.next_value()?);
                        }
                        "otherCharacteristics" => {
                            if r#other_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "otherCharacteristics",
                                ));
                            }
                            r#other_characteristics = Some(map_access.next_value()?);
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
                                    "manufactured_dose_form",
                                    "unit_of_presentation",
                                    "quantity",
                                    "manufacturer",
                                    "ingredient",
                                    "physical_characteristics",
                                    "other_characteristics",
                                ],
                            ))
                        }
                    }
                }
                Ok(MedicinalProductManufactured {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#manufactured_dose_form: r#manufactured_dose_form
                        .ok_or(serde::de::Error::missing_field("manufactured_dose_form"))?,
                    r#unit_of_presentation,
                    r#quantity: r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?,
                    r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#physical_characteristics,
                    r#other_characteristics: r#other_characteristics.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
