//! This module contains a model and a parser for HL7 v2 messages.

use std::iter;

use fhirbolt::model::r5::types::{Date, DateTime, String as FhirString};

pub use parse::parse;

mod parse;

/// HL7 v2 message.
#[derive(Debug, PartialEq)]
pub struct Message {
    header: Segment,
    segments: Vec<Segment>,
}

/// Segment (a line) in a HL7 v2 message.
#[derive(Debug, PartialEq)]
pub struct Segment {
    id: String,
    fields: Vec<RepeatedField>,
}

/// Type alias for repeated field in a HL7 v2 segment.
pub type RepeatedField = Vec<Field>;

/// A single field ofr a HL7 v2 segment.
#[derive(Debug, PartialEq)]
pub struct Field {
    components: Vec<Component>,
}

/// Component of a HL7 v2 field.
#[derive(Debug, PartialEq)]
pub struct Component {
    sub_components: Vec<SubComponent>,
}

/// Subcomponent of a HL7 v2 component.
#[derive(Debug, PartialEq)]
pub enum SubComponent {
    Empty,
    Null,
    Value(String),
}

/// Convinience accessors for a HL7 v2 message.
pub trait MessageAccess {
    /// Query all segments with givent segment id.
    fn segments_by_id<'a>(&'a self, id: &'a str) -> Box<dyn Iterator<Item = &'a Segment> + 'a>;
}

impl MessageAccess for Message {
    fn segments_by_id<'a>(&'a self, id: &'a str) -> Box<dyn Iterator<Item = &'a Segment> + 'a> {
        match id {
            "MSH" => Box::new(iter::once(&self.header)),
            id => Box::new(self.segments.iter().filter(move |s| s.id == id)),
        }
    }
}

/// Convinience accessors for a HL7 v2 segment.
pub trait SegmentAccess {
    /// Get the single field at given index.
    ///
    /// This returns only the fir field in case of repeats.
    fn field(&self, index: usize) -> Option<&Field>;
    /// Get repeated fields at given index.
    fn repeated(&self, index: usize) -> &RepeatedField;
}

const EMPTY_VEC: &Vec<Field> = &vec![];

impl SegmentAccess for Segment {
    fn field(&self, index: usize) -> Option<&Field> {
        self.fields.get(index - 1).and_then(|f| f.get(0))
    }
    fn repeated(&self, index: usize) -> &RepeatedField {
        self.fields.get(index - 1).unwrap_or(EMPTY_VEC)
    }
}

impl SegmentAccess for Option<&Segment> {
    fn field(&self, index: usize) -> Option<&Field> {
        self.and_then(|s| s.field(index))
    }
    fn repeated(&self, index: usize) -> &RepeatedField {
        self.map(|s| s.repeated(index)).unwrap_or(EMPTY_VEC)
    }
}

/// Convinience accessors for a HL7 v2 field.
pub trait FieldAccess {
    /// Get all components of this field.
    fn components(&self) -> &[Component];
    /// Get component with given index.
    fn component(&self, index: usize) -> Option<&Component>;
}

impl FieldAccess for Field {
    fn components(&self) -> &[Component] {
        &self.components
    }
    fn component(&self, index: usize) -> Option<&Component> {
        self.components.get(index - 1)
    }
}

impl FieldAccess for Option<&Field> {
    fn components(&self) -> &[Component] {
        self.map(Field::components).unwrap_or_default()
    }
    fn component(&self, index: usize) -> Option<&Component> {
        self.and_then(|f| f.component(index))
    }
}

/// Convinience accessors for a HL7 v2 component.
pub trait ComponentAccess {
    /// Get only the first sub-component.
    fn first_sub(&self) -> Option<&SubComponent>;
}

impl ComponentAccess for Component {
    fn first_sub(&self) -> Option<&SubComponent> {
        self.sub_components.get(0)
    }
}

impl ComponentAccess for Option<&Component> {
    fn first_sub(&self) -> Option<&SubComponent> {
        self.and_then(|c| c.first_sub())
    }
}

/// Convinience accessors for a HL7 v2 sub-component.
pub trait SubComponentAccess {
    /// Get sub-component as string.
    fn as_str(&self) -> Option<&str>;
    /// Get sub-component as bool.
    fn as_bool(&self) -> Option<bool>;
    /// Convert sub-component to a FHIR String.
    fn to_fhir_string(&self) -> Option<FhirString>;
    /// Convert sub-component to a FHIR Date.
    fn to_fhir_date(&self) -> Option<Date>;
    /// Convert sub-component to a FHIR DateTime.
    fn to_fhir_date_time(&self) -> Option<DateTime>;
}

impl SubComponentAccess for SubComponent {
    fn as_str(&self) -> Option<&str> {
        match self {
            SubComponent::Value(v) => Some(v),
            _ => None,
        }
    }
    fn as_bool(&self) -> Option<bool> {
        match self {
            SubComponent::Value(v) => {
                if v == "Y" {
                    Some(true)
                } else if v == "N" {
                    Some(false)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn to_fhir_string(&self) -> Option<FhirString> {
        match self {
            SubComponent::Value(v) => Some(FhirString {
                value: Some(v.clone()),
                ..Default::default()
            }),
            _ => None,
        }
    }
    fn to_fhir_date(&self) -> Option<Date> {
        match self {
            SubComponent::Value(v) => {
                let chars_iter = v.chars();
                let year: String = chars_iter.clone().take(4).collect();
                let month: String = chars_iter.clone().skip(4).take(2).collect();
                let day: String = chars_iter.skip(6).take(2).collect();

                let mut date = year;
                if !month.is_empty() {
                    date += &format!("-{}", month);
                }
                if !day.is_empty() {
                    date += &format!("-{}", day);
                }

                Some(Date {
                    value: Some(date),
                    ..Default::default()
                })
            }
            _ => None,
        }
    }
    fn to_fhir_date_time(&self) -> Option<DateTime> {
        match self {
            SubComponent::Value(v) => {
                if let Some(date) = self.to_fhir_date() {
                    let chars_iter = v.chars();
                    let hours: String = chars_iter.clone().skip(8).take(2).collect();
                    let minutes: String = chars_iter.clone().skip(10).take(2).collect();
                    let seconds: String = chars_iter.clone().skip(12).take(2).collect();

                    // TODO: support subseconds and timezone

                    let mut date_time = date.value.unwrap();
                    if !hours.is_empty() {
                        date_time += &format!(
                            "T{}:{}:{}Z",
                            hours,
                            if !minutes.is_empty() { &minutes } else { "00" },
                            if !seconds.is_empty() { &seconds } else { "00" },
                        );
                    }

                    Some(DateTime {
                        value: Some(date_time),
                        ..Default::default()
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl SubComponentAccess for Option<&SubComponent> {
    fn as_str(&self) -> Option<&str> {
        self.and_then(SubComponent::as_str)
    }
    fn as_bool(&self) -> Option<bool> {
        self.and_then(SubComponent::as_bool)
    }
    fn to_fhir_string(&self) -> Option<FhirString> {
        self.and_then(SubComponent::to_fhir_string)
    }
    fn to_fhir_date(&self) -> Option<Date> {
        self.and_then(SubComponent::to_fhir_date)
    }
    fn to_fhir_date_time(&self) -> Option<DateTime> {
        self.and_then(SubComponent::to_fhir_date_time)
    }
}
