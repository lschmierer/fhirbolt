// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength") } fn visit_map < V > (self , mut map_access : V) -> Result < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength , V :: Error > where V : serde :: de :: MapAccess < 'de > , { # [derive (serde :: Deserialize)] # [serde (field_identifier)] enum Field { # [serde (rename = "id")] Id , # [serde (rename = "extension")] Extension , # [serde (rename = "modifierExtension")] ModifierExtension , # [serde (rename = "substance")] Substance , # [serde (rename = "strength")] Strength , # [serde (rename = "strengthLowLimit")] StrengthLowLimit , # [serde (rename = "measurementPoint")] MeasurementPoint , # [serde (rename = "_measurementPoint")] MeasurementPointPrimitiveElement , # [serde (rename = "country")] Country , Unknown (std :: string :: String) , } fn unknown_field_error < T , E : serde :: de :: Error > (field : & str) -> Result < T , E > { Err (E :: unknown_field (field , & ["id" , "extension" , "modifierExtension" , "substance" , "strength" , "strengthLowLimit" , "measurementPoint" , "country" ,])) } let mut r#id : Option < std :: string :: String > = None ; let mut r#extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#modifier_extension : Option < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > > > = None ; let mut r#substance : Option < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > = None ; let mut r#strength : Option < Box < fhirbolt_model :: r4 :: types :: Ratio > > = None ; let mut r#strength_low_limit : Option < Box < fhirbolt_model :: r4 :: types :: Ratio > > = None ; let mut r#measurement_point : Option < fhirbolt_model :: r4 :: types :: String > = None ; let mut r#country : Option < Vec < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > > = None ; while let Some (map_access_key) = map_access . next_key () ? { match map_access_key { Field :: Id => { if r#id . is_some () { return Err (serde :: de :: Error :: duplicate_field ("id")) ; } r#id = Some (map_access . next_value () ?) ; } , Field :: Extension => { if self . 0 . from_json { if r#extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("extension")) ; } r#extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: ModifierExtension => { if self . 0 . from_json { if r#modifier_extension . is_some () { return Err (serde :: de :: Error :: duplicate_field ("modifierExtension")) ; } r#modifier_extension = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Extension > >> ()) ?) ; } else { let vec = r#modifier_extension . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Extension > > ()) ?) ; } } , Field :: Substance => { if r#substance . is_some () { return Err (serde :: de :: Error :: duplicate_field ("substance")) ; } r#substance = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ; } , Field :: Strength => { if r#strength . is_some () { return Err (serde :: de :: Error :: duplicate_field ("strength")) ; } r#strength = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Ratio > > ()) ?) ; } , Field :: StrengthLowLimit => { if r#strength_low_limit . is_some () { return Err (serde :: de :: Error :: duplicate_field ("strengthLowLimit")) ; } r#strength_low_limit = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Ratio > > ()) ?) ; } , Field :: MeasurementPoint => { if self . 0 . from_json { let some = r#measurement_point . get_or_insert (Default :: default ()) ; if some . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("measurementPoint")) ; } let value : _ = map_access . next_value () ? ; some . value = Some (value) ; } else { if r#measurement_point . is_some () { return Err (serde :: de :: Error :: duplicate_field ("measurementPoint")) ; } r#measurement_point = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: types :: String > ()) ?) ; } } , Field :: MeasurementPointPrimitiveElement => { if self . 0 . from_json { let some = r#measurement_point . get_or_insert (Default :: default ()) ; if some . id . is_some () || ! some . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_measurementPoint")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; some . id = id ; some . extension = extension ; } else { return unknown_field_error ("measurementPoint") ; } } , Field :: Country => { if self . 0 . from_json { if r#country . is_some () { return Err (serde :: de :: Error :: duplicate_field ("country")) ; } r#country = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > >> ()) ?) ; } else { let vec = r#country . get_or_insert (Default :: default ()) ; vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ; } } , Field :: Unknown (key) => if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Strict { return unknown_field_error (& key) ; } } } Ok (fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#substance , r#strength : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#strength . unwrap_or (Default :: default ()) } else { r#strength . ok_or (serde :: de :: Error :: missing_field ("strength")) ? } , r#strength_low_limit , r#measurement_point , r#country : r#country . unwrap_or (vec ! []) , }) } } deserializer . deserialize_map (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> { type Value = Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > () . deserialize (deserializer) . map (Box :: new) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >>) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl < 'de > serde :: de :: DeserializeSeed < 'de > for & mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> >) ; impl < 'de > serde :: de :: Visitor < 'de > for Visitor < '_ > { type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: new () ; while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> ()) ? { values . push (value) ; } Ok (values) } } deserializer . deserialize_seq (Visitor (self)) } }
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength,
    >
{
    type Value =
        fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSpecifiedSubstanceStrength")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength,
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
                    #[serde(rename = "presentation")]
                    Presentation,
                    #[serde(rename = "presentationLowLimit")]
                    PresentationLowLimit,
                    #[serde(rename = "concentration")]
                    Concentration,
                    #[serde(rename = "concentrationLowLimit")]
                    ConcentrationLowLimit,
                    #[serde(rename = "measurementPoint")]
                    MeasurementPoint,
                    #[serde(rename = "_measurementPoint")]
                    MeasurementPointPrimitiveElement,
                    #[serde(rename = "country")]
                    Country,
                    #[serde(rename = "referenceStrength")]
                    ReferenceStrength,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "presentation",
                            "presentationLowLimit",
                            "concentration",
                            "concentrationLowLimit",
                            "measurementPoint",
                            "country",
                            "referenceStrength",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#presentation: Option<Box<fhirbolt_model::r4::types::Ratio>> = None;
                let mut r#presentation_low_limit: Option<Box<fhirbolt_model::r4::types::Ratio>> =
                    None;
                let mut r#concentration: Option<Box<fhirbolt_model::r4::types::Ratio>> = None;
                let mut r#concentration_low_limit: Option<Box<fhirbolt_model::r4::types::Ratio>> =
                    None;
                let mut r#measurement_point: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#country: Option<Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>> =
                    None;
                let mut r#reference_strength : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > > = None ;
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
                        Field::Presentation => {
                            if r#presentation.is_some() {
                                return Err(serde::de::Error::duplicate_field("presentation"));
                            }
                            r#presentation = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
                            )?);
                        }
                        Field::PresentationLowLimit => {
                            if r#presentation_low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "presentationLowLimit",
                                ));
                            }
                            r#presentation_low_limit = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
                            )?);
                        }
                        Field::Concentration => {
                            if r#concentration.is_some() {
                                return Err(serde::de::Error::duplicate_field("concentration"));
                            }
                            r#concentration = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
                            )?);
                        }
                        Field::ConcentrationLowLimit => {
                            if r#concentration_low_limit.is_some() {
                                return Err(serde::de::Error::duplicate_field(
                                    "concentrationLowLimit",
                                ));
                            }
                            r#concentration_low_limit = Some(map_access.next_value_seed(
                                self.0.transmute::<Box<fhirbolt_model::r4::types::Ratio>>(),
                            )?);
                        }
                        Field::MeasurementPoint => {
                            if self.0.from_json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#measurement_point.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "measurementPoint",
                                    ));
                                }
                                r#measurement_point = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::MeasurementPointPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#measurement_point.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_measurementPoint",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("measurementPoint");
                            }
                        }
                        Field::Country => {
                            if self.0.from_json {
                                if r#country.is_some() {
                                    return Err(serde::de::Error::duplicate_field("country"));
                                }
                                r#country =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#country.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::ReferenceStrength => {
                            if self.0.from_json {
                                if r#reference_strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "referenceStrength",
                                    ));
                                }
                                r#reference_strength = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength >> ()) ?) ;
                            } else {
                                let vec = r#reference_strength.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength > ()) ?) ;
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
                Ok (fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#presentation : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#presentation . unwrap_or (Default :: default ()) } else { r#presentation . ok_or (serde :: de :: Error :: missing_field ("presentation")) ? } , r#presentation_low_limit , r#concentration , r#concentration_low_limit , r#measurement_point , r#country : r#country . unwrap_or (vec ! []) , r#reference_strength : r#reference_strength . unwrap_or (vec ! []) , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength>,
    >
{
    type Value =
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >>) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<
            Box<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength,
            >,
        >,
    >
{
    type Value = Vec<
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstanceStrength>,
    >;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >> ;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSpecifiedSubstance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
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
                    #[serde(rename = "group")]
                    Group,
                    #[serde(rename = "confidentiality")]
                    Confidentiality,
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
                            "code",
                            "group",
                            "confidentiality",
                            "strength",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#group: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#confidentiality: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> =
                    None;
                let mut r#strength : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > > = None ;
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
                        Field::Group => {
                            if r#group.is_some() {
                                return Err(serde::de::Error::duplicate_field("group"));
                            }
                            r#group = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Confidentiality => {
                            if r#confidentiality.is_some() {
                                return Err(serde::de::Error::duplicate_field("confidentiality"));
                            }
                            r#confidentiality = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Strength => {
                            if self.0.from_json {
                                if r#strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                r#strength = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >> ()) ?) ;
                            } else {
                                let vec = r#strength.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > ()) ?) ;
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
                    fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance {
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
                        r#group: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#group.unwrap_or(Default::default())
                        } else {
                            r#group.ok_or(serde::de::Error::missing_field("group"))?
                        },
                        r#confidentiality,
                        r#strength: r#strength.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstance > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstance > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<
                    Box<
                        fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
                    >,
                >,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance>,
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
                    fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
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
        fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredientSubstance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance, V::Error>
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
                    #[serde(rename = "strength")]
                    Strength,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code", "strength"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#strength : Option < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > > = None ;
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
                        Field::Strength => {
                            if self.0.from_json {
                                if r#strength.is_some() {
                                    return Err(serde::de::Error::duplicate_field("strength"));
                                }
                                r#strength = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength >> ()) ?) ;
                            } else {
                                let vec = r#strength.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstanceStrength > ()) ?) ;
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
                    fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance {
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
                        r#strength: r#strength.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSubstance > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSubstance >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4::resources::MedicinalProductIngredient {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::MedicinalProductIngredient,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductIngredient;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4::resources::MedicinalProductIngredient,
    >
{
    type Value = fhirbolt_model::r4::resources::MedicinalProductIngredient;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::MedicinalProductIngredient,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::MedicinalProductIngredient;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("MedicinalProductIngredient")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::MedicinalProductIngredient, V::Error>
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
                    #[serde(rename = "role")]
                    Role,
                    #[serde(rename = "allergenicIndicator")]
                    AllergenicIndicator,
                    #[serde(rename = "_allergenicIndicator")]
                    AllergenicIndicatorPrimitiveElement,
                    #[serde(rename = "manufacturer")]
                    Manufacturer,
                    #[serde(rename = "specifiedSubstance")]
                    SpecifiedSubstance,
                    #[serde(rename = "substance")]
                    Substance,
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
                            "role",
                            "allergenicIndicator",
                            "manufacturer",
                            "specifiedSubstance",
                            "substance",
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
                let mut r#identifier: Option<Box<fhirbolt_model::r4::types::Identifier>> = None;
                let mut r#role: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#allergenic_indicator: Option<fhirbolt_model::r4::types::Boolean> = None;
                let mut r#manufacturer: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> =
                    None;
                let mut r#specified_substance: Option<
                    Vec<
                        fhirbolt_model::r4::resources::MedicinalProductIngredientSpecifiedSubstance,
                    >,
                > = None;
                let mut r#substance: Option<
                    fhirbolt_model::r4::resources::MedicinalProductIngredientSubstance,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "MedicinalProductIngredient" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"MedicinalProductIngredient",
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
                            if r#identifier.is_some() {
                                return Err(serde::de::Error::duplicate_field("identifier"));
                            }
                            r#identifier = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Identifier>>(),
                                )?,
                            );
                        }
                        Field::Role => {
                            if r#role.is_some() {
                                return Err(serde::de::Error::duplicate_field("role"));
                            }
                            r#role = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::AllergenicIndicator => {
                            if self.0.from_json {
                                let some = r#allergenic_indicator.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergenicIndicator",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#allergenic_indicator.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allergenicIndicator",
                                    ));
                                }
                                r#allergenic_indicator = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::AllergenicIndicatorPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#allergenic_indicator.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_allergenicIndicator",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("allergenicIndicator");
                            }
                        }
                        Field::Manufacturer => {
                            if self.0.from_json {
                                if r#manufacturer.is_some() {
                                    return Err(serde::de::Error::duplicate_field("manufacturer"));
                                }
                                r#manufacturer = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#manufacturer.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
                                            ),
                                    )?,
                                );
                            }
                        }
                        Field::SpecifiedSubstance => {
                            if self.0.from_json {
                                if r#specified_substance.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "specifiedSubstance",
                                    ));
                                }
                                r#specified_substance = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstance >> ()) ?) ;
                            } else {
                                let vec = r#specified_substance.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSpecifiedSubstance > ()) ?) ;
                            }
                        }
                        Field::Substance => {
                            if r#substance.is_some() {
                                return Err(serde::de::Error::duplicate_field("substance"));
                            }
                            r#substance = Some (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredientSubstance > ()) ?) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(fhirbolt_model::r4::resources::MedicinalProductIngredient {
                    r#id,
                    r#meta,
                    r#implicit_rules,
                    r#language,
                    r#text,
                    r#contained: r#contained.unwrap_or(vec![]),
                    r#extension: r#extension.unwrap_or(vec![]),
                    r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                    r#identifier,
                    r#role: if self.0.config.mode == crate::context::de::DeserializationMode::Lax {
                        r#role.unwrap_or(Default::default())
                    } else {
                        r#role.ok_or(serde::de::Error::missing_field("role"))?
                    },
                    r#allergenic_indicator,
                    r#manufacturer: r#manufacturer.unwrap_or(vec![]),
                    r#specified_substance: r#specified_substance.unwrap_or(vec![]),
                    r#substance,
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>,
    >
{
    type Value = Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::MedicinalProductIngredient>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4::resources::MedicinalProductIngredient>,
    >
{
    type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductIngredient>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::MedicinalProductIngredient>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::MedicinalProductIngredient>;
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
                        .transmute::<fhirbolt_model::r4::resources::MedicinalProductIngredient>(),
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
        Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::MedicinalProductIngredient>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredient >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
