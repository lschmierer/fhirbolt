use super::{Component, Field, Message, RepeatedField, Segment, SubComponent};

const SEGMENT_DELIMITER: char = '\x0D'; // which is \r

/// Hold HL7 v2 delimiters.
struct Delimiters {
    field: char,
    repeat: char,
    component: char,
    sub_component: char,
    escape: char,
}

/// Parse a HL7 v2 message into an abstract datastrucutre.
///
/// # Limitations
/// The HL7 parser currently does not handle escaping!
pub fn parse(message_str: &str) -> Result<Message, String> {
    println!("\x1B[33mWarning\x1B[39m: HL7 parser currently does not handle escaping!");

    if !message_str.starts_with("MSH") {
        return Err("no MSH".into());
    }

    let mut chars_iter = message_str.chars().skip(3);
    let field_delimiter = chars_iter.next().ok_or("reading field delimiter")?;
    let component_delimiter = chars_iter.next().ok_or("reading component delimiter")?;
    let repeat_delimiter = chars_iter.next().ok_or("reading repeat delimiter")?;
    let escape_delimiter = chars_iter.next().ok_or("reading escape delimiter")?;
    let sub_component_delimiter = chars_iter
        .next()
        .ok_or("reasding sub-component delimiter")?;

    // reorder according to precedence in the data structure
    let delimiters = Delimiters {
        field: field_delimiter,
        repeat: repeat_delimiter,
        component: component_delimiter,
        sub_component: sub_component_delimiter,
        escape: escape_delimiter,
    };

    let mut segments_iter = message_str.split(SEGMENT_DELIMITER);

    let header = parse_header(
        segments_iter.next().ok_or("no message header")?,
        &delimiters,
    );
    let segments = segments_iter
        .map(|s| parse_segment(s, &delimiters))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Message { header, segments })
}

fn parse_header(segment_str: &str, delimiters: &Delimiters) -> Segment {
    let fields_iter = segment_str[8..]
        .split(delimiters.field)
        // skip the first | after the encoding characters
        .skip(1);

    let id = "MSH".into();

    let mut fields = Vec::with_capacity(fields_iter.clone().count() + 1);
    fields.push(vec![Field {
        components: vec![Component {
            sub_components: vec![SubComponent::Value(segment_str[4..8].into())],
        }],
    }]);
    fields.extend(fields_iter.map(|f| parse_repeated_field(f, delimiters)));

    Segment { id, fields }
}

fn parse_segment(segment_str: &str, delimiters: &Delimiters) -> Result<Segment, String> {
    // trim possible \n that might be added by a text editor
    let mut fields_iter = segment_str.trim().split(delimiters.field);

    let id = fields_iter.next().ok_or("no segment identifier")?.into();
    let fields = fields_iter
        .map(|f| parse_repeated_field(f, delimiters))
        .collect();

    Ok(Segment { id, fields })
}

fn parse_repeated_field(field_str: &str, delimiters: &Delimiters) -> RepeatedField {
    field_str
        .split(delimiters.repeat)
        .map(|f| Field {
            components: f
                .split(delimiters.component)
                .map(|c| parse_component(c, delimiters))
                .collect(),
        })
        .collect()
}

fn parse_component(component_str: &str, delimiters: &Delimiters) -> Component {
    let sub_components = component_str
        .split(delimiters.sub_component)
        .map(|c| parse_value(c, delimiters))
        .collect();

    Component { sub_components }
}

fn parse_value(value_str: &str, delimiters: &Delimiters) -> SubComponent {
    match value_str {
        "" => SubComponent::Empty,
        "\"\"" => SubComponent::Null,
        v => SubComponent::Value(unescape_string(v, delimiters)),
    }
}

fn unescape_string(string: &str, delimiters: &Delimiters) -> String {
    // TODO: handle escaping as described in http://www.hl7.eu/HL7v2x/v251/std251/ch02.html#Heading31

    if string.contains(delimiters.escape) {
        unimplemented!("escape characters")
    }

    string.into()
}

#[cfg(test)]
mod tests {
    use crate::hl7v2::{parse, Component, Field, Segment, SubComponent};

    #[test]
    fn test_parse_header() {
        assert_eq!(
            parse("MSH|^~\\&").unwrap().header,
            Segment {
                id: "MSH".into(),
                fields: vec![vec![Field {
                    components: vec![Component {
                        sub_components: vec![SubComponent::Value("^~\\&".into(),),],
                    },],
                },]]
            },
        );
        assert_eq!(
            parse("MSH|^~\\&|some").unwrap().header,
            Segment {
                id: "MSH".into(),
                fields: vec![
                    vec![Field {
                        components: vec![Component {
                            sub_components: vec![SubComponent::Value("^~\\&".into(),),],
                        },],
                    },],
                    vec![Field {
                        components: vec![Component {
                            sub_components: vec![SubComponent::Value("some".into(),),],
                        },],
                    },]
                ]
            },
        );
    }

    #[test]
    fn test_parse_segments() {
        assert_eq!(
            parse("MSH|^~\\&\rTST|A^B&C").unwrap().segments,
            vec![Segment {
                id: "TST".into(),
                fields: vec![vec![Field {
                    components: vec![
                        Component {
                            sub_components: vec![SubComponent::Value("A".into(),),],
                        },
                        Component {
                            sub_components: vec![
                                SubComponent::Value("B".into(),),
                                SubComponent::Value("C".into(),),
                            ],
                        },
                    ]
                },]]
            },]
        );
    }

    #[test]
    fn test_parse_segments_repeated() {
        assert_eq!(
            parse("MSH|^~\\&\rTST|A^B&C~D").unwrap().segments,
            vec![Segment {
                id: "TST".into(),
                fields: vec![vec![
                    Field {
                        components: vec![
                            Component {
                                sub_components: vec![SubComponent::Value("A".into(),),],
                            },
                            Component {
                                sub_components: vec![
                                    SubComponent::Value("B".into(),),
                                    SubComponent::Value("C".into(),),
                                ],
                            },
                        ],
                    },
                    Field {
                        components: vec![Component {
                            sub_components: vec![SubComponent::Value("D".into(),),],
                        },]
                    },
                ]]
            },]
        );
    }
}
