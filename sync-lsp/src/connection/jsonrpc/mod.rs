use std::rc::Rc;
use crate::Transport;
use log::{Level, Log, Metadata, Record, error};
use serde_json::{Value, Error as JsonError, from_value, to_value, from_str};
use serde::{Serialize, de::DeserializeOwned};
pub(super) use message::{Error as RpcError, MessageID};
pub(crate) use message::{EmptyParams, CancelParams};
pub use message::ErrorCode;
use std::sync::mpsc::Sender;

mod message;

pub(crate) trait RpcConnection: Sized + 'static {
    fn transport(&mut self) -> &mut Transport;
    fn resolve(&self, method: &str) -> Option<Callback<Self>>;
    fn take_error(&mut self) -> Option<RpcError>;
    fn log(&mut self, level: Level, message: String);
    fn set_current_request(&mut self, id: Option<MessageID>);

    fn notify(&mut self, method: &str, params: impl Serialize)
        { RpcConnectionImpl::notify(self, method, params) }
    fn request(&mut self, method: &str, tag: impl Serialize, params: impl Serialize)
        { RpcConnectionImpl::request(self, method, tag, params) }
    fn peek_notification<T: DeserializeOwned>(&mut self, method: &str) -> Option<T>
        { RpcConnectionImpl::peek_notification(self, method) }
}

pub(crate) enum Callback<T: RpcConnection> {
    Request(Rc<dyn Fn(&mut T, Value) -> Result<Value, JsonError>>),
    Notification(Rc<dyn Fn(&mut T, Value) -> Result<(), JsonError>>),
    Response(Rc<dyn Fn(&mut T, String, Option<Value>) -> Result<(), JsonError>>),
}

struct RpcLogger {
    sender: Sender<(Level, String)>
}

impl Log for RpcLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            self.sender.send((record.level(), record.args().to_string())).ok();
        }
    }

    fn flush(&self) {}
}

impl<T: RpcConnection> Clone for Callback<T> {
    fn clone(&self) -> Self {
        match self {
            Callback::Request(callback) => Callback::Request(callback.clone()),
            Callback::Notification(callback) => Callback::Notification(callback.clone()),
            Callback::Response(callback) => Callback::Response(callback.clone()),
        }
    }

}

impl<T: RpcConnection> Callback<T> {
    pub(crate) fn request<P: DeserializeOwned, R: 'static + Serialize>(callback: impl 'static + Fn(&mut T, P) -> R) -> Self {
        Self::Request(Rc::new(move |server, value| {
            let params = from_value(value)?;
            let result = callback(server, params);
            to_value(result)
        }))
    }

    pub(crate) fn notification<P: DeserializeOwned>(callback: impl 'static + Fn(&mut T, P)) -> Self {
        Self::Notification(Rc::new(move |server, value| {
            let params = from_value(value)?;
            Ok(callback(server, params))
        }))
    }

    pub(crate) fn response<D: DeserializeOwned + Default, P: DeserializeOwned + Default>(callback: impl 'static + Fn(&mut T, D, P)) -> Self {
        Self::Response(Rc::new(move |server, id, value| {
            let id = from_str(id.as_str()).unwrap_or_else(|err| {
                error!("Failed to parse id: {id}: {err}");
                D::default()
            });

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
    use log::{error, set_boxed_logger, set_max_level, LevelFilter};
    use serde::de::DeserializeOwned;
    use serde_json::{Value, from_slice, to_string, to_value, from_value};
    use std::io::{Error, ErrorKind};
    use std::sync::mpsc::channel;
    use serde::Serialize;
    use crate::connection::jsonrpc::message::{Message, MessageID, Version, Error as RpcError};

    use super::message::ErrorCode;
    use super::{RpcConnection, Callback, RpcLogger};

    pub(crate) fn serve(mut connection: impl RpcConnection) -> Result<(), Error> {

        let (sender, receiver) = channel();
        let logger = RpcLogger {
            sender
        };
        
        if let Err(error) = set_boxed_logger(Box::new(logger)) {
            return Err(Error::new(ErrorKind::Other, error.to_string()));
        }

        #[cfg(debug_assertions)]
        set_max_level(LevelFilter::Trace);
        #[cfg(not(debug_assertions))]
        set_max_level(LevelFilter::Info);

        while let Some(message) = recv(&mut connection) {
            handle(&mut connection, message);
            while let Ok((level, message)) = receiver.try_recv() {
                connection.log(level, message);
            }
        }

        if let Some(error) = connection.transport().error().take() {
            return Err(error);
        } else {
            Ok(())
        }
    }

    pub(super) fn peek_notification<T: DeserializeOwned>(connection: &mut impl RpcConnection, target: &str) -> Option<T> {
        let data = connection.transport().peek()?;
        let message = from_slice(data.as_slice()).ok()?;
        let Message::Notification { method, params, .. } = message else { return None };
        if method != target { return None };
        from_value(params).ok()
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

    pub(super) fn request(connection: &mut impl RpcConnection, method: &str, tag: impl Serialize, params: impl Serialize) {
        let id = MessageID::String(format!("{method}#{}", match to_string(&tag) {
            Ok(tag) => tag,
            Err(error) => {
                error!("Failed to serialize tag for {method} request: {}", error);
                "{}".to_string()
            }
        }));

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

        connection.set_current_request(Some(id.clone()));
        let result = handler(connection, params);
        connection.set_current_request(None);

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