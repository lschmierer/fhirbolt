// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationAdministrationPerformer,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
        if let Some(some) = self.value.r#function.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("function", ctx))?;
        }
        self.with_context(&self.value.r#actor, |ctx| {
            state.serialize_entry("actor", ctx)
        })?;
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationAdministrationPerformer,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationAdministrationPerformer;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationAdministrationPerformer,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationAdministrationPerformer;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationPerformer")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationAdministrationPerformer, V::Error>
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
                    #[serde(rename = "function")]
                    Function,
                    #[serde(rename = "actor")]
                    Actor,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "function", "actor"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
                let mut r#function: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#actor: Option<Box<fhirbolt_model::r5::types::CodeableReference>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Function => {
                            if r#function.is_some() {
                                return Err(serde::de::Error::duplicate_field("function"));
                            }
                            r#function = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Actor => {
                            if r#actor.is_some() {
                                return Err(serde::de::Error::duplicate_field("actor"));
                            }
                            r#actor = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
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
                    fhirbolt_model::r5::resources::MedicationAdministrationPerformer {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#function,
                        r#actor: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#actor.unwrap_or(Default::default())
                        } else {
                            r#actor.ok_or(serde::de::Error::missing_field("actor"))?
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
        Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationAdministrationDosage,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
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
            if let Some(some) = self.value.r#text.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("text", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_text", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#text.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("text", ctx))?;
            }
        }
        if let Some(some) = self.value.r#site.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("site", ctx))?;
        }
        if let Some(some) = self.value.r#route.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("route", ctx))?;
        }
        if let Some(some) = self.value.r#method.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("method", ctx))?;
        }
        if let Some(some) = self.value.r#dose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("dose", ctx))?;
        }
        if let Some(some) = self.value.r#rate.as_ref() {
            match some {
                fhirbolt_model::r5::resources::MedicationAdministrationDosageRate::Ratio(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("rateRatio", ctx))?;
                }
                fhirbolt_model::r5::resources::MedicationAdministrationDosageRate::Quantity(
                    ref value,
                ) => {
                    self.with_context(value, |ctx| state.serialize_entry("rateQuantity", ctx))?;
                }
                fhirbolt_model::r5::resources::MedicationAdministrationDosageRate::Invalid => {
                    return Err(serde::ser::Error::custom("rate is invalid"))
                }
            }
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::MedicationAdministrationDosage>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>>,
    >
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
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationAdministrationDosage,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationAdministrationDosage;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationAdministrationDosage,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationAdministrationDosage;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministrationDosage")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationAdministrationDosage, V::Error>
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
                    #[serde(rename = "text")]
                    Text,
                    #[serde(rename = "_text")]
                    TextPrimitiveElement,
                    #[serde(rename = "site")]
                    Site,
                    #[serde(rename = "route")]
                    Route,
                    #[serde(rename = "method")]
                    Method,
                    #[serde(rename = "dose")]
                    Dose,
                    #[serde(rename = "rateRatio")]
                    RateRatio,
                    #[serde(rename = "rateQuantity")]
                    RateQuantity,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "text",
                            "site",
                            "route",
                            "method",
                            "dose",
                            "rateRatio",
                            "rateQuantity",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
                let mut r#text: Option<fhirbolt_model::r5::types::String> = None;
                let mut r#site: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#route: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#method: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#dose: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#rate: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationDosageRate,
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Text => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#text.is_some() {
                                    return Err(serde::de::Error::duplicate_field("text"));
                                }
                                r#text = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::String>(),
                                )?);
                            }
                        }
                        Field::TextPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#text.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_text"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("text");
                            }
                        }
                        Field::Site => {
                            if r#site.is_some() {
                                return Err(serde::de::Error::duplicate_field("site"));
                            }
                            r#site = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Route => {
                            if r#route.is_some() {
                                return Err(serde::de::Error::duplicate_field("route"));
                            }
                            r#route = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Method => {
                            if r#method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            r#method = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Dose => {
                            if r#dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("dose"));
                            }
                            r#dose = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::RateRatio => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateRatio"));
                            }
                            r#rate = Some (fhirbolt_model :: r5 :: resources :: MedicationAdministrationDosageRate :: Ratio (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Ratio > > ()) ?)) ;
                        }
                        Field::RateQuantity => {
                            if r#rate.is_some() {
                                return Err(serde::de::Error::duplicate_field("rateQuantity"));
                            }
                            r#rate = Some (fhirbolt_model :: r5 :: resources :: MedicationAdministrationDosageRate :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
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
                    fhirbolt_model::r5::resources::MedicationAdministrationDosage {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#text,
                        r#site,
                        r#route,
                        r#method,
                        r#dose,
                        r#rate,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationAdministrationDosage>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationAdministrationDosage>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministrationDosage>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationAdministrationDosage>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministrationDosage>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationAdministrationDosage>(
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
        Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministrationDosage>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: resources :: MedicationAdministrationDosage >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::MedicationAdministration {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::MedicationAdministration,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicationAdministration")?;
        if let Some(value) = self.value.r#id.as_ref() {
            state.serialize_entry("id", value)?;
        }
        if let Some(some) = self.value.r#meta.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("meta", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("implicitRules", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_implicitRules", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#implicit_rules.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("implicitRules", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#language.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("language", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_language", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#language.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("language", ctx))?;
            }
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
        if !self.value.r#based_on.is_empty() {
            self.with_context(&self.value.r#based_on, |ctx| {
                state.serialize_entry("basedOn", ctx)
            })?;
        }
        if !self.value.r#part_of.is_empty() {
            self.with_context(&self.value.r#part_of, |ctx| {
                state.serialize_entry("partOf", ctx)
            })?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#status.value.as_ref() {
                let some = Ok(some)?;
                state.serialize_entry("status", &some)?;
            }
            if self.value.r#status.id.is_some() || !self.value.r#status.extension.is_empty() {
                let primitive_element = super::super::serde_helpers::PrimitiveElement {
                    id: self.value.r#status.id.as_ref(),
                    extension: &self.value.r#status.extension,
                };
                self.with_context(&primitive_element, |ctx| {
                    state.serialize_entry("_status", ctx)
                })?;
            }
        } else {
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if !self.value.r#status_reason.is_empty() {
            self.with_context(&self.value.r#status_reason, |ctx| {
                state.serialize_entry("statusReason", ctx)
            })?;
        }
        if !self.value.r#category.is_empty() {
            self.with_context(&self.value.r#category, |ctx| {
                state.serialize_entry("category", ctx)
            })?;
        }
        self.with_context(&self.value.r#medication, |ctx| {
            state.serialize_entry("medication", ctx)
        })?;
        self.with_context(&self.value.r#subject, |ctx| {
            state.serialize_entry("subject", ctx)
        })?;
        if let Some(some) = self.value.r#encounter.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("encounter", ctx))?;
        }
        if !self.value.r#supporting_information.is_empty() {
            self.with_context(&self.value.r#supporting_information, |ctx| {
                state.serialize_entry("supportingInformation", ctx)
            })?;
        }
        match self.value.r#occurence {
            fhirbolt_model::r5::resources::MedicationAdministrationOccurence::DateTime(
                ref value,
            ) => {
                if self.output_json {
                    if let Some(some) = value.value.as_ref() {
                        let some = Ok(some)?;
                        state.serialize_entry("occurenceDateTime", &some)?;
                    }
                    if value.id.is_some() || !value.extension.is_empty() {
                        let primitive_element = super::super::serde_helpers::PrimitiveElement {
                            id: value.id.as_ref(),
                            extension: &value.extension,
                        };
                        self.with_context(&primitive_element, |ctx| {
                            state.serialize_entry("_occurenceDateTime", ctx)
                        })?;
                    }
                } else {
                    self.with_context(value, |ctx| {
                        state.serialize_entry("occurenceDateTime", ctx)
                    })?;
                }
            }
            fhirbolt_model::r5::resources::MedicationAdministrationOccurence::Period(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("occurencePeriod", ctx))?;
            }
            fhirbolt_model::r5::resources::MedicationAdministrationOccurence::Timing(ref value) => {
                self.with_context(value, |ctx| state.serialize_entry("occurenceTiming", ctx))?;
            }
            fhirbolt_model::r5::resources::MedicationAdministrationOccurence::Invalid => {
                return Err(serde::ser::Error::custom("occurence is a required field"))
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#recorded.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("recorded", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_recorded", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#recorded.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("recorded", ctx))?;
            }
        }
        if self.output_json {
            if let Some(some) = self.value.r#is_sub_potent.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("isSubPotent", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_isSubPotent", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#is_sub_potent.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("isSubPotent", ctx))?;
            }
        }
        if !self.value.r#sub_potent_reason.is_empty() {
            self.with_context(&self.value.r#sub_potent_reason, |ctx| {
                state.serialize_entry("subPotentReason", ctx)
            })?;
        }
        if !self.value.r#performer.is_empty() {
            self.with_context(&self.value.r#performer, |ctx| {
                state.serialize_entry("performer", ctx)
            })?;
        }
        if !self.value.r#reason.is_empty() {
            self.with_context(&self.value.r#reason, |ctx| {
                state.serialize_entry("reason", ctx)
            })?;
        }
        if let Some(some) = self.value.r#request.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("request", ctx))?;
        }
        if !self.value.r#device.is_empty() {
            self.with_context(&self.value.r#device, |ctx| {
                state.serialize_entry("device", ctx)
            })?;
        }
        if !self.value.r#note.is_empty() {
            self.with_context(&self.value.r#note, |ctx| state.serialize_entry("note", ctx))?;
        }
        if let Some(some) = self.value.r#dosage.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("dosage", ctx))?;
        }
        if !self.value.r#event_history.is_empty() {
            self.with_context(&self.value.r#event_history, |ctx| {
                state.serialize_entry("eventHistory", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::MedicationAdministration>,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<fhirbolt_model::r5::resources::MedicationAdministration>,
    >
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
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Vec<Box<fhirbolt_model::r5::resources::MedicationAdministration>>,
    >
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
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationAdministration,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationAdministration;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::MedicationAdministration,
    >
{
    type Value = fhirbolt_model::r5::resources::MedicationAdministration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::MedicationAdministration,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::MedicationAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicationAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::MedicationAdministration, V::Error>
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
                    #[serde(rename = "identifier")]
                    Identifier,
                    #[serde(rename = "basedOn")]
                    BasedOn,
                    #[serde(rename = "partOf")]
                    PartOf,
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "statusReason")]
                    StatusReason,
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "medication")]
                    Medication,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "supportingInformation")]
                    SupportingInformation,
                    #[serde(rename = "occurenceDateTime")]
                    OccurenceDateTime,
                    #[serde(rename = "_occurenceDateTime")]
                    OccurenceDateTimePrimitiveElement,
                    #[serde(rename = "occurencePeriod")]
                    OccurencePeriod,
                    #[serde(rename = "occurenceTiming")]
                    OccurenceTiming,
                    #[serde(rename = "recorded")]
                    Recorded,
                    #[serde(rename = "_recorded")]
                    RecordedPrimitiveElement,
                    #[serde(rename = "isSubPotent")]
                    IsSubPotent,
                    #[serde(rename = "_isSubPotent")]
                    IsSubPotentPrimitiveElement,
                    #[serde(rename = "subPotentReason")]
                    SubPotentReason,
                    #[serde(rename = "performer")]
                    Performer,
                    #[serde(rename = "reason")]
                    Reason,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "note")]
                    Note,
                    #[serde(rename = "dosage")]
                    Dosage,
                    #[serde(rename = "eventHistory")]
                    EventHistory,
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
                            "basedOn",
                            "partOf",
                            "status",
                            "statusReason",
                            "category",
                            "medication",
                            "subject",
                            "encounter",
                            "supportingInformation",
                            "occurenceDateTime",
                            "occurencePeriod",
                            "occurenceTiming",
                            "recorded",
                            "isSubPotent",
                            "subPotentReason",
                            "performer",
                            "reason",
                            "request",
                            "device",
                            "note",
                            "dosage",
                            "eventHistory",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r5::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r5::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r5::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r5::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r5::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r5::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r5::types::Identifier>>> =
                    None;
                let mut r#based_on: Option<Vec<Box<fhirbolt_model::r5::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<fhirbolt_model::r5::types::Reference>>> = None;
                let mut r#status: Option<fhirbolt_model::r5::types::Code> = None;
                let mut r#status_reason: Option<
                    Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>,
                > = None;
                let mut r#category: Option<Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>> =
                    None;
                let mut r#medication: Option<Box<fhirbolt_model::r5::types::CodeableReference>> =
                    None;
                let mut r#subject: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#supporting_information: Option<
                    Vec<Box<fhirbolt_model::r5::types::Reference>>,
                > = None;
                let mut r#occurence: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationOccurence,
                > = None;
                let mut r#recorded: Option<fhirbolt_model::r5::types::DateTime> = None;
                let mut r#is_sub_potent: Option<fhirbolt_model::r5::types::Boolean> = None;
                let mut r#sub_potent_reason: Option<
                    Vec<Box<fhirbolt_model::r5::types::CodeableConcept>>,
                > = None;
                let mut r#performer: Option<
                    Vec<fhirbolt_model::r5::resources::MedicationAdministrationPerformer>,
                > = None;
                let mut r#reason: Option<Vec<Box<fhirbolt_model::r5::types::CodeableReference>>> =
                    None;
                let mut r#request: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#device: Option<Vec<Box<fhirbolt_model::r5::types::CodeableReference>>> =
                    None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r5::types::Annotation>>> = None;
                let mut r#dosage: Option<
                    fhirbolt_model::r5::resources::MedicationAdministrationDosage,
                > = None;
                let mut r#event_history: Option<Vec<Box<fhirbolt_model::r5::types::Reference>>> =
                    None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicationAdministration" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicationAdministration",
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
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r5::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r5::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r5::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::BasedOn => {
                            if self.0.from_json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::PartOf => {
                            if self.0.from_json {
                                if r#part_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("partOf"));
                                }
                                r#part_of = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#part_of.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
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
                                    self.0.transmute::<fhirbolt_model::r5::types::Code>(),
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
                        Field::StatusReason => {
                            if self.0.from_json {
                                if r#status_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("statusReason"));
                                }
                                r#status_reason =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#status_reason.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Category => {
                            if self.0.from_json {
                                if r#category.is_some() {
                                    return Err(serde::de::Error::duplicate_field("category"));
                                }
                                r#category =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#category.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Medication => {
                            if r#medication.is_some() {
                                return Err(serde::de::Error::duplicate_field("medication"));
                            }
                            r#medication = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Encounter => {
                            if r#encounter.is_some() {
                                return Err(serde::de::Error::duplicate_field("encounter"));
                            }
                            r#encounter = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::SupportingInformation => {
                            if self.0.from_json {
                                if r#supporting_information.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "supportingInformation",
                                    ));
                                }
                                r#supporting_information = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec =
                                    r#supporting_information.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::OccurenceDateTime => {
                            if self.0.from_json {
                                let r#enum = r#occurence . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: DateTime (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("occurenceDateTime")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("occurence[x]")) ; }
                            } else {
                                if r#occurence.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "occurenceDateTime",
                                    ));
                                }
                                r#occurence = Some (fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::OccurenceDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#occurence . get_or_insert (fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: DateTime (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: DateTime (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_occurenceDateTime")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_occurence[x]")) ; }
                            } else {
                                return unknown_field_error("occurenceDateTime");
                            }
                        }
                        Field::OccurencePeriod => {
                            if r#occurence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurencePeriod"));
                            }
                            r#occurence = Some (fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: Period (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Period > > ()) ?)) ;
                        }
                        Field::OccurenceTiming => {
                            if r#occurence.is_some() {
                                return Err(serde::de::Error::duplicate_field("occurenceTiming"));
                            }
                            r#occurence = Some (fhirbolt_model :: r5 :: resources :: MedicationAdministrationOccurence :: Timing (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Timing > > ()) ?)) ;
                        }
                        Field::Recorded => {
                            if self.0.from_json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#recorded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("recorded"));
                                }
                                r#recorded = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::RecordedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#recorded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_recorded"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("recorded");
                            }
                        }
                        Field::IsSubPotent => {
                            if self.0.from_json {
                                let some = r#is_sub_potent.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSubPotent"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#is_sub_potent.is_some() {
                                    return Err(serde::de::Error::duplicate_field("isSubPotent"));
                                }
                                r#is_sub_potent = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::IsSubPotentPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#is_sub_potent.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_isSubPotent"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("isSubPotent");
                            }
                        }
                        Field::SubPotentReason => {
                            if self.0.from_json {
                                if r#sub_potent_reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "subPotentReason",
                                    ));
                                }
                                r#sub_potent_reason =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#sub_potent_reason.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Performer => {
                            if self.0.from_json {
                                if r#performer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("performer"));
                                }
                                r#performer = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer >> ()) ?) ;
                            } else {
                                let vec = r#performer.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationAdministrationPerformer > ()) ?) ;
                            }
                        }
                        Field::Reason => {
                            if self.0.from_json {
                                if r#reason.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reason"));
                                }
                                r#reason =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableReference>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#reason.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                            }
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Device => {
                            if self.0.from_json {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                r#device =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r5::types::CodeableReference>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#device.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableReference > > ()) ?) ;
                            }
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Annotation > > ()) ?) ;
                            }
                        }
                        Field::Dosage => {
                            if r#dosage.is_some() {
                                return Err(serde::de::Error::duplicate_field("dosage"));
                            }
                            r#dosage = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: MedicationAdministrationDosage > ()) ?) ;
                        }
                        Field::EventHistory => {
                            if self.0.from_json {
                                if r#event_history.is_some() {
                                    return Err(serde::de::Error::duplicate_field("eventHistory"));
                                }
                                r#event_history = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r5 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#event_history.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r5::types::Reference>>(
                                            ),
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
                Ok(fhirbolt_model::r5::resources::MedicationAdministration {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier: r#identifier.unwrap_or(vec![]),
                    r#based_on: r#based_on.unwrap_or(vec![]),
                    r#part_of: r#part_of.unwrap_or(vec![]),
                    r#status: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#status.unwrap_or(Default::default())
                    } else {
                        r#status.ok_or(serde::de::Error::missing_field("status"))?
                    },
                    r#status_reason: r#status_reason.unwrap_or(vec![]),
                    r#category: r#category.unwrap_or(vec![]),
                    r#medication: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#medication.unwrap_or(Default::default())
                    } else {
                        r#medication.ok_or(serde::de::Error::missing_field("medication"))?
                    },
                    r#subject: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#subject.unwrap_or(Default::default())
                    } else {
                        r#subject.ok_or(serde::de::Error::missing_field("subject"))?
                    },
                    r#encounter,
                    r#supporting_information: r#supporting_information.unwrap_or(vec![]),
                    r#occurence: if self.0.config.mode
                        == crate::context::de::DeserializationMode::Lax
                    {
                        r#occurence.unwrap_or(Default::default())
                    } else {
                        r#occurence.ok_or(serde::de::Error::missing_field("occurence[x]"))?
                    },
                    r#recorded,
                    r#is_sub_potent,
                    r#sub_potent_reason: r#sub_potent_reason.unwrap_or(vec![]),
                    r#performer: r#performer.unwrap_or(vec![]),
                    r#reason: r#reason.unwrap_or(vec![]),
                    r#request,
                    r#device: r#device.unwrap_or(vec![]),
                    r#note: r#note.unwrap_or(vec![]),
                    r#dosage,
                    r#event_history: r#event_history.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::MedicationAdministration>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::MedicationAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::MedicationAdministration>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::MedicationAdministration>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::MedicationAdministration>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::MedicationAdministration>;
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
                        .transmute::<fhirbolt_model::r5::resources::MedicationAdministration>(),
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
        Vec<Box<fhirbolt_model::r5::resources::MedicationAdministration>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministration>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::resources::MedicationAdministration>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::resources::MedicationAdministration>>;
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
                        .transmute::<Box<fhirbolt_model::r5::resources::MedicationAdministration>>(
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
