use std::io::BufRead;

#[derive(Debug, Clone)]
/// A dap message header.
/// In the current, version of dap, a Header can only contain one field : `Content-Length`.
/// That being say, the standard was design to make it possible for a future version to add field.
/// As such, This type support header which contain unknown fields.
pub struct Header {
    /// "The length of the content part in bytes"
    content_length: usize,
    /// The list of the header field, both know and unknown.
    fields: Vec<HeaderField>,
}

impl Header {
    /// Take a list of `HeaderField` and return Header if the list of field
    fn from_raw_fields(fields: Vec<HeaderField>) -> Option<Self> {
        // try finding the ContentLength field
        let content_length = fields.iter().find_map(|field| match field {
            HeaderField::ContentLength(num) => Some(*num),
            _ => None,
        })?; // if unable to fin the content field, return none

        Some(Self {
            content_length,
            fields,
        })
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
/// A dap message header field.
enum HeaderField {
    /// "The length of the content part in bytes"
    ContentLength(usize),
    /// a unknown field
    Other { name: String, value: String },
}

pub fn parse_header<R: BufRead>(input: &mut R) -> Header {
    let mut line = String::new();
    let mut fields = Vec::new();

    input.read_line(&mut line).expect("Error: io");

    // a empty line signify the end of the header
    while line != "\r\n" {
        if let Some(field) = parse_header_field(line.as_str()) {
            fields.push(field);
        }

        line.clear();
        input.read_line(&mut line).expect("Error: io");
    }
    Header::from_raw_fields(fields).expect("Error: invalid input")
}

fn parse_header_field(line: &str) -> Option<HeaderField> {
    // header field must be terminated with "\r\n"
    assert!(line.ends_with("\r\n"), "Error: invalid input");

    // a header field is compose of a name and a value separated by ':'
    let mut part = line.split(':');

    let name = part.next().expect("Error: invalid input").to_string();
    let value = part
        .next()?
        .trim() //todo doc
        .to_string();

    // since ':' act as the separator between the name and the value,
    // the value should not contain a ':'
    assert!(part.next().is_none());

    match name.as_str() {
        "Content-Length" => {
            let length = usize::from_str_radix(value.as_str(), 10).expect("Error: invalid input");
            Some(HeaderField::ContentLength(length))
        }
        _ => Some(HeaderField::Other { name, value }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_header_field_with_newline_only_termination() {
        parse_header_field("name:value\n");
    }

    #[test]
    fn parse_header_field_valid_content_length() {
        let header = parse_header_field("Content-Length:6\r\n").unwrap();
        match header {
            HeaderField::ContentLength(6) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_header_field_valid_unknown_field() {
        let field = parse_header_field("name:value\r\n").unwrap();
        match field {
            HeaderField::Other { name, value } => {
                assert_eq!(name, "name");
                assert_eq!(value, "value");
            }
            _ => {
                panic!()
            }
        }
    }

    #[test]
    #[should_panic]
    fn parse_header_empty_input() {
        let input: [u8; 0] = [];
        parse_header(&mut input.as_ref());
    }

    #[test]
    #[should_panic]
    fn parse_header_improperly_terminated() {
        let input = "Content-Length:1\r\n";
        parse_header(&mut input.as_bytes());
    }

    #[test]
    fn parse_header_valid_header() {
        let input = "Content-Length:1\r\n\r\n";
        let header = parse_header(&mut input.as_bytes());

        assert_eq!(header.content_length, 1);

        assert_eq!(header.fields[0], HeaderField::ContentLength(1));
        assert_eq!(header.fields.get(1), None)
    }

    #[test]
    fn parse_header_valid_header_with_unknown_field() {
        let input = "Content-Length:360\r\nOther-Field:value\r\n\r\n";
        let header = parse_header(&mut input.as_bytes());

        assert_eq!(header.fields.len(), 2);
        assert_eq!(header.content_length, 360);
        assert_eq!(header.fields.get(0), Some(&HeaderField::ContentLength(360)));
        assert_eq!(
            header.fields.get(1),
            Some(&HeaderField::Other {
                name: "Other-Field".to_string(),
                value: "value".to_string()
            })
        );
        assert_eq!(header.fields.get(2), None);
    }

    #[test]
    fn from_raw_fields_valid() {
        let header = Header::from_raw_fields(vec![HeaderField::ContentLength(1)]).unwrap();

        assert_eq!(header.content_length, 1);
        assert_eq!(header.fields.get(0), Some(&HeaderField::ContentLength(1)));
        assert_eq!(header.fields.get(1), None);
    }

    #[test]
    fn from_raw_fields_valid_with_unknown_field() {
        let header = Header::from_raw_fields(vec![
            HeaderField::Other {
                name: "name".to_string(),
                value: "value".to_string(),
            },
            HeaderField::ContentLength(1),
        ])
        .unwrap();

        assert_eq!(header.content_length, 1);
        assert_eq!(
            header.fields.get(0),
            Some(&HeaderField::Other {
                name: "name".to_string(),
                value: "value".to_string()
            })
        );
        assert_eq!(header.fields.get(1), Some(&HeaderField::ContentLength(1)));
        assert_eq!(header.fields.get(2), None);
    }
}
