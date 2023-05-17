// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
use crate::{DeserializationContext, SerializationContext};
use fhirbolt_model::r5::Resource;
impl crate::Resource for Resource {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize for SerializationContext<&Resource> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self.value {
            Resource::Account(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ActivityDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::ActorDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::AdministrableProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::AdverseEvent(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::AllergyIntolerance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Appointment(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::AppointmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::ArtifactAssessment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::AuditEvent(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Basic(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Binary(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::BiologicallyDerivedProduct(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::BiologicallyDerivedProductDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::BodyStructure(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Bundle(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::CapabilityStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::CarePlan(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::CareTeam(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ChargeItem(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ChargeItemDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Citation(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Claim(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ClaimResponse(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ClinicalImpression(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::ClinicalUseDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::CodeSystem(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Communication(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::CommunicationRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::CompartmentDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Composition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ConceptMap(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Condition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ConditionDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Consent(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Contract(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Coverage(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::CoverageEligibilityRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::CoverageEligibilityResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::DetectedIssue(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Device(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceAssociation(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceDispense(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceMetric(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceRequest(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DeviceUsage(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DiagnosticReport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::DocumentReference(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Encounter(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EncounterHistory(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Endpoint(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EnrollmentRequest(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EnrollmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::EpisodeOfCare(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EventDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Evidence(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EvidenceReport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::EvidenceVariable(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ExampleScenario(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ExplanationOfBenefit(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::FamilyMemberHistory(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Flag(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::FormularyItem(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::GenomicStudy(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Goal(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::GraphDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Group(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::GuidanceResponse(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::HealthcareService(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ImagingSelection(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ImagingStudy(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Immunization(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ImmunizationEvaluation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::ImmunizationRecommendation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::ImplementationGuide(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Ingredient(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::InsurancePlan(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::InventoryItem(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::InventoryReport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Invoice(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Library(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Linkage(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::List(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Location(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ManufacturedItemDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Measure(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::MeasureReport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Medication(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::MedicationAdministration(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::MedicationDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::MedicationKnowledge(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::MedicationRequest(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::MedicationStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::MedicinalProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::MessageDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::MessageHeader(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::MolecularSequence(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::NamingSystem(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::NutritionIntake(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::NutritionOrder(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::NutritionProduct(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Observation(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ObservationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::OperationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::OperationOutcome(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Organization(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::OrganizationAffiliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::PackagedProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Parameters(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Patient(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::PaymentNotice(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::PaymentReconciliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Permission(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Person(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::PlanDefinition(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Practitioner(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::PractitionerRole(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Procedure(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Provenance(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Questionnaire(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::QuestionnaireResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::RegulatedAuthorization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::RelatedPerson(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::RequestOrchestration(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::Requirements(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ResearchStudy(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ResearchSubject(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::RiskAssessment(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Schedule(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SearchParameter(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ServiceRequest(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Slot(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Specimen(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SpecimenDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::StructureDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::StructureMap(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Subscription(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SubscriptionStatus(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::SubscriptionTopic(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Substance(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SubstanceDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::SubstanceNucleicAcid(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::SubstancePolymer(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SubstanceProtein(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SubstanceReferenceInformation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::SubstanceSourceMaterial(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::SupplyDelivery(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::SupplyRequest(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Task(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::TerminologyCapabilities(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::TestPlan(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::TestReport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::TestScript(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::Transport(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::ValueSet(r) => self.with_context(r, |ctx| ctx.serialize(serializer)),
            Resource::VerificationResult(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            Resource::VisionPrescription(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            _ => Err(serde::ser::Error::custom("invalid resource")),
        }
    }
}
impl serde::ser::Serialize for SerializationContext<&Vec<Resource>> {
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
impl<'de> serde::de::DeserializeSeed<'de> for DeserializationContext<Resource> {
    type Value = Resource;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Resource> {
    type Value = Resource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let _context =
            self.transmute::<crate::element::internal::de::InternalElement<
                { fhirbolt_shared::FhirReleases::R5 },
            >>();
        let element = _context.deserialize(deserializer)?;
        self.from = crate::context::Format::InternalElement;
        if let Some(fhirbolt_element::Value::Primitive(fhirbolt_element::Primitive::String(
            resource_type,
        ))) = element.0.get("resourceType")
        {
            match resource_type.as_str() {
                "Account" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Account>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Account)
                        .map_err(serde::de::Error::custom)
                }
                "ActivityDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ActivityDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ActivityDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ActorDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ActorDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ActorDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdministrableProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::AdministrableProductDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::AdministrableProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdverseEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::AdverseEvent>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::AdverseEvent)
                        .map_err(serde::de::Error::custom)
                }
                "AllergyIntolerance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::AllergyIntolerance>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::AllergyIntolerance)
                        .map_err(serde::de::Error::custom)
                }
                "Appointment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Appointment>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Appointment)
                        .map_err(serde::de::Error::custom)
                }
                "AppointmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::AppointmentResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::AppointmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ArtifactAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ArtifactAssessment>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ArtifactAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "AuditEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::AuditEvent>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::AuditEvent)
                        .map_err(serde::de::Error::custom)
                }
                "Basic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Basic>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Basic)
                        .map_err(serde::de::Error::custom)
                }
                "Binary" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Binary>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Binary)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::BiologicallyDerivedProduct>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::BiologicallyDerivedProduct)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProductDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::BiologicallyDerivedProductDispense>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::BiologicallyDerivedProductDispense)
                        .map_err(serde::de::Error::custom)
                }
                "BodyStructure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::BodyStructure>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::BodyStructure)
                        .map_err(serde::de::Error::custom)
                }
                "Bundle" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Bundle>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Bundle)
                        .map_err(serde::de::Error::custom)
                }
                "CapabilityStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CapabilityStatement>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CapabilityStatement)
                        .map_err(serde::de::Error::custom)
                }
                "CarePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CarePlan>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CarePlan)
                        .map_err(serde::de::Error::custom)
                }
                "CareTeam" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CareTeam>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CareTeam)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ChargeItem>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ChargeItem)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ChargeItemDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ChargeItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Citation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Citation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Citation)
                        .map_err(serde::de::Error::custom)
                }
                "Claim" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Claim>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Claim)
                        .map_err(serde::de::Error::custom)
                }
                "ClaimResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ClaimResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ClaimResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalImpression" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ClinicalImpression>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ClinicalImpression)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalUseDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ClinicalUseDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ClinicalUseDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "CodeSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CodeSystem>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CodeSystem)
                        .map_err(serde::de::Error::custom)
                }
                "Communication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Communication>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Communication)
                        .map_err(serde::de::Error::custom)
                }
                "CommunicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CommunicationRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CommunicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CompartmentDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CompartmentDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CompartmentDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Composition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Composition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Composition)
                        .map_err(serde::de::Error::custom)
                }
                "ConceptMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ConceptMap>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ConceptMap)
                        .map_err(serde::de::Error::custom)
                }
                "Condition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Condition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Condition)
                        .map_err(serde::de::Error::custom)
                }
                "ConditionDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ConditionDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ConditionDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Consent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Consent>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Consent)
                        .map_err(serde::de::Error::custom)
                }
                "Contract" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Contract>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Contract)
                        .map_err(serde::de::Error::custom)
                }
                "Coverage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Coverage>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Coverage)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CoverageEligibilityRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CoverageEligibilityRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::CoverageEligibilityResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::CoverageEligibilityResponse)
                        .map_err(serde::de::Error::custom)
                }
                "DetectedIssue" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DetectedIssue>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DetectedIssue)
                        .map_err(serde::de::Error::custom)
                }
                "Device" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Device>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Device)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceAssociation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceAssociation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceAssociation)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceDispense>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceDispense)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceMetric" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceMetric>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceMetric)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceUsage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DeviceUsage>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DeviceUsage)
                        .map_err(serde::de::Error::custom)
                }
                "DiagnosticReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DiagnosticReport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DiagnosticReport)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentReference" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::DocumentReference>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::DocumentReference)
                        .map_err(serde::de::Error::custom)
                }
                "Encounter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Encounter>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Encounter)
                        .map_err(serde::de::Error::custom)
                }
                "EncounterHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EncounterHistory>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EncounterHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Endpoint" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Endpoint>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Endpoint)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EnrollmentRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EnrollmentRequest)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EnrollmentResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EnrollmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "EpisodeOfCare" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EpisodeOfCare>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EpisodeOfCare)
                        .map_err(serde::de::Error::custom)
                }
                "EventDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EventDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EventDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Evidence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Evidence>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Evidence)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EvidenceReport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EvidenceReport)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceVariable" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::EvidenceVariable>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::EvidenceVariable)
                        .map_err(serde::de::Error::custom)
                }
                "ExampleScenario" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ExampleScenario>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ExampleScenario)
                        .map_err(serde::de::Error::custom)
                }
                "ExplanationOfBenefit" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ExplanationOfBenefit>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ExplanationOfBenefit)
                        .map_err(serde::de::Error::custom)
                }
                "FamilyMemberHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::FamilyMemberHistory>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::FamilyMemberHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Flag" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Flag>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Flag)
                        .map_err(serde::de::Error::custom)
                }
                "FormularyItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::FormularyItem>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::FormularyItem)
                        .map_err(serde::de::Error::custom)
                }
                "GenomicStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::GenomicStudy>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::GenomicStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Goal" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Goal>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Goal)
                        .map_err(serde::de::Error::custom)
                }
                "GraphDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::GraphDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::GraphDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Group" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Group>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Group)
                        .map_err(serde::de::Error::custom)
                }
                "GuidanceResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::GuidanceResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::GuidanceResponse)
                        .map_err(serde::de::Error::custom)
                }
                "HealthcareService" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::HealthcareService>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::HealthcareService)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingSelection" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ImagingSelection>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ImagingSelection)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ImagingStudy>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ImagingStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Immunization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Immunization>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Immunization)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationEvaluation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ImmunizationEvaluation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ImmunizationEvaluation)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationRecommendation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ImmunizationRecommendation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ImmunizationRecommendation)
                        .map_err(serde::de::Error::custom)
                }
                "ImplementationGuide" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ImplementationGuide>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ImplementationGuide)
                        .map_err(serde::de::Error::custom)
                }
                "Ingredient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Ingredient>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Ingredient)
                        .map_err(serde::de::Error::custom)
                }
                "InsurancePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::InsurancePlan>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::InsurancePlan)
                        .map_err(serde::de::Error::custom)
                }
                "InventoryItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::InventoryItem>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::InventoryItem)
                        .map_err(serde::de::Error::custom)
                }
                "InventoryReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::InventoryReport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::InventoryReport)
                        .map_err(serde::de::Error::custom)
                }
                "Invoice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Invoice>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Invoice)
                        .map_err(serde::de::Error::custom)
                }
                "Library" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Library>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Library)
                        .map_err(serde::de::Error::custom)
                }
                "Linkage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Linkage>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Linkage)
                        .map_err(serde::de::Error::custom)
                }
                "List" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::List>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::List)
                        .map_err(serde::de::Error::custom)
                }
                "Location" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Location>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Location)
                        .map_err(serde::de::Error::custom)
                }
                "ManufacturedItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ManufacturedItemDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ManufacturedItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Measure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Measure>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Measure)
                        .map_err(serde::de::Error::custom)
                }
                "MeasureReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MeasureReport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MeasureReport)
                        .map_err(serde::de::Error::custom)
                }
                "Medication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Medication>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Medication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationAdministration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicationAdministration>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicationAdministration)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicationDispense>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicationDispense)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationKnowledge" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicationKnowledge>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicationKnowledge)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicationRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicationStatement>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicationStatement)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MedicinalProductDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MedicinalProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MessageDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MessageDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageHeader" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MessageHeader>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MessageHeader)
                        .map_err(serde::de::Error::custom)
                }
                "MolecularSequence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::MolecularSequence>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::MolecularSequence)
                        .map_err(serde::de::Error::custom)
                }
                "NamingSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::NamingSystem>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::NamingSystem)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionIntake" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::NutritionIntake>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::NutritionIntake)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionOrder" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::NutritionOrder>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::NutritionOrder)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::NutritionProduct>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::NutritionProduct)
                        .map_err(serde::de::Error::custom)
                }
                "Observation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Observation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Observation)
                        .map_err(serde::de::Error::custom)
                }
                "ObservationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ObservationDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ObservationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::OperationDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::OperationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationOutcome" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::OperationOutcome>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::OperationOutcome)
                        .map_err(serde::de::Error::custom)
                }
                "Organization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Organization>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Organization)
                        .map_err(serde::de::Error::custom)
                }
                "OrganizationAffiliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::OrganizationAffiliation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::OrganizationAffiliation)
                        .map_err(serde::de::Error::custom)
                }
                "PackagedProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::PackagedProductDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::PackagedProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Parameters" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Parameters>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Parameters)
                        .map_err(serde::de::Error::custom)
                }
                "Patient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Patient>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Patient)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentNotice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::PaymentNotice>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::PaymentNotice)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentReconciliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::PaymentReconciliation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::PaymentReconciliation)
                        .map_err(serde::de::Error::custom)
                }
                "Permission" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Permission>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Permission)
                        .map_err(serde::de::Error::custom)
                }
                "Person" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Person>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Person)
                        .map_err(serde::de::Error::custom)
                }
                "PlanDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::PlanDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::PlanDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Practitioner" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Practitioner>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Practitioner)
                        .map_err(serde::de::Error::custom)
                }
                "PractitionerRole" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::PractitionerRole>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::PractitionerRole)
                        .map_err(serde::de::Error::custom)
                }
                "Procedure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Procedure>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Procedure)
                        .map_err(serde::de::Error::custom)
                }
                "Provenance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Provenance>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Provenance)
                        .map_err(serde::de::Error::custom)
                }
                "Questionnaire" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Questionnaire>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Questionnaire)
                        .map_err(serde::de::Error::custom)
                }
                "QuestionnaireResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::QuestionnaireResponse>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::QuestionnaireResponse)
                        .map_err(serde::de::Error::custom)
                }
                "RegulatedAuthorization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::RegulatedAuthorization>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::RegulatedAuthorization)
                        .map_err(serde::de::Error::custom)
                }
                "RelatedPerson" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::RelatedPerson>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::RelatedPerson)
                        .map_err(serde::de::Error::custom)
                }
                "RequestOrchestration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::RequestOrchestration>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::RequestOrchestration)
                        .map_err(serde::de::Error::custom)
                }
                "Requirements" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Requirements>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Requirements)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ResearchStudy>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ResearchStudy)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchSubject" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ResearchSubject>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ResearchSubject)
                        .map_err(serde::de::Error::custom)
                }
                "RiskAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::RiskAssessment>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::RiskAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "Schedule" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Schedule>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Schedule)
                        .map_err(serde::de::Error::custom)
                }
                "SearchParameter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SearchParameter>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SearchParameter)
                        .map_err(serde::de::Error::custom)
                }
                "ServiceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ServiceRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ServiceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Slot" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Slot>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Slot)
                        .map_err(serde::de::Error::custom)
                }
                "Specimen" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Specimen>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Specimen)
                        .map_err(serde::de::Error::custom)
                }
                "SpecimenDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SpecimenDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SpecimenDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::StructureDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::StructureDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::StructureMap>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::StructureMap)
                        .map_err(serde::de::Error::custom)
                }
                "Subscription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Subscription>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Subscription)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionStatus" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubscriptionStatus>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubscriptionStatus)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionTopic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubscriptionTopic>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubscriptionTopic)
                        .map_err(serde::de::Error::custom)
                }
                "Substance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Substance>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Substance)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstanceDefinition>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstanceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceNucleicAcid" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstanceNucleicAcid>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstanceNucleicAcid)
                        .map_err(serde::de::Error::custom)
                }
                "SubstancePolymer" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstancePolymer>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstancePolymer)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceProtein" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstanceProtein>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstanceProtein)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceReferenceInformation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstanceReferenceInformation>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstanceReferenceInformation)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceSourceMaterial" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SubstanceSourceMaterial>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SubstanceSourceMaterial)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyDelivery" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SupplyDelivery>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SupplyDelivery)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::SupplyRequest>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::SupplyRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Task" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Task>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Task)
                        .map_err(serde::de::Error::custom)
                }
                "TerminologyCapabilities" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::TerminologyCapabilities>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::TerminologyCapabilities)
                        .map_err(serde::de::Error::custom)
                }
                "TestPlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::TestPlan>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::TestPlan)
                        .map_err(serde::de::Error::custom)
                }
                "TestReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::TestReport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::TestReport)
                        .map_err(serde::de::Error::custom)
                }
                "TestScript" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::TestScript>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::TestScript)
                        .map_err(serde::de::Error::custom)
                }
                "Transport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::Transport>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::Transport)
                        .map_err(serde::de::Error::custom)
                }
                "ValueSet" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::ValueSet>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::ValueSet)
                        .map_err(serde::de::Error::custom)
                }
                "VerificationResult" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::VerificationResult>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::VerificationResult)
                        .map_err(serde::de::Error::custom)
                }
                "VisionPrescription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let _context: &mut DeserializationContext<
                        Box<fhirbolt_model::r5::resources::VisionPrescription>,
                    > = self.transmute();
                    _context
                        .deserialize(deserializer)
                        .map(Resource::VisionPrescription)
                        .map_err(serde::de::Error::custom)
                }
                _ => Err(serde::de::Error::invalid_type(
                    serde::de::Unexpected::Other("resourceType"),
                    &"valid resourceType",
                )),
            }
        } else {
            Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Other("an element"),
                &"a resource",
            ))
        }
    }
}
impl<'de> serde::de::DeserializeSeed<'de> for &mut DeserializationContext<Vec<Resource>> {
    type Value = Vec<Resource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(&'a mut DeserializationContext<Vec<Resource>>);
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Resource>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of resources")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element_seed(self.0.transmute::<Resource>())? {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
