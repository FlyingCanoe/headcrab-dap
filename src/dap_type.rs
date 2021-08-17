use serde::{Deserialize, Serialize};

/// A dap protocol message
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "request")]
    Request(Request),
    #[serde(rename = "event")]
    Event(Event),
    #[serde(rename = "response")]
    Response(Response),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A client or debug adapter initiated request.
pub struct Request {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: usize,
    /// The command to execute.
    command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// A debug adapter initiated event.
pub struct Event {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: usize,
    /// Type of event.
    event: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
/// Response for a request.
pub struct Response {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: usize,
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
}
