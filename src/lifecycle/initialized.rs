use crate::{Connection, TypeProvider};
use crate::connection::{Callback, EmptyParams};

pub(crate) struct Initialized<T: TypeProvider>
    (pub(crate) fn(&mut Connection<T>));

impl<T: TypeProvider> Initialized<T> {

    pub(crate) const METHOD: &'static str = "initialized";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Initialized(callback) = *self;
        Callback::notification(move |connection, _: EmptyParams| callback(connection))
    }
}