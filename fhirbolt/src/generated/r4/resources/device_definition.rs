// Generated on 2022-07-13 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DeviceDefinitionManufacturer {
    String(Box<super::super::types::String>),
    Reference(Box<super::super::types::Reference>),
}
impl Default for DeviceDefinitionManufacturer {
    fn default() -> DeviceDefinitionManufacturer {
        unimplemented!()
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#device_identifier: super::super::types::String,
    pub r#issuer: super::super::types::Uri,
    pub r#jurisdiction: super::super::types::Uri,
}
impl serde::ser::Serialize for DeviceDefinitionUdiDeviceIdentifier {
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
        if let Some(some) = self.r#device_identifier.value.as_ref() {
            state.serialize_entry("deviceIdentifier", some)?;
        }
        if self.r#device_identifier.id.is_some() || !self.r#device_identifier.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#device_identifier.id,
                extension: &self.r#device_identifier.extension,
            };
            state.serialize_entry("_deviceIdentifier", &primitive_element)?;
        }
        if let Some(some) = self.r#issuer.value.as_ref() {
            state.serialize_entry("issuer", some)?;
        }
        if self.r#issuer.id.is_some() || !self.r#issuer.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#issuer.id,
                extension: &self.r#issuer.extension,
            };
            state.serialize_entry("_issuer", &primitive_element)?;
        }
        if let Some(some) = self.r#jurisdiction.value.as_ref() {
            state.serialize_entry("jurisdiction", some)?;
        }
        if self.r#jurisdiction.id.is_some() || !self.r#jurisdiction.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#jurisdiction.id,
                extension: &self.r#jurisdiction.extension,
            };
            state.serialize_entry("_jurisdiction", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionUdiDeviceIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionUdiDeviceIdentifier;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionUdiDeviceIdentifier")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<DeviceDefinitionUdiDeviceIdentifier, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#device_identifier: Option<super::super::types::String> = None;
                let mut r#issuer: Option<super::super::types::Uri> = None;
                let mut r#jurisdiction: Option<super::super::types::Uri> = None;
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
                        "deviceIdentifier" => {
                            let some = r#device_identifier.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceIdentifier"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_deviceIdentifier" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#device_identifier.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_deviceIdentifier"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "issuer" => {
                            let some = r#issuer.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_issuer" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#issuer.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_issuer"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "jurisdiction" => {
                            let some = r#jurisdiction.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_jurisdiction" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#jurisdiction.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_jurisdiction"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "device_identifier",
                                    "issuer",
                                    "jurisdiction",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionUdiDeviceIdentifier {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#device_identifier: r#device_identifier
                        .ok_or(serde::de::Error::missing_field("device_identifier"))?,
                    r#issuer: r#issuer.ok_or(serde::de::Error::missing_field("issuer"))?,
                    r#jurisdiction: r#jurisdiction
                        .ok_or(serde::de::Error::missing_field("jurisdiction"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionDeviceName {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#type: super::super::types::Code,
}
impl serde::ser::Serialize for DeviceDefinitionDeviceName {
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
        if let Some(some) = self.r#name.value.as_ref() {
            state.serialize_entry("name", some)?;
        }
        if self.r#name.id.is_some() || !self.r#name.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#name.id,
                extension: &self.r#name.extension,
            };
            state.serialize_entry("_name", &primitive_element)?;
        }
        if let Some(some) = self.r#type.value.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if self.r#type.id.is_some() || !self.r#type.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#type.id,
                extension: &self.r#type.extension,
            };
            state.serialize_entry("_type", &primitive_element)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionDeviceName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionDeviceName;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionDeviceName")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionDeviceName, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#name: Option<super::super::types::String> = None;
                let mut r#type: Option<super::super::types::Code> = None;
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
                        "name" => {
                            let some = r#name.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_name" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#name.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_name"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            let some = r#type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_type" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &["id", "extension", "modifier_extension", "name", "type"],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionDeviceName {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#name: r#name.ok_or(serde::de::Error::missing_field("name"))?,
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionSpecialization {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system_type: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
}
impl serde::ser::Serialize for DeviceDefinitionSpecialization {
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
        if let Some(some) = self.r#system_type.value.as_ref() {
            state.serialize_entry("systemType", some)?;
        }
        if self.r#system_type.id.is_some() || !self.r#system_type.extension.is_empty() {
            #[derive(serde :: Serialize)]
            struct PrimtiveElement<'a> {
                #[serde(skip_serializing_if = "Option::is_none")]
                id: &'a Option<std::string::String>,
                #[serde(skip_serializing_if = "<[_]>::is_empty")]
                extension: &'a [Box<super::super::types::Extension>],
            }
            let primitive_element = PrimtiveElement {
                id: &self.r#system_type.id,
                extension: &self.r#system_type.extension,
            };
            state.serialize_entry("_systemType", &primitive_element)?;
        }
        if let Some(some) = self.r#version.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("version", some)?;
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
                state.serialize_entry("_version", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionSpecialization {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionSpecialization;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionSpecialization")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<DeviceDefinitionSpecialization, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#system_type: Option<super::super::types::String> = None;
                let mut r#version: Option<super::super::types::String> = None;
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
                        "systemType" => {
                            let some = r#system_type.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemType"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_systemType" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#system_type.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_systemType"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "version" => {
                            let some = r#version.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_version" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#version.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "system_type",
                                    "version",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionSpecialization {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#system_type: r#system_type
                        .ok_or(serde::de::Error::missing_field("system_type"))?,
                    r#version,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionCapability {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#description: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for DeviceDefinitionCapability {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#description.is_empty() {
            state.serialize_entry("description", &self.r#description)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionCapability;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionCapability")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionCapability, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#description: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "description" => {
                            if r#description.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            r#description = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "description",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionCapability {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#description: r#description.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionProperty {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#value_quantity: Vec<Box<super::super::types::Quantity>>,
    pub r#value_code: Vec<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for DeviceDefinitionProperty {
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
        state.serialize_entry("type", &self.r#type)?;
        if !self.r#value_quantity.is_empty() {
            state.serialize_entry("valueQuantity", &self.r#value_quantity)?;
        }
        if !self.r#value_code.is_empty() {
            state.serialize_entry("valueCode", &self.r#value_code)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionProperty")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionProperty, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#value_quantity: Option<Vec<Box<super::super::types::Quantity>>> = None;
                let mut r#value_code: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
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
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "valueQuantity" => {
                            if r#value_quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value_quantity = Some(map_access.next_value()?);
                        }
                        "valueCode" => {
                            if r#value_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueCode"));
                            }
                            r#value_code = Some(map_access.next_value()?);
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "type",
                                    "value_quantity",
                                    "value_code",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionProperty {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: r#type.ok_or(serde::de::Error::missing_field("type"))?,
                    r#value_quantity: r#value_quantity.unwrap_or(vec![]),
                    r#value_code: r#value_code.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinitionMaterial {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: Box<super::super::types::CodeableConcept>,
    pub r#alternate: Option<super::super::types::Boolean>,
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
}
impl serde::ser::Serialize for DeviceDefinitionMaterial {
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
        state.serialize_entry("substance", &self.r#substance)?;
        if let Some(some) = self.r#alternate.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("alternate", some)?;
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
                state.serialize_entry("_alternate", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#allergenic_indicator.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("allergenicIndicator", some)?;
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
                state.serialize_entry("_allergenicIndicator", &primitive_element)?;
            }
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinitionMaterial {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinitionMaterial;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinitionMaterial")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinitionMaterial, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<super::super::types::Extension>>> = None;
                let mut r#modifier_extension: Option<Vec<Box<super::super::types::Extension>>> =
                    None;
                let mut r#substance: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#alternate: Option<super::super::types::Boolean> = None;
                let mut r#allergenic_indicator: Option<super::super::types::Boolean> = None;
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
                        "substance" => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some(map_access.next_value()?);
                        }
                        "alternate" => {
                            let some = r#alternate.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("alternate"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_alternate" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#alternate.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_alternate"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "allergenicIndicator" => {
                            let some = r#allergenic_indicator.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "allergenicIndicator",
                                ));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_allergenicIndicator" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#allergenic_indicator.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_allergenicIndicator",
                                ));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                map_access_key,
                                &[
                                    "id",
                                    "extension",
                                    "modifier_extension",
                                    "substance",
                                    "alternate",
                                    "allergenic_indicator",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinitionMaterial {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#substance: r#substance.ok_or(serde::de::Error::missing_field("substance"))?,
                    r#alternate,
                    r#allergenic_indicator,
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
#[derive(Default, Debug, Clone)]
pub struct DeviceDefinition {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#udi_device_identifier: Vec<DeviceDefinitionUdiDeviceIdentifier>,
    pub r#manufacturer: Option<DeviceDefinitionManufacturer>,
    pub r#device_name: Vec<DeviceDefinitionDeviceName>,
    pub r#model_number: Option<super::super::types::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#specialization: Vec<DeviceDefinitionSpecialization>,
    pub r#version: Vec<super::super::types::String>,
    pub r#safety: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#shelf_life_storage: Vec<Box<super::super::types::ProductShelfLife>>,
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#language_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#capability: Vec<DeviceDefinitionCapability>,
    pub r#property: Vec<DeviceDefinitionProperty>,
    pub r#owner: Option<Box<super::super::types::Reference>>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#online_information: Option<super::super::types::Uri>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#parent_device: Option<Box<super::super::types::Reference>>,
    pub r#material: Vec<DeviceDefinitionMaterial>,
}
impl serde::ser::Serialize for DeviceDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "DeviceDefinition")?;
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
        if !self.r#identifier.is_empty() {
            state.serialize_entry("identifier", &self.r#identifier)?;
        }
        if !self.r#udi_device_identifier.is_empty() {
            state.serialize_entry("udiDeviceIdentifier", &self.r#udi_device_identifier)?;
        }
        if let Some(some) = self.r#manufacturer.as_ref() {
            match some {
                DeviceDefinitionManufacturer::String(ref value) => {
                    if let Some(some) = value.value.as_ref() {
                        state.serialize_entry("manufacturerString", some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        #[derive(serde :: Serialize)]
                        struct PrimtiveElement<'a> {
                            #[serde(skip_serializing_if = "Option::is_none")]
                            id: &'a Option<std::string::String>,
                            #[serde(skip_serializing_if = "<[_]>::is_empty")]
                            extension: &'a [Box<super::super::types::Extension>],
                        }
                        let primitive_element = PrimtiveElement {
                            id: &value.id,
                            extension: &value.extension,
                        };
                        state.serialize_entry("_manufacturerString", &primitive_element)?;
                    }
                }
                DeviceDefinitionManufacturer::Reference(ref value) => {
                    state.serialize_entry("manufacturerReference", value)?;
                }
            }
        }
        if !self.r#device_name.is_empty() {
            state.serialize_entry("deviceName", &self.r#device_name)?;
        }
        if let Some(some) = self.r#model_number.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("modelNumber", some)?;
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
                state.serialize_entry("_modelNumber", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#type.as_ref() {
            state.serialize_entry("type", some)?;
        }
        if !self.r#specialization.is_empty() {
            state.serialize_entry("specialization", &self.r#specialization)?;
        }
        if !self.r#version.is_empty() {
            let values: Vec<_> = self.r#version.iter().map(|v| &v.value).collect();
            if values.iter().any(|v| v.is_some()) {
                state.serialize_entry("version", &values)?;
            }
            let requires_elements = self
                .r#version
                .iter()
                .any(|e| e.id.is_some() || !e.extension.is_empty());
            if requires_elements {
                #[derive(serde :: Serialize)]
                struct PrimtiveElement<'a> {
                    #[serde(skip_serializing_if = "Option::is_none")]
                    id: &'a Option<std::string::String>,
                    #[serde(skip_serializing_if = "<[_]>::is_empty")]
                    extension: &'a [Box<super::super::types::Extension>],
                }
                let primitive_elements: Vec<_> = self
                    .r#version
                    .iter()
                    .map(|e| {
                        if e.id.is_some() || !e.extension.is_empty() {
                            Some(PrimtiveElement {
                                id: &e.id,
                                extension: &e.extension,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                state.serialize_entry("_version", &primitive_elements)?;
            }
        }
        if !self.r#safety.is_empty() {
            state.serialize_entry("safety", &self.r#safety)?;
        }
        if !self.r#shelf_life_storage.is_empty() {
            state.serialize_entry("shelfLifeStorage", &self.r#shelf_life_storage)?;
        }
        if let Some(some) = self.r#physical_characteristics.as_ref() {
            state.serialize_entry("physicalCharacteristics", some)?;
        }
        if !self.r#language_code.is_empty() {
            state.serialize_entry("languageCode", &self.r#language_code)?;
        }
        if !self.r#capability.is_empty() {
            state.serialize_entry("capability", &self.r#capability)?;
        }
        if !self.r#property.is_empty() {
            state.serialize_entry("property", &self.r#property)?;
        }
        if let Some(some) = self.r#owner.as_ref() {
            state.serialize_entry("owner", some)?;
        }
        if !self.r#contact.is_empty() {
            state.serialize_entry("contact", &self.r#contact)?;
        }
        if let Some(some) = self.r#url.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("url", some)?;
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
                state.serialize_entry("_url", &primitive_element)?;
            }
        }
        if let Some(some) = self.r#online_information.as_ref() {
            if let Some(some) = some.value.as_ref() {
                state.serialize_entry("onlineInformation", some)?;
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
                state.serialize_entry("_onlineInformation", &primitive_element)?;
            }
        }
        if !self.r#note.is_empty() {
            state.serialize_entry("note", &self.r#note)?;
        }
        if let Some(some) = self.r#quantity.as_ref() {
            state.serialize_entry("quantity", some)?;
        }
        if let Some(some) = self.r#parent_device.as_ref() {
            state.serialize_entry("parentDevice", some)?;
        }
        if !self.r#material.is_empty() {
            state.serialize_entry("material", &self.r#material)?;
        }
        state.end()
    }
}
impl<'de> serde::de::Deserialize<'de> for DeviceDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = DeviceDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("DeviceDefinition")
            }
            fn visit_map<V>(self, mut map_access: V) -> Result<DeviceDefinition, V::Error>
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
                let mut r#udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>> =
                    None;
                let mut r#manufacturer: Option<DeviceDefinitionManufacturer> = None;
                let mut r#device_name: Option<Vec<DeviceDefinitionDeviceName>> = None;
                let mut r#model_number: Option<super::super::types::String> = None;
                let mut r#type: Option<Box<super::super::types::CodeableConcept>> = None;
                let mut r#specialization: Option<Vec<DeviceDefinitionSpecialization>> = None;
                let mut r#version: Option<Vec<super::super::types::String>> = None;
                let mut r#safety: Option<Vec<Box<super::super::types::CodeableConcept>>> = None;
                let mut r#shelf_life_storage: Option<
                    Vec<Box<super::super::types::ProductShelfLife>>,
                > = None;
                let mut r#physical_characteristics: Option<
                    Box<super::super::types::ProdCharacteristic>,
                > = None;
                let mut r#language_code: Option<Vec<Box<super::super::types::CodeableConcept>>> =
                    None;
                let mut r#capability: Option<Vec<DeviceDefinitionCapability>> = None;
                let mut r#property: Option<Vec<DeviceDefinitionProperty>> = None;
                let mut r#owner: Option<Box<super::super::types::Reference>> = None;
                let mut r#contact: Option<Vec<Box<super::super::types::ContactPoint>>> = None;
                let mut r#url: Option<super::super::types::Uri> = None;
                let mut r#online_information: Option<super::super::types::Uri> = None;
                let mut r#note: Option<Vec<Box<super::super::types::Annotation>>> = None;
                let mut r#quantity: Option<Box<super::super::types::Quantity>> = None;
                let mut r#parent_device: Option<Box<super::super::types::Reference>> = None;
                let mut r#material: Option<Vec<DeviceDefinitionMaterial>> = None;
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
                        "identifier" => {
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(map_access.next_value()?);
                        }
                        "udiDeviceIdentifier" => {
                            if r#udi_device_identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "udiDeviceIdentifier",
                                ));
                            }
                            r#udi_device_identifier = Some(map_access.next_value()?);
                        }
                        "manufacturerString" => {
                            let r#enum = r#manufacturer.get_or_insert(
                                DeviceDefinitionManufacturer::String(Default::default()),
                            );
                            if let DeviceDefinitionManufacturer::String(variant) = r#enum {
                                if variant.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "manufacturerString",
                                    ));
                                }
                                variant.value = Some(map_access.next_value()?);
                            } else {
                                return Err(serde::de::Error::duplicate_field("manufacturer[x]"));
                            }
                        }
                        "_manufacturerString" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let r#enum = r#manufacturer.get_or_insert(
                                DeviceDefinitionManufacturer::String(Default::default()),
                            );
                            if let DeviceDefinitionManufacturer::String(variant) = r#enum {
                                if variant.id.is_some() || !variant.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_manufacturerString",
                                    ));
                                }
                                let PrimtiveElement { id, extension } = map_access.next_value()?;
                                variant.id = id;
                                variant.extension = extension;
                            } else {
                                return Err(serde::de::Error::duplicate_field("_manufacturer[x]"));
                            }
                        }
                        "manufacturerReference" => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "manufacturerReference",
                                ));
                            }
                            r#manufacturer = Some(DeviceDefinitionManufacturer::Reference(
                                map_access.next_value()?,
                            ));
                        }
                        "deviceName" => {
                            if r#device_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("deviceName"));
                            }
                            r#device_name = Some(map_access.next_value()?);
                        }
                        "modelNumber" => {
                            let some = r#model_number.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("modelNumber"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_modelNumber" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#model_number.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_modelNumber"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "type" => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map_access.next_value()?);
                        }
                        "specialization" => {
                            if r#specialization.is_some() {
                                return Err(serde::de::Error::duplicate_field("specialization"));
                            }
                            r#specialization = Some(map_access.next_value()?);
                        }
                        "version" => {
                            let values: Vec<_> = map_access.next_value()?;
                            let vec = r#version.get_or_insert(Vec::with_capacity(values.len()));
                            if vec.len() != values.len() {
                                return Err(serde::de::Error::invalid_length(
                                    values.len(),
                                    &"primitive elements length",
                                ));
                            }
                            if vec.iter().any(|v| v.value.is_some()) {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            for (i, value) in values.into_iter().enumerate() {
                                vec[i].value = value;
                            }
                        }
                        "_version" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let elements: Vec<PrimtiveElement> = map_access.next_value()?;
                            let vec = r#version.get_or_insert(Vec::with_capacity(elements.len()));
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
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            for (i, element) in elements.into_iter().enumerate() {
                                vec[i].id = element.id;
                                vec[i].extension = element.extension;
                            }
                        }
                        "safety" => {
                            if r#safety.is_some() {
                                return Err(serde::de::Error::duplicate_field("safety"));
                            }
                            r#safety = Some(map_access.next_value()?);
                        }
                        "shelfLifeStorage" => {
                            if r#shelf_life_storage.is_some() {
                                return Err(serde::de::Error::duplicate_field("shelfLifeStorage"));
                            }
                            r#shelf_life_storage = Some(map_access.next_value()?);
                        }
                        "physicalCharacteristics" => {
                            if r#physical_characteristics.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "physicalCharacteristics",
                                ));
                            }
                            r#physical_characteristics = Some(map_access.next_value()?);
                        }
                        "languageCode" => {
                            if r#language_code.is_some() {
                                return Err(serde::de::Error::duplicate_field("languageCode"));
                            }
                            r#language_code = Some(map_access.next_value()?);
                        }
                        "capability" => {
                            if r#capability.is_some() {
                                return Err(serde::de::Error::duplicate_field("capability"));
                            }
                            r#capability = Some(map_access.next_value()?);
                        }
                        "property" => {
                            if r#property.is_some() {
                                return Err(serde::de::Error::duplicate_field("property"));
                            }
                            r#property = Some(map_access.next_value()?);
                        }
                        "owner" => {
                            if r#owner.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            r#owner = Some(map_access.next_value()?);
                        }
                        "contact" => {
                            if r#contact.is_some() {
                                return Err(serde::de::Error::duplicate_field("contact"));
                            }
                            r#contact = Some(map_access.next_value()?);
                        }
                        "url" => {
                            let some = r#url.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_url" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#url.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field("_url"));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "onlineInformation" => {
                            let some = r#online_information.get_or_insert(Default::default());
                            if some.value.is_some() {
                                return Err(serde::de::Error::duplicate_field("onlineInformation"));
                            }
                            some.value = Some(map_access.next_value()?);
                        }
                        "_onlineInformation" => {
                            #[derive(serde :: Deserialize)]
                            struct PrimtiveElement {
                                id: Option<std::string::String>,
                                extension: Vec<Box<super::super::types::Extension>>,
                            }
                            let some = r#online_information.get_or_insert(Default::default());
                            if some.id.is_some() || !some.extension.is_empty() {
                                return Err(serde::de::Error::duplicate_field(
                                    "_onlineInformation",
                                ));
                            }
                            let PrimtiveElement { id, extension } = map_access.next_value()?;
                            some.id = id;
                            some.extension = extension;
                        }
                        "note" => {
                            if r#note.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            r#note = Some(map_access.next_value()?);
                        }
                        "quantity" => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(map_access.next_value()?);
                        }
                        "parentDevice" => {
                            if r#parent_device.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentDevice"));
                            }
                            r#parent_device = Some(map_access.next_value()?);
                        }
                        "material" => {
                            if r#material.is_some() {
                                return Err(serde::de::Error::duplicate_field("material"));
                            }
                            r#material = Some(map_access.next_value()?);
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
                                    "udi_device_identifier",
                                    "manufacturer",
                                    "device_name",
                                    "model_number",
                                    "type",
                                    "specialization",
                                    "version",
                                    "safety",
                                    "shelf_life_storage",
                                    "physical_characteristics",
                                    "language_code",
                                    "capability",
                                    "property",
                                    "owner",
                                    "contact",
                                    "url",
                                    "online_information",
                                    "note",
                                    "quantity",
                                    "parent_device",
                                    "material",
                                ],
                            ))
                        }
                    }
                }
                Ok(DeviceDefinition {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#udi_device_identifier: r#udi_device_identifier.unwrap_or(vec![]),
                    r#manufacturer,
                    r#device_name: r#device_name.unwrap_or(vec![]),
                    r#model_number,
                    r#type,
                    r#specialization: r#specialization.unwrap_or(vec![]),
                    r#version: r#version.unwrap_or(vec![]),
                    r#safety: r#safety.unwrap_or(vec![]),
                    r#shelf_life_storage: r#shelf_life_storage.unwrap_or(vec![]),
                    r#physical_characteristics,
                    r#language_code: r#language_code.unwrap_or(vec![]),
                    r#capability: r#capability.unwrap_or(vec![]),
                    r#property: r#property.unwrap_or(vec![]),
                    r#owner,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#url,
                    r#online_information,
                    r#note: r#note.unwrap_or(vec![]),
                    r#quantity,
                    r#parent_device,
                    r#material: r#material.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}
