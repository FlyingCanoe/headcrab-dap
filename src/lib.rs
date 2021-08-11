use std::io;
use std::io::BufRead;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid input")]
    Invalid,
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    InvalidJson(#[from] serde_json::error::Error),
}

#[derive(Debug, Clone)]
/// A dap message header.
/// In the current, version of dap, a Header can only contain one field : `Content-Length`.
/// That being say, the standard was design to make it possible for a future version to add field.
/// As such, This type support header which contain unknown fields.
pub struct Header {
    /// "The length of the content part in bytes"
    pub content_length: usize,
    /// The list of the header field, both know and unknown.
    pub fields: Vec<HeaderField>,
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

    pub fn from_input<R: BufRead>(input: &mut R) -> Result<Header, Error> {
        let mut fields = Vec::new();

        // a empty line signify the end of the header
        while let Some(field) = HeaderField::from_input(input)? {
            fields.push(field);
        }

        Header::from_raw_fields(fields).ok_or(Error::Invalid)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
/// A dap message header field.
pub enum HeaderField {
    /// "The length of the content part in bytes"
    ContentLength(usize),
    /// a unknown field
    Other { name: String, value: String },
}

impl HeaderField {
    fn specialize(self) -> Result<Self, Error> {
        match self {
            HeaderField::Other { name, value } if name == "Content-Length" => {
                let length = usize::from_str_radix(value.as_str(), 10).or(Err(Error::Invalid))?;
                Ok(HeaderField::ContentLength(length))
            }
            _ => Ok(self),
        }
    }

    fn from_input<R: BufRead>(input: &mut R) -> Result<Option<HeaderField>, Error> {
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
            (_, _, Some(_)) => Err(Error::Invalid),
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
            _ => Err(Error::Invalid),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    info: MessageInfo,
    #[doc(hidden)]
    pub raw_value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MessageInfo {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: u64,
    #[serde(alias = "type")]
    message_type: String,
}

impl Message {
    pub fn try_from_input<R: BufRead>(input: &mut R) -> Result<Self, Error> {
        let header = Header::from_input(input)?;
        let mut buffer = vec![0; header.content_length];

        input.read_exact(buffer.as_mut_slice())?;
        let raw_value = serde_json::from_slice(buffer.as_slice())?;
        let info = serde_json::from_slice(buffer.as_slice())?;

        Ok(Self { raw_value, info })
    }

    #[doc(hidden)]
    pub fn seq(&self) -> u64 {
        self.info.seq
    }

    #[doc(hidden)]
    pub fn message_type(&self) -> &str {
        self.info.message_type.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bstr::B;

    #[test]
    fn parse_header_field_valid_content_length() {
        let header = HeaderField::from_input(&mut B("Content-Length:6\r\n"))
            .unwrap()
            .unwrap();
        match header {
            HeaderField::ContentLength(6) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_header_field_valid_unknown_field() {
        let field = HeaderField::from_input(&mut B("name:value\r\n"))
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
        let none = HeaderField::from_input(&mut B("\r\n")).unwrap();
        assert_eq!(none, None);
    }

    #[test]
    fn parse_header_field_name_only() {
        let err = HeaderField::from_input(&mut B("name:"));
        match err {
            Err(Error::Invalid) => (),
            _ => panic!(),
        }
    }

    #[test]
    #[should_panic]
    fn parse_header_empty_input() {
        Header::from_input(&mut B("")).unwrap();
    }

    #[test]
    fn parse_header_valid_header() {
        let header = Header::from_input(&mut B("Content-Length:415\r\n\r\n")).unwrap();

        assert_eq!(header.content_length, 415);

        assert_eq!(header.fields[0], HeaderField::ContentLength(415));
        assert_eq!(header.fields.get(1), None)
    }

    #[test]
    fn parse_header_valid_header_with_unknown_field() {
        let header =
            Header::from_input(&mut B("Content-Length:360\r\nOther-Field:value\r\n\r\n")).unwrap();

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

    #[test]
    fn message_from_input_valid() {
        use serde_json::Value;

        let body = r#"{
            "seq": 1,
            "type": "fake"
          }"#;

        let raw_message = format!("Content-Length:{}\r\n\r\n{}", body.as_bytes().len(), body);

        let message = Message::try_from_input(&mut raw_message.as_bytes()).unwrap();

        assert_eq!(message.seq(), 1);
        assert_eq!(message.message_type(), "fake");
        assert_eq!(
            message.raw_value,
            serde_json::from_str::<Value>(body).unwrap()
        );
    }
}
