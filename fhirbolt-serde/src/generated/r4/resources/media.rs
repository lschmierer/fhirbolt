// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::resources::Media>
{
    type Value = fhirbolt_model::r4::resources::Media;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                fhirbolt_model::r4::resources::Media,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = fhirbolt_model::r4::resources::Media;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Media")
            }
            fn visit_map<V>(
                self,
                mut map_access: V,
            ) -> Result<fhirbolt_model::r4::resources::Media, V::Error>
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
                    #[serde(rename = "type")]
                    Type,
                    #[serde(rename = "modality")]
                    Modality,
                    #[serde(rename = "view")]
                    View,
                    #[serde(rename = "subject")]
                    Subject,
                    #[serde(rename = "encounter")]
                    Encounter,
                    #[serde(rename = "createdDateTime")]
                    CreatedDateTime,
                    #[serde(rename = "_createdDateTime")]
                    CreatedDateTimePrimitiveElement,
                    #[serde(rename = "createdPeriod")]
                    CreatedPeriod,
                    #[serde(rename = "issued")]
                    Issued,
                    #[serde(rename = "_issued")]
                    IssuedPrimitiveElement,
                    #[serde(rename = "operator")]
                    Operator,
                    #[serde(rename = "reasonCode")]
                    ReasonCode,
                    #[serde(rename = "bodySite")]
                    BodySite,
                    #[serde(rename = "deviceName")]
                    DeviceName,
                    #[serde(rename = "_deviceName")]
                    DeviceNamePrimitiveElement,
                    #[serde(rename = "device")]
                    Device,
                    #[serde(rename = "height")]
                    Height,
                    #[serde(rename = "_height")]
                    HeightPrimitiveElement,
                    #[serde(rename = "width")]
                    Width,
                    #[serde(rename = "_width")]
                    WidthPrimitiveElement,
                    #[serde(rename = "frames")]
                    Frames,
                    #[serde(rename = "_frames")]
                    FramesPrimitiveElement,
                    #[serde(rename = "duration")]
                    Duration,
                    #[serde(rename = "_duration")]
                    DurationPrimitiveElement,
                    #[serde(rename = "content")]
                    Content,
                    #[serde(rename = "note")]
                    Note,
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
                            "type",
                            "modality",
                            "view",
                            "subject",
                            "encounter",
                            "createdDateTime",
                            "createdPeriod",
                            "issued",
                            "operator",
                            "reasonCode",
                            "bodySite",
                            "deviceName",
                            "device",
                            "height",
                            "width",
                            "frames",
                            "duration",
                            "content",
                            "note",
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
                let mut r#based_on: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#part_of: Option<Vec<Box<fhirbolt_model::r4::types::Reference>>> = None;
                let mut r#status: Option<fhirbolt_model::r4::types::Code> = None;
                let mut r#type: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#modality: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#view: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#subject: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#encounter: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#created: Option<fhirbolt_model::r4::resources::MediaCreated> = None;
                let mut r#issued: Option<fhirbolt_model::r4::types::Instant> = None;
                let mut r#operator: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#reason_code: Option<
                    Vec<Box<fhirbolt_model::r4::types::CodeableConcept>>,
                > = None;
                let mut r#body_site: Option<Box<fhirbolt_model::r4::types::CodeableConcept>> = None;
                let mut r#device_name: Option<fhirbolt_model::r4::types::String> = None;
                let mut r#device: Option<Box<fhirbolt_model::r4::types::Reference>> = None;
                let mut r#height: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#width: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#frames: Option<fhirbolt_model::r4::types::PositiveInt> = None;
                let mut r#duration: Option<fhirbolt_model::r4::types::Decimal> = None;
                let mut r#content: Option<Box<fhirbolt_model::r4::types::Attachment>> = None;
                let mut r#note: Option<Vec<Box<fhirbolt_model::r4::types::Annotation>>> = None;
                while let Some(map_access_key) = map_access.next_key()? {
                    match map_access_key {
                        Field::ResourceType => {
                            let value: std::borrow::Cow<str> = map_access.next_value()?;
                            if value != "Media" {
                                return Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&value),
                                    &"Media",
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
                        Field::BasedOn => {
                            if self.0.from_json {
                                if r#based_on.is_some() {
                                    return Err(serde::de::Error::duplicate_field("basedOn"));
                                }
                                r#based_on = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#based_on.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
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
                                r#part_of = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Reference > >> ()) ?) ;
                            } else {
                                let vec = r#part_of.get_or_insert(Default::default());
                                vec.push(
                                    map_access.next_value_seed(
                                        self.0
                                            .transmute::<Box<fhirbolt_model::r4::types::Reference>>(
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
                                    self.0.transmute::<fhirbolt_model::r4::types::Code>(),
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
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Modality => {
                            if r#modality.is_some() {
                                return Err(serde::de::Error::duplicate_field("modality"));
                            }
                            r#modality = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::View => {
                            if r#view.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            r#view = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::Subject => {
                            if r#subject.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            r#subject = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
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
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::CreatedDateTime => {
                            if self.0.from_json {
                                let r#enum = r#created.get_or_insert(
                                    fhirbolt_model::r4::resources::MediaCreated::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::MediaCreated::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.value.is_some() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "createdDateTime",
                                        ));
                                    }
                                    let value: _ = map_access.next_value()?;
                                    variant.value = Some(value);
                                } else {
                                    return Err(serde::de::Error::duplicate_field("created[x]"));
                                }
                            } else {
                                if r#created.is_some() {
                                    return Err(serde::de::Error::duplicate_field(
                                        "createdDateTime",
                                    ));
                                }
                                r#created = Some (fhirbolt_model :: r4 :: resources :: MediaCreated :: DateTime (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: DateTime > > ()) ?)) ;
                            }
                        }
                        Field::CreatedDateTimePrimitiveElement => {
                            if self.0.from_json {
                                let r#enum = r#created.get_or_insert(
                                    fhirbolt_model::r4::resources::MediaCreated::DateTime(
                                        Default::default(),
                                    ),
                                );
                                if let fhirbolt_model::r4::resources::MediaCreated::DateTime(
                                    variant,
                                ) = r#enum
                                {
                                    if variant.id.is_some() || !variant.extension.is_empty() {
                                        return Err(serde::de::Error::duplicate_field(
                                            "_createdDateTime",
                                        ));
                                    }
                                    let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                    variant.id = id;
                                    variant.extension = extension;
                                } else {
                                    return Err(serde::de::Error::duplicate_field("_created[x]"));
                                }
                            } else {
                                return unknown_field_error("createdDateTime");
                            }
                        }
                        Field::CreatedPeriod => {
                            if r#created.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdPeriod"));
                            }
                            r#created = Some(fhirbolt_model::r4::resources::MediaCreated::Period(
                                map_access.next_value_seed(
                                    self.0.transmute::<Box<fhirbolt_model::r4::types::Period>>(),
                                )?,
                            ));
                        }
                        Field::Issued => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#issued.is_some() {
                                    return Err(serde::de::Error::duplicate_field("issued"));
                                }
                                r#issued = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Instant>(),
                                )?);
                            }
                        }
                        Field::IssuedPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#issued.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_issued"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("issued");
                            }
                        }
                        Field::Operator => {
                            if r#operator.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            r#operator = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::ReasonCode => {
                            if self.0.from_json {
                                if r#reason_code.is_some() {
                                    return Err(serde::de::Error::duplicate_field("reasonCode"));
                                }
                                r#reason_code =
                                    Some(
                                        map_access.next_value_seed(
                                            self.0.transmute::<Vec<
                                                Box<fhirbolt_model::r4::types::CodeableConcept>,
                                            >>(),
                                        )?,
                                    );
                            } else {
                                let vec = r#reason_code.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                            }
                        }
                        Field::BodySite => {
                            if r#body_site.is_some() {
                                return Err(serde::de::Error::duplicate_field("bodySite"));
                            }
                            r#body_site = Some (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: CodeableConcept > > ()) ?) ;
                        }
                        Field::DeviceName => {
                            if self.0.from_json {
                                let some = r#device_name.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("deviceName"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#device_name.is_some() {
                                    return Err(serde::de::Error::duplicate_field("deviceName"));
                                }
                                r#device_name = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::String>(),
                                )?);
                            }
                        }
                        Field::DeviceNamePrimitiveElement => {
                            if self.0.from_json {
                                let some = r#device_name.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_deviceName"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("deviceName");
                            }
                        }
                        Field::Device => {
                            if r#device.is_some() {
                                return Err(serde::de::Error::duplicate_field("device"));
                            }
                            r#device = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Reference>>(),
                                )?,
                            );
                        }
                        Field::Height => {
                            if self.0.from_json {
                                let some = r#height.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("height"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#height.is_some() {
                                    return Err(serde::de::Error::duplicate_field("height"));
                                }
                                r#height = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::PositiveInt>(),
                                )?);
                            }
                        }
                        Field::HeightPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#height.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_height"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("height");
                            }
                        }
                        Field::Width => {
                            if self.0.from_json {
                                let some = r#width.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("width"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#width.is_some() {
                                    return Err(serde::de::Error::duplicate_field("width"));
                                }
                                r#width = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::PositiveInt>(),
                                )?);
                            }
                        }
                        Field::WidthPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#width.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_width"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("width");
                            }
                        }
                        Field::Frames => {
                            if self.0.from_json {
                                let some = r#frames.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frames"));
                                }
                                let value: _ = map_access.next_value()?;
                                some.value = Some(value);
                            } else {
                                if r#frames.is_some() {
                                    return Err(serde::de::Error::duplicate_field("frames"));
                                }
                                r#frames = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::PositiveInt>(),
                                )?);
                            }
                        }
                        Field::FramesPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#frames.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_frames"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("frames");
                            }
                        }
                        Field::Duration => {
                            if self.0.from_json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.value.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                let value: serde_json::Number = map_access.next_value()?;
                                some.value = Some(format!("{}", value));
                            } else {
                                if r#duration.is_some() {
                                    return Err(serde::de::Error::duplicate_field("duration"));
                                }
                                r#duration = Some(map_access.next_value_seed(
                                    self.0.transmute::<fhirbolt_model::r4::types::Decimal>(),
                                )?);
                            }
                        }
                        Field::DurationPrimitiveElement => {
                            if self.0.from_json {
                                let some = r#duration.get_or_insert(Default::default());
                                if some.id.is_some() || !some.extension.is_empty() {
                                    return Err(serde::de::Error::duplicate_field("_duration"));
                                }
                                let super :: super :: serde_helpers :: PrimitiveElementOwned { id , extension } = map_access . next_value_seed (self . 0 . transmute :: < super :: super :: serde_helpers :: PrimitiveElementOwned > ()) ? ;
                                some.id = id;
                                some.extension = extension;
                            } else {
                                return unknown_field_error("duration");
                            }
                        }
                        Field::Content => {
                            if r#content.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            r#content = Some(
                                map_access.next_value_seed(
                                    self.0
                                        .transmute::<Box<fhirbolt_model::r4::types::Attachment>>(),
                                )?,
                            );
                        }
                        Field::Note => {
                            if self.0.from_json {
                                if r#note.is_some() {
                                    return Err(serde::de::Error::duplicate_field("note"));
                                }
                                r#note = Some (map_access . next_value_seed (self . 0 . transmute :: < Vec < Box < fhirbolt_model :: r4 :: types :: Annotation > >> ()) ?) ;
                            } else {
                                let vec = r#note.get_or_insert(Default::default());
                                vec . push (map_access . next_value_seed (self . 0 . transmute :: < Box < fhirbolt_model :: r4 :: types :: Annotation > > ()) ?) ;
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
                Ok(fhirbolt_model::r4::resources::Media {
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
                    r#type,
                    r#modality,
                    r#view,
                    r#subject,
                    r#encounter,
                    r#created,
                    r#issued,
                    r#operator,
                    r#reason_code: r#reason_code.unwrap_or(vec![]),
                    r#body_site,
                    r#device_name,
                    r#device,
                    r#height,
                    r#width,
                    r#frames,
                    r#duration,
                    r#content: if self.0.config.mode == crate::context::de::DeserializationMode::Lax
                    {
                        r#content.unwrap_or(Default::default())
                    } else {
                        r#content.ok_or(serde::de::Error::missing_field("content"))?
                    },
                    r#note: r#note.unwrap_or(vec![]),
                })
            }
        }
        deserializer.deserialize_map(Visitor(self))
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4::resources::Media>>
{
    type Value = Box<fhirbolt_model::r4::resources::Media>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::resources::Media>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<fhirbolt_model::r4::resources::Media>>
{
    type Value = Vec<fhirbolt_model::r4::resources::Media>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<fhirbolt_model::r4::resources::Media>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<fhirbolt_model::r4::resources::Media>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq
                    .next_element_seed(self.0.transmute::<fhirbolt_model::r4::resources::Media>())?
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
        Vec<Box<fhirbolt_model::r4::resources::Media>>,
    >
{
    type Value = Vec<Box<fhirbolt_model::r4::resources::Media>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::resources::Media>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::resources::Media>>;
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
                        .transmute::<Box<fhirbolt_model::r4::resources::Media>>(),
                )? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
