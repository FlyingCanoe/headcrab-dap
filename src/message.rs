use std::{convert::TryFrom, io::BufRead};

use serde::{Deserialize, Serialize};

use crate::Error;
use crate::Header;

/// A dap message
pub enum Message {
    Generic(GenericMessage),
}

impl Message {
    /// Read a `Message` from the wire.
    pub fn read_from<R: BufRead>(input: &mut R) -> Result<Self, Error> {
        let header = Header::read_from(input)?;

        let mut buffer = vec![0; header.len];
        input.read_exact(buffer.as_mut_slice())?;

        let generic = GenericMessage::parse(buffer.as_slice())?;
        Ok(Self::from(generic))
    }

    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    pub fn seq(&self) -> usize {
        match self {
            Message::Generic(msg) => msg.seq(),
        }
    }
}

impl From<GenericMessage> for Message {
    // this function is a bit silly right now,
    // but it will hold the specialization logic
    fn from(msg: GenericMessage) -> Self {
        Message::Generic(msg)
    }
}

#[derive(Debug, Clone)]
pub struct GenericMessage {
    serde: MessageSerde,
    value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct MessageSerde {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    seq: usize,
    #[serde(rename = "type")]
    message_type: String,
}

impl GenericMessage {
    /// Sequence number (also known as message ID). For protocol messages of type
    /// 'request' this ID can be used to cancel the request.
    pub fn seq(&self) -> usize {
        self.serde.seq
    }

    pub fn message_type(&self) -> &str {
        self.serde.message_type.as_str()
    }

    fn parse(input: &[u8]) -> Result<Self, Error> {
        let value = serde_json::from_slice(input)?;
        let serde = serde_json::from_slice(input)?;

        Ok(Self { value, serde })
    }
}

impl TryFrom<serde_json::Value> for GenericMessage {
    type Error = Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Error> {
        let message = serde_json::from_value(value.clone())?;
        Ok(Self {
            value,
            serde: message,
        })
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test {
    use super::*;

    fn generate_raw_msg_body(seq: usize, message_type: &str) -> String {
        format!(
            "{{
            \"seq\": {},
            \"type\": \"{}\"
        }}",
            seq, message_type
        )
    }

    #[test]
    fn parse_generic_message_valid() {
        let body = generate_raw_msg_body(1, "fake");

        let header = Header::new(body.len()).into_string();

        let mut raw_msg = header;
        raw_msg.push_str(body.as_str());

        let msg = Message::read_from(&mut raw_msg.as_bytes()).unwrap();

        match msg {
            Message::Generic(msg) => {
                assert_eq!(msg.seq(), 1);
                assert_eq!(msg.message_type(), "fake");
            }
        }
    }

    #[test]
    fn generic_message_from_value_valid() {
        let body = generate_raw_msg_body(10, "fake");
        let value: serde_json::Value = serde_json::from_str(body.as_str()).unwrap();

        let msg = GenericMessage::try_from(value).unwrap();
        assert_eq!(msg.seq(), 10);
        assert_eq!(msg.message_type(), "fake")
    }
}
