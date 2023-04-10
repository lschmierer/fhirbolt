// Generated on 2023-04-10 by fhirbolt-codegen v0.1.0
#[doc = "The age of the specific population."]
#[derive(Debug, Clone, PartialEq)]
pub enum PopulationAge {
    Range(Box<super::super::types::Range>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Invalid,
}
impl Default for PopulationAge {
    fn default() -> PopulationAge {
        PopulationAge::Invalid
    }
}
#[doc = "Base StructureDefinition for Population Type: A populatioof people with some set of grouping criteria."]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Population {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    #[doc = "The age of the specific population."]
    pub r#age: Option<PopulationAge>,
    #[doc = "The gender of the specific population."]
    pub r#gender: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "Race of the specific population."]
    pub r#race: Option<Box<super::super::types::CodeableConcept>>,
    #[doc = "The existing physiological conditions of the specific population to which this applies."]
    pub r#physiological_condition: Option<Box<super::super::types::CodeableConcept>>,
}
impl serde::ser::Serialize for Population {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;
        fhirbolt_shared::serde_context::ser::SERIALIZATION_CONTEXT.with(|_ctx| {
            let _ctx = _ctx.borrow();
            let mut state = serializer.serialize_map(None)?;
            if let Some(some) = self.r#id.as_ref() {
                state.serialize_entry("id", some)?;
            }
            if !self.r#extension.is_empty() {
                state.serialize_entry("extension", &self.r#extension)?;
            }
            if !self.r#modifier_extension.is_empty() {
                state.serialize_entry("modifierExtension", &self.r#modifier_extension)?;
            }
            if let Some(some) = self.r#age.as_ref() {
                match some {
                    PopulationAge::Range(ref value) => {
                        state.serialize_entry("ageRange", value)?;
                    }
                    PopulationAge::CodeableConcept(ref value) => {
                        state.serialize_entry("ageCodeableConcept", value)?;
                    }
                    PopulationAge::Invalid => {
                        return Err(serde::ser::Error::custom("age is invalid"))
                    }
                }
            }
            if let Some(some) = self.r#gender.as_ref() {
                state.serialize_entry("gender", some)?;
            }
            if let Some(some) = self.r#race.as_ref() {
                state.serialize_entry("race", some)?;
            }
            if let Some(some) = self.r#physiological_condition.as_ref() {
                state.serialize_entry("physiologicalCondition", some)?;
            }
            state.end()
        })
    }
}
