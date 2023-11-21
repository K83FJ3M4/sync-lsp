use crate::{Connection, TypeProvider};
use crate::connection::{Callback, CancelParams};

pub(crate) struct Cancel<T: TypeProvider>
    (pub(crate) fn(&mut Connection<T>));

impl<T: TypeProvider> Cancel<T> {

    pub(crate) const METHOD: &'static str = "$/cancelRequest";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Cancel(callback) = *self;
        Callback::notification(move |connection, _: CancelParams| callback(connection))
    }
}