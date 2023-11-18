use crate::{Connection, TypeProvider};
use crate::connection::Callback;

pub(crate) struct Exit<T: TypeProvider>
    (pub(crate) fn(&mut Connection<T>));

impl<T: TypeProvider> Exit<T> {

    pub(crate) const METHOD: &'static str = "exit";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Exit(callback) = *self;
        Callback::notification(move |connection, _: ()| callback(connection))
    }
}