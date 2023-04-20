// Generated on 2023-04-20 by fhirbolt-codegen v0.4.0
#[doc = "Times the {item} is available."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityAvailableTime {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "mon | tue | wed | thu | fri | sat | sun."]
    pub r#days_of_week: Vec<super::super::types::Code>,
    #[doc = "Always available? i.e. 24 hour service."]
    pub r#all_day: Option<super::super::types::Boolean>,
    #[doc = "Opening time of day (ignored if allDay = true)."]
    pub r#available_start_time: Option<super::super::types::Time>,
    #[doc = "Closing time of day (ignored if allDay = true)."]
    pub r#available_end_time: Option<super::super::types::Time>,
}
#[doc = "Not available during this time due to provided reason."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AvailabilityNotAvailableTime {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Reason presented to the user explaining why time not available."]
    pub r#description: Option<super::super::types::String>,
    #[doc = "Service not available during this period."]
    pub r#during: Option<Box<super::super::types::Period>>,
}
#[doc = "Availability Type: Availability data for an {item}."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Availability {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "Times the {item} is available."]
    pub r#available_time: Vec<AvailabilityAvailableTime>,
    #[doc = "Not available during this time due to provided reason."]
    pub r#not_available_time: Vec<AvailabilityNotAvailableTime>,
}
