use serde_repr::Serialize_repr;

use crate::{Server, TypeProvider};
use crate::connection::Callback;

use self::log_message::LogMessage;
use self::show_message::ShowMessage;
use self::show_message_request::ShowMessageRequest;
use self::telemetry::Telemetry;

mod show_message;
mod log_message;
mod telemetry;
pub mod show_message_request;

pub(super) struct WindowService<T: TypeProvider> {
    #[allow(unused)]
    show_message: ShowMessage,
    show_message_request: ShowMessageRequest<T>,
    #[allow(unused)]
    log_message: LogMessage,
    #[allow(unused)]
    telemetry: Telemetry,
}

#[repr(i32)]
#[derive(Serialize_repr, Debug)]
pub enum MessageType {
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
}

impl<T: TypeProvider> WindowService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Server<T>>> {
        match method {
            ShowMessageRequest::<T>::METHOD => Some(self.show_message_request.callback()),
            _ => None
        }
    }
}

impl<T: TypeProvider> Default for WindowService<T> {
    fn default() -> Self {
        Self {
            show_message: ShowMessage::default(),
            show_message_request: ShowMessageRequest::default(),
            log_message: LogMessage::default(),
            telemetry: Telemetry::default(),
        }
    }
}