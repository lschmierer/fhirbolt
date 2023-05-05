// Generated on 2023-05-05 by fhirbolt-codegen v0.8.0
#[doc = "Base StructureDefinition for ProdCharacteristic Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available."]
#[derive(Debug, Clone, PartialEq)]
pub struct ProdCharacteristic {
    #[doc = "Unique id for the element within a resource (for internal references). This may be any string value that does not contain spaces."]
    pub r#id: Option<std::string::String>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element. To make the use of extensions safe and manageable, there is a strict set of governance  applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension."]
    pub r#extension: Vec<super::super::types::Extension>,
    #[doc = "May be used to represent additional information that is not part of the basic definition of the element and that modifies the understanding of the element in which it is contained and/or the understanding of the containing element's descendants. Usually modifier elements provide negation or qualification. To make the use of extensions safe and manageable, there is a strict set of governance applied to the definition and use of extensions. Though any implementer can define an extension, there is a set of requirements that SHALL be met as part of the definition of the extension. Applications processing a resource are required to check for modifier extensions.\n\nModifier extensions SHALL NOT change the meaning of any elements on Resource or DomainResource (including cannot change the meaning of modifierExtension itself)."]
    pub r#modifier_extension: Vec<super::super::types::Extension>,
    #[doc = "Where applicable, the height can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#height: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the width can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#width: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the depth can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#depth: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the weight can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#weight: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the nominal volume can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#nominal_volume: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the external diameter can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used."]
    pub r#external_diameter: Option<Box<super::super::types::Quantity>>,
    #[doc = "Where applicable, the shape can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#shape: Option<super::super::types::String>,
    #[doc = "Where applicable, the color can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#color: Vec<super::super::types::String>,
    #[doc = "Where applicable, the imprint can be specified as text."]
    pub r#imprint: Vec<super::super::types::String>,
    #[doc = "Where applicable, the image can be provided The format of the image attachment shall be specified by regional implementations."]
    pub r#image: Vec<super::super::types::Attachment>,
    #[doc = "Where applicable, the scoring can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used."]
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
}
#[allow(clippy::derivable_impls)]
impl Default for ProdCharacteristic {
    fn default() -> Self {
        Self {
            r#id: Default::default(),
            r#extension: Default::default(),
            r#modifier_extension: Default::default(),
            r#height: Default::default(),
            r#width: Default::default(),
            r#depth: Default::default(),
            r#weight: Default::default(),
            r#nominal_volume: Default::default(),
            r#external_diameter: Default::default(),
            r#shape: Default::default(),
            r#color: Default::default(),
            r#imprint: Default::default(),
            r#image: Default::default(),
            r#scoring: Default::default(),
        }
    }
}
