// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Enum representing all possible FHIR resources."]
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Resource {
    #[doc = "A financial tool for tracking value accrued for a particular purpose.  In the healthcare field, used to track charges for a patient, cost centers, etc."]
    Account(Box<super::resources::Account>),
    #[doc = "This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context."]
    ActivityDefinition(Box<super::resources::ActivityDefinition>),
    #[doc = "The ActorDefinition resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays."]
    ActorDefinition(Box<super::resources::ActorDefinition>),
    #[doc = "A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed)."]
    AdministrableProductDefinition(Box<super::resources::AdministrableProductDefinition>),
    #[doc = "An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatment, hospitalization, or may result in death. The AdverseEvent resource also extends to potential or avoided events that could have had such effects. There are two major domains where the AdverseEvent resource is expected to be used. One is in clinical care reported adverse events and the other is in reporting adverse events in clinical  research trial management.  Adverse events can be reported by healthcare providers, patients, caregivers or by medical products manufacturers.  Given the differences between these two concepts, we recommend consulting the domain specific implementation guides when implementing the AdverseEvent Resource. The implementation guides include specific extensions, value sets and constraints."]
    AdverseEvent(Box<super::resources::AdverseEvent>),
    #[doc = "Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.\n\nTo record a clinical assessment of a propensity, or potential risk to an individual, of an adverse reaction upon future exposure to the specified substance, or class of substance."]
    AllergyIntolerance(Box<super::resources::AllergyIntolerance>),
    #[doc = "A booking of a healthcare event among patient(s), practitioner(s), related person(s) and/or device(s) for a specific date/time. This may result in one or more Encounter(s)."]
    Appointment(Box<super::resources::Appointment>),
    #[doc = "A reply to an appointment request for a patient and/or practitioner(s), such as a confirmation or rejection."]
    AppointmentResponse(Box<super::resources::AppointmentResponse>),
    #[doc = "This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content."]
    ArtifactAssessment(Box<super::resources::ArtifactAssessment>),
    #[doc = "A record of an event relevant for purposes such as operations, privacy, security, maintenance, and performance analysis."]
    AuditEvent(Box<super::resources::AuditEvent>),
    #[doc = "Basic is used for handling concepts not yet defined in FHIR, narrative-only resources that don't map to an existing resource, and custom resources not appropriate for inclusion in the FHIR specification.\n\nNeed some way to safely (without breaking interoperability) allow implementers to exchange content not supported by the initial set of declared resources."]
    Basic(Box<super::resources::Basic>),
    #[doc = "A resource that represents the data of a single raw artifact as digital content accessible in its native format.  A Binary resource can contain any content, whether text, image, pdf, zip archive, etc.\n\nThere are situations where it is useful or required to handle pure binary content using the same framework as other resources."]
    Binary(Box<super::resources::Binary>),
    #[doc = "A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity."]
    BiologicallyDerivedProduct(Box<super::resources::BiologicallyDerivedProduct>),
    #[doc = "A record of dispensation of a biologically derived product."]
    BiologicallyDerivedProductDispense(Box<super::resources::BiologicallyDerivedProductDispense>),
    #[doc = "Record details about an anatomical structure.  This resource may be used when a coded concept does not provide the necessary detail needed for the use case."]
    BodyStructure(Box<super::resources::BodyStructure>),
    #[doc = "A container for a collection of resources."]
    Bundle(Box<super::resources::Bundle>),
    #[doc = "A Capability Statement documents a set of capabilities (behaviors) of a FHIR Server or Client for a particular version of FHIR that may be used as a statement of actual server functionality or a statement of required or desired server implementation."]
    CapabilityStatement(Box<super::resources::CapabilityStatement>),
    #[doc = "Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition or set of conditions."]
    CarePlan(Box<super::resources::CarePlan>),
    #[doc = "The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care."]
    CareTeam(Box<super::resources::CareTeam>),
    #[doc = "The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provision, like date, time, amounts and participating organizations and persons. Main Usage of the ChargeItem is to enable the billing process and internal cost allocation."]
    ChargeItem(Box<super::resources::ChargeItem>),
    #[doc = "The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, therefore this resource gives only a rough structure and requires profiling for each type of billing code system."]
    ChargeItemDefinition(Box<super::resources::ChargeItemDefinition>),
    #[doc = "The Citation Resource enables reference to any knowledge artifact for purposes of identification and attribution. The Citation Resource supports existing reference structures and developing publication practices such as versioning, expressing complex contributorship roles, and referencing computable resources."]
    Citation(Box<super::resources::Citation>),
    #[doc = "A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.\n\nThe Claim resource is used by providers to exchange services and products rendered to patients or planned to be rendered with insurers for reimbuserment. It is also used by insurers to exchange claims information with statutory reporting and data analytics firms."]
    Claim(Box<super::resources::Claim>),
    #[doc = "This resource provides the adjudication details from the processing of a Claim resource."]
    ClaimResponse(Box<super::resources::ClaimResponse>),
    #[doc = "A record of a clinical assessment performed to determine what problem(s) may affect the patient and before planning the treatments or management strategies that are best to manage a patient's condition. Assessments are often 1:1 with a clinical consultation / encounter,  but this varies greatly depending on the clinical workflow. This resource is called \"ClinicalImpression\" rather than \"ClinicalAssessment\" to avoid confusion with the recording of assessment tools such as Apgar score."]
    ClinicalImpression(Box<super::resources::ClinicalImpression>),
    #[doc = "A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure."]
    ClinicalUseDefinition(Box<super::resources::ClinicalUseDefinition>),
    #[doc = "The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content."]
    CodeSystem(Box<super::resources::CodeSystem>),
    #[doc = "A clinical or business level record of information being transmitted or shared; e.g. an alert that was sent to a responsible provider, a public health agency communication to a provider/reporter in response to a case report for a reportable condition."]
    Communication(Box<super::resources::Communication>),
    #[doc = "A request to convey information; e.g. the CDS system proposes that an alert be sent to a responsible provider, the CDS system proposes that the public health agency be notified about a reportable condition."]
    CommunicationRequest(Box<super::resources::CommunicationRequest>),
    #[doc = "A compartment definition that defines how resources are accessed on a server."]
    CompartmentDefinition(Box<super::resources::CompartmentDefinition>),
    #[doc = "A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clinical attestation with regard to who is making the statement. A Composition defines the structure and narrative content necessary for a document. However, a Composition alone does not constitute a document. Rather, the Composition must be the first entry in a Bundle where Bundle.type=document, and any other resources referenced from Composition must be included as subsequent entries in the Bundle (for example Patient, Practitioner, Encounter, etc.).\n\nTo support documents, and also to capture the EN13606 notion of an attested commit to the patient EHR, and to allow a set of disparate resources at the information/engineering level to be gathered into a clinical statement."]
    Composition(Box<super::resources::Composition>),
    #[doc = "A statement of relationships from one set of concepts to one or more other concepts - either concepts in code systems, or data element/data element concepts, or classes in class models."]
    ConceptMap(Box<super::resources::ConceptMap>),
    #[doc = "A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern."]
    Condition(Box<super::resources::Condition>),
    #[doc = "A definition of a condition and information relevant to managing it."]
    ConditionDefinition(Box<super::resources::ConditionDefinition>),
    #[doc = "A record of a healthcare consumer’s  choices  or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purposes and periods of time."]
    Consent(Box<super::resources::Consent>),
    #[doc = "Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement."]
    Contract(Box<super::resources::Contract>),
    #[doc = "Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.\n\nCoverage provides a link between covered parties (patients) and the payors of their healthcare costs (both insurance and self-pay)."]
    Coverage(Box<super::resources::Coverage>),
    #[doc = "The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding whether the stated coverage is valid and in-force and optionally to provide the insurance details of the policy."]
    CoverageEligibilityRequest(Box<super::resources::CoverageEligibilityRequest>),
    #[doc = "This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource."]
    CoverageEligibilityResponse(Box<super::resources::CoverageEligibilityResponse>),
    #[doc = "Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure-condition conflict, gaps in care, etc."]
    DetectedIssue(Box<super::resources::DetectedIssue>),
    #[doc = "This resource describes the properties (regulated, has real time clock, etc.), adminstrative (manufacturer name, model number, serial number, firmware, etc.), and type (knee replacement, blood pressure cuff, MRI, etc.) of a physical unit (these values do not change much within a given module, for example the serail number, manufacturer name, and model number). An actual unit may consist of several modules in a distinct hierarchy and these are represented by multiple Device resources and bound through the 'parent' element.\n\nAllows institutions to track their devices."]
    Device(Box<super::resources::Device>),
    #[doc = "A record of association of a device."]
    DeviceAssociation(Box<super::resources::DeviceAssociation>),
    #[doc = "This is a specialized resource that defines the characteristics and capabilities of a device."]
    DeviceDefinition(Box<super::resources::DeviceDefinition>),
    #[doc = "Indicates that a device is to be or has been dispensed for a named person/patient.  This includes a description of the product (supply) provided and the instructions for using the device."]
    DeviceDispense(Box<super::resources::DeviceDispense>),
    #[doc = "Describes a measurement, calculation or setting capability of a device.  The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely applicable. "]
    DeviceMetric(Box<super::resources::DeviceMetric>),
    #[doc = "Represents a request a device to be provided to a specific patient. The device may be an implantable device to be subsequently implanted, or an external assistive device, such as a walker, to be delivered and subsequently be used."]
    DeviceRequest(Box<super::resources::DeviceRequest>),
    #[doc = "A record of a device being used by a patient where the record is the result of a report from the patient or a clinician."]
    DeviceUsage(Box<super::resources::DeviceUsage>),
    #[doc = "The findings and interpretation of diagnostic tests performed on patients, groups of patients, products, substances, devices, and locations, and/or specimens derived from these. The report includes clinical context such as requesting provider information, and some mix of atomic results, images, textual and coded interpretations, and formatted representation of diagnostic reports. The report also includes non-clinical context such as batch analysis and stability reporting of products and substances.\n\nTo support reporting for any diagnostic report into a clinical data repository."]
    DiagnosticReport(Box<super::resources::DiagnosticReport>),
    #[doc = "A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mime-type, it includes formal patient-centric documents (CDA), clinical notes, scanned paper, non-patient specific documents like policy text, as well as a photo, video, or audio recording acquired or used in healthcare.  The DocumentReference resource provides metadata about the document so that the document can be discovered and managed.  The actual content may be inline base64 encoded data or provided by direct reference."]
    DocumentReference(Box<super::resources::DocumentReference>),
    #[doc = "An interaction between healthcare provider(s), and/or patient(s) for the purpose of providing healthcare service(s) or assessing the health status of patient(s)."]
    Encounter(Box<super::resources::Encounter>),
    #[doc = "A record of significant events/milestones key data throughout the history of an Encounter"]
    EncounterHistory(Box<super::resources::EncounterHistory>),
    #[doc = "The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may include any security context information."]
    Endpoint(Box<super::resources::Endpoint>),
    #[doc = "This resource provides the insurance enrollment details to the insurer regarding a specified coverage."]
    EnrollmentRequest(Box<super::resources::EnrollmentRequest>),
    #[doc = "This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource."]
    EnrollmentResponse(Box<super::resources::EnrollmentResponse>),
    #[doc = "An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient during this time."]
    EpisodeOfCare(Box<super::resources::EpisodeOfCare>),
    #[doc = "The EventDefinition resource provides a reusable description of when a particular event can occur."]
    EventDefinition(Box<super::resources::EventDefinition>),
    #[doc = "The Evidence Resource provides a machine-interpretable expression of an evidence concept including the evidence variables (e.g., population, exposures/interventions, comparators, outcomes, measured variables, confounding variables), the statistics, and the certainty of this evidence."]
    Evidence(Box<super::resources::Evidence>),
    #[doc = "The EvidenceReport Resource is a specialized container for a collection of resources and codeable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and related concepts."]
    EvidenceReport(Box<super::resources::EvidenceReport>),
    #[doc = "The EvidenceVariable resource describes an element that knowledge (Evidence) is about.\n\nNeed to be able to define and reuse the definition of individual elements of a research question."]
    EvidenceVariable(Box<super::resources::EvidenceVariable>),
    #[doc = "A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time."]
    ExampleScenario(Box<super::resources::ExampleScenario>),
    #[doc = "This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided."]
    ExplanationOfBenefit(Box<super::resources::ExplanationOfBenefit>),
    #[doc = "Significant health conditions for a person related to the patient relevant in the context of care for the patient."]
    FamilyMemberHistory(Box<super::resources::FamilyMemberHistory>),
    #[doc = "Prospective warnings of potential issues when providing care to the patient."]
    Flag(Box<super::resources::Flag>),
    #[doc = "This resource describes a product or service that is available through a program and includes the conditions and constraints of availability.  All of the information in this resource is specific to the inclusion of the item in the formulary and is not inherent to the item itself."]
    FormularyItem(Box<super::resources::FormularyItem>),
    #[doc = "A set of analyses performed to analyze and generate genomic data."]
    GenomicStudy(Box<super::resources::GenomicStudy>),
    #[doc = "Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a process improvement objective, etc."]
    Goal(Box<super::resources::Goal>),
    #[doc = "A formal computable definition of a graph of resources - that is, a coherent set of resources that form a graph by following references. The Graph Definition resource defines a set and makes rules about the set."]
    GraphDefinition(Box<super::resources::GraphDefinition>),
    #[doc = "Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a collection of entities that isn't an Organization."]
    Group(Box<super::resources::Group>),
    #[doc = "A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.\n\nThe GuidanceResponse resource supports recording the results of decision support interactions, reportability determination for public health, as well as the communication of additional data requirements for subsequent interactions."]
    GuidanceResponse(Box<super::resources::GuidanceResponse>),
    #[doc = "The details of a healthcare service available at a location or in a catalog.  In the case where there is a hierarchy of services (for example, Lab -> Pathology -> Wound Cultures), this can be represented using a set of linked HealthcareServices."]
    HealthcareService(Box<super::resources::HealthcareService>),
    #[doc = "A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allowing linkage to an Observation Resource or transferring this information along with the ImagingStudy Resource."]
    ImagingSelection(Box<super::resources::ImagingSelection>),
    #[doc = "Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other data) acquired or produced in a common context.  A series is of only one modality (e.g. X-ray, CT, MR, ultrasound), but a study may have multiple series of different modalities."]
    ImagingStudy(Box<super::resources::ImagingStudy>),
    #[doc = "Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party."]
    Immunization(Box<super::resources::Immunization>),
    #[doc = "Describes a comparison of an immunization event against published recommendations to determine if the administration is \"valid\" in relation to those  recommendations."]
    ImmunizationEvaluation(Box<super::resources::ImmunizationEvaluation>),
    #[doc = "A patient's point-in-time set of recommendations (i.e. forecasting) according to a published schedule with optional supporting justification."]
    ImmunizationRecommendation(Box<super::resources::ImmunizationRecommendation>),
    #[doc = "A set of rules of how a particular interoperability or standards problem is solved - typically through the use of FHIR resources. This resource is used to gather all the parts of an implementation guide into a logical whole and to publish a computable definition of all the parts.\n\nAn implementation guide is able to define default profiles that must apply to any use of a resource, so validation services may need to take one or more implementation guide resources when validating."]
    ImplementationGuide(Box<super::resources::ImplementationGuide>),
    #[doc = "An ingredient of a manufactured item or pharmaceutical product."]
    Ingredient(Box<super::resources::Ingredient>),
    #[doc = "Details of a Health Insurance product/plan provided by an organization."]
    InsurancePlan(Box<super::resources::InsurancePlan>),
    #[doc = "functional description of an inventory item used in inventory and supply-related workflows."]
    InventoryItem(Box<super::resources::InventoryItem>),
    #[doc = "A report of inventory or stock items."]
    InventoryReport(Box<super::resources::InventoryReport>),
    #[doc = "Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose."]
    Invoice(Box<super::resources::Invoice>),
    #[doc = "The Library resource is a general-purpose container for knowledge asset definitions. It can be used to describe and expose existing knowledge assets such as logic libraries and information model descriptions, as well as to describe a collection of knowledge assets."]
    Library(Box<super::resources::Library>),
    #[doc = "Identifies two or more records (resource instances) that refer to the same real-world \"occurrence\"."]
    Linkage(Box<super::resources::Linkage>),
    #[doc = "A List is a curated collection of resources, for things such as problem lists, allergy lists, facility list, organization list, etc."]
    List(Box<super::resources::List>),
    #[doc = "Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated."]
    Location(Box<super::resources::Location>),
    #[doc = "The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product."]
    ManufacturedItemDefinition(Box<super::resources::ManufacturedItemDefinition>),
    #[doc = "The Measure resource provides the definition of a quality measure."]
    Measure(Box<super::resources::Measure>),
    #[doc = "The MeasureReport resource contains the results of the calculation of a measure; and optionally a reference to the resources involved in that calculation."]
    MeasureReport(Box<super::resources::MeasureReport>),
    #[doc = "This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as for making statements about medication use."]
    Medication(Box<super::resources::Medication>),
    #[doc = "Describes the event of a patient consuming or otherwise being administered a medication.  This may be as simple as swallowing a tablet or it may be a long running infusion. Related resources tie this event to the authorizing prescription, and the specific encounter between patient and health care practitioner. This event can also be used to record waste using a status of not-done and the appropriate statusReason."]
    MedicationAdministration(Box<super::resources::MedicationAdministration>),
    #[doc = "Indicates that a medication product is to be or has been dispensed for a named person/patient.  This includes a description of the medication product (supply) provided and the instructions for administering the medication.  The medication dispense is the result of a pharmacy system responding to a medication order."]
    MedicationDispense(Box<super::resources::MedicationDispense>),
    #[doc = "Information about a medication that is used to support knowledge."]
    MedicationKnowledge(Box<super::resources::MedicationKnowledge>),
    #[doc = "An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called \"MedicationRequest\" rather than \"MedicationPrescription\" or \"MedicationOrder\" to generalize the use across inpatient and outpatient settings, including care plans, etc., and to harmonize with workflow patterns."]
    MedicationRequest(Box<super::resources::MedicationRequest>),
    #[doc = "A record of a medication that is being consumed by a patient.   A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be taking the medication in the future.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay.   The medication information may come from sources such as the patient's memory, from a prescription bottle,  or from a list of medications the patient, clinician or other party maintains. \n\nThe primary difference between a medicationstatement and a medicationadministration is that the medication administration has complete administration information and is based on actual administration information from the person who administered the medication.  A medicationstatement is often, if not always, less specific.  There is no required date/time when the medication was administered, in fact we only know that a source has reported the patient is taking this medication, where details such as time, quantity, or rate or even medication product may be incomplete or missing or less precise.  As stated earlier, the Medication Statement information may come from the patient's memory, from a prescription bottle or from a list of medications the patient, clinician or other party maintains.  Medication administration is more formal and is not missing detailed information."]
    MedicationStatement(Box<super::resources::MedicationStatement>),
    #[doc = "Detailed definition of a medicinal product, typically for uses other than direct patient care (e.g. regulatory use, drug catalogs, to support prescribing, adverse events management etc.)."]
    MedicinalProductDefinition(Box<super::resources::MedicinalProductDefinition>),
    #[doc = "Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are permitted.\n\nAllows messages to be defined once and re-used across systems."]
    MessageDefinition(Box<super::resources::MessageDefinition>),
    #[doc = "The header for a message exchange that is either requesting or responding to an action.  The reference(s) that are the subject of the action as well as other information related to the action are typically transmitted in a bundle in which the MessageHeader resource instance is the first resource in the bundle.\n\nMany implementations are not prepared to use REST and need a messaging based infrastructure."]
    MessageHeader(Box<super::resources::MessageHeader>),
    #[doc = "Representation of a molecular sequence."]
    MolecularSequence(Box<super::resources::MolecularSequence>),
    #[doc = "A curated namespace that issues unique symbols within that namespace for the identification of concepts, people, devices, etc.  Represents a \"System\" used within the Identifier and Coding data types."]
    NamingSystem(Box<super::resources::NamingSystem>),
    #[doc = "A record of food or fluid that is being consumed by a patient.  A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past.  The source of this information can be the patient, significant other (such as a family member or spouse), or a clinician.  A common scenario where this information is captured is during the history taking process during a patient visit or stay or through an app that tracks food or fluids consumed.   The consumption information may come from sources such as the patient's memory, from a nutrition label,  or from a clinician documenting observed intake."]
    NutritionIntake(Box<super::resources::NutritionIntake>),
    #[doc = "A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident."]
    NutritionOrder(Box<super::resources::NutritionOrder>),
    #[doc = "A food or supplement that is consumed by patients."]
    NutritionProduct(Box<super::resources::NutritionProduct>),
    #[doc = "Measurements and simple assertions made about a patient, device or other subject.\n\nObservations are a key aspect of healthcare.  This resource is used to capture those that do not require more sophisticated mechanisms."]
    Observation(Box<super::resources::Observation>),
    #[doc = "Set of definitional characteristics for a kind of observation or measurement produced or consumed by an orderable health care service.\n\nIn a catalog of health-related services that use or produce observations and measurements, this resource describes the expected characteristics of these observation / measurements."]
    ObservationDefinition(Box<super::resources::ObservationDefinition>),
    #[doc = "A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction)."]
    OperationDefinition(Box<super::resources::OperationDefinition>),
    #[doc = "A collection of error, warning, or information messages that result from a system action."]
    OperationOutcome(Box<super::resources::OperationOutcome>),
    #[doc = "A formally or informally recognized grouping of people or organizations formed for the purpose of achieving some form of collective action.  Includes companies, institutions, corporations, departments, community groups, healthcare practice groups, payer/insurer, etc."]
    Organization(Box<super::resources::Organization>),
    #[doc = "Defines an affiliation/assotiation/relationship between 2 distinct organizations, that is not a part-of relationship/sub-division relationship.\n\nNeed to define relationships between organizations that are not sub-divisions of the same organization (part-of relationships)."]
    OrganizationAffiliation(Box<super::resources::OrganizationAffiliation>),
    #[doc = "A medically related item or items, in a container or package."]
    PackagedProductDefinition(Box<super::resources::PackagedProductDefinition>),
    #[doc = "This resource is used to pass information into and back from an operation (whether invoked directly from REST or within a messaging environment).  It is not persisted or allowed to be referenced by other resources except as described in the definition of the Parameters resource."]
    Parameters(Box<super::resources::Parameters>),
    #[doc = "Demographics and other administrative information about an individual or animal receiving care or other health-related services.\n\nTracking patient is the center of the healthcare process."]
    Patient(Box<super::resources::Patient>),
    #[doc = "This resource provides the status of the payment for goods and services rendered, and the request and response resource references."]
    PaymentNotice(Box<super::resources::PaymentNotice>),
    #[doc = "This resource provides the details including amount of a payment and allocates the payment items being paid."]
    PaymentReconciliation(Box<super::resources::PaymentReconciliation>),
    #[doc = "Permission resource holds access rules for a given data and context."]
    Permission(Box<super::resources::Permission>),
    #[doc = "Demographics and administrative information about a person independent of a specific health-related context.\n\nNeed to track persons potentially across multiple roles."]
    Person(Box<super::resources::Person>),
    #[doc = "This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact. The resource is general enough to support the description of a broad range of clinical and non-clinical artifacts such as clinical decision support rules, order sets, protocols, and drug quality specifications."]
    PlanDefinition(Box<super::resources::PlanDefinition>),
    #[doc = "A person who is directly or indirectly involved in the provisioning of healthcare or related services.\n\nNeed to track doctors, staff, locums etc. for both healthcare practitioners, funders, etc."]
    Practitioner(Box<super::resources::Practitioner>),
    #[doc = "A specific set of Roles/Locations/specialties/services that a practitioner may perform, or has performed at an organization during a period of time.\n\nNeed to track services that a healthcare provider is able to provide at an organization's location, and the services that they can perform there."]
    PractitionerRole(Box<super::resources::PractitionerRole>),
    #[doc = "An action that is or was performed on or for a patient, practitioner, device, organization, or location. For example, this can be a physical intervention on a patient like an operation, or less invasive like long term services, counseling, or hypnotherapy.  This can be a quality or safety inspection for a location, organization, or device.  This can be an accreditation procedure on a practitioner for licensing."]
    Procedure(Box<super::resources::Procedure>),
    #[doc = "Provenance of a resource is a record that describes entities and processes involved in producing and delivering or otherwise influencing that resource. Provenance provides a critical foundation for assessing authenticity, enabling trust, and allowing reproducibility. Provenance assertions are a form of contextual metadata and can themselves become important records with their own provenance. Provenance statement indicates clinical significance in terms of confidence in authenticity, reliability, and trustworthiness, integrity, and stage in lifecycle (e.g. Document Completion - has the artifact been legally authenticated), all of which may impact security, privacy, and trust policies."]
    Provenance(Box<super::resources::Provenance>),
    #[doc = "A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coherent, consistent data collection.\n\nTo support structured, hierarchical registration of data gathered using digital forms and other questionnaires.  Questionnaires provide greater control over presentation and allow capture of data in a domain-independent way (i.e. capturing information that would otherwise require multiple distinct types of resources)."]
    Questionnaire(Box<super::resources::Questionnaire>),
    #[doc = "A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.\n\nTo support structured, hierarchical reporting of data gathered using digital forms and other questionnaires."]
    QuestionnaireResponse(Box<super::resources::QuestionnaireResponse>),
    #[doc = "Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Authorization relating to a Medicinal Product."]
    RegulatedAuthorization(Box<super::resources::RegulatedAuthorization>),
    #[doc = "Information about a person that is involved in a patient's health or the care for a patient, but who is not the target of healthcare, nor has a formal responsibility in the care process.\n\nNeed to track persons related to the patient or the healthcare process."]
    RelatedPerson(Box<super::resources::RelatedPerson>),
    #[doc = "A set of related requests that can be used to capture intended activities that have inter-dependencies such as \"give this medication after that one\"."]
    RequestOrchestration(Box<super::resources::RequestOrchestration>),
    #[doc = "The Requirements resource is used to describe an actor - a human or an application that plays a role in data exchange, and that may have obligations associated with the role the actor plays."]
    Requirements(Box<super::resources::Requirements>),
    #[doc = "A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to new ways to screen, prevent, diagnose, and treat disease. They may also study certain outcomes and certain groups of people by looking at data collected in the past or future."]
    ResearchStudy(Box<super::resources::ResearchStudy>),
    #[doc = "A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study."]
    ResearchSubject(Box<super::resources::ResearchSubject>),
    #[doc = "An assessment of the likely outcome(s) for a patient or other subject as well as the likelihood of each outcome."]
    RiskAssessment(Box<super::resources::RiskAssessment>),
    #[doc = "A container for slots of time that may be available for booking appointments."]
    Schedule(Box<super::resources::Schedule>),
    #[doc = "A search parameter that defines a named search item that can be used to search/filter on a resource."]
    SearchParameter(Box<super::resources::SearchParameter>),
    #[doc = "A record of a request for service such as diagnostic investigations, treatments, or operations to be performed."]
    ServiceRequest(Box<super::resources::ServiceRequest>),
    #[doc = "A slot of time on a schedule that may be available for booking appointments."]
    Slot(Box<super::resources::Slot>),
    #[doc = "A sample to be used for analysis."]
    Specimen(Box<super::resources::Specimen>),
    #[doc = "A kind of specimen with associated set of requirements."]
    SpecimenDefinition(Box<super::resources::SpecimenDefinition>),
    #[doc = "A definition of a FHIR structure. This resource is used to describe the underlying resources, data types defined in FHIR, and also for describing extensions and constraints on resources and data types."]
    StructureDefinition(Box<super::resources::StructureDefinition>),
    #[doc = "A Map of relationships between 2 structures that can be used to transform data."]
    StructureMap(Box<super::resources::StructureMap>),
    #[doc = "The subscription resource describes a particular client's request to be notified about a SubscriptionTopic."]
    Subscription(Box<super::resources::Subscription>),
    #[doc = "The SubscriptionStatus resource describes the state of a Subscription during notifications. It is not persisted."]
    SubscriptionStatus(Box<super::resources::SubscriptionStatus>),
    #[doc = "Describes a stream of resource state changes identified by trigger criteria and annotated with labels useful to filter projections from this topic."]
    SubscriptionTopic(Box<super::resources::SubscriptionTopic>),
    #[doc = "A homogeneous material with a definite composition."]
    Substance(Box<super::resources::Substance>),
    #[doc = "The detailed description of a substance, typically at a level beyond what is used for prescribing."]
    SubstanceDefinition(Box<super::resources::SubstanceDefinition>),
    #[doc = "Nucleic acids are defined by three distinct elements: the base, sugar and linkage. Individual substance/moiety IDs will be created for each of these elements. The nucleotide sequence will be always entered in the 5’-3’ direction."]
    SubstanceNucleicAcid(Box<super::resources::SubstanceNucleicAcid>),
    #[doc = "Properties of a substance specific to it being a polymer."]
    SubstancePolymer(Box<super::resources::SubstancePolymer>),
    #[doc = "A SubstanceProtein is defined as a single unit of a linear amino acid sequence, or a combination of subunits that are either covalently linked or have a defined invariant stoichiometric relationship. This includes all synthetic, recombinant and purified SubstanceProteins of defined sequence, whether the use is therapeutic or prophylactic. This set of elements will be used to describe albumins, coagulation factors, cytokines, growth factors, peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant vaccines, and immunomodulators."]
    SubstanceProtein(Box<super::resources::SubstanceProtein>),
    #[doc = "Todo."]
    SubstanceReferenceInformation(Box<super::resources::SubstanceReferenceInformation>),
    #[doc = "Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data elements shall be used to define polymer substances isolated from biological matrices. Taxonomic and anatomical origins shall be described using a controlled vocabulary as required. This information is captured for naturally derived polymers ( . starch) and structurally diverse substances. For Organisms belonging to the Kingdom Plantae the Substance level defines the fresh material of a single species or infraspecies, the Herbal Drug and the Herbal preparation. For Herbal preparations, the fraction information will be captured at the Substance information level and additional information for herbal extracts will be captured at the Specified Substance Group 1 information level. See for further explanation the Substance Class: Structurally Diverse and the herbal annex."]
    SubstanceSourceMaterial(Box<super::resources::SubstanceSourceMaterial>),
    #[doc = "Record of delivery of what is supplied."]
    SupplyDelivery(Box<super::resources::SupplyDelivery>),
    #[doc = "A record of a non-patient specific request for a medication, substance, device, certain types of biologically derived product, and nutrition product used in the healthcare setting."]
    SupplyRequest(Box<super::resources::SupplyRequest>),
    #[doc = "A task to be performed."]
    Task(Box<super::resources::Task>),
    #[doc = "A TerminologyCapabilities resource documents a set of capabilities (behaviors) of a FHIR Terminology Server that may be used as a statement of actual server functionality or a statement of required or desired server implementation."]
    TerminologyCapabilities(Box<super::resources::TerminologyCapabilities>),
    #[doc = "A plan for executing testing on an artifact or specifications"]
    TestPlan(Box<super::resources::TestPlan>),
    #[doc = "A summary of information based on the results of executing a TestScript."]
    TestReport(Box<super::resources::TestReport>),
    #[doc = "A structured set of tests against a FHIR server or client implementation to determine compliance against the FHIR specification."]
    TestScript(Box<super::resources::TestScript>),
    #[doc = "Record of transport."]
    Transport(Box<super::resources::Transport>),
    #[doc = "A ValueSet resource instance specifies a set of codes drawn from one or more code systems, intended for use in a particular context. Value sets link between `CodeSystem` definitions and their use in [coded elements](https://hl7.org/FHIR/terminologies.html))."]
    ValueSet(Box<super::resources::ValueSet>),
    #[doc = "Describes validation requirements, source(s), status and dates for one or more elements."]
    VerificationResult(Box<super::resources::VerificationResult>),
    #[doc = "An authorization for the provision of glasses and/or contact lenses to a patient."]
    VisionPrescription(Box<super::resources::VisionPrescription>),
    #[default]
    Invalid,
}
impl Resource {
    pub fn id(&self) -> Option<&crate::r5::types::Id> {
        match self {
            Resource::Account(r) => r.id.as_deref(),
            Resource::ActivityDefinition(r) => r.id.as_deref(),
            Resource::ActorDefinition(r) => r.id.as_deref(),
            Resource::AdministrableProductDefinition(r) => r.id.as_deref(),
            Resource::AdverseEvent(r) => r.id.as_deref(),
            Resource::AllergyIntolerance(r) => r.id.as_deref(),
            Resource::Appointment(r) => r.id.as_deref(),
            Resource::AppointmentResponse(r) => r.id.as_deref(),
            Resource::ArtifactAssessment(r) => r.id.as_deref(),
            Resource::AuditEvent(r) => r.id.as_deref(),
            Resource::Basic(r) => r.id.as_deref(),
            Resource::Binary(r) => r.id.as_deref(),
            Resource::BiologicallyDerivedProduct(r) => r.id.as_deref(),
            Resource::BiologicallyDerivedProductDispense(r) => r.id.as_deref(),
            Resource::BodyStructure(r) => r.id.as_deref(),
            Resource::Bundle(r) => r.id.as_deref(),
            Resource::CapabilityStatement(r) => r.id.as_deref(),
            Resource::CarePlan(r) => r.id.as_deref(),
            Resource::CareTeam(r) => r.id.as_deref(),
            Resource::ChargeItem(r) => r.id.as_deref(),
            Resource::ChargeItemDefinition(r) => r.id.as_deref(),
            Resource::Citation(r) => r.id.as_deref(),
            Resource::Claim(r) => r.id.as_deref(),
            Resource::ClaimResponse(r) => r.id.as_deref(),
            Resource::ClinicalImpression(r) => r.id.as_deref(),
            Resource::ClinicalUseDefinition(r) => r.id.as_deref(),
            Resource::CodeSystem(r) => r.id.as_deref(),
            Resource::Communication(r) => r.id.as_deref(),
            Resource::CommunicationRequest(r) => r.id.as_deref(),
            Resource::CompartmentDefinition(r) => r.id.as_deref(),
            Resource::Composition(r) => r.id.as_deref(),
            Resource::ConceptMap(r) => r.id.as_deref(),
            Resource::Condition(r) => r.id.as_deref(),
            Resource::ConditionDefinition(r) => r.id.as_deref(),
            Resource::Consent(r) => r.id.as_deref(),
            Resource::Contract(r) => r.id.as_deref(),
            Resource::Coverage(r) => r.id.as_deref(),
            Resource::CoverageEligibilityRequest(r) => r.id.as_deref(),
            Resource::CoverageEligibilityResponse(r) => r.id.as_deref(),
            Resource::DetectedIssue(r) => r.id.as_deref(),
            Resource::Device(r) => r.id.as_deref(),
            Resource::DeviceAssociation(r) => r.id.as_deref(),
            Resource::DeviceDefinition(r) => r.id.as_deref(),
            Resource::DeviceDispense(r) => r.id.as_deref(),
            Resource::DeviceMetric(r) => r.id.as_deref(),
            Resource::DeviceRequest(r) => r.id.as_deref(),
            Resource::DeviceUsage(r) => r.id.as_deref(),
            Resource::DiagnosticReport(r) => r.id.as_deref(),
            Resource::DocumentReference(r) => r.id.as_deref(),
            Resource::Encounter(r) => r.id.as_deref(),
            Resource::EncounterHistory(r) => r.id.as_deref(),
            Resource::Endpoint(r) => r.id.as_deref(),
            Resource::EnrollmentRequest(r) => r.id.as_deref(),
            Resource::EnrollmentResponse(r) => r.id.as_deref(),
            Resource::EpisodeOfCare(r) => r.id.as_deref(),
            Resource::EventDefinition(r) => r.id.as_deref(),
            Resource::Evidence(r) => r.id.as_deref(),
            Resource::EvidenceReport(r) => r.id.as_deref(),
            Resource::EvidenceVariable(r) => r.id.as_deref(),
            Resource::ExampleScenario(r) => r.id.as_deref(),
            Resource::ExplanationOfBenefit(r) => r.id.as_deref(),
            Resource::FamilyMemberHistory(r) => r.id.as_deref(),
            Resource::Flag(r) => r.id.as_deref(),
            Resource::FormularyItem(r) => r.id.as_deref(),
            Resource::GenomicStudy(r) => r.id.as_deref(),
            Resource::Goal(r) => r.id.as_deref(),
            Resource::GraphDefinition(r) => r.id.as_deref(),
            Resource::Group(r) => r.id.as_deref(),
            Resource::GuidanceResponse(r) => r.id.as_deref(),
            Resource::HealthcareService(r) => r.id.as_deref(),
            Resource::ImagingSelection(r) => r.id.as_deref(),
            Resource::ImagingStudy(r) => r.id.as_deref(),
            Resource::Immunization(r) => r.id.as_deref(),
            Resource::ImmunizationEvaluation(r) => r.id.as_deref(),
            Resource::ImmunizationRecommendation(r) => r.id.as_deref(),
            Resource::ImplementationGuide(r) => r.id.as_deref(),
            Resource::Ingredient(r) => r.id.as_deref(),
            Resource::InsurancePlan(r) => r.id.as_deref(),
            Resource::InventoryItem(r) => r.id.as_deref(),
            Resource::InventoryReport(r) => r.id.as_deref(),
            Resource::Invoice(r) => r.id.as_deref(),
            Resource::Library(r) => r.id.as_deref(),
            Resource::Linkage(r) => r.id.as_deref(),
            Resource::List(r) => r.id.as_deref(),
            Resource::Location(r) => r.id.as_deref(),
            Resource::ManufacturedItemDefinition(r) => r.id.as_deref(),
            Resource::Measure(r) => r.id.as_deref(),
            Resource::MeasureReport(r) => r.id.as_deref(),
            Resource::Medication(r) => r.id.as_deref(),
            Resource::MedicationAdministration(r) => r.id.as_deref(),
            Resource::MedicationDispense(r) => r.id.as_deref(),
            Resource::MedicationKnowledge(r) => r.id.as_deref(),
            Resource::MedicationRequest(r) => r.id.as_deref(),
            Resource::MedicationStatement(r) => r.id.as_deref(),
            Resource::MedicinalProductDefinition(r) => r.id.as_deref(),
            Resource::MessageDefinition(r) => r.id.as_deref(),
            Resource::MessageHeader(r) => r.id.as_deref(),
            Resource::MolecularSequence(r) => r.id.as_deref(),
            Resource::NamingSystem(r) => r.id.as_deref(),
            Resource::NutritionIntake(r) => r.id.as_deref(),
            Resource::NutritionOrder(r) => r.id.as_deref(),
            Resource::NutritionProduct(r) => r.id.as_deref(),
            Resource::Observation(r) => r.id.as_deref(),
            Resource::ObservationDefinition(r) => r.id.as_deref(),
            Resource::OperationDefinition(r) => r.id.as_deref(),
            Resource::OperationOutcome(r) => r.id.as_deref(),
            Resource::Organization(r) => r.id.as_deref(),
            Resource::OrganizationAffiliation(r) => r.id.as_deref(),
            Resource::PackagedProductDefinition(r) => r.id.as_deref(),
            Resource::Parameters(r) => r.id.as_deref(),
            Resource::Patient(r) => r.id.as_deref(),
            Resource::PaymentNotice(r) => r.id.as_deref(),
            Resource::PaymentReconciliation(r) => r.id.as_deref(),
            Resource::Permission(r) => r.id.as_deref(),
            Resource::Person(r) => r.id.as_deref(),
            Resource::PlanDefinition(r) => r.id.as_deref(),
            Resource::Practitioner(r) => r.id.as_deref(),
            Resource::PractitionerRole(r) => r.id.as_deref(),
            Resource::Procedure(r) => r.id.as_deref(),
            Resource::Provenance(r) => r.id.as_deref(),
            Resource::Questionnaire(r) => r.id.as_deref(),
            Resource::QuestionnaireResponse(r) => r.id.as_deref(),
            Resource::RegulatedAuthorization(r) => r.id.as_deref(),
            Resource::RelatedPerson(r) => r.id.as_deref(),
            Resource::RequestOrchestration(r) => r.id.as_deref(),
            Resource::Requirements(r) => r.id.as_deref(),
            Resource::ResearchStudy(r) => r.id.as_deref(),
            Resource::ResearchSubject(r) => r.id.as_deref(),
            Resource::RiskAssessment(r) => r.id.as_deref(),
            Resource::Schedule(r) => r.id.as_deref(),
            Resource::SearchParameter(r) => r.id.as_deref(),
            Resource::ServiceRequest(r) => r.id.as_deref(),
            Resource::Slot(r) => r.id.as_deref(),
            Resource::Specimen(r) => r.id.as_deref(),
            Resource::SpecimenDefinition(r) => r.id.as_deref(),
            Resource::StructureDefinition(r) => r.id.as_deref(),
            Resource::StructureMap(r) => r.id.as_deref(),
            Resource::Subscription(r) => r.id.as_deref(),
            Resource::SubscriptionStatus(r) => r.id.as_deref(),
            Resource::SubscriptionTopic(r) => r.id.as_deref(),
            Resource::Substance(r) => r.id.as_deref(),
            Resource::SubstanceDefinition(r) => r.id.as_deref(),
            Resource::SubstanceNucleicAcid(r) => r.id.as_deref(),
            Resource::SubstancePolymer(r) => r.id.as_deref(),
            Resource::SubstanceProtein(r) => r.id.as_deref(),
            Resource::SubstanceReferenceInformation(r) => r.id.as_deref(),
            Resource::SubstanceSourceMaterial(r) => r.id.as_deref(),
            Resource::SupplyDelivery(r) => r.id.as_deref(),
            Resource::SupplyRequest(r) => r.id.as_deref(),
            Resource::Task(r) => r.id.as_deref(),
            Resource::TerminologyCapabilities(r) => r.id.as_deref(),
            Resource::TestPlan(r) => r.id.as_deref(),
            Resource::TestReport(r) => r.id.as_deref(),
            Resource::TestScript(r) => r.id.as_deref(),
            Resource::Transport(r) => r.id.as_deref(),
            Resource::ValueSet(r) => r.id.as_deref(),
            Resource::VerificationResult(r) => r.id.as_deref(),
            Resource::VisionPrescription(r) => r.id.as_deref(),
            _ => None,
        }
    }
    pub fn meta(&self) -> Option<&crate::r5::types::Meta> {
        match self {
            Resource::Account(r) => r.meta.as_deref(),
            Resource::ActivityDefinition(r) => r.meta.as_deref(),
            Resource::ActorDefinition(r) => r.meta.as_deref(),
            Resource::AdministrableProductDefinition(r) => r.meta.as_deref(),
            Resource::AdverseEvent(r) => r.meta.as_deref(),
            Resource::AllergyIntolerance(r) => r.meta.as_deref(),
            Resource::Appointment(r) => r.meta.as_deref(),
            Resource::AppointmentResponse(r) => r.meta.as_deref(),
            Resource::ArtifactAssessment(r) => r.meta.as_deref(),
            Resource::AuditEvent(r) => r.meta.as_deref(),
            Resource::Basic(r) => r.meta.as_deref(),
            Resource::Binary(r) => r.meta.as_deref(),
            Resource::BiologicallyDerivedProduct(r) => r.meta.as_deref(),
            Resource::BiologicallyDerivedProductDispense(r) => r.meta.as_deref(),
            Resource::BodyStructure(r) => r.meta.as_deref(),
            Resource::Bundle(r) => r.meta.as_deref(),
            Resource::CapabilityStatement(r) => r.meta.as_deref(),
            Resource::CarePlan(r) => r.meta.as_deref(),
            Resource::CareTeam(r) => r.meta.as_deref(),
            Resource::ChargeItem(r) => r.meta.as_deref(),
            Resource::ChargeItemDefinition(r) => r.meta.as_deref(),
            Resource::Citation(r) => r.meta.as_deref(),
            Resource::Claim(r) => r.meta.as_deref(),
            Resource::ClaimResponse(r) => r.meta.as_deref(),
            Resource::ClinicalImpression(r) => r.meta.as_deref(),
            Resource::ClinicalUseDefinition(r) => r.meta.as_deref(),
            Resource::CodeSystem(r) => r.meta.as_deref(),
            Resource::Communication(r) => r.meta.as_deref(),
            Resource::CommunicationRequest(r) => r.meta.as_deref(),
            Resource::CompartmentDefinition(r) => r.meta.as_deref(),
            Resource::Composition(r) => r.meta.as_deref(),
            Resource::ConceptMap(r) => r.meta.as_deref(),
            Resource::Condition(r) => r.meta.as_deref(),
            Resource::ConditionDefinition(r) => r.meta.as_deref(),
            Resource::Consent(r) => r.meta.as_deref(),
            Resource::Contract(r) => r.meta.as_deref(),
            Resource::Coverage(r) => r.meta.as_deref(),
            Resource::CoverageEligibilityRequest(r) => r.meta.as_deref(),
            Resource::CoverageEligibilityResponse(r) => r.meta.as_deref(),
            Resource::DetectedIssue(r) => r.meta.as_deref(),
            Resource::Device(r) => r.meta.as_deref(),
            Resource::DeviceAssociation(r) => r.meta.as_deref(),
            Resource::DeviceDefinition(r) => r.meta.as_deref(),
            Resource::DeviceDispense(r) => r.meta.as_deref(),
            Resource::DeviceMetric(r) => r.meta.as_deref(),
            Resource::DeviceRequest(r) => r.meta.as_deref(),
            Resource::DeviceUsage(r) => r.meta.as_deref(),
            Resource::DiagnosticReport(r) => r.meta.as_deref(),
            Resource::DocumentReference(r) => r.meta.as_deref(),
            Resource::Encounter(r) => r.meta.as_deref(),
            Resource::EncounterHistory(r) => r.meta.as_deref(),
            Resource::Endpoint(r) => r.meta.as_deref(),
            Resource::EnrollmentRequest(r) => r.meta.as_deref(),
            Resource::EnrollmentResponse(r) => r.meta.as_deref(),
            Resource::EpisodeOfCare(r) => r.meta.as_deref(),
            Resource::EventDefinition(r) => r.meta.as_deref(),
            Resource::Evidence(r) => r.meta.as_deref(),
            Resource::EvidenceReport(r) => r.meta.as_deref(),
            Resource::EvidenceVariable(r) => r.meta.as_deref(),
            Resource::ExampleScenario(r) => r.meta.as_deref(),
            Resource::ExplanationOfBenefit(r) => r.meta.as_deref(),
            Resource::FamilyMemberHistory(r) => r.meta.as_deref(),
            Resource::Flag(r) => r.meta.as_deref(),
            Resource::FormularyItem(r) => r.meta.as_deref(),
            Resource::GenomicStudy(r) => r.meta.as_deref(),
            Resource::Goal(r) => r.meta.as_deref(),
            Resource::GraphDefinition(r) => r.meta.as_deref(),
            Resource::Group(r) => r.meta.as_deref(),
            Resource::GuidanceResponse(r) => r.meta.as_deref(),
            Resource::HealthcareService(r) => r.meta.as_deref(),
            Resource::ImagingSelection(r) => r.meta.as_deref(),
            Resource::ImagingStudy(r) => r.meta.as_deref(),
            Resource::Immunization(r) => r.meta.as_deref(),
            Resource::ImmunizationEvaluation(r) => r.meta.as_deref(),
            Resource::ImmunizationRecommendation(r) => r.meta.as_deref(),
            Resource::ImplementationGuide(r) => r.meta.as_deref(),
            Resource::Ingredient(r) => r.meta.as_deref(),
            Resource::InsurancePlan(r) => r.meta.as_deref(),
            Resource::InventoryItem(r) => r.meta.as_deref(),
            Resource::InventoryReport(r) => r.meta.as_deref(),
            Resource::Invoice(r) => r.meta.as_deref(),
            Resource::Library(r) => r.meta.as_deref(),
            Resource::Linkage(r) => r.meta.as_deref(),
            Resource::List(r) => r.meta.as_deref(),
            Resource::Location(r) => r.meta.as_deref(),
            Resource::ManufacturedItemDefinition(r) => r.meta.as_deref(),
            Resource::Measure(r) => r.meta.as_deref(),
            Resource::MeasureReport(r) => r.meta.as_deref(),
            Resource::Medication(r) => r.meta.as_deref(),
            Resource::MedicationAdministration(r) => r.meta.as_deref(),
            Resource::MedicationDispense(r) => r.meta.as_deref(),
            Resource::MedicationKnowledge(r) => r.meta.as_deref(),
            Resource::MedicationRequest(r) => r.meta.as_deref(),
            Resource::MedicationStatement(r) => r.meta.as_deref(),
            Resource::MedicinalProductDefinition(r) => r.meta.as_deref(),
            Resource::MessageDefinition(r) => r.meta.as_deref(),
            Resource::MessageHeader(r) => r.meta.as_deref(),
            Resource::MolecularSequence(r) => r.meta.as_deref(),
            Resource::NamingSystem(r) => r.meta.as_deref(),
            Resource::NutritionIntake(r) => r.meta.as_deref(),
            Resource::NutritionOrder(r) => r.meta.as_deref(),
            Resource::NutritionProduct(r) => r.meta.as_deref(),
            Resource::Observation(r) => r.meta.as_deref(),
            Resource::ObservationDefinition(r) => r.meta.as_deref(),
            Resource::OperationDefinition(r) => r.meta.as_deref(),
            Resource::OperationOutcome(r) => r.meta.as_deref(),
            Resource::Organization(r) => r.meta.as_deref(),
            Resource::OrganizationAffiliation(r) => r.meta.as_deref(),
            Resource::PackagedProductDefinition(r) => r.meta.as_deref(),
            Resource::Parameters(r) => r.meta.as_deref(),
            Resource::Patient(r) => r.meta.as_deref(),
            Resource::PaymentNotice(r) => r.meta.as_deref(),
            Resource::PaymentReconciliation(r) => r.meta.as_deref(),
            Resource::Permission(r) => r.meta.as_deref(),
            Resource::Person(r) => r.meta.as_deref(),
            Resource::PlanDefinition(r) => r.meta.as_deref(),
            Resource::Practitioner(r) => r.meta.as_deref(),
            Resource::PractitionerRole(r) => r.meta.as_deref(),
            Resource::Procedure(r) => r.meta.as_deref(),
            Resource::Provenance(r) => r.meta.as_deref(),
            Resource::Questionnaire(r) => r.meta.as_deref(),
            Resource::QuestionnaireResponse(r) => r.meta.as_deref(),
            Resource::RegulatedAuthorization(r) => r.meta.as_deref(),
            Resource::RelatedPerson(r) => r.meta.as_deref(),
            Resource::RequestOrchestration(r) => r.meta.as_deref(),
            Resource::Requirements(r) => r.meta.as_deref(),
            Resource::ResearchStudy(r) => r.meta.as_deref(),
            Resource::ResearchSubject(r) => r.meta.as_deref(),
            Resource::RiskAssessment(r) => r.meta.as_deref(),
            Resource::Schedule(r) => r.meta.as_deref(),
            Resource::SearchParameter(r) => r.meta.as_deref(),
            Resource::ServiceRequest(r) => r.meta.as_deref(),
            Resource::Slot(r) => r.meta.as_deref(),
            Resource::Specimen(r) => r.meta.as_deref(),
            Resource::SpecimenDefinition(r) => r.meta.as_deref(),
            Resource::StructureDefinition(r) => r.meta.as_deref(),
            Resource::StructureMap(r) => r.meta.as_deref(),
            Resource::Subscription(r) => r.meta.as_deref(),
            Resource::SubscriptionStatus(r) => r.meta.as_deref(),
            Resource::SubscriptionTopic(r) => r.meta.as_deref(),
            Resource::Substance(r) => r.meta.as_deref(),
            Resource::SubstanceDefinition(r) => r.meta.as_deref(),
            Resource::SubstanceNucleicAcid(r) => r.meta.as_deref(),
            Resource::SubstancePolymer(r) => r.meta.as_deref(),
            Resource::SubstanceProtein(r) => r.meta.as_deref(),
            Resource::SubstanceReferenceInformation(r) => r.meta.as_deref(),
            Resource::SubstanceSourceMaterial(r) => r.meta.as_deref(),
            Resource::SupplyDelivery(r) => r.meta.as_deref(),
            Resource::SupplyRequest(r) => r.meta.as_deref(),
            Resource::Task(r) => r.meta.as_deref(),
            Resource::TerminologyCapabilities(r) => r.meta.as_deref(),
            Resource::TestPlan(r) => r.meta.as_deref(),
            Resource::TestReport(r) => r.meta.as_deref(),
            Resource::TestScript(r) => r.meta.as_deref(),
            Resource::Transport(r) => r.meta.as_deref(),
            Resource::ValueSet(r) => r.meta.as_deref(),
            Resource::VerificationResult(r) => r.meta.as_deref(),
            Resource::VisionPrescription(r) => r.meta.as_deref(),
            _ => None,
        }
    }
}
