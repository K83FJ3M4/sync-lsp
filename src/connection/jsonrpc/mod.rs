use crate::Transport;
use serde_json::{Value, Error as JsonError, from_value, to_value};
use serde::{Serialize, de::DeserializeOwned};
pub(super) use message::Error as RpcError;
pub(crate) use message::EmptyParams;
pub use message::ErrorCode;

mod message;

pub(crate) trait RpcConnection: Sized + 'static {
    fn transport(&mut self) -> &mut Transport;
    fn resolve(&self, method: &str) -> Option<Callback<Self>>;
    fn take_error(&mut self) -> Option<RpcError>;

    fn notify(&mut self, method: &str, params: impl Serialize)
        { RpcConnectionImpl::notify(self, method, params) }
    fn request(&mut self, method: &str, tag: &str, params: impl Serialize)
        { RpcConnectionImpl::request(self, method, tag, params) }
}

pub(crate) enum Callback<T: RpcConnection> {
    Request(Box<dyn Fn(&mut T, Value) -> Result<Value, JsonError>>),
    Notification(Box<dyn Fn(&mut T, Value) -> Result<(), JsonError>>),
    Response(Box<dyn Fn(&mut T, String, Option<Value>) -> Result<(), JsonError>>),
}

impl<T: RpcConnection> Callback<T> {
    pub(crate) fn request<P: DeserializeOwned, R: 'static + Serialize>(callback: impl 'static + Fn(&mut T, P) -> R) -> Self {
        Self::Request(Box::new(move |server, value| {
            let params = from_value(value)?;
            let result = callback(server, params);
            to_value(result)
        }))
    }

    pub(crate) fn notification<P: DeserializeOwned>(callback: impl 'static + Fn(&mut T, P)) -> Self {
        Self::Notification(Box::new(move |server, value| {
            let params = from_value(value)?;
            Ok(callback(server, params))
        }))
    }

    pub(crate) fn response<P: DeserializeOwned + Default>(callback: impl 'static + Fn(&mut T, String, P)) -> Self {
        Self::Response(Box::new(move |server, id, value| {
            match value.map(|value| from_value(value)) {
                Some(Ok(params)) => Ok(callback(server, id, params)),
                Some(Err(error)) => Err(error),
                None => Ok(callback(server, id, P::default())),
            }
        }))
    }
}

#[allow(non_snake_case)]
pub(super) mod RpcConnectionImpl {
    use log::error;
    use serde_json::{Value, from_slice, to_string, to_value};
    use std::io::Error;
    use serde::Serialize;
    use crate::connection::jsonrpc::message::{Message, MessageID, Version, Error as RpcError};

    use super::message::ErrorCode;
    use super::{RpcConnection, Callback};

    pub(crate) fn serve(mut connection: impl RpcConnection) -> Result<(), Error> {
        while let Some(message) = recv(&mut connection) {
            handle(&mut connection, message)
        }

        if let Some(error) = connection.transport().error().take() {
            return Err(error);
        } else {
            Ok(())
        }
    }

    pub(super) fn notify(connection: &mut impl RpcConnection, method: &str, params: impl Serialize) {
        send(connection, Message::Notification {
            jsonrpc: Version::Current,
            method: method.to_owned(),
            params: match to_value(params) {
                Ok(params) => params,
                Err(error) => {
                    return error!("Failed to serialize params for {method} notification: {}", error);
                }
            }
        });
    }

    pub(super) fn request(connection: &mut impl RpcConnection, method: &str, tag: &str, params: impl Serialize) {
        let id = MessageID::String(format!("{method}#{tag}"));

        let message = 'message: {
            if !send(connection, Message::Request {
                jsonrpc: Version::Current,
                method: method.to_owned(),
                id: id.clone(),
                params: match to_value(params) {
                    Ok(params) => params,
                    Err(error) => {
                        break 'message format!("Failed to serialize params: {error}")
                    }
                }
            }) {
                break 'message "A io error occured during the request".to_string()
            } else {
                return
            }
        };

