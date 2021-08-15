use std::convert::{TryFrom, TryInto};
use std::io;
use std::io::BufRead;

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod dap_type;

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
struct Header {
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

    fn from_input<R: BufRead>(input: &mut R) -> Result<Header, Error> {
        let mut fields = Vec::new();

        // a empty line signify the end of the header
        while let Some(field) = HeaderField::from_input(input)? {
            fields.push(field);
        }

        Header::from_raw_fields(fields).ok_or(Error::Invalid)
    }

    fn new(len: usize) -> Self {
        let fields = vec![HeaderField::ContentLength(len)];
        Self::from_raw_fields(fields).expect("bug: this header should be valid")
    }
}

impl Into<String> for Header {
    fn into(self) -> String {
        let mut output = String::new();
        for field in self.fields {
            let (name, value) = match field {
                HeaderField::ContentLength(value) => {
                    ("Content-Length".to_string(), format!("{}", value))
                }
                HeaderField::Other { name, value } => (name, value),
            };

            output.push_str(format!("{}:{}\r\n", name, value).as_str());
        }
        output.push_str("{}:{}\r\n");
        output
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

impl GenericMessage {
    pub fn seq(&self) -> usize {
        self.info.seq
    }

    pub fn message_type(&self) -> &str {
        self.info.message_type.as_str()
    }

    fn into_specialize(self) -> Result<Message, Error> {
        match self.info.message_type.as_str() {
            "request" => todo!(),
            "response" => todo!(),
            _ => Ok(Message::Generic(self)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericMessage {
    info: GenericMessageSerde,
    raw_value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GenericMessageSerde {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: usize,
    #[serde(rename = "type")]
    message_type: String,
}

pub enum Message {
    Request(Request),
    Generic(GenericMessage),
}

impl Message {
    pub fn read_from<R: BufRead>(input: &mut R) -> Result<Self, Error> {
        let generic = GenericMessage::from_input(input)?;
        generic.into_specialize()
    }
}

impl TryInto<String> for GenericMessage {
    type Error = Error;

    fn try_into(self) -> Result<String, Error> {
        Ok(serde_json::to_string(&self.raw_value)?)
    }
}

impl GenericMessage {
    pub fn from_input<R: BufRead>(input: &mut R) -> Result<Self, Error> {
        use serde_json::Value;

        let header = Header::from_input(input)?;
        let mut buffer = vec![0; header.content_length];

        input.read_exact(buffer.as_mut_slice())?;
        let raw_value: Value = serde_json::from_slice(buffer.as_slice())?;
        let info: GenericMessageSerde = serde_json::from_slice(buffer.as_slice())?;

        Ok(Self { raw_value, info })
    }
}

#[derive(Debug, Clone)]
pub struct GenericRequest {
    serde: GenericRequestSerde,
    value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct GenericRequestSerde {
    //#[serde(flatten)]
    //message: GenericMessageSerde,todo

    /**
     * The command to execute.
     */
    command: String,

    /**
     * Object containing arguments for the command.
     */
    arguments: Option<serde_json::Value>,
}

impl GenericRequest {
    fn new(value: serde_json::Value) -> Result<Self, Error> {
        let serde = serde_json::from_value(value.clone())?;
        Ok(Self { serde, value })
    }

    pub fn command(&self) -> &str {
        self.serde.command.as_str()
    }

    pub fn arguments(&self) -> Option<serde_json::Value> {
        self.serde.arguments.clone()
    }

    fn into_specialized(self) -> Result<Request, Error> {
        let kind = match self.command() {
            "initialize" => Request::Initialize(InitializeRequest::try_from(self.value)?),
            "disconnect" => Request::Disconnect(DisconnectRequest::try_from(self.value)?),
            _ => Request::Generic(self),
        };
        Ok(kind)
    }
}

#[derive(Debug, Clone)]
pub enum Request {
    Initialize(InitializeRequest),
    Disconnect(DisconnectRequest),
    Generic(GenericRequest),
}

impl TryFrom<serde_json::Value> for Request {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Error> {
        let generic = GenericRequest::new(value)?;
        Ok(generic.into_specialized()?)
    }
}

pub enum Response {
    Generic(GenericResponse),
    Initialize(InitializeResponse),
}

#[derive(Debug, Clone)]
pub struct GenericResponse {
    serde: ResponseSerde,
    value: serde_json::Value,
}

/// Response for a request
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResponseSerde {
    #[serde(flatten)]
    protocol_message: GenericMessageSerde,

    /// Sequence number of the corresponding request.
    request_seq: usize,

    /// Outcome of the request.
    /// If true, the request was successful and the 'body' attribute may contain
    /// the result of the request.
    /// If the value is false, the attribute 'message' contains the error in short
    /// form and the 'body' may contain additional information (see
    /// 'ErrorResponse.body.error').
    success: bool,

    /// The command requested.
    command: String,

    /// Contains the raw error in short form if 'success' is false.
    /// This raw error might be interpreted by the frontend and is not shown in the
    /// UI.
    /// Some predefined values exist.
    /// Values:
    /// 'cancelled': request was cancelled.
    /// etc.
    message: Option<String>,

    /// Contains request result if success is true and optional error details if
    /// success is false.
    body: Option<serde_json::Value>,
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
    serde: InitializeRequestSerde,
    value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequestSerde {
    #[serde(flatten)]
    request: GenericRequestSerde,
    arguments: InitializeRequestArguments,
}

impl TryFrom<serde_json::Value> for InitializeRequest {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Error> {
        let request = Self {
            serde: serde_json::from_value(value.clone())?,
            value,
        };
        Ok(request)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitializeRequestArguments {
    /**
     * The ID of the (frontend) client using this adapter.
     */
    #[serde(alias = "clientID")]
    client_id: Option<String>,

    /**
     * The human readable name of the (frontend) client using this adapter.
     */
    #[serde(alias = "clientName")]
    client_name: Option<String>,

    /**
     * The ID of the debug adapter.
     */
    #[serde(alias = "adapterID")]
    adapter_id: String,

    /**
     * The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
     * or de-CH.
     */
    locale: Option<String>,

    /**
     * If true all line numbers are 1-based (default).
     */
    #[serde(alias = "linesStartAt1")]
    lines_start_at1: Option<bool>,

    /**
     * If true all column numbers are 1-based (default).
     */
    #[serde(alias = "columnStartAt1")]
    columns_start_at1: Option<bool>,

    /**
     * Determines in what format paths are specified. The default is 'path', which
     * is the native format.
     * Values: 'path', 'uri', etc.
     */
    #[serde(alias = "pathFormat")]
    path_format: Option<PathFormat>,

    /**
     * Client supports the optional type attribute for variables.
     */
    #[serde(alias = "supportsVariableType")]
    supports_variable_type: Option<bool>,

    /**
     * Client supports the paging of variables.
     */
    #[serde(alias = "supportVariablePaging")]
    supports_variable_paging: Option<bool>,

    /**
     * Client supports the runInTerminal request.
     */
    #[serde(alias = "supportsRunInTerminalRequest")]
    supports_run_in_terminal_request: Option<bool>,

    /**
     * Client supports memory references.
     */
    #[serde(alias = "supportsMemoryReferences")]
    supports_memory_references: Option<bool>,

    /**
     * Client supports progress reporting.
     */
    #[serde(alias = "supportsProgressReporting")]
    supports_progress_reporting: Option<bool>,

    /**
     * Client supports the invalidated event.
     */
    #[serde(alias = "supportsInvalidatedEvent")]
    supports_invalidated_event: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InitializeResponse {
    info: InitializeResponseSerde,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InitializeResponseSerde {
    #[serde(flatten)]
    response: ResponseSerde,
    /**
     * The capabilities of this debug adapter.
     */
    body: Option<dap_type::Capabilities>,
}

impl InitializeResponse {
    pub fn new(
        response_seq: usize,
        request_seq: usize,
        capability: Option<dap_type::Capabilities>,
    ) -> Self {
        use serde_json::to_value;

        let protocol_message = GenericMessageSerde {
            seq: response_seq,
            message_type: "response".to_string(),
        };

        let response = ResponseSerde {
            protocol_message,
            request_seq,
            success: true,
            command: "initialize".to_string(),
            message: None,
            body: capability.clone().map_or(None, |cap| to_value(cap).ok()),
        };
        let body = capability;

        let info = InitializeResponseSerde { response, body };

        Self { info }
    }

    pub fn send_to<W: io::Write>(self, output: &mut W) -> Result<(), Error> {
        let body = serde_json::to_value(self.info)?.to_string();
        let header: String = Header::new(body.len()).into();

        output.write_all(header.as_bytes())?;
        output.write_all(body.as_bytes())?;
        Ok(())
    }
}

/// The ‘disconnect’ request is sent from the client to the debug adapter in order to stop debugging.
/// It asks the debug adapter to disconnect from the debuggee and to terminate the debug adapter.
/// If the debuggee has been started with the ‘launch’ request,
/// the ‘disconnect’ request terminates the debuggee.
/// If the ‘attach’ request was used to connect to the debuggee,
/// ‘disconnect’ does not terminate the debuggee.
/// This behavior can be controlled with the ‘terminateDebuggee’ argument
/// (if supported by the debug adapter).
#[derive(Debug, Clone)]
pub struct DisconnectRequest {
    serde: DisconnectRequestSerde,
    value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DisconnectRequestSerde {
    #[serde(flatten)]
    request: GenericRequestSerde,
    arguments: Option<DisconnectArguments>,
}

/// Arguments for ‘disconnect’ request.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DisconnectArguments {
    /// A value of true indicates that this 'disconnect' request is part of a
    /// restart sequence.
    restart: Option<bool>,

    /// Indicates whether the debuggee should be terminated when the debugger is
    /// disconnected.
    /// If unspecified, the debug adapter is free to do whatever it thinks is best.
    /// The attribute is only honored by a debug adapter if the capability
    /// 'supportTerminateDebuggee' is true.
    #[serde(rename = "terminateDebuggee")]
    terminate_debuggee: Option<bool>,

    /// Indicates whether the debuggee should stay suspended when the debugger is
    /// disconnected.
    /// If unspecified, the debuggee should resume execution.
    /// The attribute is only honored by a debug adapter if the capability
    /// 'supportSuspendDebuggee' is true.
    #[serde(rename = "suspendDebuggee")]
    suspend_debuggee: Option<bool>,
}

impl TryFrom<serde_json::Value> for DisconnectRequest {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Error> {
        let serde = serde_json::from_value(value.clone())?;
        Ok(Self { serde, value })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct InitializeResponseInfo {
    /**
     * The capabilities of this debug adapter.
     */
    body: Option<dap_type::Capabilities>,
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
#[cfg(not(tarpaulin_include))]
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

        let message = GenericMessage::from_input(&mut raw_message.as_bytes()).unwrap();

        assert_eq!(message.info.seq, 1);
        assert_eq!(message.info.message_type, "fake");
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
