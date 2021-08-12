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
    message_kind: Option<Request>,
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
        use serde_json::Value;

        let header = Header::from_input(input)?;
        let mut buffer = vec![0; header.content_length];

        input.read_exact(buffer.as_mut_slice())?;
        let raw_value: Value = serde_json::from_slice(buffer.as_slice())?;
        let info: MessageInfo = serde_json::from_slice(buffer.as_slice())?;

        let message_kind = Request::new(info.message_type.as_str(), raw_value.clone());

        Ok(Self {
            raw_value,
            info,
            message_kind,
        })
    }

    #[doc(hidden)]
    pub fn seq(&self) -> u64 {
        self.info.seq
    }

    #[doc(hidden)]
    pub fn message_type(&self) -> &str {
        self.info.message_type.as_str()
    }

    pub fn message_kind(&self) -> Option<&Request> {
        self.message_kind.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Request {
    request_info: RequestInfo,
    request_kind: Option<InitializeRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct RequestInfo {
    /**
     * The command to execute.
     */
    command: String,

    /**
     * Object containing arguments for the command.
     */
    arguments: Option<serde_json::Value>,
}

impl Request {
    fn new(message_type: &str, value: serde_json::Value) -> Option<Self> {
        let info: Result<RequestInfo, _> = serde_json::from_value(value);

        match (message_type, info) {
            ("request", Ok(request_info)) => {
                let request_kind =
                    InitializeRequest::new(request_info.clone());
                Some(Self { request_info, request_kind })
            }
            _ => None,
        }
    }

    #[doc(hidden)]
    pub fn command(&self) -> &str {
        self.request_info.command.as_str()
    }

    #[doc(hidden)]
    pub fn arguments(&self) -> Option<serde_json::Value> {
        self.request_info.arguments.clone()
    }

    pub fn request_kind(&self) -> Option<&InitializeRequest> {
        self.request_kind.as_ref()
    }
}
/// The ‘initialize’ request is sent as the first request from the client to the debug adapter
///
/// in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.
///
/// Until the debug adapter has responded to with an ‘initialize’ response, the client must not send any additional requests or events to the debug adapter.
///
/// In addition the debug adapter is not allowed to send any requests or events to the client until it has responded with an ‘initialize’ response.
///
/// The ‘initialize’ request may only be sent once.
#[derive(Debug, Clone)]
pub struct InitializeRequest {
    arguments: InitializeRequestArguments,
}

impl InitializeRequest {
    fn new(info: RequestInfo) -> Option<Self> {
        let arguments = serde_json::from_value(info.arguments?);

        match (info.command.as_str(), arguments) {
            ("initialize", Ok(arguments)) => Some(Self { arguments }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitializeRequestArguments {
    /**
     * The ID of the (frontend) client using this adapter.
     */
     #[serde(alias="clientID")]
    client_id: Option<String>,

    /**
     * The human readable name of the (frontend) client using this adapter.
     */
     #[serde(alias="clientName")]
    client_name: Option<String>,

    /**
     * The ID of the debug adapter.
     */
     #[serde(alias="adapterID")]
    adapter_id: String,

    /**
     * The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
     * or de-CH.
     */
    locale: Option<String>,

    /**
     * If true all line numbers are 1-based (default).
     */
     #[serde(alias="linesStartAt1")]
     lines_start_at1: Option<bool>,

    /**
     * If true all column numbers are 1-based (default).
     */
     #[serde(alias="columnStartAt1")]
    columns_start_at1: Option<bool>,

    /**
     * Determines in what format paths are specified. The default is 'path', which
     * is the native format.
     * Values: 'path', 'uri', etc.
     */
     #[serde(alias="pathFormat")]
    path_format: Option<PathFormat>,

    /**
     * Client supports the optional type attribute for variables.
     */
     #[serde(alias="supportsVariableType")]
    supports_variable_type: Option<bool>,

    /**
     * Client supports the paging of variables.
     */
     #[serde(alias="supportVariablePaging")]
    supports_variable_paging: Option<bool>,

    /**
     * Client supports the runInTerminal request.
     */
     #[serde(alias="supportsRunInTerminalRequest")]
    supports_run_in_terminal_request: Option<bool>,

    /**
     * Client supports memory references.
     */
     #[serde(alias="supportsMemoryReferences")]
    supports_memory_references: Option<bool>,

    /**
     * Client supports progress reporting.
     */
     #[serde(alias="supportsProgressReporting")]
    supports_progress_reporting: Option<bool>,

    /**
     * Client supports the invalidated event.
     */
     #[serde(alias="supportsInvalidatedEvent")]
    supports_invalidated_event: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PathFormat {
    #[serde(alias = "path")]
    Path,
    #[serde(alias = "url")]
    Url,
    Other(String),
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

    #[test]
    fn initialize_request_valid() {
        let arg = r#"{
            "adapterID": "headcrab-rs",
            "clientID": "vscode",
            "clientName": "Visual Studio Code",
            "columnsStartAt1": true,
            "linesStartAt1": true,
            "locale": "en-us",
            "pathFormat": "path",
            "supportsInvalidatedEvent": true,
            "supportsMemoryReferences": true,
            "supportsProgressReporting": true,
            "supportsRunInTerminalRequest": true,
            "supportsVariablePaging": true,
            "supportsVariableType": true
          }"#;

          let r: Result<InitializeRequestArguments, _> = serde_json::from_str(arg);
          dbg!(r).unwrap();
    }
}
