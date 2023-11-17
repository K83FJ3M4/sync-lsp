use serde::Serialize;
use crate::{Connection, TypeProvider};
use crate::connection::RpcConnection;

#[derive(Default)]
pub(super) struct Telemetry;

impl<T: TypeProvider> Connection<T> {
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