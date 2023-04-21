// Generated on 2023-04-20 by fhirbolt-codegen v0.5.0
impl crate::Resource for fhirbolt_model::r4::Resource {
    const FHIR_RELEASE: fhirbolt_shared::FhirRelease = fhirbolt_shared::FhirReleases::R4;
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&fhirbolt_model::r4::Resource>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self.value {
            fhirbolt_model::r4::Resource::Account(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ActivityDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::AdverseEvent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::AllergyIntolerance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Appointment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::AppointmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::AuditEvent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Basic(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Binary(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::BiologicallyDerivedProduct(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::BodyStructure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Bundle(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CapabilityStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CarePlan(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CareTeam(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CatalogEntry(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ChargeItem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ChargeItemDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Claim(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ClaimResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ClinicalImpression(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CodeSystem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Communication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CommunicationRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CompartmentDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Composition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ConceptMap(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Condition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Consent(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Contract(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Coverage(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CoverageEligibilityRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::CoverageEligibilityResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DetectedIssue(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Device(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DeviceDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DeviceMetric(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DeviceRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DeviceUseStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DiagnosticReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DocumentManifest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::DocumentReference(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EffectEvidenceSynthesis(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Encounter(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Endpoint(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EnrollmentRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EnrollmentResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EpisodeOfCare(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EventDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Evidence(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::EvidenceVariable(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ExampleScenario(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ExplanationOfBenefit(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::FamilyMemberHistory(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Flag(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Goal(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::GraphDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Group(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::GuidanceResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::HealthcareService(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ImagingStudy(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Immunization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ImmunizationEvaluation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ImmunizationRecommendation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ImplementationGuide(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::InsurancePlan(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Invoice(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Library(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Linkage(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::List(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Location(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Measure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MeasureReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Media(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Medication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicationAdministration(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicationDispense(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicationKnowledge(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicationRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicationStatement(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProduct(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductAuthorization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductContraindication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductIndication(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductIngredient(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductInteraction(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductManufactured(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductPackaged(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductPharmaceutical(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MedicinalProductUndesirableEffect(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MessageDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MessageHeader(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::MolecularSequence(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::NamingSystem(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::NutritionOrder(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Observation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ObservationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::OperationDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::OperationOutcome(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Organization(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::OrganizationAffiliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Parameters(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Patient(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::PaymentNotice(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::PaymentReconciliation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Person(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::PlanDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Practitioner(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::PractitionerRole(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Procedure(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Provenance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Questionnaire(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::QuestionnaireResponse(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::RelatedPerson(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::RequestGroup(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ResearchDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ResearchElementDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ResearchStudy(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ResearchSubject(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::RiskAssessment(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::RiskEvidenceSynthesis(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Schedule(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SearchParameter(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ServiceRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Slot(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Specimen(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SpecimenDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::StructureDefinition(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::StructureMap(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Subscription(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Substance(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstanceNucleicAcid(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstancePolymer(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstanceProtein(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstanceReferenceInformation(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstanceSourceMaterial(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SubstanceSpecification(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SupplyDelivery(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::SupplyRequest(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::Task(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::TerminologyCapabilities(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::TestReport(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::TestScript(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::ValueSet(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::VerificationResult(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            fhirbolt_model::r4::Resource::VisionPrescription(r) => {
                self.with_context(r, |ctx| ctx.serialize(serializer))
            }
            _ => Err(serde::ser::Error::custom("invalid resource")),
        }
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Box<fhirbolt_model::r4::Resource>>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.with_context(self.value.as_ref(), |ctx| ctx.serialize(serializer))
    }
}
impl serde::ser::Serialize
    for crate::context::ser::SerializationContext<&Vec<Box<fhirbolt_model::r4::Resource>>>
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
    for crate::context::de::DeserializationContext<fhirbolt_model::r4::Resource>
{
    type Value = fhirbolt_model::r4::Resource;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4::Resource>
{
    type Value = fhirbolt_model::r4::Resource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let mut element_context =
            self.clone::<crate::element::internal::de::InternalElement<
                { fhirbolt_shared::FhirReleases::R4 },
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
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Account>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Account)
                        .map_err(serde::de::Error::custom)
                }
                "ActivityDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ActivityDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ActivityDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdverseEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::AdverseEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::AdverseEvent)
                        .map_err(serde::de::Error::custom)
                }
                "AllergyIntolerance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::AllergyIntolerance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::AllergyIntolerance)
                        .map_err(serde::de::Error::custom)
                }
                "Appointment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Appointment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Appointment)
                        .map_err(serde::de::Error::custom)
                }
                "AppointmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::AppointmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::AppointmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "AuditEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::AuditEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::AuditEvent)
                        .map_err(serde::de::Error::custom)
                }
                "Basic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Basic>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Basic)
                        .map_err(serde::de::Error::custom)
                }
                "Binary" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Binary>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Binary)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: BiologicallyDerivedProduct >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::BiologicallyDerivedProduct)
                        .map_err(serde::de::Error::custom)
                }
                "BodyStructure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::BodyStructure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::BodyStructure)
                        .map_err(serde::de::Error::custom)
                }
                "Bundle" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Bundle>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Bundle)
                        .map_err(serde::de::Error::custom)
                }
                "CapabilityStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::CapabilityStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CapabilityStatement)
                        .map_err(serde::de::Error::custom)
                }
                "CarePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::CarePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CarePlan)
                        .map_err(serde::de::Error::custom)
                }
                "CareTeam" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::CareTeam>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CareTeam)
                        .map_err(serde::de::Error::custom)
                }
                "CatalogEntry" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::CatalogEntry>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CatalogEntry)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ChargeItem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ChargeItem)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::ChargeItemDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ChargeItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Claim" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Claim>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Claim)
                        .map_err(serde::de::Error::custom)
                }
                "ClaimResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ClaimResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ClaimResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalImpression" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ClinicalImpression>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ClinicalImpression)
                        .map_err(serde::de::Error::custom)
                }
                "CodeSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::CodeSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CodeSystem)
                        .map_err(serde::de::Error::custom)
                }
                "Communication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Communication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Communication)
                        .map_err(serde::de::Error::custom)
                }
                "CommunicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::CommunicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CommunicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CompartmentDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::CompartmentDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CompartmentDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Composition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Composition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Composition)
                        .map_err(serde::de::Error::custom)
                }
                "ConceptMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ConceptMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ConceptMap)
                        .map_err(serde::de::Error::custom)
                }
                "Condition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Condition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Condition)
                        .map_err(serde::de::Error::custom)
                }
                "Consent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Consent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Consent)
                        .map_err(serde::de::Error::custom)
                }
                "Contract" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Contract>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Contract)
                        .map_err(serde::de::Error::custom)
                }
                "Coverage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Coverage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Coverage)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: CoverageEligibilityRequest >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CoverageEligibilityRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: CoverageEligibilityResponse >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::CoverageEligibilityResponse)
                        .map_err(serde::de::Error::custom)
                }
                "DetectedIssue" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DetectedIssue>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DetectedIssue)
                        .map_err(serde::de::Error::custom)
                }
                "Device" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Device>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Device)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DeviceDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DeviceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceMetric" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DeviceMetric>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DeviceMetric)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DeviceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DeviceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceUseStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DeviceUseStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DeviceUseStatement)
                        .map_err(serde::de::Error::custom)
                }
                "DiagnosticReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DiagnosticReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DiagnosticReport)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentManifest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DocumentManifest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DocumentManifest)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentReference" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::DocumentReference>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::DocumentReference)
                        .map_err(serde::de::Error::custom)
                }
                "EffectEvidenceSynthesis" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::EffectEvidenceSynthesis>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EffectEvidenceSynthesis)
                        .map_err(serde::de::Error::custom)
                }
                "Encounter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Encounter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Encounter)
                        .map_err(serde::de::Error::custom)
                }
                "Endpoint" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Endpoint>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Endpoint)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::EnrollmentRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EnrollmentRequest)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::EnrollmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EnrollmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "EpisodeOfCare" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::EpisodeOfCare>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EpisodeOfCare)
                        .map_err(serde::de::Error::custom)
                }
                "EventDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::EventDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EventDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Evidence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Evidence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Evidence)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceVariable" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::EvidenceVariable>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::EvidenceVariable)
                        .map_err(serde::de::Error::custom)
                }
                "ExampleScenario" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ExampleScenario>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ExampleScenario)
                        .map_err(serde::de::Error::custom)
                }
                "ExplanationOfBenefit" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::ExplanationOfBenefit>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ExplanationOfBenefit)
                        .map_err(serde::de::Error::custom)
                }
                "FamilyMemberHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::FamilyMemberHistory>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::FamilyMemberHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Flag" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Flag>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Flag)
                        .map_err(serde::de::Error::custom)
                }
                "Goal" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Goal>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Goal)
                        .map_err(serde::de::Error::custom)
                }
                "GraphDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::GraphDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::GraphDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Group" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Group>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Group)
                        .map_err(serde::de::Error::custom)
                }
                "GuidanceResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::GuidanceResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::GuidanceResponse)
                        .map_err(serde::de::Error::custom)
                }
                "HealthcareService" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::HealthcareService>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::HealthcareService)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ImagingStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ImagingStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Immunization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Immunization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Immunization)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationEvaluation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::ImmunizationEvaluation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ImmunizationEvaluation)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationRecommendation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: ImmunizationRecommendation >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ImmunizationRecommendation)
                        .map_err(serde::de::Error::custom)
                }
                "ImplementationGuide" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ImplementationGuide>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ImplementationGuide)
                        .map_err(serde::de::Error::custom)
                }
                "InsurancePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::InsurancePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::InsurancePlan)
                        .map_err(serde::de::Error::custom)
                }
                "Invoice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Invoice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Invoice)
                        .map_err(serde::de::Error::custom)
                }
                "Library" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Library>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Library)
                        .map_err(serde::de::Error::custom)
                }
                "Linkage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Linkage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Linkage)
                        .map_err(serde::de::Error::custom)
                }
                "List" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::List>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::List)
                        .map_err(serde::de::Error::custom)
                }
                "Location" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Location>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Location)
                        .map_err(serde::de::Error::custom)
                }
                "Measure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Measure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Measure)
                        .map_err(serde::de::Error::custom)
                }
                "MeasureReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MeasureReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MeasureReport)
                        .map_err(serde::de::Error::custom)
                }
                "Media" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Media>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Media)
                        .map_err(serde::de::Error::custom)
                }
                "Medication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Medication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Medication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationAdministration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::MedicationAdministration>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicationAdministration)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MedicationDispense>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicationDispense)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationKnowledge" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MedicationKnowledge>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicationKnowledge)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MedicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MedicationStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicationStatement)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MedicinalProduct>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProduct)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductAuthorization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductAuthorization >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductAuthorization)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductContraindication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductContraindication >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductContraindication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductIndication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIndication >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductIndication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductIngredient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductIngredient >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductIngredient)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductInteraction" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductInteraction >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductInteraction)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductManufactured" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductManufactured >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductManufactured)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductPackaged" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::MedicinalProductPackaged>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductPackaged)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductPharmaceutical" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductPharmaceutical >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductPharmaceutical)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductUndesirableEffect" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: MedicinalProductUndesirableEffect >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MedicinalProductUndesirableEffect)
                        .map_err(serde::de::Error::custom)
                }
                "MessageDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MessageDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MessageDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageHeader" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MessageHeader>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MessageHeader)
                        .map_err(serde::de::Error::custom)
                }
                "MolecularSequence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::MolecularSequence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::MolecularSequence)
                        .map_err(serde::de::Error::custom)
                }
                "NamingSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::NamingSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::NamingSystem)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionOrder" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::NutritionOrder>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::NutritionOrder)
                        .map_err(serde::de::Error::custom)
                }
                "Observation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Observation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Observation)
                        .map_err(serde::de::Error::custom)
                }
                "ObservationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::ObservationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ObservationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::OperationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::OperationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationOutcome" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::OperationOutcome>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::OperationOutcome)
                        .map_err(serde::de::Error::custom)
                }
                "Organization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Organization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Organization)
                        .map_err(serde::de::Error::custom)
                }
                "OrganizationAffiliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::OrganizationAffiliation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::OrganizationAffiliation)
                        .map_err(serde::de::Error::custom)
                }
                "Parameters" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Parameters>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Parameters)
                        .map_err(serde::de::Error::custom)
                }
                "Patient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Patient>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Patient)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentNotice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::PaymentNotice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::PaymentNotice)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentReconciliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::PaymentReconciliation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::PaymentReconciliation)
                        .map_err(serde::de::Error::custom)
                }
                "Person" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Person>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Person)
                        .map_err(serde::de::Error::custom)
                }
                "PlanDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::PlanDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::PlanDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Practitioner" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Practitioner>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Practitioner)
                        .map_err(serde::de::Error::custom)
                }
                "PractitionerRole" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::PractitionerRole>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::PractitionerRole)
                        .map_err(serde::de::Error::custom)
                }
                "Procedure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Procedure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Procedure)
                        .map_err(serde::de::Error::custom)
                }
                "Provenance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Provenance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Provenance)
                        .map_err(serde::de::Error::custom)
                }
                "Questionnaire" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Questionnaire>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Questionnaire)
                        .map_err(serde::de::Error::custom)
                }
                "QuestionnaireResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::QuestionnaireResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::QuestionnaireResponse)
                        .map_err(serde::de::Error::custom)
                }
                "RelatedPerson" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::RelatedPerson>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::RelatedPerson)
                        .map_err(serde::de::Error::custom)
                }
                "RequestGroup" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::RequestGroup>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::RequestGroup)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ResearchDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ResearchDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchElementDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::ResearchElementDefinition>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ResearchElementDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ResearchStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ResearchStudy)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchSubject" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ResearchSubject>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ResearchSubject)
                        .map_err(serde::de::Error::custom)
                }
                "RiskAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::RiskAssessment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::RiskAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "RiskEvidenceSynthesis" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::RiskEvidenceSynthesis>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::RiskEvidenceSynthesis)
                        .map_err(serde::de::Error::custom)
                }
                "Schedule" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Schedule>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Schedule)
                        .map_err(serde::de::Error::custom)
                }
                "SearchParameter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SearchParameter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SearchParameter)
                        .map_err(serde::de::Error::custom)
                }
                "ServiceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::ServiceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ServiceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Slot" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Slot>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Slot)
                        .map_err(serde::de::Error::custom)
                }
                "Specimen" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Specimen>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Specimen)
                        .map_err(serde::de::Error::custom)
                }
                "SpecimenDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SpecimenDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SpecimenDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::StructureDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::StructureDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::StructureMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::StructureMap)
                        .map_err(serde::de::Error::custom)
                }
                "Subscription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::Subscription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Subscription)
                        .map_err(serde::de::Error::custom)
                }
                "Substance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Substance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Substance)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceNucleicAcid" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::SubstanceNucleicAcid>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstanceNucleicAcid)
                        .map_err(serde::de::Error::custom)
                }
                "SubstancePolymer" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SubstancePolymer>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstancePolymer)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceProtein" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SubstanceProtein>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstanceProtein)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceReferenceInformation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4 :: resources :: SubstanceReferenceInformation >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstanceReferenceInformation)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceSourceMaterial" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::SubstanceSourceMaterial>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstanceSourceMaterial)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceSpecification" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::SubstanceSpecification>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SubstanceSpecification)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyDelivery" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SupplyDelivery>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SupplyDelivery)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::SupplyRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::SupplyRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Task" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::Task>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::Task)
                        .map_err(serde::de::Error::custom)
                }
                "TerminologyCapabilities" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4::resources::TerminologyCapabilities>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::TerminologyCapabilities)
                        .map_err(serde::de::Error::custom)
                }
                "TestReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::TestReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::TestReport)
                        .map_err(serde::de::Error::custom)
                }
                "TestScript" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::TestScript>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::TestScript)
                        .map_err(serde::de::Error::custom)
                }
                "ValueSet" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4::resources::ValueSet>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::ValueSet)
                        .map_err(serde::de::Error::custom)
                }
                "VerificationResult" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::VerificationResult>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::VerificationResult)
                        .map_err(serde::de::Error::custom)
                }
                "VisionPrescription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4::resources::VisionPrescription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4::Resource::VisionPrescription)
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
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4::Resource>>
{
    type Value = Box<fhirbolt_model::r4::Resource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4::Resource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<Box<fhirbolt_model::r4::Resource>>>
{
    type Value = Vec<Box<fhirbolt_model::r4::Resource>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4::Resource>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4::Resource>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of resources")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<Box<fhirbolt_model::r4::Resource>>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}
