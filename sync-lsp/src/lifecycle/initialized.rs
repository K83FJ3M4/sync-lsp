use crate::{Server, TypeProvider};
use crate::connection::{Callback, EmptyParams};

pub(crate) struct Initialized<T: TypeProvider>
    (pub(crate) fn(&mut Server<T>));

impl<T: TypeProvider> Initialized<T> {

    pub(crate) const METHOD: &'static str = "initialized";
    
    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        let Initialized(callback) = *self;
        Callback::notification(move |server, _: EmptyParams| callback(server))
    }
}