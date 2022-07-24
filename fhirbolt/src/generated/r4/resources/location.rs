// Generated on 2022-07-24 by fhirbolt-codegen v0.1.0
#[derive(Default, Debug, Clone)]
pub struct LocationPosition {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#longitude: super::super::types::Decimal,
    pub r#latitude: super::super::types::Decimal,
    pub r#altitude: Option<super::super::types::Decimal>,
}
impl serde::ser::Serialize for LocationPosition {
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
        if let Some(some) = self.r#longitude.value.as_ref() {
            state.serialize_entry("longitude", some)?;
        }
        if self.r#longitude.id.is_some() || !self.r#longitude.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#longitude.id,
                extension: &self.r#longitude.extension,
            };
            state.serialize_entry("_longitude", &primitive_element)?;
        }
        if let Some(some) = self.r#latitude.value.as_ref() {
            state.serialize_entry("latitude", some)?;
        }
        if self.r#latitude.id.is_some() || !self.r#latitude.extension.is_empty() {
            let primitive_element = super::super::serde_helpers::PrimitiveElement {
                id: &self.r#latitude.id,
                extension: &self.r#latitude.extension,
            };
            state.serialize_entry("_latitude", &primitive_element)?;
        }
        if let Some(some) = self.r#altitude.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("altitude", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_altitude", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for LocationPosition {
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
            #[serde(rename = "longitude")]
            Longitude,
            #[serde(rename = "_longitude")]
            LongitudePrimitiveElement,
            #[serde(rename = "latitude")]
            Latitude,
            #[serde(rename = "_latitude")]
            LatitudePrimitiveElement,
            #[serde(rename = "altitude")]
            Altitude,
            #[serde(rename = "_altitude")]
            AltitudePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocationPosition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("LocationPosition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<LocationPosition, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#longitude: Option<super::super::types::Decimal> = None;
                let mut r#latitude: Option<super::super::types::Decimal> = None;
                let mut r#altitude: Option<super::super::types::Decimal> = None;
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
                        Field::Longitude => {
                            let some = r#longitude.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("longitude"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LongitudePrimitiveElement => {
                            let some = r#longitude.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_longitude"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Latitude => {
                            let some = r#latitude.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("latitude"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LatitudePrimitiveElement => {
                            let some = r#latitude.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_latitude"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Altitude => {
                            let some = r#altitude.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("altitude"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AltitudePrimitiveElement => {
                            let some = r#altitude.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_altitude"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(LocationPosition {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#longitude: r#longitude.ok_or(serde::de::Error::missing_field("longitude"))?,
                    r#latitude: r#latitude.ok_or(serde::de::Error::missing_field("latitude"))?,
                    r#altitude,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct LocationHoursOfOperation {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#days_of_week: Vec<super::super::types::Code>,
    pub r#all_day: Option<super::super::types::Boolean>,
    pub r#opening_time: Option<super::super::types::Time>,
    pub r#closing_time: Option<super::super::types::Time>,
}
impl serde::ser::Serialize for LocationHoursOfOperation {
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
        if !self.r#days_of_week.is_empty() {
            let values: Vec<_> = self.r#days_of_week.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("daysOfWeek", &values)?;
            }
            let requires_elements = self
                .r#days_of_week
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#days_of_week
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
                state.serialize_entry("_daysOfWeek", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#all_day.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("allDay", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_allDay", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#opening_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("openingTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_openingTime", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#closing_time.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("closingTime", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_closingTime", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for LocationHoursOfOperation {
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
            #[serde(rename = "daysOfWeek")]
            DaysOfWeek,
            #[serde(rename = "_daysOfWeek")]
            DaysOfWeekPrimitiveElement,
            #[serde(rename = "allDay")]
            AllDay,
            #[serde(rename = "_allDay")]
            AllDayPrimitiveElement,
            #[serde(rename = "openingTime")]
            OpeningTime,
            #[serde(rename = "_openingTime")]
            OpeningTimePrimitiveElement,
            #[serde(rename = "closingTime")]
            ClosingTime,
            #[serde(rename = "_closingTime")]
            ClosingTimePrimitiveElement,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = LocationHoursOfOperation;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("LocationHoursOfOperation")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<LocationHoursOfOperation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#days_of_week: Option<Vec<super::super::types::Code>> = None;
                let mut r#all_day: Option<super::super::types::Boolean> = None;
                let mut r#opening_time: Option<super::super::types::Time> = None;
                let mut r#closing_time: Option<super::super::types::Time> = None;
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
                        Field::DaysOfWeek => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#days_of_week.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("daysOfWeek"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        Field::DaysOfWeekPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
                            let vec = r#days_of_week.get_or_insert(
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
                                return Err(serde::de::Error::duplicate_field("_daysOfWeek"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                if let Some(element) = element {
                                    vec[i].id = element.id;
                                    vec[i].extension = element.extension;
                                }
                            }
                        }
                        Field::AllDay => {
                            let some = r#all_day.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("allDay"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AllDayPrimitiveElement => {
                            let some = r#all_day.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_allDay"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::OpeningTime => {
                            let some = r#opening_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("openingTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::OpeningTimePrimitiveElement => {
                            let some = r#opening_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_openingTime"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::ClosingTime => {
                            let some = r#closing_time.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("closingTime"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ClosingTimePrimitiveElement => {
                            let some = r#closing_time.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_closingTime"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                    }
                }
                Ok(LocationHoursOfOperation {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#days_of_week: r#days_of_week.unwrap_or(vec![]),
                    r#all_day,
                    r#opening_time,
                    r#closing_time,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct Location {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#operational_status: Option<Box<super::super::types::Coding>>,
    pub r#name: Option<super::super::types::String>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#mode: Option<super::super::types::Code>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#address: Option<Box<super::super::types::Address>>,
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#position: Option<LocationPosition>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    pub r#hours_of_operation: Vec<LocationHoursOfOperation>,
    pub r#availability_exceptions: Option<super::super::types::String>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
}
impl serde::ser::Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "Location")?;
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
        if let Some(some) = self.r#status.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("status", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_status", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#operational_status.as_ref() {
            state.serialize_entry("operationalStatus", some)?;
        }
        if let Some(some) = self.r#name.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("name", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_name", &primitive_element)?;
            }
        }
        if !self.r#alias.is_empty() {
            let values: Vec<_> = self.r#alias.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("alias", &values)?;
            }
            let requires_elements = self
                .r#alias
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                let primitive_elements: Vec<_> = self
                    .r#alias
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
                state.serialize_entry("_alias", &primitive_elements)?;
            }
        }
        if let Some(some) = self.r#description.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("description", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_description", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#mode.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("mode", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_mode", &primitive_element)?;
            }
        }
        if !self.r#type.is_empty() {
            state.serialize_entry("type", &self.r#type)?;
        }
        if !self.r#telecom.is_empty() {
            state.serialize_entry("telecom", &self.r#telecom)?;
        }
        if let Some(some) = self.r#address.as_ref() {
            state.serialize_entry("address", some)?;
        }
        if let Some(some) = self.r#physical_type.as_ref() {
            state.serialize_entry("physicalType", some)?;
        }
        if let Some(some) = self.r#position.as_ref() {
            state.serialize_entry("position", some)?;
        }
        if let Some(some) = self.r#managing_organization.as_ref() {
            state.serialize_entry("managingOrganization", some)?;
        }
        if let Some(some) = self.r#part_of.as_ref() {
            state.serialize_entry("partOf", some)?;
        }
        if !self.r#hours_of_operation.is_empty() {
            state.serialize_entry("hoursOfOperation", &self.r#hours_of_operation)?;
        }
        if let Some(some) = self.r#availability_exceptions.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("availabilityExceptions", some)?;
            }
            if some.id.is_some() || !some.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: &some.id,
                    extension: &some.extension,
                };
                state.serialize_entry("_availabilityExceptions", &primitive_element)?;
            }
        }
        if !self.r#endpoint.is_empty() {
            state.serialize_entry("endpoint", &self.r#endpoint)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
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
            #[serde(rename = "status")]
            Status,
            #[serde(rename = "_status")]
            StatusPrimitiveElement,
            #[serde(rename = "operationalStatus")]
            OperationalStatus,
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
            #[serde(rename = "mode")]
            Mode,
            #[serde(rename = "_mode")]
            ModePrimitiveElement,
            #[serde(rename = "type")]
            Type,
            #[serde(rename = "telecom")]
            Telecom,
            #[serde(rename = "address")]
            Address,
            #[serde(rename = "physicalType")]
            PhysicalType,
            #[serde(rename = "position")]
            Position,
            #[serde(rename = "managingOrganization")]
            ManagingOrganization,
            #[serde(rename = "partOf")]
            PartOf,
            #[serde(rename = "hoursOfOperation")]
            HoursOfOperation,
            #[serde(rename = "availabilityExceptions")]
            AvailabilityExceptions,
            #[serde(rename = "_availabilityExceptions")]
            AvailabilityExceptionsPrimitiveElement,
            #[serde(rename = "endpoint")]
            Endpoint,
        }
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Location;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Location")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<Location, V::Error>
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
                let mut r#operational_status: Option<Box<super::super::types::Coding>> = None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#alias: Option<Vec<super::super::types::String>> = None;
                let mut r#description: Option<super::super::types::String> = None;
                let mut r#mode: Option<super::super::types::Code> = None;
                let mut r#type: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#telecom: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#address: Option<Box<super::super::types::Address>> = None;
                let mut r#physical_type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#position: Option<LocationPosition> = None;
                let mut r#managing_organization: Option<Box<super::super::types::Reference>> = None;
                let mut r#part_of: Option<Box<super::super::types::Reference>> = None;
                let mut r#hours_of_operation: Option<Vec<LocationHoursOfOperation>> = None;
                let mut r#availability_exceptions: Option<super::super::types::String> = None;
                let mut r#endpoint: Option<Vec<Box<super::super::types::Reference>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Location" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Location",
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
                            r#meta = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRules => {
                            let some = r#implicit_rules.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("implicitRules"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ImplicitRulesPrimitiveElement => {
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
                        Field::Language => {
                            let some = r#language.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("language"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::LanguagePrimitiveElement => {
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
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(map_access.next_value()?);
                        }
                        Field::Contained => {
                            if r#contained.is_some() {
                                return Err(serde::de::Error::duplicate_field("contained"));
                            }
                            r#contained = Some(map_access.next_value()?);
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
                        Field::Identifier => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        Field::Status => {
                            let some = r#status.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::StatusPrimitiveElement => {
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
                        Field::OperationalStatus => {
                            if r#operational_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationalStatus"));
                            }
                            r#operational_status = Some(map_access.next_value()?);
                        }
                        Field::Name => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::NamePrimitiveElement => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Alias => {
                            let values: Vec<_> = map_access.next_value()?;
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
                                vec[i].value = value;
                            }
                        }
                        Field::AliasPrimitiveElement => {
                            let elements: Vec<
                                Option<super::super::serde_helpers::PrimitiveElementOwned>,
                            > = map_access.next_value()?;
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
                        }
                        Field::Description => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::DescriptionPrimitiveElement => {
                            let some = r#description.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_description"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Mode => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::ModePrimitiveElement => {
                            let some = r#mode.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_mode"));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        Field::Telecom => {
                            if r#telecom.is_some() {
                                return Err(serde::de::Error::duplicate_field("telecom"));
                            }
                            r#telecom = Some(map_access.next_value()?);
                        }
                        Field::Address => {
                            if r#address.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            r#address = Some(map_access.next_value()?);
                        }
                        Field::PhysicalType => {
                            if r#physical_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("physicalType"));
                            }
                            r#physical_type = Some(map_access.next_value()?);
                        }
                        Field::Position => {
                            if r#position.is_some() {
                                return Err(serde::de::Error::duplicate_field("position"));
                            }
                            r#position = Some(map_access.next_value()?);
                        }
                        Field::ManagingOrganization => {
                            if r#managing_organization.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "managingOrganization",
                                ));
                            }
                            r#managing_organization = Some(map_access.next_value()?);
                        }
                        Field::PartOf => {
                            if r#part_of.is_some() {
                                return Err(serde::de::Error::duplicate_field("partOf"));
                            }
                            r#part_of = Some(map_access.next_value()?);
                        }
                        Field::HoursOfOperation => {
                            if r#hours_of_operation.is_some() {
                                return Err(serde::de::Error::duplicate_field("hoursOfOperation"));
                            }
                            r#hours_of_operation = Some(map_access.next_value()?);
                        }
                        Field::AvailabilityExceptions => {
                            let some = r#availability_exceptions.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "availabilityExceptions",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        Field::AvailabilityExceptionsPrimitiveElement => {
                            let some = r#availability_exceptions.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_availabilityExceptions",
                                ));
                            }
                            let super::super::serde_helpers::PrimitiveElementOwned {
                                id,
                                extension,
                            } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        Field::Endpoint => {
                            if r#endpoint.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpoint"));
                            }
                            r#endpoint = Some(map_access.next_value()?);
                        }
                    }
                }
                Ok(Location {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#status,
                    r#operational_status,
                    r#name,
                    r#alias: r#alias.unwrap_or(vec![]),
                    r#description,
                    r#mode,
                    r#type: r#type.unwrap_or(vec![]),
                    r#telecom: r#telecom.unwrap_or(vec![]),
                    r#address,
                    r#physical_type,
                    r#position,
                    r#managing_organization,
                    r#part_of,
                    r#hours_of_operation: r#hours_of_operation.unwrap_or(vec![]),
                    r#availability_exceptions,
                    r#endpoint: r#endpoint.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
