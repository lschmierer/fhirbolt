// Generated on 2023-04-12 by fhirbolt-codegen v0.1.0
impl crate::Resource for fhirbolt_model::r4b::Resource {
    const FHIR_RELEASE: crate::FhirRelease = crate::FhirRelease::R4B;
}
impl<'de> serde::de::DeserializeSeed<'de>
    for crate::context::de::DeserializationContext<fhirbolt_model::r4b::Resource>
{
    type Value = fhirbolt_model::r4b::Resource;
    fn deserialize<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        (&mut self).deserialize(deserializer)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<fhirbolt_model::r4b::Resource>
{
    type Value = fhirbolt_model::r4b::Resource;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use crate::Resource;
        let element_context = self . clone :: < fhirbolt_shared :: element :: Element < { fhirbolt_model :: r4b :: Resource :: FHIR_RELEASE } >> () ;
        let element = element_context.deserialize(deserializer)?;
        self.from_json = false;
        if let Some(fhirbolt_shared::element::Value::Primitive(
            fhirbolt_shared::element::Primitive::String(resource_type),
        )) = element.get("resourceType")
        {
            match resource_type.as_str() {
                "Account" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Account>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Account)
                        .map_err(serde::de::Error::custom)
                }
                "ActivityDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ActivityDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ActivityDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdministrableProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: AdministrableProductDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::AdministrableProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "AdverseEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::AdverseEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::AdverseEvent)
                        .map_err(serde::de::Error::custom)
                }
                "AllergyIntolerance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::AllergyIntolerance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::AllergyIntolerance)
                        .map_err(serde::de::Error::custom)
                }
                "Appointment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Appointment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Appointment)
                        .map_err(serde::de::Error::custom)
                }
                "AppointmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::AppointmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::AppointmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "AuditEvent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::AuditEvent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::AuditEvent)
                        .map_err(serde::de::Error::custom)
                }
                "Basic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Basic>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Basic)
                        .map_err(serde::de::Error::custom)
                }
                "Binary" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Binary>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Binary)
                        .map_err(serde::de::Error::custom)
                }
                "BiologicallyDerivedProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: BiologicallyDerivedProduct >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::BiologicallyDerivedProduct)
                        .map_err(serde::de::Error::custom)
                }
                "BodyStructure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::BodyStructure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::BodyStructure)
                        .map_err(serde::de::Error::custom)
                }
                "Bundle" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Bundle>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Bundle)
                        .map_err(serde::de::Error::custom)
                }
                "CapabilityStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::CapabilityStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CapabilityStatement)
                        .map_err(serde::de::Error::custom)
                }
                "CarePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::CarePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CarePlan)
                        .map_err(serde::de::Error::custom)
                }
                "CareTeam" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::CareTeam>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CareTeam)
                        .map_err(serde::de::Error::custom)
                }
                "CatalogEntry" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::CatalogEntry>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CatalogEntry)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ChargeItem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ChargeItem)
                        .map_err(serde::de::Error::custom)
                }
                "ChargeItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ChargeItemDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ChargeItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Citation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Citation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Citation)
                        .map_err(serde::de::Error::custom)
                }
                "Claim" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Claim>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Claim)
                        .map_err(serde::de::Error::custom)
                }
                "ClaimResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ClaimResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ClaimResponse)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalImpression" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ClinicalImpression>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ClinicalImpression)
                        .map_err(serde::de::Error::custom)
                }
                "ClinicalUseDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ClinicalUseDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ClinicalUseDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "CodeSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::CodeSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CodeSystem)
                        .map_err(serde::de::Error::custom)
                }
                "Communication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Communication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Communication)
                        .map_err(serde::de::Error::custom)
                }
                "CommunicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::CommunicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CommunicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CompartmentDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::CompartmentDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CompartmentDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Composition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Composition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Composition)
                        .map_err(serde::de::Error::custom)
                }
                "ConceptMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ConceptMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ConceptMap)
                        .map_err(serde::de::Error::custom)
                }
                "Condition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Condition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Condition)
                        .map_err(serde::de::Error::custom)
                }
                "Consent" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Consent>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Consent)
                        .map_err(serde::de::Error::custom)
                }
                "Contract" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Contract>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Contract)
                        .map_err(serde::de::Error::custom)
                }
                "Coverage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Coverage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Coverage)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CoverageEligibilityRequest >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CoverageEligibilityRequest)
                        .map_err(serde::de::Error::custom)
                }
                "CoverageEligibilityResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: CoverageEligibilityResponse >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::CoverageEligibilityResponse)
                        .map_err(serde::de::Error::custom)
                }
                "DetectedIssue" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DetectedIssue>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DetectedIssue)
                        .map_err(serde::de::Error::custom)
                }
                "Device" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Device>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Device)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DeviceDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DeviceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceMetric" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DeviceMetric>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DeviceMetric)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DeviceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DeviceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "DeviceUseStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DeviceUseStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DeviceUseStatement)
                        .map_err(serde::de::Error::custom)
                }
                "DiagnosticReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DiagnosticReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DiagnosticReport)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentManifest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DocumentManifest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DocumentManifest)
                        .map_err(serde::de::Error::custom)
                }
                "DocumentReference" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::DocumentReference>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::DocumentReference)
                        .map_err(serde::de::Error::custom)
                }
                "Encounter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Encounter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Encounter)
                        .map_err(serde::de::Error::custom)
                }
                "Endpoint" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Endpoint>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Endpoint)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EnrollmentRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EnrollmentRequest)
                        .map_err(serde::de::Error::custom)
                }
                "EnrollmentResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EnrollmentResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EnrollmentResponse)
                        .map_err(serde::de::Error::custom)
                }
                "EpisodeOfCare" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EpisodeOfCare>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EpisodeOfCare)
                        .map_err(serde::de::Error::custom)
                }
                "EventDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EventDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EventDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Evidence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Evidence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Evidence)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EvidenceReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EvidenceReport)
                        .map_err(serde::de::Error::custom)
                }
                "EvidenceVariable" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::EvidenceVariable>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::EvidenceVariable)
                        .map_err(serde::de::Error::custom)
                }
                "ExampleScenario" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ExampleScenario>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ExampleScenario)
                        .map_err(serde::de::Error::custom)
                }
                "ExplanationOfBenefit" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ExplanationOfBenefit>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ExplanationOfBenefit)
                        .map_err(serde::de::Error::custom)
                }
                "FamilyMemberHistory" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::FamilyMemberHistory>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::FamilyMemberHistory)
                        .map_err(serde::de::Error::custom)
                }
                "Flag" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Flag>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Flag)
                        .map_err(serde::de::Error::custom)
                }
                "Goal" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Goal>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Goal)
                        .map_err(serde::de::Error::custom)
                }
                "GraphDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::GraphDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::GraphDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Group" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Group>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Group)
                        .map_err(serde::de::Error::custom)
                }
                "GuidanceResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::GuidanceResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::GuidanceResponse)
                        .map_err(serde::de::Error::custom)
                }
                "HealthcareService" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::HealthcareService>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::HealthcareService)
                        .map_err(serde::de::Error::custom)
                }
                "ImagingStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ImagingStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ImagingStudy)
                        .map_err(serde::de::Error::custom)
                }
                "Immunization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Immunization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Immunization)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationEvaluation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ImmunizationEvaluation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ImmunizationEvaluation)
                        .map_err(serde::de::Error::custom)
                }
                "ImmunizationRecommendation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: ImmunizationRecommendation >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ImmunizationRecommendation)
                        .map_err(serde::de::Error::custom)
                }
                "ImplementationGuide" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ImplementationGuide>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ImplementationGuide)
                        .map_err(serde::de::Error::custom)
                }
                "Ingredient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Ingredient>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Ingredient)
                        .map_err(serde::de::Error::custom)
                }
                "InsurancePlan" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::InsurancePlan>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::InsurancePlan)
                        .map_err(serde::de::Error::custom)
                }
                "Invoice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Invoice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Invoice)
                        .map_err(serde::de::Error::custom)
                }
                "Library" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Library>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Library)
                        .map_err(serde::de::Error::custom)
                }
                "Linkage" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Linkage>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Linkage)
                        .map_err(serde::de::Error::custom)
                }
                "List" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::List>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::List)
                        .map_err(serde::de::Error::custom)
                }
                "Location" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Location>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Location)
                        .map_err(serde::de::Error::custom)
                }
                "ManufacturedItemDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: ManufacturedItemDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ManufacturedItemDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Measure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Measure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Measure)
                        .map_err(serde::de::Error::custom)
                }
                "MeasureReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MeasureReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MeasureReport)
                        .map_err(serde::de::Error::custom)
                }
                "Media" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Media>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Media)
                        .map_err(serde::de::Error::custom)
                }
                "Medication" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Medication>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Medication)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationAdministration" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::MedicationAdministration>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicationAdministration)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationDispense" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MedicationDispense>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicationDispense)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationKnowledge" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::MedicationKnowledge>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicationKnowledge)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MedicationRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicationRequest)
                        .map_err(serde::de::Error::custom)
                }
                "MedicationStatement" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::MedicationStatement>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicationStatement)
                        .map_err(serde::de::Error::custom)
                }
                "MedicinalProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: MedicinalProductDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MedicinalProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MessageDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MessageDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "MessageHeader" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MessageHeader>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MessageHeader)
                        .map_err(serde::de::Error::custom)
                }
                "MolecularSequence" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::MolecularSequence>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::MolecularSequence)
                        .map_err(serde::de::Error::custom)
                }
                "NamingSystem" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::NamingSystem>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::NamingSystem)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionOrder" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::NutritionOrder>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::NutritionOrder)
                        .map_err(serde::de::Error::custom)
                }
                "NutritionProduct" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::NutritionProduct>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::NutritionProduct)
                        .map_err(serde::de::Error::custom)
                }
                "Observation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Observation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Observation)
                        .map_err(serde::de::Error::custom)
                }
                "ObservationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::ObservationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ObservationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::OperationDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::OperationDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "OperationOutcome" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::OperationOutcome>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::OperationOutcome)
                        .map_err(serde::de::Error::custom)
                }
                "Organization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Organization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Organization)
                        .map_err(serde::de::Error::custom)
                }
                "OrganizationAffiliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::OrganizationAffiliation>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::OrganizationAffiliation)
                        .map_err(serde::de::Error::custom)
                }
                "PackagedProductDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: PackagedProductDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::PackagedProductDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Parameters" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Parameters>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Parameters)
                        .map_err(serde::de::Error::custom)
                }
                "Patient" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Patient>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Patient)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentNotice" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::PaymentNotice>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::PaymentNotice)
                        .map_err(serde::de::Error::custom)
                }
                "PaymentReconciliation" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::PaymentReconciliation>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::PaymentReconciliation)
                        .map_err(serde::de::Error::custom)
                }
                "Person" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Person>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Person)
                        .map_err(serde::de::Error::custom)
                }
                "PlanDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::PlanDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::PlanDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "Practitioner" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Practitioner>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Practitioner)
                        .map_err(serde::de::Error::custom)
                }
                "PractitionerRole" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::PractitionerRole>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::PractitionerRole)
                        .map_err(serde::de::Error::custom)
                }
                "Procedure" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Procedure>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Procedure)
                        .map_err(serde::de::Error::custom)
                }
                "Provenance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Provenance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Provenance)
                        .map_err(serde::de::Error::custom)
                }
                "Questionnaire" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Questionnaire>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Questionnaire)
                        .map_err(serde::de::Error::custom)
                }
                "QuestionnaireResponse" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::QuestionnaireResponse>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::QuestionnaireResponse)
                        .map_err(serde::de::Error::custom)
                }
                "RegulatedAuthorization" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::RegulatedAuthorization>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::RegulatedAuthorization)
                        .map_err(serde::de::Error::custom)
                }
                "RelatedPerson" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::RelatedPerson>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::RelatedPerson)
                        .map_err(serde::de::Error::custom)
                }
                "RequestGroup" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::RequestGroup>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::RequestGroup)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ResearchDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ResearchDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchElementDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self . transmute :: < Box < fhirbolt_model :: r4b :: resources :: ResearchElementDefinition >> () ;
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ResearchElementDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchStudy" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ResearchStudy>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ResearchStudy)
                        .map_err(serde::de::Error::custom)
                }
                "ResearchSubject" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ResearchSubject>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ResearchSubject)
                        .map_err(serde::de::Error::custom)
                }
                "RiskAssessment" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::RiskAssessment>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::RiskAssessment)
                        .map_err(serde::de::Error::custom)
                }
                "Schedule" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Schedule>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Schedule)
                        .map_err(serde::de::Error::custom)
                }
                "SearchParameter" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SearchParameter>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SearchParameter)
                        .map_err(serde::de::Error::custom)
                }
                "ServiceRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::ServiceRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ServiceRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Slot" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Slot>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Slot)
                        .map_err(serde::de::Error::custom)
                }
                "Specimen" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Specimen>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Specimen)
                        .map_err(serde::de::Error::custom)
                }
                "SpecimenDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SpecimenDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SpecimenDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::StructureDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::StructureDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "StructureMap" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::StructureMap>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::StructureMap)
                        .map_err(serde::de::Error::custom)
                }
                "Subscription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Subscription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Subscription)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionStatus" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SubscriptionStatus>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SubscriptionStatus)
                        .map_err(serde::de::Error::custom)
                }
                "SubscriptionTopic" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SubscriptionTopic>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SubscriptionTopic)
                        .map_err(serde::de::Error::custom)
                }
                "Substance" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::Substance>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Substance)
                        .map_err(serde::de::Error::custom)
                }
                "SubstanceDefinition" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::SubstanceDefinition>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SubstanceDefinition)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyDelivery" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SupplyDelivery>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SupplyDelivery)
                        .map_err(serde::de::Error::custom)
                }
                "SupplyRequest" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::SupplyRequest>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::SupplyRequest)
                        .map_err(serde::de::Error::custom)
                }
                "Task" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::Task>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::Task)
                        .map_err(serde::de::Error::custom)
                }
                "TerminologyCapabilities" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self
                        .transmute::<Box<fhirbolt_model::r4b::resources::TerminologyCapabilities>>(
                        );
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::TerminologyCapabilities)
                        .map_err(serde::de::Error::custom)
                }
                "TestReport" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::TestReport>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::TestReport)
                        .map_err(serde::de::Error::custom)
                }
                "TestScript" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::TestScript>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::TestScript)
                        .map_err(serde::de::Error::custom)
                }
                "ValueSet" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context = self.transmute::<Box<fhirbolt_model::r4b::resources::ValueSet>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::ValueSet)
                        .map_err(serde::de::Error::custom)
                }
                "VerificationResult" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::VerificationResult>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::VerificationResult)
                        .map_err(serde::de::Error::custom)
                }
                "VisionPrescription" => {
                    let deserializer = crate::element::de::Deserializer(element);
                    let context =
                        self.transmute::<Box<fhirbolt_model::r4b::resources::VisionPrescription>>();
                    context
                        .deserialize(deserializer)
                        .map(fhirbolt_model::r4b::Resource::VisionPrescription)
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
    for &mut crate::context::de::DeserializationContext<Box<fhirbolt_model::r4b::Resource>>
{
    type Value = Box<fhirbolt_model::r4b::Resource>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        self.transmute::<fhirbolt_model::r4b::Resource>()
            .deserialize(deserializer)
            .map(Box::new)
    }
}
impl<'de> serde::de::DeserializeSeed<'de>
    for &mut crate::context::de::DeserializationContext<Vec<Box<fhirbolt_model::r4b::Resource>>>
{
    type Value = Vec<Box<fhirbolt_model::r4b::Resource>>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct Visitor<'a>(
            &'a mut crate::context::de::DeserializationContext<
                Vec<Box<fhirbolt_model::r4b::Resource>>,
            >,
        );
        impl<'de> serde::de::Visitor<'de> for Visitor<'_> {
            type Value = Vec<Box<fhirbolt_model::r4b::Resource>>;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of resources")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) =
                    seq.next_element_seed(self.0.transmute::<Box<fhirbolt_model::r4b::Resource>>())?
                {
                    values.push(value);
                }
                Ok(values)
            }
        }
        deserializer.deserialize_seq(Visitor(self))
    }
}