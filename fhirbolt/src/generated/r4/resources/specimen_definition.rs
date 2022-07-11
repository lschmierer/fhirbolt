// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerAdditiveAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum SpecimenDefinitionTypeTestedContainerMinimumVolume {
    Quantity(Box<super::super::types::Quantity>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainerAdditive {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#additive: SpecimenDefinitionTypeTestedContainerAdditiveAdditive,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedContainer {
    pub r#additive: Vec<SpecimenDefinitionTypeTestedContainerAdditive>,
    pub r#material: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#minimum_volume: Option<SpecimenDefinitionTypeTestedContainerMinimumVolume>,
    pub r#preparation: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cap: Option<Box<super::super::types::CodeableConcept>>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTestedHandling {
    pub r#temperature_qualifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#max_duration: Option<Box<super::super::types::Duration>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#temperature_range: Option<Box<super::super::types::Range>>,
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinitionTypeTested {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#preference: super::super::types::Code,
    pub r#requirement: Option<super::super::types::String>,
    pub r#retention_time: Option<Box<super::super::types::Duration>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#container: Option<SpecimenDefinitionTypeTestedContainer>,
    pub r#rejection_criterion: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#handling: Vec<SpecimenDefinitionTypeTestedHandling>,
    pub r#is_derived: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct SpecimenDefinition {
    pub r#language: Option<super::super::types::Code>,
    pub r#patient_preparation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#collection: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type_collected: Option<Box<super::super::types::CodeableConcept>>,
    pub r#time_aspect: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type_tested: Vec<SpecimenDefinitionTypeTested>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
