// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic,
    >
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSubjectCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueRange")]
                    ValueRange,
                    #[serde(rename = "exclude")]
                    Exclude,
                    #[serde(rename = "_exclude")]
                    ExcludePrimitiveElement,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "code",
                            "valueReference",
                            "valueCodeableConcept",
                            "valueBoolean",
                            "valueQuantity",
                            "valueRange",
                            "exclude",
                            "period",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristicValue,
                > = None;
                let mut r#exclude: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueRange => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueRange"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristicValue :: Range (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Range > > ()) ?)) ;
                        }
                        Field::Exclude => {
                            if self.0.from_json {
                                let some = r#exclude.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exclude"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#exclude.is_some() {
                                    return Err(serde::de::Error::duplicate_field("exclude"));
                                }
                                r#exclude = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ExcludePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#exclude.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_exclude"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("exclude");
                            }
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
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
                    fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#code: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#code.unwrap_or(Default::default())
                        } else {
                            r#code.ok_or(serde::de::Error::missing_field("code"))?
                        },
                        r#value: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#value.unwrap_or(Default::default())
                        } else {
                            r#value.ok_or(serde::de::Error::missing_field("value[x]"))?
                        },
                        r#exclude,
                        r#period,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristic > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(
                        self.0.transmute::<Box<
                            fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic,
                        >>(),
                    )?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::EvidenceReportSubject,
    >
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReportSubject;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::EvidenceReportSubject,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::EvidenceReportSubject;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSubject")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::EvidenceReportSubject, V::Error>
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
                    #[serde(rename = "characteristic")]
                    Characteristic,
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
                            "characteristic",
                            "note",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#characteristic: Option<
                    Vec<fhirbolt_model::r4b::resources::EvidenceReportSubjectCharacteristic>,
                > = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Characteristic => {
                            if self.0.from_json {
                                if r#characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristic",
                                    ));
                                }
                                r#characteristic = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristic >> ()) ?) ;
                            } else {
                                let vec = r#characteristic.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportSubjectCharacteristic > ()) ?) ;
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::EvidenceReportSubject {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#characteristic: r#characteristic.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::EvidenceReportSubject>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::EvidenceReportSubject>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSubject>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::EvidenceReportSubject>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSubject>;
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
                        .transmute::<fhirbolt_model::r4b::resources::EvidenceReportSubject>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::EvidenceReportSubject>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::EvidenceReportRelatesTo,
    >
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReportRelatesTo;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::EvidenceReportRelatesTo,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::EvidenceReportRelatesTo;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportRelatesTo")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "_code")]
                    CodePrimitiveElement,
                    #[serde(rename = "targetIdentifier")]
                    TargetIdentifier,
                    #[serde(rename = "targetReference")]
                    TargetReference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "code",
                            "targetIdentifier",
                            "targetReference",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#code: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#target: Option<
                    fhirbolt_model::r4b::resources::EvidenceReportRelatesToTarget,
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Code => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("code"));
                                }
                                r#code = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::CodePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#code.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_code"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("code");
                            }
                        }
                        Field::TargetIdentifier => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetIdentifier"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportRelatesToTarget :: Identifier (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?)) ;
                        }
                        Field::TargetReference => {
                            if r#target.is_some() {
                                return Err(serde::de::Error::duplicate_field("targetReference"));
                            }
                            r#target = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportRelatesToTarget :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::EvidenceReportRelatesTo {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#code.unwrap_or(Default::default())
                    } else {
                        r#code.ok_or(serde::de::Error::missing_field("code"))?
                    },
                    r#target: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#target.unwrap_or(Default::default())
                    } else {
                        r#target.ok_or(serde::de::Error::missing_field("target[x]"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>;
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
                        .transmute::<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>>(
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::EvidenceReportSection,
    >
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReportSection;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::EvidenceReportSection,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::EvidenceReportSection;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReportSection")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::EvidenceReportSection, V::Error>
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
                    #[serde(rename = "title")]
                    Title,
                    #[serde(rename = "_title")]
                    TitlePrimitiveElement,
                    #[serde(rename = "focus")]
                    Focus,
                    #[serde(rename = "focusReference")]
                    FocusReference,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "mode")]
                    Mode,
                    #[serde(rename = "_mode")]
                    ModePrimitiveElement,
                    #[serde(rename = "orderedBy")]
                    OrderedBy,
                    #[serde(rename = "entryClassifier")]
                    EntryClassifier,
                    #[serde(rename = "entryReference")]
                    EntryReference,
                    #[serde(rename = "entryQuantity")]
                    EntryQuantity,
                    #[serde(rename = "emptyReason")]
                    EmptyReason,
                    #[serde(rename = "section")]
                    Section,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "title",
                            "focus",
                            "focusReference",
                            "author",
                            "text",
                            "mode",
                            "orderedBy",
                            "entryClassifier",
                            "entryReference",
                            "entryQuantity",
                            "emptyReason",
                            "section",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#title: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#focus: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#focus_reference: Option<Box<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#author: Option<Vec<Box<fhirbolt_model::r4b::types::Reference>>> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#mode: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#ordered_by: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#entry_classifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#entry_reference: Option<Vec<Box<fhirbolt_model::r4b::types::Reference>>> =
                    None;
                let mut r#entry_quantity: Option<Vec<Box<fhirbolt_model::r4b::types::Quantity>>> =
                    None;
                let mut r#empty_reason: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#section: Option<
                    Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>,
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Title => {
                            if self.0.from_json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#title.is_some() {
                                    return Err(serde::de::Error::duplicate_field("title"));
                                }
                                r#title = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::TitlePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#title.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_title"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("title");
                            }
                        }
                        Field::Focus => {
                            if r#focus.is_some() {
                                return Err(serde::de::Error::duplicate_field("focus"));
                            }
                            r#focus = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::FocusReference => {
                            if r#focus_reference.is_some() {
                                return Err(serde::de::Error::duplicate_field("focusReference"));
                            }
                            r#focus_reference = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Author => {
                            if self.0.from_json {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#author.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?) ;
                            }
                        }
                        Field::Text => {
                            if r#text.is_some() {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            r#text = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Mode => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#mode.is_some() {
                                    return Err(serde::de::Error::duplicate_field("mode"));
                                }
                                r#mode = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::ModePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#mode.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_mode"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("mode");
                            }
                        }
                        Field::OrderedBy => {
                            if r#ordered_by.is_some() {
                                return Err(serde::de::Error::duplicate_field("orderedBy"));
                            }
                            r#ordered_by = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::EntryClassifier => {
                            if self.0.from_json {
                                if r#entry_classifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entryClassifier",
                                    ));
                                }
                                r#entry_classifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#entry_classifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::EntryReference => {
                            if self.0.from_json {
                                if r#entry_reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "entryReference",
                                    ));
                                }
                                r#entry_reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#entry_reference.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?) ;
                            }
                        }
                        Field::EntryQuantity => {
                            if self.0.from_json {
                                if r#entry_quantity.is_some() {
                                    return Err(serde::de::Error::duplicate_field("entryQuantity"));
                                }
                                r#entry_quantity = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Quantity > >> ()) ?) ;
                            } else {
                                let vec = r#entry_quantity.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::EmptyReason => {
                            if r#empty_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("emptyReason"));
                            }
                            r#empty_reason = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Section => {
                            if self.0.from_json {
                                if r#section.is_some() {
                                    return Err(serde::de::Error::duplicate_field("section"));
                                }
                                r#section =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4b::resources::EvidenceReportSection,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#section.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportSection > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::EvidenceReportSection {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#title,
                    r#focus,
                    r#focus_reference,
                    r#author: r#author.unwrap_or(vec![]),
                    r#text,
                    r#mode,
                    r#ordered_by,
                    r#entry_classifier: r#entry_classifier.unwrap_or(vec![]),
                    r#entry_reference: r#entry_reference.unwrap_or(vec![]),
                    r#entry_quantity: r#entry_quantity.unwrap_or(vec![]),
                    r#empty_reason,
                    r#section: r#section.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::EvidenceReportSection>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::EvidenceReportSection>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::EvidenceReportSection>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>;
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
                        .transmute::<fhirbolt_model::r4b::resources::EvidenceReportSection>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSection>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSection>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSection>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReportSection>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::EvidenceReportSection>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::EvidenceReport {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r4b::resources::EvidenceReport>
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReport;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::EvidenceReport,
    >
{
    type Value = fhirbolt_model::r4b::resources::EvidenceReport;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::EvidenceReport,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::EvidenceReport;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("EvidenceReport")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::EvidenceReport, V::Error>
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
                    #[serde(rename = "url")]
                    Url,
                    #[serde(rename = "_url")]
                    UrlPrimitiveElement,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "useContext")]
                    UseContext,
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "relatedIdentifier")]
                    RelatedIdentifier,
                    #[serde(rename = "citeAsReference")]
                    CiteAsReference,
                    #[serde(rename = "citeAsMarkdown")]
                    CiteAsMarkdown,
                    #[serde(rename = "_citeAsMarkdown")]
                    CiteAsMarkdownPrimitiveElement,
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "relatedArtifact")]
                    RelatedArtifact,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "publisher")]
                    Publisher,
                    #[serde(rename = "_publisher")]
                    PublisherPrimitiveElement,
                    #[serde(rename = "contact")]
                    Contact,
                    #[serde(rename = "author")]
                    Author,
                    #[serde(rename = "editor")]
                    Editor,
                    #[serde(rename = "reviewer")]
                    Reviewer,
                    #[serde(rename = "endorser")]
                    Endorser,
                    #[serde(rename = "relatesTo")]
                    RelatesTo,
                    #[serde(rename = "section")]
                    Section,
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
                            "url",
                            "status",
                            "useContext",
                            "identifier",
                            "relatedIdentifier",
                            "citeAsReference",
                            "citeAsMarkdown",
                            "type",
                            "note",
                            "relatedArtifact",
                            "subject",
                            "publisher",
                            "contact",
                            "author",
                            "editor",
                            "reviewer",
                            "endorser",
                            "relatesTo",
                            "section",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4b::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4b::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4b::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#url: Option<fhirbolt_model::r4b::types::Uri> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#use_context: Option<Vec<Box<fhirbolt_model::r4b::types::UsageContext>>> =
                    None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#related_identifier: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Identifier>>,
                > = None;
                let mut r#cite_as: Option<fhirbolt_model::r4b::resources::EvidenceReportCiteAs> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4b::types::Annotation>>> = None;
                let mut r#related_artifact: Option<
                    Vec<Box<fhirbolt_model::r4b::types::RelatedArtifact>>,
                > = None;
                let mut r#subject: Option<fhirbolt_model::r4b::resources::EvidenceReportSubject> =
                    None;
                let mut r#publisher: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#contact: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#author: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#editor: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#reviewer: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#endorser: Option<Vec<Box<fhirbolt_model::r4b::types::ContactDetail>>> =
                    None;
                let mut r#relates_to: Option<
                    Vec<fhirbolt_model::r4b::resources::EvidenceReportRelatesTo>,
                > = None;
                let mut r#section: Option<
                    Vec<fhirbolt_model::r4b::resources::EvidenceReportSection>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "EvidenceReport" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"EvidenceReport",
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
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r4b::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<Box<fhirbolt_model::r4b::Resource>>>(),
                                    )?,
                                );
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4b::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ;
                            }
                        }
                        Field::Url => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#url.is_some() {
                                    return Err(serde::de::Error::duplicate_field("url"));
                                }
                                r#url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::UrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_url"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("url");
                            }
                        }
                        Field::Status => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#status.is_some() {
                                    return Err(serde::de::Error::duplicate_field("status"));
                                }
                                r#status = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::StatusPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#status.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_status"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("status");
                            }
                        }
                        Field::UseContext => {
                            if self.0.from_json {
                                if r#use_context.is_some() {
                                    return Err(serde::de::Error::duplicate_field("useContext"));
                                }
                                r#use_context = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: UsageContext > >> ()) ?) ;
                            } else {
                                let vec = r#use_context.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UsageContext > > ()) ?) ;
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::RelatedIdentifier => {
                            if self.0.from_json {
                                if r#related_identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedIdentifier",
                                    ));
                                }
                                r#related_identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#related_identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::CiteAsReference => {
                            if r#cite_as.is_some() {
                                return Err(serde::de::Error::duplicate_field("citeAsReference"));
                            }
                            r#cite_as = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportCiteAs :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::CiteAsMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#cite_as.get_or_insert(
                                    fhirbolt_model::r4b::resources::EvidenceReportCiteAs::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: EvidenceReportCiteAs :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("citeAsMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("citeAs[x]")) ; }
                            } else {
                                if r#cite_as.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "citeAsMarkdown",
                                    ));
                                }
                                r#cite_as = Some (fhirbolt_model :: r4b :: resources :: EvidenceReportCiteAs :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::CiteAsMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#cite_as.get_or_insert(
                                    fhirbolt_model::r4b::resources::EvidenceReportCiteAs::Markdown(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model :: r4b :: resources :: EvidenceReportCiteAs :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_citeAsMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_citeAs[x]")) ; }
                            } else {
                                return unknown_field_error("citeAsMarkdown");
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::RelatedArtifact => {
                            if self.0.from_json {
                                if r#related_artifact.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedArtifact",
                                    ));
                                }
                                r#related_artifact =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::RelatedArtifact>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#related_artifact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: RelatedArtifact > > ()) ?) ;
                            }
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportSubject > ()) ?) ;
                        }
                        Field::Publisher => {
                            if self.0.from_json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#publisher.is_some() {
                                    return Err(serde::de::Error::duplicate_field("publisher"));
                                }
                                r#publisher = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PublisherPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#publisher.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_publisher"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("publisher");
                            }
                        }
                        Field::Contact => {
                            if self.0.from_json {
                                if r#contact.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contact"));
                                }
                                r#contact = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#contact.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Author => {
                            if self.0.from_json {
                                if r#author.is_some() {
                                    return Err(serde::de::Error::duplicate_field("author"));
                                }
                                r#author = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#author.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Editor => {
                            if self.0.from_json {
                                if r#editor.is_some() {
                                    return Err(serde::de::Error::duplicate_field("editor"));
                                }
                                r#editor = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#editor.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Reviewer => {
                            if self.0.from_json {
                                if r#reviewer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reviewer"));
                                }
                                r#reviewer = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#reviewer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::Endorser => {
                            if self.0.from_json {
                                if r#endorser.is_some() {
                                    return Err(serde::de::Error::duplicate_field("endorser"));
                                }
                                r#endorser = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: ContactDetail > >> ()) ?) ;
                            } else {
                                let vec = r#endorser.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: ContactDetail > > ()) ?) ;
                            }
                        }
                        Field::RelatesTo => {
                            if self.0.from_json {
                                if r#relates_to.is_some() {
                                    return Err(serde::de::Error::duplicate_field("relatesTo"));
                                }
                                r#relates_to = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::EvidenceReportRelatesTo,
                                    >>(),
                                )?);
                            } else {
                                let vec = r#relates_to.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportRelatesTo > ()) ?) ;
                            }
                        }
                        Field::Section => {
                            if self.0.from_json {
                                if r#section.is_some() {
                                    return Err(serde::de::Error::duplicate_field("section"));
                                }
                                r#section =
                                    Some(map_access.next_value_seed(
                                        self.0.transmute::<Vec<
                                            fhirbolt_model::r4b::resources::EvidenceReportSection,
                                        >>(),
                                    )?);
                            } else {
                                let vec = r#section.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: EvidenceReportSection > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::EvidenceReport {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#url,
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#use_context: r#use_context.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#related_identifier: r#related_identifier.unwrap_or(vec![]),
                    r#cite_as,
                    r#type,
                    r#note: r#note.unwrap_or(vec![]),
                    r#related_artifact: r#related_artifact.unwrap_or(vec![]),
                    r#subject: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#subject.unwrap_or(Default::default())
                    } else {
                        r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                    },
                    r#publisher,
                    r#contact: r#contact.unwrap_or(vec![]),
                    r#author: r#author.unwrap_or(vec![]),
                    r#editor: r#editor.unwrap_or(vec![]),
                    r#reviewer: r#reviewer.unwrap_or(vec![]),
                    r#endorser: r#endorser.unwrap_or(vec![]),
                    r#relates_to: r#relates_to.unwrap_or(vec![]),
                    r#section: r#section.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::EvidenceReport>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::EvidenceReport>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::EvidenceReport>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::EvidenceReport>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReport>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::EvidenceReport>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::EvidenceReport>;
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
                        .transmute::<fhirbolt_model::r4b::resources::EvidenceReport>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::EvidenceReport>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReport>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::EvidenceReport>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::EvidenceReport>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::EvidenceReport>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
