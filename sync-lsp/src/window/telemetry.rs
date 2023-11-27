//! implementation of the `window/telemetry` notification.
//! 
//! # Usage
//! [`Connection::telemetry`] sends arbitrary telemetry data to the client.

use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

#[derive(Default)]
pub(super) struct Telemetry;

impl<T: TypeProvider> Connection<T> {

    /// This notification sends arbitrary [telemetry data](self) to the client.
    /// 
    /// # Arguments
    /// * `params` - The data to send.

    pub fn telemetry(&mut self, params: impl Serialize) {
        self.notify(
            Telemetry::METHOD,
            params
        );
    }
}

impl Telemetry {
    const METHOD: &'static str = "telemetry/event";
}