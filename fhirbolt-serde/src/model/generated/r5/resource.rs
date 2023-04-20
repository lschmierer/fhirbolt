// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
impl crate::Resource for fhirbolt_model::r5::Resource {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R5;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r5::Resource>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self.value {
            fhirbolt_model::r5::Resource::Account(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ActivityDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ActorDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::AdministrableProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::AdverseEvent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::AllergyIntolerance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Appointment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::AppointmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ArtifactAssessment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::AuditEvent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Basic(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Binary(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::BiologicallyDerivedProduct(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::BiologicallyDerivedProductDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::BodyStructure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Bundle(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CapabilityStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CarePlan(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CareTeam(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ChargeItem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ChargeItemDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Citation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Claim(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ClaimResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ClinicalImpression(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ClinicalUseDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CodeSystem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Communication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CommunicationRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CompartmentDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Composition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ConceptMap(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Condition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ConditionDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Consent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Contract(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Coverage(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CoverageEligibilityRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::CoverageEligibilityResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DetectedIssue(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Device(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceAssociation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceMetric(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DeviceUsage(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DiagnosticReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::DocumentReference(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Encounter(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EncounterHistory(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Endpoint(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EnrollmentRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EnrollmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EpisodeOfCare(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EventDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Evidence(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EvidenceReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::EvidenceVariable(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ExampleScenario(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ExplanationOfBenefit(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::FamilyMemberHistory(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Flag(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::FormularyItem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::GenomicStudy(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Goal(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::GraphDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Group(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::GuidanceResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::HealthcareService(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ImagingSelection(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ImagingStudy(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Immunization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ImmunizationEvaluation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ImmunizationRecommendation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ImplementationGuide(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Ingredient(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::InsurancePlan(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::InventoryItem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::InventoryReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Invoice(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Library(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Linkage(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::List(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Location(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ManufacturedItemDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Measure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MeasureReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Medication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicationAdministration(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicationDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicationKnowledge(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicationRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicationStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MedicinalProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MessageDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MessageHeader(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::MolecularSequence(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::NamingSystem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::NutritionIntake(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::NutritionOrder(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::NutritionProduct(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Observation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ObservationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::OperationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::OperationOutcome(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Organization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::OrganizationAffiliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::PackagedProductDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Parameters(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Patient(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::PaymentNotice(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::PaymentReconciliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Permission(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Person(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::PlanDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Practitioner(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::PractitionerRole(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Procedure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Provenance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Questionnaire(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::QuestionnaireResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::RegulatedAuthorization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::RelatedPerson(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::RequestOrchestration(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Requirements(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ResearchStudy(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ResearchSubject(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::RiskAssessment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Schedule(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SearchParameter(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ServiceRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Slot(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Specimen(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SpecimenDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::StructureDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::StructureMap(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Subscription(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubscriptionStatus(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubscriptionTopic(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Substance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstanceDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstanceNucleicAcid(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstancePolymer(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstanceProtein(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstanceReferenceInformation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SubstanceSourceMaterial(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SupplyDelivery(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::SupplyRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Task(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::TerminologyCapabilities(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::TestPlan(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::TestReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::TestScript(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::Transport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::ValueSet(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::VerificationResult(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r5::Resource::VisionPrescription(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            _ => Err(serde::ser::Error::custom("invalid resource")),
        }
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r5::Resource>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<Box<fhirbolt_model::r5::Resource>>>
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r5::Resource>
{
    type Value = fhirbolt_model::r5::Resource;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r5::Resource>
{
    type Value = fhirbolt_model::r5::Resource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let mut element_context =
            self.clone::<crate::element::internal::de::InternalElement<
                { fhirbolt_shared::FhirReleases::R5 },
            >>();
        let element = element_context.deserialize(deserializer)?;
        self.from_json = false;
        if let Some(fhirbolt_element::Value::Primitive(fhirbolt_element::Primitive::String(
            resource_type,
        ))) = element.0.get("resourceType")
        {
            match resource_type.as_str() {
                "Account" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Account>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Account)
                        .map_err(serde::de::Error::custom)
                }
                "ActivityDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ActivityDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ActivityDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ActorDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ActorDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ActorDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdministrableProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: AdministrableProductDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::AdministrableProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdverseEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::AdverseEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::AdverseEvent)
                        .map_err(serde::de::Error::custom)
                }
                "AllergyIntolerance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::AllergyIntolerance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::AllergyIntolerance)
                        .map_err(serde::de::Error::custom)
                }
                "Appointment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Appointment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Appointment)
                        .map_err(serde::de::Error::custom)
                }
                "AppointmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::AppointmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::AppointmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ArtifactAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ArtifactAssessment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ArtifactAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "AuditEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::AuditEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::AuditEvent)
                        .map_err(serde::de::Error::custom)
                }
                "Basic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Basic>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Basic)
                        .map_err(serde::de::Error::custom)
                }
                "Binary" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Binary>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Binary)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: BiologicallyDerivedProduct >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::BiologicallyDerivedProduct)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProductDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: BiologicallyDerivedProductDispense >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::BiologicallyDerivedProductDispense)
                        .map_err(serde::de::Error::custom)
                }
                "BodyStructure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::BodyStructure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::BodyStructure)
                        .map_err(serde::de::Error::custom)
                }
                "Bundle" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Bundle>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Bundle)
                        .map_err(serde::de::Error::custom)
                }
                "CapabilityStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::CapabilityStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CapabilityStatement)
                        .map_err(serde::de::Error::custom)
                }
                "CarePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::CarePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CarePlan)
                        .map_err(serde::de::Error::custom)
                }
                "CareTeam" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::CareTeam>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CareTeam)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ChargeItem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ChargeItem)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::ChargeItemDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ChargeItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Citation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Citation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Citation)
                        .map_err(serde::de::Error::custom)
                }
                "Claim" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Claim>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Claim)
                        .map_err(serde::de::Error::custom)
                }
                "ClaimResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ClaimResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ClaimResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalImpression" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ClinicalImpression>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ClinicalImpression)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalUseDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::ClinicalUseDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ClinicalUseDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "CodeSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::CodeSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CodeSystem)
                        .map_err(serde::de::Error::custom)
                }
                "Communication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Communication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Communication)
                        .map_err(serde::de::Error::custom)
                }
                "CommunicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::CommunicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CommunicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CompartmentDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::CompartmentDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CompartmentDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Composition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Composition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Composition)
                        .map_err(serde::de::Error::custom)
                }
                "ConceptMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ConceptMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ConceptMap)
                        .map_err(serde::de::Error::custom)
                }
                "Condition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Condition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Condition)
                        .map_err(serde::de::Error::custom)
                }
                "ConditionDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ConditionDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ConditionDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Consent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Consent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Consent)
                        .map_err(serde::de::Error::custom)
                }
                "Contract" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Contract>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Contract)
                        .map_err(serde::de::Error::custom)
                }
                "Coverage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Coverage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Coverage)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: CoverageEligibilityRequest >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CoverageEligibilityRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: CoverageEligibilityResponse >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::CoverageEligibilityResponse)
                        .map_err(serde::de::Error::custom)
                }
                "DetectedIssue" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DetectedIssue>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DetectedIssue)
                        .map_err(serde::de::Error::custom)
                }
                "Device" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Device>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Device)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceAssociation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceAssociation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceAssociation)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceDispense>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceDispense)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceMetric" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceMetric>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceMetric)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceUsage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DeviceUsage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DeviceUsage)
                        .map_err(serde::de::Error::custom)
                }
                "DiagnosticReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DiagnosticReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DiagnosticReport)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentReference" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::DocumentReference>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::DocumentReference)
                        .map_err(serde::de::Error::custom)
                }
                "Encounter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Encounter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Encounter)
                        .map_err(serde::de::Error::custom)
                }
                "EncounterHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EncounterHistory>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EncounterHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Endpoint" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Endpoint>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Endpoint)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EnrollmentRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EnrollmentRequest)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EnrollmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EnrollmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "EpisodeOfCare" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EpisodeOfCare>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EpisodeOfCare)
                        .map_err(serde::de::Error::custom)
                }
                "EventDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EventDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EventDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Evidence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Evidence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Evidence)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EvidenceReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EvidenceReport)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceVariable" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::EvidenceVariable>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::EvidenceVariable)
                        .map_err(serde::de::Error::custom)
                }
                "ExampleScenario" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ExampleScenario>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ExampleScenario)
                        .map_err(serde::de::Error::custom)
                }
                "ExplanationOfBenefit" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::ExplanationOfBenefit>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ExplanationOfBenefit)
                        .map_err(serde::de::Error::custom)
                }
                "FamilyMemberHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::FamilyMemberHistory>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::FamilyMemberHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Flag" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Flag>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Flag)
                        .map_err(serde::de::Error::custom)
                }
                "FormularyItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::FormularyItem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::FormularyItem)
                        .map_err(serde::de::Error::custom)
                }
                "GenomicStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::GenomicStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::GenomicStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Goal" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Goal>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Goal)
                        .map_err(serde::de::Error::custom)
                }
                "GraphDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::GraphDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::GraphDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Group" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Group>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Group)
                        .map_err(serde::de::Error::custom)
                }
                "GuidanceResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::GuidanceResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::GuidanceResponse)
                        .map_err(serde::de::Error::custom)
                }
                "HealthcareService" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::HealthcareService>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::HealthcareService)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingSelection" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ImagingSelection>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ImagingSelection)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ImagingStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ImagingStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Immunization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Immunization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Immunization)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationEvaluation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::ImmunizationEvaluation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ImmunizationEvaluation)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationRecommendation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: ImmunizationRecommendation >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ImmunizationRecommendation)
                        .map_err(serde::de::Error::custom)
                }
                "ImplementationGuide" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ImplementationGuide>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ImplementationGuide)
                        .map_err(serde::de::Error::custom)
                }
                "Ingredient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Ingredient>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Ingredient)
                        .map_err(serde::de::Error::custom)
                }
                "InsurancePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::InsurancePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::InsurancePlan)
                        .map_err(serde::de::Error::custom)
                }
                "InventoryItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::InventoryItem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::InventoryItem)
                        .map_err(serde::de::Error::custom)
                }
                "InventoryReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::InventoryReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::InventoryReport)
                        .map_err(serde::de::Error::custom)
                }
                "Invoice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Invoice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Invoice)
                        .map_err(serde::de::Error::custom)
                }
                "Library" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Library>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Library)
                        .map_err(serde::de::Error::custom)
                }
                "Linkage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Linkage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Linkage)
                        .map_err(serde::de::Error::custom)
                }
                "List" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::List>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::List)
                        .map_err(serde::de::Error::custom)
                }
                "Location" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Location>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Location)
                        .map_err(serde::de::Error::custom)
                }
                "ManufacturedItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: ManufacturedItemDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ManufacturedItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Measure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Measure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Measure)
                        .map_err(serde::de::Error::custom)
                }
                "MeasureReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MeasureReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MeasureReport)
                        .map_err(serde::de::Error::custom)
                }
                "Medication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Medication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Medication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationAdministration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::MedicationAdministration>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicationAdministration)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MedicationDispense>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicationDispense)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationKnowledge" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MedicationKnowledge>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicationKnowledge)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MedicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MedicationStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicationStatement)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: MedicinalProductDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MedicinalProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MessageDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MessageDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageHeader" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MessageHeader>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MessageHeader)
                        .map_err(serde::de::Error::custom)
                }
                "MolecularSequence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::MolecularSequence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::MolecularSequence)
                        .map_err(serde::de::Error::custom)
                }
                "NamingSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::NamingSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::NamingSystem)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionIntake" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::NutritionIntake>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::NutritionIntake)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionOrder" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::NutritionOrder>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::NutritionOrder)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::NutritionProduct>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::NutritionProduct)
                        .map_err(serde::de::Error::custom)
                }
                "Observation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Observation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Observation)
                        .map_err(serde::de::Error::custom)
                }
                "ObservationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::ObservationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ObservationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::OperationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::OperationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationOutcome" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::OperationOutcome>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::OperationOutcome)
                        .map_err(serde::de::Error::custom)
                }
                "Organization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Organization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Organization)
                        .map_err(serde::de::Error::custom)
                }
                "OrganizationAffiliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::OrganizationAffiliation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::OrganizationAffiliation)
                        .map_err(serde::de::Error::custom)
                }
                "PackagedProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::PackagedProductDefinition>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::PackagedProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Parameters" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Parameters>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Parameters)
                        .map_err(serde::de::Error::custom)
                }
                "Patient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Patient>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Patient)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentNotice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::PaymentNotice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::PaymentNotice)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentReconciliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::PaymentReconciliation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::PaymentReconciliation)
                        .map_err(serde::de::Error::custom)
                }
                "Permission" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Permission>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Permission)
                        .map_err(serde::de::Error::custom)
                }
                "Person" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Person>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Person)
                        .map_err(serde::de::Error::custom)
                }
                "PlanDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::PlanDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::PlanDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Practitioner" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Practitioner>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Practitioner)
                        .map_err(serde::de::Error::custom)
                }
                "PractitionerRole" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::PractitionerRole>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::PractitionerRole)
                        .map_err(serde::de::Error::custom)
                }
                "Procedure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Procedure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Procedure)
                        .map_err(serde::de::Error::custom)
                }
                "Provenance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Provenance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Provenance)
                        .map_err(serde::de::Error::custom)
                }
                "Questionnaire" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Questionnaire>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Questionnaire)
                        .map_err(serde::de::Error::custom)
                }
                "QuestionnaireResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::QuestionnaireResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::QuestionnaireResponse)
                        .map_err(serde::de::Error::custom)
                }
                "RegulatedAuthorization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::RegulatedAuthorization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::RegulatedAuthorization)
                        .map_err(serde::de::Error::custom)
                }
                "RelatedPerson" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::RelatedPerson>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::RelatedPerson)
                        .map_err(serde::de::Error::custom)
                }
                "RequestOrchestration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::RequestOrchestration>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::RequestOrchestration)
                        .map_err(serde::de::Error::custom)
                }
                "Requirements" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Requirements>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Requirements)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ResearchStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ResearchStudy)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchSubject" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ResearchSubject>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ResearchSubject)
                        .map_err(serde::de::Error::custom)
                }
                "RiskAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::RiskAssessment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::RiskAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "Schedule" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Schedule>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Schedule)
                        .map_err(serde::de::Error::custom)
                }
                "SearchParameter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SearchParameter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SearchParameter)
                        .map_err(serde::de::Error::custom)
                }
                "ServiceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::ServiceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ServiceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Slot" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Slot>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Slot)
                        .map_err(serde::de::Error::custom)
                }
                "Specimen" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Specimen>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Specimen)
                        .map_err(serde::de::Error::custom)
                }
                "SpecimenDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SpecimenDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SpecimenDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::StructureDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::StructureDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::StructureMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::StructureMap)
                        .map_err(serde::de::Error::custom)
                }
                "Subscription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::Subscription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Subscription)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionStatus" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SubscriptionStatus>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubscriptionStatus)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionTopic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SubscriptionTopic>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubscriptionTopic)
                        .map_err(serde::de::Error::custom)
                }
                "Substance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Substance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Substance)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SubstanceDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstanceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceNucleicAcid" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::SubstanceNucleicAcid>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstanceNucleicAcid)
                        .map_err(serde::de::Error::custom)
                }
                "SubstancePolymer" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SubstancePolymer>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstancePolymer)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceProtein" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SubstanceProtein>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstanceProtein)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceReferenceInformation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r5 :: resources :: SubstanceReferenceInformation >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstanceReferenceInformation)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceSourceMaterial" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::SubstanceSourceMaterial>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SubstanceSourceMaterial)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyDelivery" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SupplyDelivery>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SupplyDelivery)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::SupplyRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::SupplyRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Task" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Task>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Task)
                        .map_err(serde::de::Error::custom)
                }
                "TerminologyCapabilities" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r5::resources::TerminologyCapabilities>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::TerminologyCapabilities)
                        .map_err(serde::de::Error::custom)
                }
                "TestPlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::TestPlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::TestPlan)
                        .map_err(serde::de::Error::custom)
                }
                "TestReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::TestReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::TestReport)
                        .map_err(serde::de::Error::custom)
                }
                "TestScript" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::TestScript>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::TestScript)
                        .map_err(serde::de::Error::custom)
                }
                "Transport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::Transport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::Transport)
                        .map_err(serde::de::Error::custom)
                }
                "ValueSet" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r5::resources::ValueSet>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::ValueSet)
                        .map_err(serde::de::Error::custom)
                }
                "VerificationResult" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::VerificationResult>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::VerificationResult)
                        .map_err(serde::de::Error::custom)
                }
                "VisionPrescription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r5::resources::VisionPrescription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r5::Resource::VisionPrescription)
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
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r5::Resource>>
{
    type Value = Box<fhirbolt_model::r5::Resource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r5::Resource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<Box<fhirbolt_model::r5::Resource>>>
{
    type Value = Vec<Box<fhirbolt_model::r5::Resource>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r5::Resource>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r5::Resource>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of resources")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<Box<fhirbolt_model::r5::Resource>>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
