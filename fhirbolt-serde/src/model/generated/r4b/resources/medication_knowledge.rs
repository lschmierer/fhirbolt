// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRelatedMedicationKnowledge")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "reference")]
                    Reference,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "reference"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#reference: Option<Vec<Box<fhirbolt_model::r4b::types::Reference>>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Reference => {
                            if self.0.from_json {
                                if r#reference.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reference"));
                                }
                                r#reference = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#reference.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?) ;
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#reference: r#reference.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge>,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRelatedMedicationKnowledge,
                >>())? {
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonograph")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "source")]
                    Source,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "source"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#source: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Source => {
                            if r#source.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            r#source = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#source,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>(
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonograph >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeIngredient")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient, V::Error>
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
                    #[serde(rename = "itemCodeableConcept")]
                    ItemCodeableConcept,
                    #[serde(rename = "itemReference")]
                    ItemReference,
                    #[serde(rename = "isActive")]
                    IsActive,
                    #[serde(rename = "_isActive")]
                    IsActivePrimitiveElement,
                    #[serde(rename = "strength")]
                    Strength,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "itemCodeableConcept",
                            "itemReference",
                            "isActive",
                            "strength",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#item: Option<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeIngredientItem,
                > = None;
                let mut r#is_active: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#strength: Option<Box<fhirbolt_model::r4b::types::Ratio>> = None;
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
                        Field::ItemCodeableConcept => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "itemCodeableConcept",
                                ));
                            }
                            r#item = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeIngredientItem :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ItemReference => {
                            if r#item.is_some() {
                                return Err(serde::de::Error::duplicate_field("itemReference"));
                            }
                            r#item = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeIngredientItem :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::IsActive => {
                            if self.0.from_json {
                                let some = r#is_active.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isActive"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#is_active.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isActive"));
                                }
                                r#is_active = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::IsActivePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_active.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isActive"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isActive");
                            }
                        }
                        Field::Strength => {
                            if r#strength.is_some() {
                                return Err(serde::de::Error::duplicate_field("strength"));
                            }
                            r#strength = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Ratio>>(),
                            )?);
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#item: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#item.unwrap_or(Default::default())
                        } else {
                            r#item.ok_or(serde::de::Error::missing_field("item[x]"))?
                        },
                        r#is_active,
                        r#strength,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>(
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeIngredient >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeCost,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeCost;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeCost,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeCost;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeCost")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgeCost, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "source")]
                    Source,
                    #[serde(rename = "_source")]
                    SourcePrimitiveElement,
                    #[serde(rename = "cost")]
                    Cost,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "source",
                            "cost",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#source: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#cost: Option<Box<fhirbolt_model::r4b::types::Money>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Source => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#source.is_some() {
                                    return Err(serde::de::Error::duplicate_field("source"));
                                }
                                r#source = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::SourcePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#source.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_source"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("source");
                            }
                        }
                        Field::Cost => {
                            if r#cost.is_some() {
                                return Err(serde::de::Error::duplicate_field("cost"));
                            }
                            r#cost = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4b::types::Money>>(),
                            )?);
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4b::resources::MedicationKnowledgeCost {
                    r#id,
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#type: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#type.unwrap_or(Default::default())
                    } else {
                        r#type.ok_or(serde::de::Error::missing_field("type"))?
                    },
                    r#source,
                    r#cost: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#cost.unwrap_or(Default::default())
                    } else {
                        r#cost.ok_or(serde::de::Error::missing_field("cost"))?
                    },
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>>(
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMonitoringProgram")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "name"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::String> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Name => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("name"));
                                }
                                r#name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::NamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_name"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("name");
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#name,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonitoringProgram > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>>;
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
                            fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram,
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeAdministrationGuidelinesDosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "dosage")]
                    Dosage,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "dosage"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#dosage: Option<Vec<Box<fhirbolt_model::r4b::types::Dosage>>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Dosage => {
                            if self.0.from_json {
                                if r#dosage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dosage"));
                                }
                                r#dosage = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Dosage > >> ()) ?) ;
                            } else {
                                let vec = r#dosage.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Dosage>>(),
                                    )?,
                                );
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
                Ok (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#dosage : r#dosage . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>,
    >
{
    type Value =
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelinesDosage>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage >> ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > { type Value = fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "characteristicCodeableConcept")] CharacteristicCodeableConcept , # [serde (rename = "characteristicQuantity")] CharacteristicQuantity , # [serde (rename = "value")] Value , # [serde (rename = "_value")] ValuePrimitiveElement , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "characteristicCodeableConcept" , "characteristicQuantity" , "value" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > > > = None ; let mut r#characteristic : Option < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic > = None ; let mut r#value : Option < Vec < fhirbolt_model :: r4b :: types :: String > > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Extension > > ()) ?) ; } } , Field :: CharacteristicCodeableConcept => { if r#characteristic . is_some () { return Err (serde :: de :: Error :: duplicate_field ("characteristicCodeableConcept")) ; } r#characteristic = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ; } , Field :: CharacteristicQuantity => { if r#characteristic . is_some () { return Err (serde :: de :: Error :: duplicate_field ("characteristicQuantity")) ; } r#characteristic = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristicsCharacteristic :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Quantity > > ()) ?)) ; } , Field :: Value => { if self . 0 . from_json { let values : Vec < Option < _ >> = map_access . next_value () ? ; let vec = r#value . get_or_insert (std :: iter :: repeat (Default :: default ()) . take (values . len ()) . collect :: < Vec < _ >> ()) ; if vec . len () != values . len () { return Err (serde :: de :: Error :: invalid_length (values . len () , & "primitive elements length")) ; } if vec . iter () . any (| v | v . value . is_some ()) { return Err (serde :: de :: Error :: duplicate_field ("value")) ; } for (i , value) in values . into_iter () . enumerate () { if let Some (value) = value { vec [i] . value = Some (value) ; } } } else { let vec = r#value . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: types :: String > ()) ?) ; } } , Field :: ValuePrimitiveElement => { if self . 0 . from_json { let elements : Vec < Option < super :: super :: serde_helpers :: PrimitiveElementOwned >> = map_access . next_value_seed (self . 0 . transmute :: < Vec < Option < super :: super :: serde_helpers :: PrimitiveElementOwned >> > ()) ? ; let vec = r#value . get_or_insert (std :: iter :: repeat (Default :: default ()) . take (elements . len ()) . collect :: < Vec < _ >> ()) ; if vec . len () != elements . len () { return Err (serde :: de :: Error :: invalid_length (elements . len () , & "primitive values length")) ; } if vec . iter () . any (| e | e . id . is_some () || ! e . extension . is_empty ()) { return Err (serde :: de :: Error :: duplicate_field ("_value")) ; } for (i , element) in elements . into_iter () . enumerate () { if let Some (element) = element { vec [i] . id = element . id ; vec [i] . extension = element . extension ; } } } else { return unknown_field_error ("value") ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#characteristic : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#characteristic . unwrap_or (Default :: default ()) } else { r#characteristic . ok_or (serde :: de :: Error :: missing_field ("characteristic[x]")) ? } , r#value : r#value . unwrap_or (vec ! []) , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> { type Value = Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> { type Value = Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeAdministrationGuidelines")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
                V::Error,
            >
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
                    #[serde(rename = "dosage")]
                    Dosage,
                    #[serde(rename = "indicationCodeableConcept")]
                    IndicationCodeableConcept,
                    #[serde(rename = "indicationReference")]
                    IndicationReference,
                    #[serde(rename = "patientCharacteristics")]
                    PatientCharacteristics,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "dosage",
                            "indicationCodeableConcept",
                            "indicationReference",
                            "patientCharacteristics",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#dosage : Option < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage > > = None ;
                let mut r#indication : Option < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesIndication > = None ;
                let mut r#patient_characteristics : Option < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > > = None ;
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
                        Field::Dosage => {
                            if self.0.from_json {
                                if r#dosage.is_some() {
                                    return Err(serde::de::Error::duplicate_field("dosage"));
                                }
                                r#dosage = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage >> ()) ?) ;
                            } else {
                                let vec = r#dosage.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesDosage > ()) ?) ;
                            }
                        }
                        Field::IndicationCodeableConcept => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationCodeableConcept",
                                ));
                            }
                            r#indication = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesIndication :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::IndicationReference => {
                            if r#indication.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "indicationReference",
                                ));
                            }
                            r#indication = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesIndication :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?)) ;
                        }
                        Field::PatientCharacteristics => {
                            if self.0.from_json {
                                if r#patient_characteristics.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "patientCharacteristics",
                                    ));
                                }
                                r#patient_characteristics = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics >> ()) ?) ;
                            } else {
                                let vec =
                                    r#patient_characteristics.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelinesPatientCharacteristics > ()) ?) ;
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#dosage: r#dosage.unwrap_or(vec![]),
                        r#indication,
                        r#patient_characteristics: r#patient_characteristics.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelines > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelines > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<
                    Box<
                        fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
                    >,
                >,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines>,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
                >>())? {
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeMedicineClassification")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "classification")]
                    Classification,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "classification",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#classification: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Classification => {
                            if self.0.from_json {
                                if r#classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "classification",
                                    ));
                                }
                                r#classification =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#classification: r#classification.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>(
        )
        .deserialize(deserializer)
        .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMedicineClassification > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification,
                >>())? {
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
        fhirbolt_model::r4b::resources::MedicationKnowledgePackaging,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgePackaging;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgePackaging,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgePackaging;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgePackaging")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "quantity")]
                    Quantity,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "quantity"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgePackaging {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#quantity,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>(
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgePackaging>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgePackaging >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeDrugCharacteristic")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "valueCodeableConcept")]
                    ValueCodeableConcept,
                    #[serde(rename = "valueString")]
                    ValueString,
                    #[serde(rename = "_valueString")]
                    ValueStringPrimitiveElement,
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueBase64Binary")]
                    ValueBase64Binary,
                    #[serde(rename = "_valueBase64Binary")]
                    ValueBase64BinaryPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "type",
                            "valueCodeableConcept",
                            "valueString",
                            "valueQuantity",
                            "valueBase64Binary",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristicValue,
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueString => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueString"));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::ValueStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueString");
                            }
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueBase64Binary => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Base64Binary (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBase64Binary")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "valueBase64Binary",
                                    ));
                                }
                                r#value = Some (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Base64Binary (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Base64Binary > > ()) ?)) ;
                            }
                        }
                        Field::ValueBase64BinaryPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Base64Binary (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristicValue :: Base64Binary (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBase64Binary")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBase64Binary");
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type,
                        r#value,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristic > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>>;
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
                            fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic,
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatorySubstitution")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution,
                V::Error,
            >
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "allowed")]
                    Allowed,
                    #[serde(rename = "_allowed")]
                    AllowedPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "type", "allowed"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#allowed: Option<fhirbolt_model::r4b::types::Boolean> = None;
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Allowed => {
                            if self.0.from_json {
                                let some = r#allowed.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowed"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowed"));
                                }
                                r#allowed = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::AllowedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#allowed.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_allowed"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("allowed");
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
                Ok(
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#type: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#type.unwrap_or(Default::default())
                        } else {
                            r#type.ok_or(serde::de::Error::missing_field("type"))?
                        },
                        r#allowed: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#allowed.unwrap_or(Default::default())
                        } else {
                            r#allowed.ok_or(serde::de::Error::missing_field("allowed"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>(
        )
        .deserialize(deserializer)
        .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySubstitution > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Box<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution,
                >>())? {
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatorySchedule")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule,
                V::Error,
            >
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
                    #[serde(rename = "schedule")]
                    Schedule,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "schedule"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#schedule: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                        Field::Schedule => {
                            if r#schedule.is_some() {
                                return Err(serde::de::Error::duplicate_field("schedule"));
                            }
                            r#schedule = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#schedule: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#schedule.unwrap_or(Default::default())
                        } else {
                            r#schedule.ok_or(serde::de::Error::missing_field("schedule"))?
                        },
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySchedule > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>>;
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
                            fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule,
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatoryMaxDispense")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense,
                V::Error,
            >
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
                    #[serde(rename = "quantity")]
                    Quantity,
                    #[serde(rename = "period")]
                    Period,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "quantity", "period"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#quantity: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#period: Option<Box<fhirbolt_model::r4b::types::Duration>> = None;
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
                        Field::Quantity => {
                            if r#quantity.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            r#quantity = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::Period => {
                            if r#period.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            r#period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(),
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#quantity: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#quantity.unwrap_or(Default::default())
                        } else {
                            r#quantity.ok_or(serde::de::Error::missing_field("quantity"))?
                        },
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
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatoryMaxDispense > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<Box<
                        fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense,
                    >>())?
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
        fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeRegulatory")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory, V::Error>
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
                    #[serde(rename = "regulatoryAuthority")]
                    RegulatoryAuthority,
                    #[serde(rename = "substitution")]
                    Substitution,
                    #[serde(rename = "schedule")]
                    Schedule,
                    #[serde(rename = "maxDispense")]
                    MaxDispense,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "regulatoryAuthority",
                            "substitution",
                            "schedule",
                            "maxDispense",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#regulatory_authority: Option<Box<fhirbolt_model::r4b::types::Reference>> =
                    None;
                let mut r#substitution: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySubstitution>,
                > = None;
                let mut r#schedule: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatorySchedule>,
                > = None;
                let mut r#max_dispense: Option<
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatoryMaxDispense,
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
                        Field::RegulatoryAuthority => {
                            if r#regulatory_authority.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "regulatoryAuthority",
                                ));
                            }
                            r#regulatory_authority = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Substitution => {
                            if self.0.from_json {
                                if r#substitution.is_some() {
                                    return Err(serde::de::Error::duplicate_field("substitution"));
                                }
                                r#substitution = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySubstitution >> ()) ?) ;
                            } else {
                                let vec = r#substitution.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySubstitution > ()) ?) ;
                            }
                        }
                        Field::Schedule => {
                            if self.0.from_json {
                                if r#schedule.is_some() {
                                    return Err(serde::de::Error::duplicate_field("schedule"));
                                }
                                r#schedule = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySchedule >> ()) ?) ;
                            } else {
                                let vec = r#schedule.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatorySchedule > ()) ?) ;
                            }
                        }
                        Field::MaxDispense => {
                            if r#max_dispense.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDispense"));
                            }
                            r#max_dispense = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatoryMaxDispense > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#regulatory_authority: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#regulatory_authority.unwrap_or(Default::default())
                        } else {
                            r#regulatory_authority
                                .ok_or(serde::de::Error::missing_field("regulatoryAuthority"))?
                        },
                        r#substitution: r#substitution.unwrap_or(vec![]),
                        r#schedule: r#schedule.unwrap_or(vec![]),
                        r#max_dispense,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>(
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatory >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledgeKinetics")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics, V::Error>
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
                    #[serde(rename = "areaUnderCurve")]
                    AreaUnderCurve,
                    #[serde(rename = "lethalDose50")]
                    LethalDose50,
                    #[serde(rename = "halfLifePeriod")]
                    HalfLifePeriod,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "areaUnderCurve",
                            "lethalDose50",
                            "halfLifePeriod",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#area_under_curve: Option<Vec<Box<fhirbolt_model::r4b::types::Quantity>>> =
                    None;
                let mut r#lethal_dose_50: Option<Vec<Box<fhirbolt_model::r4b::types::Quantity>>> =
                    None;
                let mut r#half_life_period: Option<Box<fhirbolt_model::r4b::types::Duration>> =
                    None;
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
                        Field::AreaUnderCurve => {
                            if self.0.from_json {
                                if r#area_under_curve.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "areaUnderCurve",
                                    ));
                                }
                                r#area_under_curve = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Quantity > >> ()) ?) ;
                            } else {
                                let vec = r#area_under_curve.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::LethalDose50 => {
                            if self.0.from_json {
                                if r#lethal_dose_50.is_some() {
                                    return Err(serde::de::Error::duplicate_field("lethalDose50"));
                                }
                                r#lethal_dose_50 = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Quantity > >> ()) ?) ;
                            } else {
                                let vec = r#lethal_dose_50.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::HalfLifePeriod => {
                            if r#half_life_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("halfLifePeriod"));
                            }
                            r#half_life_period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Duration>>(),
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
                    fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#area_under_curve: r#area_under_curve.unwrap_or(vec![]),
                        r#lethal_dose_50: r#lethal_dose_50.unwrap_or(vec![]),
                        r#half_life_period,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeKinetics >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::MedicationKnowledge {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledge;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::MedicationKnowledge,
    >
{
    type Value = fhirbolt_model::r4b::resources::MedicationKnowledge;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::MedicationKnowledge,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::MedicationKnowledge;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationKnowledge")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::MedicationKnowledge, V::Error>
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "manufacturer")]
                    Manufacturer,
                    #[serde(rename = "doseForm")]
                    DoseForm,
                    #[serde(rename = "amount")]
                    Amount,
                    #[serde(rename = "synonym")]
                    Synonym,
                    #[serde(rename = "_synonym")]
                    SynonymPrimitiveElement,
                    #[serde(rename = "relatedMedicationKnowledge")]
                    RelatedMedicationKnowledge,
                    #[serde(rename = "associatedMedication")]
                    AssociatedMedication,
                    #[serde(rename = "productType")]
                    ProductType,
                    #[serde(rename = "monograph")]
                    Monograph,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "preparationInstruction")]
                    PreparationInstruction,
                    #[serde(rename = "_preparationInstruction")]
                    PreparationInstructionPrimitiveElement,
                    #[serde(rename = "intendedRoute")]
                    IntendedRoute,
                    #[serde(rename = "cost")]
                    Cost,
                    #[serde(rename = "monitoringProgram")]
                    MonitoringProgram,
                    #[serde(rename = "administrationGuidelines")]
                    AdministrationGuidelines,
                    #[serde(rename = "medicineClassification")]
                    MedicineClassification,
                    #[serde(rename = "packaging")]
                    Packaging,
                    #[serde(rename = "drugCharacteristic")]
                    DrugCharacteristic,
                    #[serde(rename = "contraindication")]
                    Contraindication,
                    #[serde(rename = "regulatory")]
                    Regulatory,
                    #[serde(rename = "kinetics")]
                    Kinetics,
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
                            "code",
                            "status",
                            "manufacturer",
                            "doseForm",
                            "amount",
                            "synonym",
                            "relatedMedicationKnowledge",
                            "associatedMedication",
                            "productType",
                            "monograph",
                            "ingredient",
                            "preparationInstruction",
                            "intendedRoute",
                            "cost",
                            "monitoringProgram",
                            "administrationGuidelines",
                            "medicineClassification",
                            "packaging",
                            "drugCharacteristic",
                            "contraindication",
                            "regulatory",
                            "kinetics",
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
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#manufacturer: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#dose_form: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> =
                    None;
                let mut r#amount: Option<Box<fhirbolt_model::r4b::types::Quantity>> = None;
                let mut r#synonym: Option<Vec<fhirbolt_model::r4b::types::String>> = None;
                let mut r#related_medication_knowledge : Option < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > > = None ;
                let mut r#associated_medication: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Reference>>,
                > = None;
                let mut r#product_type: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#monograph: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonograph>,
                > = None;
                let mut r#ingredient: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeIngredient>,
                > = None;
                let mut r#preparation_instruction: Option<fhirbolt_model::r4b::types::Markdown> =
                    None;
                let mut r#intended_route: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#cost: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeCost>,
                > = None;
                let mut r#monitoring_program: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMonitoringProgram>,
                > = None;
                let mut r#administration_guidelines: Option<
                    Vec<
                        fhirbolt_model::r4b::resources::MedicationKnowledgeAdministrationGuidelines,
                    >,
                > = None;
                let mut r#medicine_classification: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeMedicineClassification>,
                > = None;
                let mut r#packaging: Option<
                    fhirbolt_model::r4b::resources::MedicationKnowledgePackaging,
                > = None;
                let mut r#drug_characteristic: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeDrugCharacteristic>,
                > = None;
                let mut r#contraindication: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Reference>>,
                > = None;
                let mut r#regulatory: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeRegulatory>,
                > = None;
                let mut r#kinetics: Option<
                    Vec<fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationKnowledge" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationKnowledge",
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
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
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
                        Field::Manufacturer => {
                            if r#manufacturer.is_some() {
                                return Err(serde::de::Error::duplicate_field("manufacturer"));
                            }
                            r#manufacturer = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::DoseForm => {
                            if r#dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field("doseForm"));
                            }
                            r#dose_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Amount => {
                            if r#amount.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            r#amount = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::Synonym => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#synonym.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("synonym"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#synonym.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::SynonymPrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#synonym.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_synonym"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("synonym");
                            }
                        }
                        Field::RelatedMedicationKnowledge => {
                            if self.0.from_json {
                                if r#related_medication_knowledge.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "relatedMedicationKnowledge",
                                    ));
                                }
                                r#related_medication_knowledge = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge >> ()) ?) ;
                            } else {
                                let vec = r#related_medication_knowledge
                                    .get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRelatedMedicationKnowledge > ()) ?) ;
                            }
                        }
                        Field::AssociatedMedication => {
                            if self.0.from_json {
                                if r#associated_medication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "associatedMedication",
                                    ));
                                }
                                r#associated_medication = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#associated_medication.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?) ;
                            }
                        }
                        Field::ProductType => {
                            if self.0.from_json {
                                if r#product_type.is_some() {
                                    return Err(serde::de::Error::duplicate_field("productType"));
                                }
                                r#product_type =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#product_type.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Monograph => {
                            if self.0.from_json {
                                if r#monograph.is_some() {
                                    return Err(serde::de::Error::duplicate_field("monograph"));
                                }
                                r#monograph = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonograph >> ()) ?) ;
                            } else {
                                let vec = r#monograph.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonograph > ()) ?) ;
                            }
                        }
                        Field::Ingredient => {
                            if self.0.from_json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeIngredient >> ()) ?) ;
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeIngredient > ()) ?) ;
                            }
                        }
                        Field::PreparationInstruction => {
                            if self.0.from_json {
                                let some =
                                    r#preparation_instruction.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preparationInstruction",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#preparation_instruction.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "preparationInstruction",
                                    ));
                                }
                                r#preparation_instruction = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::PreparationInstructionPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#preparation_instruction.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_preparationInstruction",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preparationInstruction");
                            }
                        }
                        Field::IntendedRoute => {
                            if self.0.from_json {
                                if r#intended_route.is_some() {
                                    return Err(serde::de::Error::duplicate_field("intendedRoute"));
                                }
                                r#intended_route =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#intended_route.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Cost => {
                            if self.0.from_json {
                                if r#cost.is_some() {
                                    return Err(serde::de::Error::duplicate_field("cost"));
                                }
                                r#cost = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::MedicationKnowledgeCost,
                                    >>(),
                                )?);
                            } else {
                                let vec = r#cost.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeCost > ()) ?) ;
                            }
                        }
                        Field::MonitoringProgram => {
                            if self.0.from_json {
                                if r#monitoring_program.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "monitoringProgram",
                                    ));
                                }
                                r#monitoring_program = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonitoringProgram >> ()) ?) ;
                            } else {
                                let vec = r#monitoring_program.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMonitoringProgram > ()) ?) ;
                            }
                        }
                        Field::AdministrationGuidelines => {
                            if self.0.from_json {
                                if r#administration_guidelines.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "administrationGuidelines",
                                    ));
                                }
                                r#administration_guidelines = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelines >> ()) ?) ;
                            } else {
                                let vec =
                                    r#administration_guidelines.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeAdministrationGuidelines > ()) ?) ;
                            }
                        }
                        Field::MedicineClassification => {
                            if self.0.from_json {
                                if r#medicine_classification.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "medicineClassification",
                                    ));
                                }
                                r#medicine_classification = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMedicineClassification >> ()) ?) ;
                            } else {
                                let vec =
                                    r#medicine_classification.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeMedicineClassification > ()) ?) ;
                            }
                        }
                        Field::Packaging => {
                            if r#packaging.is_some() {
                                return Err(serde::de::Error::duplicate_field("packaging"));
                            }
                            r#packaging = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgePackaging > ()) ?) ;
                        }
                        Field::DrugCharacteristic => {
                            if self.0.from_json {
                                if r#drug_characteristic.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "drugCharacteristic",
                                    ));
                                }
                                r#drug_characteristic = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristic >> ()) ?) ;
                            } else {
                                let vec = r#drug_characteristic.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeDrugCharacteristic > ()) ?) ;
                            }
                        }
                        Field::Contraindication => {
                            if self.0.from_json {
                                if r#contraindication.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "contraindication",
                                    ));
                                }
                                r#contraindication = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4b :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#contraindication.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Reference > > ()) ?) ;
                            }
                        }
                        Field::Regulatory => {
                            if self.0.from_json {
                                if r#regulatory.is_some() {
                                    return Err(serde::de::Error::duplicate_field("regulatory"));
                                }
                                r#regulatory = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatory >> ()) ?) ;
                            } else {
                                let vec = r#regulatory.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeRegulatory > ()) ?) ;
                            }
                        }
                        Field::Kinetics => {
                            if self.0.from_json {
                                if r#kinetics.is_some() {
                                    return Err(serde::de::Error::duplicate_field("kinetics"));
                                }
                                r#kinetics =
                                    Some(map_access.next_value_seed(self.0.transmute::<Vec<
                                        fhirbolt_model::r4b::resources::MedicationKnowledgeKinetics,
                                    >>(
                                    ))?);
                            } else {
                                let vec = r#kinetics.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: MedicationKnowledgeKinetics > ()) ?) ;
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
                Ok(fhirbolt_model::r4b::resources::MedicationKnowledge {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#code,
                    r#status,
                    r#manufacturer,
                    r#dose_form,
                    r#amount,
                    r#synonym: r#synonym.unwrap_or(vec![]),
                    r#related_medication_knowledge: r#related_medication_knowledge
                        .unwrap_or(vec![]),
                    r#associated_medication: r#associated_medication.unwrap_or(vec![]),
                    r#product_type: r#product_type.unwrap_or(vec![]),
                    r#monograph: r#monograph.unwrap_or(vec![]),
                    r#ingredient: r#ingredient.unwrap_or(vec![]),
                    r#preparation_instruction,
                    r#intended_route: r#intended_route.unwrap_or(vec![]),
                    r#cost: r#cost.unwrap_or(vec![]),
                    r#monitoring_program: r#monitoring_program.unwrap_or(vec![]),
                    r#administration_guidelines: r#administration_guidelines.unwrap_or(vec![]),
                    r#medicine_classification: r#medicine_classification.unwrap_or(vec![]),
                    r#packaging,
                    r#drug_characteristic: r#drug_characteristic.unwrap_or(vec![]),
                    r#contraindication: r#contraindication.unwrap_or(vec![]),
                    r#regulatory: r#regulatory.unwrap_or(vec![]),
                    r#kinetics: r#kinetics.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::MedicationKnowledge>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::MedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::MedicationKnowledge>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::MedicationKnowledge>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledge>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::MedicationKnowledge>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::MedicationKnowledge>;
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
                        .transmute::<fhirbolt_model::r4b::resources::MedicationKnowledge>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>;
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
                        .transmute::<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
