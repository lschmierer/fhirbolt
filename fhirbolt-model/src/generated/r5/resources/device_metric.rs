// Generated on 2023-05-17 by fhirbolt-codegen v0.10.0
#[doc = "Describes the calibrations that have been performed or that are required to be performed."]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceMetricCalibration {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and managable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Describes the type of the calibration method."]
    pub r#type: Option<super::super::types::Code>,
    #[doc = "Describes the state of the calibration."]
    pub r#state: Option<super::super::types::Code>,
    #[doc = "Describes the time last calibration has been performed."]
    pub r#time: Option<super::super::types::Instant>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceMetricCalibration {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#type: Default::default(),
            r#state: Default::default(),
            r#time: Default::default(),
        }
    }
}
#[doc = "Describes a measurement, calculation or setting capability of a device.  The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely applicable. "]
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceMetric {
    #[doc = "The logical id of the resource, as used in the URL for the resource. Once assigned, this value never changes."]
    pub r#id: Option<Box<super::super::types::Id>>,
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
    #[doc = "Instance identifiers assigned to a device, by the device or gateway software, manufacturers, other organizations or owners. For example, handle ID."]
    pub r#identifier: Vec<super::super::types::Identifier>,
    #[doc = "Describes the type of the metric. For example: Heart Rate, PEEP Setting, etc."]
    pub r#type: Box<super::super::types::CodeableConcept>,
    #[doc = "Describes the unit that an observed value determined for this metric will have. For example: Percent, Seconds, etc."]
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Describes the link to the Device.  This is also known as a channel device."]
    pub r#device: Box<super::super::types::Reference>,
    #[doc = "Indicates current operational state of the device. For example: On, Off, Standby, etc."]
    pub r#operational_status: Option<super::super::types::Code>,
    #[doc = "The preferred color associated with the metric (e.g., display color). This is often used to aid clinicians to track and identify parameter types by color. In practice, consider a Patient Monitor that has ECG/HR and Pleth; the metrics are displayed in different characteristic colors, such as HR in blue, BP in green, and PR and SpO2 in magenta."]
    pub r#color: Option<super::super::types::Code>,
    #[doc = "Indicates the category of the observation generation process. A DeviceMetric can be for example a setting, measurement, or calculation."]
    pub r#category: super::super::types::Code,
    #[doc = "The frequency at which the metric is taken or recorded. Devices measure metrics at a wide range of frequencies; for example, an ECG might sample measurements in the millisecond range, while an NIBP might trigger only once an hour. Less often, the measurementFrequency may be based on a unit other than time, such as distance (e.g. for a measuring wheel). The update period may be different than the measurement frequency, if the device does not update the published observed value with the same frequency as it was measured."]
    pub r#measurement_frequency: Option<Box<super::super::types::Quantity>>,
    #[doc = "Describes the calibrations that have been performed or that are required to be performed."]
    pub r#calibration: Vec<DeviceMetricCalibration>,
}
#[allow(clippy::derivable_impls)]
impl Default for DeviceMetric {
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
            r#type: Box::new(super::super::types::CodeableConcept {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#unit: Default::default(),
            r#device: Box::new(super::super::types::Reference {
                id: Some("$invalid".to_string()),
                ..Default::default()
            }),
            r#operational_status: Default::default(),
            r#color: Default::default(),
            r#category: super::super::types::Code {
                id: Some("$invalid".to_string()),
                ..Default::default()
            },
            r#measurement_frequency: Default::default(),
            r#calibration: Default::default(),
        }
    }
}