        handle(connection, Message::Error {
            jsonrpc: Version::Current,
            id,
            error: RpcError {
                code: ErrorCode::RequestFailed,
                message,
            }
        });
    }

    fn recv(connection: &mut impl RpcConnection) -> Option<Message> {
        loop {
            let buffer = connection.transport().recv()?;
            match from_slice(buffer.as_slice()) {
                Ok(message) => return Some(message),
                Err(err) => {
                    error!("Failed to parse message: {}", err);
                }
            }
        }
    }

    fn send(connection: &mut impl RpcConnection, message: Message) -> bool {
        match to_string(&message) {
            Ok(message) => {
                connection.transport().send(message);
                true
            },
            Err(err) => {
                error!("Failed to serialize message: {}", err);
                false
            }
        }
    }

    fn handle(connection: &mut impl RpcConnection, message: Message) {
        match message {
            Message::Request { id, params, method, .. } => handle_request(connection, method, id, params),
            Message::Notification { params, method, .. } => handle_notification(connection, method, params),
            Message::Response { id, result, .. } => handle_result(connection, id, result),
            Message::Error { id, error, .. } => handle_error(connection, id, error),
        }
    }

    fn handle_result(connection: &mut impl RpcConnection, id: MessageID, result: Value) {
        let MessageID::String(id) = id else {
            return error!("Response without request: {id:?}")
        };

        let Some((method, tag)) = id.split_once('#') else {
            return error!("Failed to parse id: {}", id);
        };

        let Some(handler) = connection.resolve(method) else {
            return error!("Response without request: {method}")
        };

        let handler = match handler {
            Callback::Response(handler) => handler,
            Callback::Request(..) | Callback::Notification(..) => return error!("{method} is not a response endpoint"),
        };

        let result = handler(connection, tag.to_string(), Some(result));

        if let Some(error) = connection.take_error() {
            return error!("Failed to process {method}#{tag}: {}", error.message);
        }

        if let Err(error) = result {
            return error!("Failed to parse result for {method}#{tag}: {}", error);
        }
    }

    fn handle_error(connection: &mut impl RpcConnection, id: MessageID, error: RpcError) {
        let MessageID::String(id) = id else {
            return error!("Response without request: {id:?}")
        };

        let Some((method, tag)) = id.split_once('#') else {
            return error!("Failed to parse id: {}", id);
        };

        let Some(handler) = connection.resolve(method) else {
            return error!("Response without request: {method}")
        };

        let handler = match handler {
            Callback::Response(handler) => handler,
            Callback::Request(..) | Callback::Notification(..) => return error!("{method} is not a response endpoint"),
        };

        error!("Error({:?}) for {method}#{tag}: {}", error.code, error.message);
        handler(connection, tag.to_string(), None).ok();

        if let Some(error) = connection.take_error() {
            return error!("Failed to process {method}#{tag}: {}", error.message);
        }
    }

    fn handle_notification(connection: &mut impl RpcConnection, method: String, params: Value) {

        let Some(handler) = connection.resolve(method.as_str()) else {
            return error!("Method not found: {method}")
        };

        let handler = match handler {
            Callback::Notification(handler) => handler,
            Callback::Request(..) | Callback::Response(..) => return error!("{method} is not a notification endpoint"),
        };

        let result = handler(connection, params);

        if let Some(error) = connection.take_error() {
            return error!("Failed to process {method}: {}", error.message);
        }

        if let Err(error) = result {
            return error!("Failed to parse params for {method}: {}", error);
        }
    }

    fn handle_request(connection: &mut impl RpcConnection, method: String, id: MessageID, params: Value) {
        let Some(handler) = connection.resolve(method.as_str()) else {
            send(connection, Message::Error {
                jsonrpc: Version::Current,
                id,
                error: RpcError {
                    code: ErrorCode::MethodNotFound,
                    message: format!("Method not found: {method}"),
                }
            });
            
            return
        };

        let handler = match handler {
            Callback::Request(handler) => handler,
            Callback::Notification(..) | Callback::Response(..) => {
                send(connection, Message::Error {
                    jsonrpc: Version::Current,
                    id,
                    error: RpcError {
                        code: ErrorCode::MethodNotFound,
                        message: format!("{method} is not a request endpoint"),
                    }
                });

                return
            }
        };

        let result = handler(connection, params);

        if let Some(error) = connection.take_error() {
            send(connection, Message::Error {
                jsonrpc: Version::Current,
                id,
                error
            });

            return
        }

        match result {
            Ok(result) => send(connection, Message::Response {
                jsonrpc: Version::Current,
                id,
                result
            }),
            Err(error) => send(connection, Message::Error {
                jsonrpc: Version::Current,
                id,
                error: RpcError {
                    code: ErrorCode::InvalidParams,
                    message: format!("Failed to parse params: {error}"),
                }
            })
        };
    }
}