// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "The absolute geographic location of the Location, expressed using the WGS84 datum (This is the same co-ordinate system used in KML)."]
#[derive(Debug, Clone, PartialEq)]
pub struct LocationPosition {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Longitude. The value domain and the interpretation are the same as for the text of the longitude element in KML (see notes on Location main page)."]
    pub r#longitude: super::super::types::Decimal,
    #[doc = "Latitude. The value domain and the interpretation are the same as for the text of the latitude element in KML (see notes on Location main page)."]
    pub r#latitude: super::super::types::Decimal,
    #[doc = "Altitude. The value domain and the interpretation are the same as for the text of the altitude element in KML (see notes on Location main page)."]
    pub r#altitude: Option<super::super::types::Decimal>,
}
#[allow(clippy::derivable_impls)]
impl Default for LocationPosition {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#longitude: super::super::types::Decimal {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#latitude: super::super::types::Decimal {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#altitude: Default::default(),
        }
    }
}
#[doc = "Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated."]
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<super::super::types::Id>,
    #[doc = "The metadata about the resource. This is content that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the resource."]
    pub r#meta: Option<Box<super::super::types::Meta>>,
    #[doc = "A reference to a set of rules that were followed when the resource was constructed, and which must be understood when processing the content. Often, this is a reference to an implementation guide that defines the special rules along with other profiles etc."]
    pub r#implicit_rules: Option<super::super::types::Uri>,
    #[doc = "The base language in which the resource is written."]
    pub r#language: Option<super::super::types::Code>,
    #[doc = "A human-readable narrative that contains a summary of the resource and can be used to represent the content of the resource to a human. The narrative need not encode all the structured data, but is required to contain sufficient detail to make it \"clinically safe\" for a human to just read the narrative. Resource definitions may define what content should be represented in the narrative to ensure clinical safety."]
    pub r#text: Option<Box<super::super::types::Narrative>>,
    #[doc = "These resources do not have an independent existence apart from the resource that contains them - they cannot be identified independently, nor can they have their own independent transaction scope. This is allowed to be a Parameters resource if and only if it is referenced by a resource that provides context/meaning."]
    pub r#contained: Vec<super::super::Resource>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the resource and that modifies the understanding of the element that contains it and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer is allowed to define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Unique code or number identifying the location to its users."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "The status property covers the general availability of the resource, not the current value which may be covered by the operationStatus, or by a schedule/slots if they are configured for the location."]
    pub r#status: Option<super::super::types::Code>,
    #[doc = "The operational status covers operation values most relevant to beds (but can also apply to rooms/units/chairs/etc. such as an isolation unit/dialysis chair). This typically covers concepts such as contamination, housekeeping, and other activities like maintenance."]
    pub r#operational_status: Option<Box<super::super::types::Coding>>,
    #[doc = "Name of the location as used by humans. Does not need to be unique."]
    pub r#name: Option<super::super::types::String>,
    #[doc = "A list of alternate names that the location is known as, or was known as, in the past."]
    pub r#alias: Vec<super::super::types::String>,
    #[doc = "Description of the Location, which helps in finding or referencing the place."]
    pub r#description: Option<super::super::types::Markdown>,
    #[doc = "Indicates whether a resource instance represents a specific location or a class of locations."]
    pub r#mode: Option<super::super::types::Code>,
    #[doc = "Indicates the type of function performed at the location."]
    pub r#type: Vec<super::super::types::CodeableConcept>,
    #[doc = "The contact details of communication devices available at the location. This can include addresses, phone numbers, fax numbers, mobile numbers, email addresses and web sites."]
    pub r#contact: Vec<super::super::types::ExtendedContactDetail>,
    #[doc = "Physical location."]
    pub r#address: Option<Box<super::super::types::Address>>,
    #[doc = "Physical form of the location, e.g. building, room, vehicle, road, virtual."]
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The absolute geographic location of the Location, expressed using the WGS84 datum (This is the same co-ordinate system used in KML)."]
    pub r#position: Option<LocationPosition>,
    #[doc = "The organization responsible for the provisioning and upkeep of the location."]
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    #[doc = "Another Location of which this Location is physically a part of."]
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    #[doc = "Collection of characteristics (attributes)."]
    pub r#characteristic: Vec<super::super::types::CodeableConcept>,
    #[doc = "What days/times during a week is this location usually open, and any exceptions where the location is not available."]
    pub r#hours_of_operation: Vec<super::super::types::Availability>,
    #[doc = "Connection details of a virtual service (e.g. shared conference call facility with dedicated number/details)."]
    pub r#virtual_service: Vec<super::super::types::VirtualServiceDetail>,
    #[doc = "Technical endpoints providing access to services operated for the location."]
    pub r#endpoint: Vec<super::super::types::Reference>,
}
#[allow(clippy::derivable_impls)]
impl Default for Location {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#meta: Default::default(),
            r#implicit_rules: Default::default(),
            r#language: Default::default(),
            r#text: Default::default(),
            r#contained: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#identifier: Default::default(),
            r#status: Default::default(),
            r#operational_status: Default::default(),
            r#name: Default::default(),
            r#alias: Default::default(),
            r#description: Default::default(),
            r#mode: Default::default(),
            r#type: Default::default(),
            r#contact: Default::default(),
            r#address: Default::default(),
            r#form: Default::default(),
            r#position: Default::default(),
            r#managing_organization: Default::default(),
            r#part_of: Default::default(),
            r#characteristic: Default::default(),
            r#hours_of_operation: Default::default(),
            r#virtual_service: Default::default(),
            r#endpoint: Default::default(),
        }
    }
}
