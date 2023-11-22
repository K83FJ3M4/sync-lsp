use crate::{TypeProvider, Server};
use crate::connection::{Callback, CancelParams};

pub(crate) struct Cancel<T: TypeProvider>
    (pub(crate) fn(&mut Server<T>));

impl<T: TypeProvider> Cancel<T> {

    pub(crate) const METHOD: &'static str = "$/cancelRequest";
    
    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        let Cancel(callback) = *self;
        Callback::notification(move |server, _: CancelParams| callback(server))
    }
}