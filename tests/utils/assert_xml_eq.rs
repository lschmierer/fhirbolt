use std::str;

use quick_xml::{
    events::{
        attributes::{AttrError, Attribute},
        BytesEnd, BytesStart, Event,
    },
    Reader,
};

pub fn assert_xml_eq<R: AsRef<[u8]>>(left: R, right: R, skip_status: bool) {
    let mut left_reader = Reader::from_reader(left.as_ref());
    let mut right_reader = Reader::from_reader(right.as_ref());

    left_reader.trim_text(true);
    right_reader.trim_text(true);

    loop {
        let left_event = next_event(&mut left_reader, skip_status);
        let right_event = next_event(&mut right_reader, skip_status);

        match (&left_event, &right_event) {
            (Event::Start(left_start), Event::Start(right_start))
            | (Event::Empty(left_start), Event::Empty(right_start)) => {
                assert_eq_start(left_start, right_start)
            }

            // examples contains empty tags e.g. <code value="[diop]"></code>
            (Event::Empty(left_start), Event::Start(right_start)) => {
                assert_eq_start(left_start, right_start);
                assert_eq!(
                    next_event(&mut right_reader, skip_status),
                    Event::End(BytesEnd::new(
                        str::from_utf8(right_start.name().as_ref()).unwrap()
                    ))
                )
            }
            _ => assert_eq!(left_event, right_event),
        }

        if left_event == Event::Eof {
            return;
        }
    }
}

fn next_event<'a>(reader: &mut Reader<&'a [u8]>, skip_status: bool) -> Event<'a> {
    loop {
        let event = reader.read_event().unwrap();
        match &event {
            Event::Comment(_) => continue,
            Event::Empty(start) => {
                if skip_status && start.name().as_ref() == b"status" {
                    continue;
                } else {
                    return event;
                }
            }
            _ => return event,
        }
    }
}

fn assert_eq_start(left: &BytesStart, right: &BytesStart) {
    assert_eq!(left.name(), right.name());

    fn map_attribute(result: Result<Attribute, AttrError>) -> (String, String) {
        let attr = result.unwrap();
        (
            str::from_utf8(attr.key.as_ref()).unwrap().into(),
            attr.unescape_value().unwrap().into(),
        )
    }

    assert!(
        left.attributes()
            .map(map_attribute)
            .eq(right.attributes().map(map_attribute)),
        "attributes not equal\nleft: {:?}\nright: {:?}",
        left,
        right,
    )
}
