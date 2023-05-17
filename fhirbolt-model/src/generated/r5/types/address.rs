// Generated on 2023-05-15 by fhirbolt-codegen v0.8.0
#[doc = "Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats).  This data type may be used to convey addresses for use in delivering mail as well as for visiting locations which might not be valid for mail delivery.  There are a variety of postal address formats defined around the world.\nThe ISO21090-codedString may be used to provide a coded representation of the contents of strings in an Address.\n\nNeed to be able to record postal addresses, along with notes about their use."]
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "The purpose of this address."]
    pub r#use: Option<super::super::types::Code>,
    #[doc = "Distinguishes between physical addresses (those you can visit) and mailing addresses (e.g. PO Boxes and care-of addresses). Most addresses are both."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Specifies the entire address as it should be displayed e.g. on a postal label. This may be provided instead of or as well as the specific parts."]
    pub r#text: Option<super::super::types::String>,
    #[doc = "This component contains the house number, apartment number, street name, street direction,  P.O. Box number, delivery hints, and similar address information."]
    pub r#line: Vec<super::super::types::String>,
    #[doc = "The name of the city, town, suburb, village or other community or delivery center."]
    pub r#city: Option<super::super::types::String>,
    #[doc = "The name of the administrative area (county)."]
    pub r#district: Option<super::super::types::String>,
    #[doc = "Sub-unit of a country with limited sovereignty in a federally organized country. A code may be used if codes are in common use (e.g. US 2 letter state codes)."]
    pub r#state: Option<super::super::types::String>,
    #[doc = "A postal code designating a region defined by the postal service."]
    pub r#postal_code: Option<super::super::types::String>,
    #[doc = "Country - a nation as commonly understood or generally accepted."]
    pub r#country: Option<super::super::types::String>,
    #[doc = "Time period when address was/is in use."]
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[allow(clippy::derivable_impls)]
impl Default for Address {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#use: Default::default(),
            r#type: Default::default(),
            r#text: Default::default(),
            r#line: Default::default(),
            r#city: Default::default(),
            r#district: Default::default(),
            r#state: Default::default(),
            r#postal_code: Default::default(),
            r#country: Default::default(),
            r#period: Default::default(),
        }
    }
}
