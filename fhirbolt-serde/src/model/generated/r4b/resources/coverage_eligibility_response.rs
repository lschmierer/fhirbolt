// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItemBenefit")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
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
                    #[serde(rename = "allowedUnsignedInt")]
                    AllowedUnsignedInt,
                    #[serde(rename = "_allowedUnsignedInt")]
                    AllowedUnsignedIntPrimitiveElement,
                    #[serde(rename = "allowedString")]
                    AllowedString,
                    #[serde(rename = "_allowedString")]
                    AllowedStringPrimitiveElement,
                    #[serde(rename = "allowedMoney")]
                    AllowedMoney,
                    #[serde(rename = "usedUnsignedInt")]
                    UsedUnsignedInt,
                    #[serde(rename = "_usedUnsignedInt")]
                    UsedUnsignedIntPrimitiveElement,
                    #[serde(rename = "usedString")]
                    UsedString,
                    #[serde(rename = "_usedString")]
                    UsedStringPrimitiveElement,
                    #[serde(rename = "usedMoney")]
                    UsedMoney,
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
                            "allowedUnsignedInt",
                            "allowedString",
                            "allowedMoney",
                            "usedUnsignedInt",
                            "usedString",
                            "usedMoney",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#type: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#allowed : Option < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed > = None ;
                let mut r#used : Option < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed > = None ;
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
                        Field::AllowedUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#allowed . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; }
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "allowedUnsignedInt",
                                    ));
                                }
                                r#allowed = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::AllowedUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#allowed . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; }
                            } else {
                                return unknown_field_error("allowedUnsignedInt");
                            }
                        }
                        Field::AllowedString => {
                            if self.0.from_json {
                                let r#enum = r#allowed . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("allowedString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("allowed[x]")) ; }
                            } else {
                                if r#allowed.is_some() {
                                    return Err(serde::de::Error::duplicate_field("allowedString"));
                                }
                                r#allowed = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::AllowedStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#allowed . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_allowedString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_allowed[x]")) ; }
                            } else {
                                return unknown_field_error("allowedString");
                            }
                        }
                        Field::AllowedMoney => {
                            if r#allowed.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedMoney"));
                            }
                            r#allowed = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitAllowed :: Money (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Money > > ()) ?)) ;
                        }
                        Field::UsedUnsignedInt => {
                            if self.0.from_json {
                                let r#enum = r#used . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedUnsignedInt")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("used[x]")) ; }
                            } else {
                                if r#used.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "usedUnsignedInt",
                                    ));
                                }
                                r#used = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: UnsignedInt > > ()) ?)) ;
                            }
                        }
                        Field::UsedUnsignedIntPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#used . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: UnsignedInt (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_usedUnsignedInt")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_used[x]")) ; }
                            } else {
                                return unknown_field_error("usedUnsignedInt");
                            }
                        }
                        Field::UsedString => {
                            if self.0.from_json {
                                let r#enum = r#used . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("usedString")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("used[x]")) ; }
                            } else {
                                if r#used.is_some() {
                                    return Err(serde::de::Error::duplicate_field("usedString"));
                                }
                                r#used = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: String > > ()) ?)) ;
                            }
                        }
                        Field::UsedStringPrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#used . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: String (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_usedString")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_used[x]")) ; }
                            } else {
                                return unknown_field_error("usedString");
                            }
                        }
                        Field::UsedMoney => {
                            if r#used.is_some() {
                                return Err(serde::de::Error::duplicate_field("usedMoney"));
                            }
                            r#used = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefitUsed :: Money (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Money > > ()) ?)) ;
                        }
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit { r#id , r#extension : r#extension . unwrap_or (vec ! []) , r#modifier_extension : r#modifier_extension . unwrap_or (vec ! []) , r#type : if self . 0 . config . mode == crate :: context :: de :: DeserializationMode :: Lax { r#type . unwrap_or (Default :: default ()) } else { r#type . ok_or (serde :: de :: Error :: missing_field ("type")) ? } , r#allowed , r#used , })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>,
    >
{
    type Value =
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > () . deserialize (deserializer) . map (Box :: new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>,
    >
{
    type Value =
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
                >,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
            >;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>>,
    >
{
    type Value =
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor < 'a > (& 'a mut crate :: context :: de :: DeserializationContext < Vec < Box < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit >> >) ;
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<
                Box<
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
                >,
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
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItemBenefit,
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
        fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsuranceItem")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem,
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
                    #[serde(rename = "category")]
                    Category,
                    #[serde(rename = "productOrService")]
                    ProductOrService,
                    #[serde(rename = "modifier")]
                    Modifier,
                    #[serde(rename = "provider")]
                    Provider,
                    #[serde(rename = "excluded")]
                    Excluded,
                    #[serde(rename = "_excluded")]
                    ExcludedPrimitiveElement,
                    #[serde(rename = "name")]
                    Name,
                    #[serde(rename = "_name")]
                    NamePrimitiveElement,
                    #[serde(rename = "description")]
                    Description,
                    #[serde(rename = "_description")]
                    DescriptionPrimitiveElement,
                    #[serde(rename = "network")]
                    Network,
                    #[serde(rename = "unit")]
                    Unit,
                    #[serde(rename = "term")]
                    Term,
                    #[serde(rename = "benefit")]
                    Benefit,
                    #[serde(rename = "authorizationRequired")]
                    AuthorizationRequired,
                    #[serde(rename = "_authorizationRequired")]
                    AuthorizationRequiredPrimitiveElement,
                    #[serde(rename = "authorizationSupporting")]
                    AuthorizationSupporting,
                    #[serde(rename = "authorizationUrl")]
                    AuthorizationUrl,
                    #[serde(rename = "_authorizationUrl")]
                    AuthorizationUrlPrimitiveElement,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "category",
                            "productOrService",
                            "modifier",
                            "provider",
                            "excluded",
                            "name",
                            "description",
                            "network",
                            "unit",
                            "term",
                            "benefit",
                            "authorizationRequired",
                            "authorizationSupporting",
                            "authorizationUrl",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#category: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#product_or_service: Option<
                    Box<fhirbolt_model::r4b::types::CodeableConcept>,
                > = None;
                let mut r#modifier: Option<Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>> =
                    None;
                let mut r#provider: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#excluded: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#name: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#description: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#network: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#unit: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#term: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#benefit : Option < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > > = None ;
                let mut r#authorization_required: Option<fhirbolt_model::r4b::types::Boolean> =
                    None;
                let mut r#authorization_supporting: Option<
                    Vec<Box<fhirbolt_model::r4b::types::CodeableConcept>>,
                > = None;
                let mut r#authorization_url: Option<fhirbolt_model::r4b::types::Uri> = None;
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
                        Field::Category => {
                            if r#category.is_some() {
                                return Err(serde::de::Error::duplicate_field("category"));
                            }
                            r#category = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::ProductOrService => {
                            if r#product_or_service.is_some() {
                                return Err(serde::de::Error::duplicate_field("productOrService"));
                            }
                            r#product_or_service = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Modifier => {
                            if self.0.from_json {
                                if r#modifier.is_some() {
                                    return Err(serde::de::Error::duplicate_field("modifier"));
                                }
                                r#modifier =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#modifier.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::Provider => {
                            if r#provider.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            r#provider = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Excluded => {
                            if self.0.from_json {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("excluded"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#excluded.is_some() {
                                    return Err(serde::de::Error::duplicate_field("excluded"));
                                }
                                r#excluded = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::ExcludedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#excluded.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_excluded"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("excluded");
                            }
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
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
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
                        Field::Network => {
                            if r#network.is_some() {
                                return Err(serde::de::Error::duplicate_field("network"));
                            }
                            r#network = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Unit => {
                            if r#unit.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            r#unit = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Term => {
                            if r#term.is_some() {
                                return Err(serde::de::Error::duplicate_field("term"));
                            }
                            r#term = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Benefit => {
                            if self.0.from_json {
                                if r#benefit.is_some() {
                                    return Err(serde::de::Error::duplicate_field("benefit"));
                                }
                                r#benefit = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit >> ()) ?) ;
                            } else {
                                let vec = r#benefit.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItemBenefit > ()) ?) ;
                            }
                        }
                        Field::AuthorizationRequired => {
                            if self.0.from_json {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationRequired",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#authorization_required.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationRequired",
                                    ));
                                }
                                r#authorization_required = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::AuthorizationRequiredPrimitiveElement => {
                            if self.0.from_json {
                                let some =
                                    r#authorization_required.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationRequired",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("authorizationRequired");
                            }
                        }
                        Field::AuthorizationSupporting => {
                            if self.0.from_json {
                                if r#authorization_supporting.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationSupporting",
                                    ));
                                }
                                r#authorization_supporting =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4b::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec =
                                    r#authorization_supporting.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::AuthorizationUrl => {
                            if self.0.from_json {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationUrl",
                                    ));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#authorization_url.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "authorizationUrl",
                                    ));
                                }
                                r#authorization_url = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Uri>(),
                                )?);
                            }
                        }
                        Field::AuthorizationUrlPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#authorization_url.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "_authorizationUrl",
                                    ));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("authorizationUrl");
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
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#category,
                        r#product_or_service,
                        r#modifier: r#modifier.unwrap_or(vec![]),
                        r#provider,
                        r#excluded,
                        r#name,
                        r#description,
                        r#network,
                        r#unit,
                        r#term,
                        r#benefit: r#benefit.unwrap_or(vec![]),
                        r#authorization_required,
                        r#authorization_supporting: r#authorization_supporting.unwrap_or(vec![]),
                        r#authorization_url,
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItem > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>>;
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
                        fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem,
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
        fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseInsurance")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance,
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
                    #[serde(rename = "coverage")]
                    Coverage,
                    #[serde(rename = "inforce")]
                    Inforce,
                    #[serde(rename = "_inforce")]
                    InforcePrimitiveElement,
                    #[serde(rename = "benefitPeriod")]
                    BenefitPeriod,
                    #[serde(rename = "item")]
                    Item,
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &[
                            "id",
                            "extension",
                            "modifierExtension",
                            "coverage",
                            "inforce",
                            "benefitPeriod",
                            "item",
                        ],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#coverage: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#inforce: Option<fhirbolt_model::r4b::types::Boolean> = None;
                let mut r#benefit_period: Option<Box<fhirbolt_model::r4b::types::Period>> = None;
                let mut r#item: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsuranceItem>,
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
                        Field::Coverage => {
                            if r#coverage.is_some() {
                                return Err(serde::de::Error::duplicate_field("coverage"));
                            }
                            r#coverage = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Inforce => {
                            if self.0.from_json {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inforce"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#inforce.is_some() {
                                    return Err(serde::de::Error::duplicate_field("inforce"));
                                }
                                r#inforce = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Boolean>(),
                                )?);
                            }
                        }
                        Field::InforcePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#inforce.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_inforce"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("inforce");
                            }
                        }
                        Field::BenefitPeriod => {
                            if r#benefit_period.is_some() {
                                return Err(serde::de::Error::duplicate_field("benefitPeriod"));
                            }
                            r#benefit_period = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Period>>(),
                                )?,
                            );
                        }
                        Field::Item => {
                            if self.0.from_json {
                                if r#item.is_some() {
                                    return Err(serde::de::Error::duplicate_field("item"));
                                }
                                r#item = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItem >> ()) ?) ;
                            } else {
                                let vec = r#item.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsuranceItem > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance {
                        r#id,
                        r#extension: r#extension.unwrap_or(vec![]),
                        r#modifier_extension: r#modifier_extension.unwrap_or(vec![]),
                        r#coverage: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#coverage.unwrap_or(Default::default())
                        } else {
                            r#coverage.ok_or(serde::de::Error::missing_field("coverage"))?
                        },
                        r#inforce,
                        r#benefit_period,
                        r#item: r#item.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsurance > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value =
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>>;
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
                            fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance,
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
        fhirbolt_model::r4b::resources::CoverageEligibilityResponseError,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseError;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponseError,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponseError;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponseError")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError, V::Error>
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
                    Unknown(std::string::String),
                }
                fn unknown_field_error<T, E: serde::de::Error>(field: &str) -> Result<T, E> {
                    Err(E::unknown_field(
                        field,
                        &["id", "extension", "modifierExtension", "code"],
                    ))
                }
                let mut r#id: Option<std::string::String> = None;
                let mut r#extension: Option<Vec<Box<fhirbolt_model::r4b::types::Extension>>> = None;
                let mut r#modifier_extension: Option<
                    Vec<Box<fhirbolt_model::r4b::types::Extension>>,
                > = None;
                let mut r#code: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
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
                        Field::Unknown(key) => {
                            if self.0.config.mode == crate::context::de::DeserializationMode::Strict
                            {
                                return unknown_field_error(&key);
                            }
                        }
                    }
                }
                Ok(
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseError {
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
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError > ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
impl crate::Resource for fhirbolt_model::r4b::resources::CoverageEligibilityResponse {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::CoverageEligibilityResponse,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponse;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        fhirbolt_model::r4b::resources::CoverageEligibilityResponse,
    >
{
    type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponse;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4b::resources::CoverageEligibilityResponse,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4b::resources::CoverageEligibilityResponse;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("CoverageEligibilityResponse")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4b::resources::CoverageEligibilityResponse, V::Error>
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
                    #[serde(rename = "purpose")]
                    Purpose,
                    #[serde(rename = "_purpose")]
                    PurposePrimitiveElement,
                    #[serde(rename = "patient")]
                    Patient,
                    #[serde(rename = "servicedDate")]
                    ServicedDate,
                    #[serde(rename = "_servicedDate")]
                    ServicedDatePrimitiveElement,
                    #[serde(rename = "servicedPeriod")]
                    ServicedPeriod,
                    #[serde(rename = "created")]
                    Created,
                    #[serde(rename = "_created")]
                    CreatedPrimitiveElement,
                    #[serde(rename = "requestor")]
                    Requestor,
                    #[serde(rename = "request")]
                    Request,
                    #[serde(rename = "outcome")]
                    Outcome,
                    #[serde(rename = "_outcome")]
                    OutcomePrimitiveElement,
                    #[serde(rename = "disposition")]
                    Disposition,
                    #[serde(rename = "_disposition")]
                    DispositionPrimitiveElement,
                    #[serde(rename = "insurer")]
                    Insurer,
                    #[serde(rename = "insurance")]
                    Insurance,
                    #[serde(rename = "preAuthRef")]
                    PreAuthRef,
                    #[serde(rename = "_preAuthRef")]
                    PreAuthRefPrimitiveElement,
                    #[serde(rename = "form")]
                    Form,
                    #[serde(rename = "error")]
                    Error,
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
                            "purpose",
                            "patient",
                            "servicedDate",
                            "servicedPeriod",
                            "created",
                            "requestor",
                            "request",
                            "outcome",
                            "disposition",
                            "insurer",
                            "insurance",
                            "preAuthRef",
                            "form",
                            "error",
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
                let mut r#identifier: Option<Vec<Box<fhirbolt_model::r4b::types::Identifier>>> =
                    None;
                let mut r#status: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#purpose: Option<Vec<fhirbolt_model::r4b::types::Code>> = None;
                let mut r#patient: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#serviced: Option<
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponseServiced,
                > = None;
                let mut r#created: Option<fhirbolt_model::r4b::types::DateTime> = None;
                let mut r#requestor: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#request: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#outcome: Option<fhirbolt_model::r4b::types::Code> = None;
                let mut r#disposition: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#insurer: Option<Box<fhirbolt_model::r4b::types::Reference>> = None;
                let mut r#insurance: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseInsurance>,
                > = None;
                let mut r#pre_auth_ref: Option<fhirbolt_model::r4b::types::String> = None;
                let mut r#form: Option<Box<fhirbolt_model::r4b::types::CodeableConcept>> = None;
                let mut r#error: Option<
                    Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponseError>,
                > = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "CoverageEligibilityResponse" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"CoverageEligibilityResponse",
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
                        Field::Purpose => {
                            if self.0.from_json {
                                let values: Vec<Option<_>> = map_access.next_value()?;
                                let vec = r#purpose.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("purpose"));
                                }
                                for (i, value) in values.into_iter().enumerate() {
                                    if let Some(value) = value {
                                        vec[i].value = Some(value);
                                    }
                                }
                            } else {
                                let vec = r#purpose.get_or_insert(Default::default());
                                vec.push(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::PurposePrimitiveElement => {
                            if self.0.from_json {
                                let elements: Vec<
                                    Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                > =
                                    map_access.next_value_seed(self.0.transmute::<Vec<
                                        Option<super::super::serde_helpers::PrimitiveElementOwned>,
                                    >>(
                                    ))?;
                                let vec = r#purpose.get_or_insert(
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
                                    return Err(serde::de::Error::duplicate_field("_purpose"));
                                }
                                for (i, element) in elements.into_iter().enumerate() {
                                    if let Some(element) = element {
                                        vec[i].id = element.id;
                                        vec[i].extension = element.extension;
                                    }
                                }
                            } else {
                                return unknown_field_error("purpose");
                            }
                        }
                        Field::Patient => {
                            if r#patient.is_some() {
                                return Err(serde::de::Error::duplicate_field("patient"));
                            }
                            r#patient = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ServicedDate => {
                            if self.0.from_json {
                                let r#enum = r#serviced . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Date (variant) = r#enum { if variant . value . is_some () { return Err (serde :: de :: Error :: duplicate_field ("servicedDate")) ; } let value : _ = map_access . next_value () ? ; variant . value = Some (value) ; } else { return Err (serde :: de :: Error :: duplicate_field ("serviced[x]")) ; }
                            } else {
                                if r#serviced.is_some() {
                                    return Err(serde::de::Error::duplicate_field("servicedDate"));
                                }
                                r#serviced = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Date (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Date > > ()) ?)) ;
                            }
                        }
                        Field::ServicedDatePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#serviced . get_or_insert (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Date (Default :: default ())) ;
                                if let fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Date (variant) = r#enum { if variant . id . is_some () || ! variant . extension . is_empty () { return Err (serde :: de :: Error :: duplicate_field ("_servicedDate")) ; } let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ; variant . id = id ; variant . extension = extension ; } else { return Err (serde :: de :: Error :: duplicate_field ("_serviced[x]")) ; }
                            } else {
                                return unknown_field_error("servicedDate");
                            }
                        }
                        Field::ServicedPeriod => {
                            if r#serviced.is_some() {
                                return Err(serde::de::Error::duplicate_field("servicedPeriod"));
                            }
                            r#serviced = Some (fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseServiced :: Period (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: Period > > ()) ?)) ;
                        }
                        Field::Created => {
                            if self.0.from_json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#created.is_some() {
                                    return Err(serde::de::Error::duplicate_field("created"));
                                }
                                r#created = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::DateTime>(),
                                )?);
                            }
                        }
                        Field::CreatedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#created.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_created"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("created");
                            }
                        }
                        Field::Requestor => {
                            if r#requestor.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestor"));
                            }
                            r#requestor = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Request => {
                            if r#request.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            r#request = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Outcome => {
                            if self.0.from_json {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#outcome.is_some() {
                                    return Err(serde::de::Error::duplicate_field("outcome"));
                                }
                                r#outcome = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::Code>(),
                                )?);
                            }
                        }
                        Field::OutcomePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#outcome.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_outcome"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("outcome");
                            }
                        }
                        Field::Disposition => {
                            if self.0.from_json {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disposition"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#disposition.is_some() {
                                    return Err(serde::de::Error::duplicate_field("disposition"));
                                }
                                r#disposition = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::DispositionPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#disposition.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_disposition"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("disposition");
                            }
                        }
                        Field::Insurer => {
                            if r#insurer.is_some() {
                                return Err(serde::de::Error::duplicate_field("insurer"));
                            }
                            r#insurer = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4b::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Insurance => {
                            if self.0.from_json {
                                if r#insurance.is_some() {
                                    return Err(serde::de::Error::duplicate_field("insurance"));
                                }
                                r#insurance = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsurance >> ()) ?) ;
                            } else {
                                let vec = r#insurance.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseInsurance > ()) ?) ;
                            }
                        }
                        Field::PreAuthRef => {
                            if self.0.from_json {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#pre_auth_ref.is_some() {
                                    return Err(serde::de::Error::duplicate_field("preAuthRef"));
                                }
                                r#pre_auth_ref = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4b::types::String>(),
                                )?);
                            }
                        }
                        Field::PreAuthRefPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#pre_auth_ref.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_preAuthRef"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("preAuthRef");
                            }
                        }
                        Field::Form => {
                            if r#form.is_some() {
                                return Err(serde::de::Error::duplicate_field("form"));
                            }
                            r#form = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Error => {
                            if self.0.from_json {
                                if r#error.is_some() {
                                    return Err(serde::de::Error::duplicate_field("error"));
                                }
                                r#error = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError >> ()) ?) ;
                            } else {
                                let vec = r#error.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponseError > ()) ?) ;
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
                    fhirbolt_model::r4b::resources::CoverageEligibilityResponse {
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
                        r#purpose: r#purpose.unwrap_or(vec![]),
                        r#patient: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#patient.unwrap_or(Default::default())
                        } else {
                            r#patient.ok_or(serde::de::Error::missing_field("patient"))?
                        },
                        r#serviced,
                        r#created: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#created.unwrap_or(Default::default())
                        } else {
                            r#created.ok_or(serde::de::Error::missing_field("created"))?
                        },
                        r#requestor,
                        r#request: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#request.unwrap_or(Default::default())
                        } else {
                            r#request.ok_or(serde::de::Error::missing_field("request"))?
                        },
                        r#outcome: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#outcome.unwrap_or(Default::default())
                        } else {
                            r#outcome.ok_or(serde::de::Error::missing_field("outcome"))?
                        },
                        r#disposition,
                        r#insurer: if self.0.config.mode
                            == crate::context::de::DeserializationMode::Lax
                        {
                            r#insurer.unwrap_or(Default::default())
                        } else {
                            r#insurer.ok_or(serde::de::Error::missing_field("insurer"))?
                        },
                        r#insurance: r#insurance.unwrap_or(vec![]),
                        r#pre_auth_ref,
                        r#form,
                        r#error: r#error.unwrap_or(vec![]),
                    },
                )
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>,
    >
{
    type Value = Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<
        Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>,
    >
{
    type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>;
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
                        .transmute::<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>(),
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
        Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::resources::CoverageEligibilityResponse>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some (value) = seq . next_element_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponse >> ()) ? { values . push (value) ; }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
