use std::io::BufRead;

use crate::Error;

#[derive(Debug, Clone)]
/// A dap message header.
/// In the current, version of dap, a Header can only contain one field : `Content-Length`.
/// That being say, the standard was design to make it possible for a future version to add field.
/// As such, This type support header which contain unknown fields.
pub struct Header {
    /// "The length of the content part in bytes"
    pub len: usize,
    /// The list of the header field, both know and unknown.
    pub fields: Vec<HeaderField>,
}

impl Header {
    /// Take a list of `HeaderField` and return Header if the list of field
    fn from_raw_fields(fields: Vec<HeaderField>) -> Option<Self> {
        // try finding the ContentLength field
        let len = fields.iter().find_map(|field| match field {
            HeaderField::Len(num) => Some(*num),
            _ => None,
        })?; // if unable to find the content field, return none

        Some(Self { len, fields })
    }

    pub fn read_from<R: BufRead>(input: &mut R) -> Result<Header, Error> {
        let mut fields = Vec::new();

        // a empty line signify the end of the header
        while let Some(field) = HeaderField::read_from(input)? {
            fields.push(field);
        }

        Header::from_raw_fields(fields).ok_or(Error::BadMessage)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// A dap message header field.
pub enum HeaderField {
    /// "The length of the content part in bytes"
    Len(usize),
    /// a unknown field
    Other { name: String, value: String },
}

impl HeaderField {
    fn specialize(self) -> Result<Self, Error> {
        match self {
            HeaderField::Other { name, value } if name == "Content-Length" => {
                let length = value.as_str().parse().or(Err(Error::BadMessage))?;
                Ok(HeaderField::Len(length))
            }
            _ => Ok(self),
        }
    }

    fn read_from<R: BufRead>(input: &mut R) -> Result<Option<HeaderField>, Error> {
        let mut line = String::new();
        input.read_line(&mut line)?;

        // a header field is compose of a name and a value separated by ':'
        let mut parts = line
            .split(':')
            .map(str::trim)
            .filter(|part| !part.is_empty());

        let name = parts.next();
        let value = parts.next();

        match (name, value, parts.next()) {
            // since ':' act as the separator between the name and the value,
            // the value should not contain a ':'
            (_, _, Some(_)) => Err(Error::BadMessage),
            // if the line is empty: return None
            (None, None, None) => Ok(None),
            (Some(name), Some(value), None) => {
                let header = HeaderField::Other {
                    name: name.to_string(),
                    value: value.to_string(),
                }
                .specialize()?;
                Ok(Some(header))
            }
            _ => Err(Error::BadMessage),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bstr::B;

    #[test]
    fn parse_header_field_valid_content_length() {
        let header = HeaderField::read_from(&mut B("Content-Length:6\r\n"))
            .unwrap()
            .unwrap();
        match header {
            HeaderField::Len(6) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_header_field_valid_unknown_field() {
        let field = HeaderField::read_from(&mut B("name:value\r\n"))
            .unwrap()
            .unwrap();
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
    fn parse_header_field_empty_line() {
        let none = HeaderField::read_from(&mut B("\r\n")).unwrap();
        assert_eq!(none, None);
    }

    #[test]
    fn parse_header_field_name_only() {
        let err = HeaderField::read_from(&mut B("name:"));
        match err {
            Err(Error::BadMessage) => (),
            _ => panic!(),
        }
    }

    #[test]
    #[should_panic]
    fn parse_header_empty_input() {
        Header::read_from(&mut B("")).unwrap();
    }

    #[test]
    fn parse_header_valid_header() {
        let header = Header::read_from(&mut B("Content-Length:415\r\n\r\n")).unwrap();

        assert_eq!(header.len, 415);

        assert_eq!(header.fields[0], HeaderField::Len(415));
        assert_eq!(header.fields.get(1), None)
    }

    #[test]
    fn parse_header_valid_header_with_unknown_field() {
        let header =
            Header::read_from(&mut B("Content-Length:360\r\nOther-Field:value\r\n\r\n")).unwrap();

        assert_eq!(header.fields.len(), 2);
        assert_eq!(header.len, 360);
        assert_eq!(header.fields.get(0), Some(&HeaderField::Len(360)));
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
        let header = Header::from_raw_fields(vec![HeaderField::Len(1)]).unwrap();

        assert_eq!(header.len, 1);
        assert_eq!(header.fields.get(0), Some(&HeaderField::Len(1)));
        assert_eq!(header.fields.get(1), None);
    }

    #[test]
    fn from_raw_fields_valid_with_unknown_field() {
        let header = Header::from_raw_fields(vec![
            HeaderField::Other {
                name: "name".to_string(),
                value: "value".to_string(),
            },
            HeaderField::Len(1),
        ])
        .unwrap();

        assert_eq!(header.len, 1);
        assert_eq!(
            header.fields.get(0),
            Some(&HeaderField::Other {
                name: "name".to_string(),
                value: "value".to_string()
            })
        );
        assert_eq!(header.fields.get(1), Some(&HeaderField::Len(1)));
        assert_eq!(header.fields.get(2), None);
    }
}
