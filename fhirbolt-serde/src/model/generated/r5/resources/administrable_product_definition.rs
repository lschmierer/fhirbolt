// Generated on 2023-04-24 by fhirbolt-codegen v0.6.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AdministrableProductDefinition.property", field
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
        if self.value.r#type.id.as_deref() == Some("$invalid") {
            return missing_field_error("type");
        }
        self.with_context(&self.value.r#type, |ctx| state.serialize_entry("type", ctx))?;
        if let Some(some) = self.value.r#value.as_ref() {
            match some { fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: CodeableConcept (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueCodeableConcept" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Quantity (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueQuantity" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueDate" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueDate" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueDate" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueBoolean" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueBoolean" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueBoolean" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (ref value) => { if self . output_json { if let Some (some) = value . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("valueMarkdown" , & some) ? ; } if value . id . is_some () || ! value . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : value . id . as_ref () , extension : & value . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_valueMarkdown" , ctx)) ? ; } } else { self . with_context (value , | ctx | state . serialize_entry ("valueMarkdown" , ctx)) ? ; } } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Attachment (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueAttachment" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Reference (ref value) => { self . with_context (value , | ctx | state . serialize_entry ("valueReference" , ctx)) ? ; } , fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Invalid => { return Err (serde :: ser :: Error :: custom ("value is invalid")) } }
        }
        if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
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
        &Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
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
        fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty,
    >
{
    type Value = fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinitionProperty")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty,
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
                    #[serde(rename = "valueQuantity")]
                    ValueQuantity,
                    #[serde(rename = "valueDate")]
                    ValueDate,
                    #[serde(rename = "_valueDate")]
                    ValueDatePrimitiveElement,
                    #[serde(rename = "valueBoolean")]
                    ValueBoolean,
                    #[serde(rename = "_valueBoolean")]
                    ValueBooleanPrimitiveElement,
                    #[serde(rename = "valueMarkdown")]
                    ValueMarkdown,
                    #[serde(rename = "_valueMarkdown")]
                    ValueMarkdownPrimitiveElement,
                    #[serde(rename = "valueAttachment")]
                    ValueAttachment,
                    #[serde(rename = "valueReference")]
                    ValueReference,
                    #[serde(rename = "status")]
                    Status,
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
                            "valueQuantity",
                            "valueDate",
                            "valueBoolean",
                            "valueMarkdown",
                            "valueAttachment",
                            "valueReference",
                            "status",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#type: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#value: Option<
                    fhirbolt_model::r5::resources::AdministrableProductDefinitionPropertyValue,
                > = None;
                let mut r#status: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ValueCodeableConcept => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "valueCodeableConcept",
                                ));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: CodeableConcept (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?)) ;
                        }
                        Field::ValueQuantity => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueQuantity"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Quantity (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?)) ;
                        }
                        Field::ValueDate => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueDate"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::ValueDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueDate");
                            }
                        }
                        Field::ValueBoolean => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueBoolean")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueBoolean"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Boolean > > ()) ?)) ;
                            }
                        }
                        Field::ValueBooleanPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Boolean (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueBoolean")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueBoolean");
                            }
                        }
                        Field::ValueMarkdown => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("valueMarkdown")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("value[x]")) ; }
                            } else {
                                if r#value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("valueMarkdown"));
                                }
                                r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Markdown > > ()) ?)) ;
                            }
                        }
                        Field::ValueMarkdownPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#value . get_or_insert (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (Default :: default ())) ;
                                if let fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Markdown (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_valueMarkdown")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_value[x]")) ; }
                            } else {
                                return unknown_field_error("valueMarkdown");
                            }
                        }
                        Field::ValueAttachment => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueAttachment"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Attachment (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Attachment > > ()) ?)) ;
                        }
                        Field::ValueReference => {
                            if r#value.is_some() {
                                return Err(serde::de::Error::duplicate_field("valueReference"));
                            }
                            r#value = Some (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionPropertyValue :: Reference (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Reference > > ()) ?)) ;
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
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
                    fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty {
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
                        r#value,
                        r#status,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionProperty > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; # [allow (dead_code)] fn missing_field_error < T , E : serde :: ser :: Error > (field : & str) -> Result < T , E > { Err (E :: custom (format ! ("missing required field `{}.{}`" , "AdministrableProductDefinition.routeOfAdministration.targetSpecies.withdrawalPeriod" , field))) } let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } if self . value . r#tissue . id . as_deref () == Some ("$invalid") { return missing_field_error ("tissue") } self . with_context (& self . value . r#tissue , | ctx | state . serialize_entry ("tissue" , ctx)) ? ; if self . value . r#value . id . as_deref () == Some ("$invalid") { return missing_field_error ("value") } self . with_context (& self . value . r#value , | ctx | state . serialize_entry ("value" , ctx)) ? ; if self . output_json { if let Some (some) = self . value . r#supporting_information . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("supportingInformation" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_supportingInformation" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#supporting_information . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("supportingInformation" , ctx)) ? ; } } state . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > { type Value = fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "tissue")] Tissue , # [serde (rename = "value")] Value , # [serde (rename = "supportingInformation")] SupportingInformation , # [serde (rename = "_supportingInformation")] SupportingInformationPrimitiveElement , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "tissue" , "value" , "supportingInformation" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#modifier_extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#tissue : Option < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > = None ; let mut r#value : Option < Box < fhirbolt_model :: r5 :: types :: Quantity > > = None ; let mut r#supporting_information : Option < fhirbolt_model :: r5 :: types :: String > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: Tissue => { if r#tissue . is_some () { return Err (serde :: de :: Error :: duplicate_field ("tissue")) ; } r#tissue = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ; } , Field :: Value => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("value")) ; } r#value = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: Quantity > > ()) ?) ; } , Field :: SupportingInformation => { if self . 0 . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#supporting_information . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } r#supporting_information = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: String > ()) ?) ; } } , Field :: SupportingInformationPrimitiveElement => { if self . 0 . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_supportingInformation")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("supportingInformation") ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#tissue : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#tissue . unwrap_or (Default :: default ()) } else { r#tissue . ok_or (serde :: de :: Error :: missing_field ("tissue")) ? } , r#value : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#value . unwrap_or (Default :: default ()) } else { r#value . ok_or (serde :: de :: Error :: missing_field ("value")) ? } , r#supporting_information , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { type Value = Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { type Value = Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; # [allow (dead_code)] fn missing_field_error < T , E : serde :: ser :: Error > (field : & str) -> Result < T , E > { Err (E :: custom (format ! ("missing required field `{}.{}`" , "AdministrableProductDefinition.routeOfAdministration.targetSpecies" , field))) } let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } if self . value . r#code . id . as_deref () == Some ("$invalid") { return missing_field_error ("code") } self . with_context (& self . value . r#code , | ctx | state . serialize_entry ("code" , ctx)) ? ; if ! self . value . r#withdrawal_period . is_empty () { self . with_context (& self . value . r#withdrawal_period , | ctx | state . serialize_entry ("withdrawalPeriod" , ctx)) ? ; } state . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > { type Value = fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("AdministrableProductDefinitionRouteOfAdministrationTargetSpecies") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "code")] Code , # [serde (rename = "withdrawalPeriod")] WithdrawalPeriod , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "code" , "withdrawalPeriod" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#modifier_extension : Option < Vec < fhirbolt_model :: r5 :: types :: Extension > > = None ; let mut r#code : Option < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > = None ; let mut r#withdrawal_period : Option < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Extension >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: Extension > ()) ?) ; } } , Field :: Code => { if r#code . is_some () { return Err (serde :: de :: Error :: duplicate_field ("code")) ; } r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ; } , Field :: WithdrawalPeriod => { if self . 0 . from_json { if r#withdrawal_period . is_some () { return Err (serde :: de :: Error :: duplicate_field ("withdrawalPeriod")) ; } r#withdrawal_period = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> ()) ?) ; } else { let vec = r#withdrawal_period . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ()) ?) ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#code : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#code . unwrap_or (Default :: default ()) } else { r#code . ok_or (serde :: de :: Error :: missing_field ("code")) ? } , r#withdrawal_period : r#withdrawal_period . unwrap_or (vec ! []) , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >> { type Value = Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >> { type Value = Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AdministrableProductDefinition.routeOfAdministration", field
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
        if self.value.r#code.id.as_deref() == Some("$invalid") {
            return missing_field_error("code");
        }
        self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        if let Some(some) = self.value.r#first_dose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("firstDose", ctx))?;
        }
        if let Some(some) = self.value.r#max_single_dose.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxSingleDose", ctx))?;
        }
        if let Some(some) = self.value.r#max_dose_per_day.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxDosePerDay", ctx))?;
        }
        if let Some(some) = self.value.r#max_dose_per_treatment_period.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("maxDosePerTreatmentPeriod", ctx)
            })?;
        }
        if let Some(some) = self.value.r#max_treatment_period.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("maxTreatmentPeriod", ctx))?;
        }
        if !self.value.r#target_species.is_empty() {
            self.with_context(&self.value.r#target_species, |ctx| {
                state.serialize_entry("targetSpecies", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>,
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
        &Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>,
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
        fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration,
    >
{
    type Value = fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinitionRouteOfAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration,
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
                    #[serde(rename = "code")]
                    Code,
                    #[serde(rename = "firstDose")]
                    FirstDose,
                    #[serde(rename = "maxSingleDose")]
                    MaxSingleDose,
                    #[serde(rename = "maxDosePerDay")]
                    MaxDosePerDay,
                    #[serde(rename = "maxDosePerTreatmentPeriod")]
                    MaxDosePerTreatmentPeriod,
                    #[serde(rename = "maxTreatmentPeriod")]
                    MaxTreatmentPeriod,
                    #[serde(rename = "targetSpecies")]
                    TargetSpecies,
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
                            "firstDose",
                            "maxSingleDose",
                            "maxDosePerDay",
                            "maxDosePerTreatmentPeriod",
                            "maxTreatmentPeriod",
                            "targetSpecies",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<fhirbolt_model::r5::types::Extension>> = None;
                let mut r#modifier_extension: Option<Vec<fhirbolt_model::r5::types::Extension>> =
                    None;
                let mut r#code: Option<Box<fhirbolt_model::r5::types::CodeableConcept>> = None;
                let mut r#first_dose: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#max_single_dose: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#max_dose_per_day: Option<Box<fhirbolt_model::r5::types::Quantity>> = None;
                let mut r#max_dose_per_treatment_period: Option<
                    Box<fhirbolt_model::r5::types::Ratio>,
                > = None;
                let mut r#max_treatment_period: Option<Box<fhirbolt_model::r5::types::Duration>> =
                    None;
                let mut r#target_species : Option < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > > = None ;
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
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::FirstDose => {
                            if r#first_dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstDose"));
                            }
                            r#first_dose = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::MaxSingleDose => {
                            if r#max_single_dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSingleDose"));
                            }
                            r#max_single_dose = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::MaxDosePerDay => {
                            if r#max_dose_per_day.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxDosePerDay"));
                            }
                            r#max_dose_per_day = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Quantity>>(),
                                )?,
                            );
                        }
                        Field::MaxDosePerTreatmentPeriod => {
                            if r#max_dose_per_treatment_period.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxDosePerTreatmentPeriod",
                                ));
                            }
                            r#max_dose_per_treatment_period = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r5::types::Ratio>>(),
                            )?);
                        }
                        Field::MaxTreatmentPeriod => {
                            if r#max_treatment_period.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "maxTreatmentPeriod",
                                ));
                            }
                            r#max_treatment_period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Duration>>(),
                                )?,
                            );
                        }
                        Field::TargetSpecies => {
                            if self.0.from_json {
                                if r#target_species.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetSpecies"));
                                }
                                r#target_species = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies >> ()) ?) ;
                            } else {
                                let vec = r#target_species.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministrationTargetSpecies > ()) ?) ;
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
                Ok (fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#code : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#code . unwrap_or (Default :: default ()) } else { r#code . ok_or (serde :: de :: Error :: missing_field ("code")) ? } , r#first_dose , r#max_single_dose , r#max_dose_per_day , r#max_dose_per_treatment_period , r#max_treatment_period , r#target_species : r#target_species . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>,
    >
{
    type Value =
        Box<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>,
    >
{
    type Value =
        Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r5::resources::AdministrableProductDefinitionRouteOfAdministration,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r5::resources::AdministrableProductDefinition {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r5::resources::AdministrableProductDefinition,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        #[allow(dead_code)]
        fn missing_field_error<T, E: serde::ser::Error>(field: &str) -> Result<T, E> {
            Err(E::custom(format!(
                "missing required field `{}.{}`",
                "AdministrableProductDefinition", field
            )))
        }
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "AdministrableProductDefinition")?;
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
        if self.output_json {
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
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
            if self.value.r#status.id.as_deref() == Some("$invalid") {
                return missing_field_error("status");
            }
            self.with_context(&self.value.r#status, |ctx| {
                state.serialize_entry("status", ctx)
            })?;
        }
        if !self.value.r#form_of.is_empty() {
            self.with_context(&self.value.r#form_of, |ctx| {
                state.serialize_entry("formOf", ctx)
            })?;
        }
        if let Some(some) = self.value.r#administrable_dose_form.as_ref() {
            self.with_context(some, |ctx| {
                state.serialize_entry("administrableDoseForm", ctx)
            })?;
        }
        if let Some(some) = self.value.r#unit_of_presentation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitOfPresentation", ctx))?;
        }
        if !self.value.r#produced_from.is_empty() {
            self.with_context(&self.value.r#produced_from, |ctx| {
                state.serialize_entry("producedFrom", ctx)
            })?;
        }
        if !self.value.r#ingredient.is_empty() {
            self.with_context(&self.value.r#ingredient, |ctx| {
                state.serialize_entry("ingredient", ctx)
            })?;
        }
        if let Some(some) = self.value.r#device.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("device", ctx))?;
        }
        if self.output_json {
            if let Some(some) = self.value.r#description.as_ref() {
                if let Some(some) = some.value.as_ref() {
                    let some = Ok(some)?;
                    state.serialize_entry("description", &some)?;
                }
                if some.id.is_some() || !some.extension.is_empty() {
                    let primitive_element = super::super::serde_helpers::PrimitiveElement {
                        id: some.id.as_ref(),
                        extension: &some.extension,
                    };
                    self.with_context(&primitive_element, |ctx| {
                        state.serialize_entry("_description", ctx)
                    })?;
                }
            }
        } else {
            if let Some(some) = self.value.r#description.as_ref() {
                self.with_context(some, |ctx| state.serialize_entry("description", ctx))?;
            }
        }
        if !self.value.r#property.is_empty() {
            self.with_context(&self.value.r#property, |ctx| {
                state.serialize_entry("property", ctx)
            })?;
        }
        if !self.value.r#route_of_administration.is_empty() {
            self.with_context(&self.value.r#route_of_administration, |ctx| {
                state.serialize_entry("routeOfAdministration", ctx)
            })?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
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
        &Vec<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
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
        fhirbolt_model::r5::resources::AdministrableProductDefinition,
    >
{
    type Value = fhirbolt_model::r5::resources::AdministrableProductDefinition;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r5::resources::AdministrableProductDefinition,
    >
{
    type Value = fhirbolt_model::r5::resources::AdministrableProductDefinition;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r5::resources::AdministrableProductDefinition,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r5::resources::AdministrableProductDefinition;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("AdministrableProductDefinition")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r5::resources::AdministrableProductDefinition, V::Error>
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
                    #[serde(rename = "status")]
                    Status,
                    #[serde(rename = "_status")]
                    StatusPrimitiveElement,
                    #[serde(rename = "formOf")]
                    FormOf,
                    #[serde(rename = "administrableDoseForm")]
                    AdministrableDoseForm,
                    #[serde(rename = "unitOfPresentation")]
                    UnitOfPresentation,
                    #[serde(rename = "producedFrom")]
                    ProducedFrom,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "property")]
                    Property,
                    #[serde(rename = "routeOfAdministration")]
                    RouteOfAdministration,
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
                            "formOf",
                            "administrableDoseForm",
                            "unitOfPresentation",
                            "producedFrom",
                            "ingredient",
                            "device",
                            "description",
                            "property",
                            "routeOfAdministration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
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
                let mut r#form_of: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#administrable_dose_form: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#unit_of_presentation: Option<
                    Box<fhirbolt_model::r5::types::CodeableConcept>,
                > = None;
                let mut r#produced_from: Option<Vec<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#ingredient: Option<Vec<fhirbolt_model::r5::types::CodeableConcept>> =
                    None;
                let mut r#device: Option<Box<fhirbolt_model::r5::types::Reference>> = None;
                let mut r#description: Option<fhirbolt_model::r5::types::Markdown> = None;
                let mut r#property: Option<
                    Vec<fhirbolt_model::r5::resources::AdministrableProductDefinitionProperty>,
                > = None;
                let mut r#route_of_administration : Option < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration > > = None ;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "AdministrableProductDefinition" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"AdministrableProductDefinition",
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
                                    self.0.transmute::<Vec<fhirbolt_model::r5::Resource>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::Resource>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::ModifierExtension => {
                            if self.0.from_json {
                                if r#modifier_extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "modifierExtension",
                                    ));
                                }
                                r#modifier_extension = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Extension>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Extension>(),
                                )?);
                            }
                        }
                        Field::Identifier => {
                            if self.0.from_json {
                                if r#identifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("identifier"));
                                }
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: Identifier >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Identifier>(),
                                )?);
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
                        Field::FormOf => {
                            if self.0.from_json {
                                if r#form_of.is_some() {
                                    return Err(serde::de::Error::duplicate_field("formOf"));
                                }
                                r#form_of = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#form_of.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::AdministrableDoseForm => {
                            if r#administrable_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrableDoseForm",
                                ));
                            }
                            r#administrable_dose_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::UnitOfPresentation => {
                            if r#unit_of_presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unitOfPresentation",
                                ));
                            }
                            r#unit_of_presentation = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r5 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ProducedFrom => {
                            if self.0.from_json {
                                if r#produced_from.is_some() {
                                    return Err(serde::de::Error::duplicate_field("producedFrom"));
                                }
                                r#produced_from = Some(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Vec<fhirbolt_model::r5::types::Reference>>(
                                            ),
                                    )?,
                                );
                            } else {
                                let vec = r#produced_from.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Reference>(),
                                )?);
                            }
                        }
                        Field::Ingredient => {
                            if self.0.from_json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: types :: CodeableConcept >> ()) ?) ;
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: types :: CodeableConcept > ()) ?) ;
                            }
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r5::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Description => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#description.is_some() {
                                    return Err(serde::de::Error::duplicate_field("description"));
                                }
                                r#description = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r5::types::Markdown>(),
                                )?);
                            }
                        }
                        Field::DescriptionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#description.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_description"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("description");
                            }
                        }
                        Field::Property => {
                            if self.0.from_json {
                                if r#property.is_some() {
                                    return Err(serde::de::Error::duplicate_field("property"));
                                }
                                r#property = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionProperty >> ()) ?) ;
                            } else {
                                let vec = r#property.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionProperty > ()) ?) ;
                            }
                        }
                        Field::RouteOfAdministration => {
                            if self.0.from_json {
                                if r#route_of_administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "routeOfAdministration",
                                    ));
                                }
                                r#route_of_administration = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration >> ()) ?) ;
                            } else {
                                let vec =
                                    r#route_of_administration.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinitionRouteOfAdministration > ()) ?) ;
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
                    fhirbolt_model::r5::resources::AdministrableProductDefinition {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#status: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#status.unwrap_or(Default::default())
                        } else {
                            r#status.ok_or(serde::de::Error::missing_field("status"))?
                        },
                        r#form_of: r#form_of.unwrap_or(vec![]),
                        r#administrable_dose_form,
                        r#unit_of_presentation,
                        r#produced_from: r#produced_from.unwrap_or(vec![]),
                        r#ingredient: r#ingredient.unwrap_or(vec![]),
                        r#device,
                        r#description,
                        r#property: r#property.unwrap_or(vec![]),
                        r#route_of_administration: r#route_of_administration.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
    >
{
    type Value = Box<fhirbolt_model::r5::resources::AdministrableProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::resources::AdministrableProductDefinition>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
    >
{
    type Value = Vec<fhirbolt_model::r5::resources::AdministrableProductDefinition>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r5::resources::AdministrableProductDefinition>;
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
                        .transmute::<fhirbolt_model::r5::resources::AdministrableProductDefinition>(
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
