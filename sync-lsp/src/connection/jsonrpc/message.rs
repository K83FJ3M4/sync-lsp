use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Formatter, Result as FmtResult};
use serde::de::{Visitor, MapAccess, Deserializer, Error as SerdeError};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub(crate) enum MessageID {
    Integer(u64),
    String(String),
    Null
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CancelParams {
    pub(crate) id: MessageID
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EmptyParams {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) enum Version {
    #[serde(rename = "2.0")]
    Current
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Error {
	pub code: ErrorCode,
	pub message: String,
	//pub data: T
}

#[repr(i32)]
#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
pub enum ErrorCode {
	// Defined by initialize
	UnknownProtocolVersion = 1,

    // Defined by JSON-RPC
    ParseError = -32700,
    InvalidRequest = -32600,
    MethodNotFound = -32601,
    InvalidParams = -32602,
    InternalError = -32603,

	ServerNotInitialized =-32002,
    #[serde(other)]
	UnknownErrorCode = -32001,
	RequestFailed = -32803,
	ServerCancelled = -32802,
	ContentModified = -32801,
	RequestCancelled = -32800,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub(super) enum Message {
    Request {
        jsonrpc: Version,
        id: MessageID,
        method: String,
        params: Value
    },
    Notification {
        jsonrpc: Version,
        method: String,
        params: Value
    },
    Response {
        jsonrpc: Version,
        id: MessageID,
        result: Value
    },
    Error {
        jsonrpc: Version,
        id: MessageID,
        error: Error
    }
}

struct MessageVisitor;

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "camelCase")]
enum MessageField {
    Jsonrpc,
    Id,
    Method,
    Params,
    Result,
    Error
}

impl<'a> Deserialize<'a> for Message where  {
    fn deserialize<D: Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(MessageVisitor)
    }
}

impl<'a> Visitor<'a> for MessageVisitor {
    type Value = Message;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("Hello world")
    }

    fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {

        let mut jsonrpc = None::<Version>;
        let mut id = None::<MessageID>;
        let mut method = None::<String>;
        let mut params = None::<Value>;
        let mut result = None::<Value>;
        let mut error = None::<Error>;

        while let Some(key) = map.next_key()? {
            match key {
                MessageField::Jsonrpc => {
                    if jsonrpc.is_some() {
                        return Err(SerdeError::duplicate_field("jsonrpc"))
                    }
                    jsonrpc = Some(map.next_value()?);
                },
                MessageField::Id => {
                    if id.is_some() {
                        return Err(SerdeError::duplicate_field("id"))
                    }
                    id = Some(map.next_value()?);
                },
                MessageField::Method => {
                    if method.is_some() {
                        return Err(SerdeError::duplicate_field("method"))
                    }
                    method = Some(map.next_value()?)
                }
                MessageField::Params => {
                    if params.is_some() {
                        return Err(SerdeError::duplicate_field("params"))
                    }
                    params = Some(map.next_value()?)
                }
                MessageField::Result => {
                    if result.is_some() {
                        return Err(SerdeError::duplicate_field("result"))
                    }
                    result = Some(map.next_value()?)
                }
                MessageField::Error => {
                    if error.is_some() {
                        return Err(SerdeError::duplicate_field("error"))
                    }
                    error = Some(map.next_value()?)
                }
            }
        }

        let Some(jsonrpc) = jsonrpc else {
            return Err(SerdeError::missing_field("jsonrpc"))
        };

        if let Some(params) = params {
            let fields = &["jsonrpc", "id", "method", "params"];
            
            let Some(method) = method else { return Err(SerdeError::missing_field("method")) };
            if error.is_some() { return Err(SerdeError::unknown_field("error", fields)) }
            if result.is_some() { return Err(SerdeError::unknown_field("result", fields)) }

            if let Some(id) = id {
                return Ok(Message::Request { jsonrpc, id, method, params })
            } else {
                return Ok(Message::Notification { jsonrpc, method, params })
            }
        }

        if let Some(result) = result {
            let fields = &["jsonrpc", "id", "result"];

            let Some(id) = id else { return Err(SerdeError::missing_field("id")) };
            if error.is_some() { return Err(SerdeError::unknown_field("error", fields)) }
            if params.is_some() { return Err(SerdeError::unknown_field("params", fields)) }
            if method.is_some() { return Err(SerdeError::unknown_field("method", fields)) }

            return Ok(Message::Response { jsonrpc, id, result })
        }

        if let Some(error) = error {
            let fields = &["jsonrpc", "id", "error"];

            let Some(id) = id else { return Err(SerdeError::missing_field("id")) };
            if result.is_some() { return Err(SerdeError::unknown_field("result", fields)) }
            if params.is_some() { return Err(SerdeError::unknown_field("params", fields)) }
            if method.is_some() { return Err(SerdeError::unknown_field("method", fields)) }

            return Ok(Message::Error { jsonrpc, id, error })
        }

        Err(SerdeError::unknown_variant("unknown", &["notification" ,"request", "result", "error"]))
    }
}