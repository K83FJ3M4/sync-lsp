//! This module contains functionality centered arround logging and ui features.
//! 
//! This module contains logging utilities and various notifications and requests related to the user interface.

use serde_repr::Serialize_repr;

use crate::{Server, TypeProvider};
use crate::connection::Callback;

use self::log_message::LogMessage;
use self::show_message::ShowMessage;
use self::show_message_request::ShowMessageRequest;
use self::telemetry::Telemetry;

pub mod show_message;
pub mod log_message;
pub mod telemetry;
pub mod show_message_request;

/// This struct contains all window endpoints, which are mainly used to resolve callbacks.
pub(super) struct WindowService<T: TypeProvider> {
    #[allow(unused)]
    show_message: ShowMessage,
    show_message_request: ShowMessageRequest<T>,
    #[allow(unused)]
    log_message: LogMessage,
    #[allow(unused)]
    telemetry: Telemetry,
}

/// The priority of a message which is either logged or shown to the user.
/// On debug builds, all messages are shown.
/// On release builds, only messages with a priority of `Info` or higher are shown.
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