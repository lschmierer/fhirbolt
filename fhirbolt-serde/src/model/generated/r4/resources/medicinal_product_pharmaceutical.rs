// Generated on 2023-04-14 by fhirbolt-codegen v0.1.0
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics,
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
        self.with_context(&self.value.r#code, |ctx| state.serialize_entry("code", ctx))?;
        if let Some(some) = self.value.r#status.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("status", ctx))?;
        }
        state.end()
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
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
        &Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
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
        &Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>>,
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
        fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceuticalCharacteristics")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics,
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
                    #[serde(rename = "status")]
                    Status,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code", "status"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#status: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Status => {
                            if r#status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            r#status = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
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
                    fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics {
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
        Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics>,
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
                    fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalCharacteristics,
                >>())? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } self . with_context (& self . value . r#tissue , | ctx | state . serialize_entry ("tissue" , ctx)) ? ; self . with_context (& self . value . r#value , | ctx | state . serialize_entry ("value" , ctx)) ? ; if self . output_json { if let Some (some) = self . value . r#supporting_information . as_ref () { if let Some (some) = some . value . as_ref () { let some = Ok (some) ? ; state . serialize_entry ("supportingInformation" , & some) ? ; } if some . id . is_some () || ! some . extension . is_empty () { let primitive_element = super :: super :: serde_helpers :: PrimitiveElement { id : some . id . as_ref () , extension : & some . extension , } ; self . with_context (& primitive_element , | ctx | state . serialize_entry ("_supportingInformation" , ctx)) ? ; } } } else { if let Some (some) = self . value . r#supporting_information . as_ref () { self . with_context (some , | ctx | state . serialize_entry ("supportingInformation" , ctx)) ? ; } } state . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "tissue")] Tissue , # [serde (rename = "value")] Value , # [serde (rename = "supportingInformation")] SupportingInformation , # [serde (rename = "_supportingInformation")] SupportingInformationPrimitiveElement , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "tissue" , "value" , "supportingInformation" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#tissue : Option < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > = None ; let mut r#value : Option < Box < fhirbolt_model :: r4 :: types :: Quantity > > = None ; let mut r#supporting_information : Option < fhirbolt_model :: r4 :: types :: String > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: Tissue => { if r#tissue . is_some () { return Err (serde :: de :: Error :: duplicate_field ("tissue")) ; } r#tissue = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ; } , Field :: Value => { if r#value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("value")) ; } r#value = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Quantity > > ()) ?) ; } , Field :: SupportingInformation => { if self . 0 . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#supporting_information . is_some () { return Err (serde :: de :: Error :: duplicate_field ("supportingInformation")) ; } r#supporting_information = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: types :: String > ()) ?) ; } } , Field :: SupportingInformationPrimitiveElement => { if self . 0 . from_json { let some = r#supporting_information . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_supportingInformation")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("supportingInformation") ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#tissue : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#tissue . unwrap_or (Default :: default ()) } else { r#tissue . ok_or (serde :: de :: Error :: missing_field ("tissue")) ? } , r#value : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#value . unwrap_or (Default :: default ()) } else { r#value . ok_or (serde :: de :: Error :: missing_field ("value")) ? } , r#supporting_information , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { type Value = Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeMap ; let mut state = serializer . serialize_map (None) ? ; if let Some (value) = self . value . r#id . as_ref () { state . serialize_entry ("id" , value) ? ; } if ! self . value . r#extension . is_empty () { self . with_context (& self . value . r#extension , | ctx | state . serialize_entry ("extension" , ctx)) ? ; } if ! self . value . r#modifier_extension . is_empty () { self . with_context (& self . value . r#modifier_extension , | ctx | state . serialize_entry ("modifierExtension" , ctx)) ? ; } self . with_context (& self . value . r#code , | ctx | state . serialize_entry ("code" , ctx)) ? ; if ! self . value . r#withdrawal_period . is_empty () { self . with_context (& self . value . r#withdrawal_period , | ctx | state . serialize_entry ("withdrawalPeriod" , ctx)) ? ; } state . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { self . with_context (self . value . as_ref () , | ctx | ctx . serialize (serializer)) } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl serde :: ser :: Serialize for crate :: context :: ser :: SerializationContext < & Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: ser :: Serializer , { use serde :: ser :: SerializeSeq ; let mut seq_serializer = serializer . serialize_seq (Some (self . value . len ())) ? ; for value in self . value { self . with_context (value , | ctx | { seq_serializer . serialize_element (ctx) }) ? } seq_serializer . end () } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "code")] Code , # [serde (rename = "withdrawalPeriod")] WithdrawalPeriod , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "code" , "withdrawalPeriod" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#code : Option < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > = None ; let mut r#withdrawal_period : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: Code => { if r#code . is_some () { return Err (serde :: de :: Error :: duplicate_field ("code")) ; } r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ; } , Field :: WithdrawalPeriod => { if self . 0 . from_json { if r#withdrawal_period . is_some () { return Err (serde :: de :: Error :: duplicate_field ("withdrawalPeriod")) ; } r#withdrawal_period = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod >> ()) ?) ; } else { let vec = r#withdrawal_period . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod > ()) ?) ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#code : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#code . unwrap_or (Default :: default ()) } else { r#code . ok_or (serde :: de :: Error :: missing_field ("code")) ? } , r#withdrawal_period : r#withdrawal_period . unwrap_or (vec ! []) , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> { type Value = Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration,
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
        &Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
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
        &Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
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
        &Vec<
            Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
        >,
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
        fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceuticalRouteOfAdministration")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration,
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
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#first_dose: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#max_single_dose: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#max_dose_per_day: Option<Box<fhirbolt_model::r4::types::Quantity>> = None;
                let mut r#max_dose_per_treatment_period: Option<
                    Box<fhirbolt_model::r4::types::Ratio>,
                > = None;
                let mut r#max_treatment_period: Option<Box<fhirbolt_model::r4::types::Duration>> =
                    None;
                let mut r#target_species : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > > = None ;
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
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Code => {
                            if r#code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            r#code = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::FirstDose => {
                            if r#first_dose.is_some() {
                                return Err(serde::de::Error::duplicate_field("firstDose"));
                            }
                            r#first_dose = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Quantity>>(),
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
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Duration>>(),
                                )?,
                            );
                        }
                        Field::TargetSpecies => {
                            if self.0.from_json {
                                if r#target_species.is_some() {
                                    return Err(serde::de::Error::duplicate_field("targetSpecies"));
                                }
                                r#target_species = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies >> ()) ?) ;
                            } else {
                                let vec = r#target_species.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies > ()) ?) ;
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
                Ok (fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#code : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#code . unwrap_or (Default :: default ()) } else { r#code . ok_or (serde :: de :: Error :: missing_field ("code")) ? } , r#first_dose , r#max_single_dose , r#max_dose_per_day , r#max_dose_per_treatment_period , r#max_treatment_period , r#target_species : r#target_species . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
    >
{
    type Value =
        Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<
            Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
        >,
    >
{
    type Value = Vec<
        Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceuticalRouteOfAdministration>,
    >;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration >> ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::MedicinalProductPharmaceutical {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<
        &fhirbolt_model::r4::resources::MedicinalProductPharmaceutical,
    >
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut state = serializer.serialize_map(None)?;
        state.serialize_entry("resourceType", "MedicinalProductPharmaceutical")?;
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
        self.with_context(&self.value.r#administrable_dose_form, |ctx| {
            state.serialize_entry("administrableDoseForm", ctx)
        })?;
        if let Some(some) = self.value.r#unit_of_presentation.as_ref() {
            self.with_context(some, |ctx| state.serialize_entry("unitOfPresentation", ctx))?;
        }
        if !self.value.r#ingredient.is_empty() {
            self.with_context(&self.value.r#ingredient, |ctx| {
                state.serialize_entry("ingredient", ctx)
            })?;
        }
        if !self.value.r#device.is_empty() {
            self.with_context(&self.value.r#device, |ctx| {
                state.serialize_entry("device", ctx)
            })?;
        }
        if !self.value.r#characteristics.is_empty() {
            self.with_context(&self.value.r#characteristics, |ctx| {
                state.serialize_entry("characteristics", ctx)
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
        &Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>,
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
        &Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>,
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
        &Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>>,
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
        fhirbolt_model::r4::resources::MedicinalProductPharmaceutical,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductPharmaceutical;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::MedicinalProductPharmaceutical,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductPharmaceutical;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductPharmaceutical,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::MedicinalProductPharmaceutical;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductPharmaceutical")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical, V::Error>
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
                    #[serde(rename = "administrableDoseForm")]
                    AdministrableDoseForm,
                    #[serde(rename = "unitOfPresentation")]
                    UnitOfPresentation,
                    #[serde(rename = "ingredient")]
                    Ingredient,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "characteristics")]
                    Characteristics,
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
                            "administrableDoseForm",
                            "unitOfPresentation",
                            "ingredient",
                            "device",
                            "characteristics",
                            "routeOfAdministration",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#meta: Option<Box<fhirbolt_model::r4::types::Meta>> = None;
                let mut r#implicit_rules: Option<fhirbolt_model::r4::types::Uri> = None;
                let mut r#language: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#text: Option<Box<fhirbolt_model::r4::types::Narrative>> = None;
                let mut r#contained: Option<Vec<Box<fhirbolt_model::r4::Resource>>> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4::types::Identifier>>> =
                    None;
                let mut r#administrable_dose_form: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#unit_of_presentation: Option<
                    Box<fhirbolt_model::r4::types::CodeableConcept>,
                > = None;
                let mut r#ingredient: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#device: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#characteristics : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics > > = None ;
                let mut r#route_of_administration : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration > > = None ;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductPharmaceutical" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductPharmaceutical",
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
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Meta>>(),
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Uri>(),
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Narrative>>(),
                                )?,
                            );
                        }
                        Field::Contained => {
                            if self.0.from_json {
                                if r#contained.is_some() {
                                    return Err(serde::de::Error::duplicate_field("contained"));
                                }
                                r#contained = Some(map_access.next_value_seed(
                                    self.0.transmute::<Vec<Box<fhirbolt_model::r4::Resource>>>(),
                                )?);
                            } else {
                                let vec = r#contained.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::Resource>>(),
                                )?);
                            }
                        }
                        Field::Extension => {
                            if self.0.from_json {
                                if r#extension.is_some() {
                                    return Err(serde::de::Error::duplicate_field("extension"));
                                }
                                r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
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
                                r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ;
                            } else {
                                let vec = r#modifier_extension.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Extension>>(
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
                                r#identifier = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Identifier > >> ()) ?) ;
                            } else {
                                let vec = r#identifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Identifier > > ()) ?) ;
                            }
                        }
                        Field::AdministrableDoseForm => {
                            if r#administrable_dose_form.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "administrableDoseForm",
                                ));
                            }
                            r#administrable_dose_form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::UnitOfPresentation => {
                            if r#unit_of_presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "unitOfPresentation",
                                ));
                            }
                            r#unit_of_presentation = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Ingredient => {
                            if self.0.from_json {
                                if r#ingredient.is_some() {
                                    return Err(serde::de::Error::duplicate_field("ingredient"));
                                }
                                r#ingredient = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#ingredient.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Device => {
                            if self.0.from_json {
                                if r#device.is_some() {
                                    return Err(serde::de::Error::duplicate_field("device"));
                                }
                                r#device = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#device.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::Characteristics => {
                            if self.0.from_json {
                                if r#characteristics.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "characteristics",
                                    ));
                                }
                                r#characteristics = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics >> ()) ?) ;
                            } else {
                                let vec = r#characteristics.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalCharacteristics > ()) ?) ;
                            }
                        }
                        Field::RouteOfAdministration => {
                            if self.0.from_json {
                                if r#route_of_administration.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "routeOfAdministration",
                                    ));
                                }
                                r#route_of_administration = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration >> ()) ?) ;
                            } else {
                                let vec =
                                    r#route_of_administration.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceuticalRouteOfAdministration > ()) ?) ;
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
                    fhirbolt_model::r4::resources::MedicinalProductPharmaceutical {
                        r#id,
                        r#meta,
                        r#implicit_rules,
                        r#language,
                        r#text,
                        r#contained: r#contained.unwrap_or(vec![]),
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#identifier: r#identifier.unwrap_or(vec![]),
                        r#administrable_dose_form: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#administrable_dose_form.unwrap_or(Default::default())
                        } else {
                            r#administrable_dose_form
                                .ok_or(serde::de::Error::missing_field("administrableDoseForm"))?
                        },
                        r#unit_of_presentation,
                        r#ingredient: r#ingredient.unwrap_or(vec![]),
                        r#device: r#device.unwrap_or(vec![]),
                        r#characteristics: r#characteristics.unwrap_or(vec![]),
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
        Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>;
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
                        .transmute::<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>(
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
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::MedicinalProductPharmaceutical>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceutical >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
