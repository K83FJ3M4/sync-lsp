use crate::{Server, TypeProvider};
use crate::connection::Callback;

pub(crate) struct Shutdown<T: TypeProvider>
    (pub(crate) fn(&mut Server<T>));

impl<T: TypeProvider> Shutdown<T> {

    pub(crate) const METHOD: &'static str = "shutdown";
    
    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        let Shutdown(callback) = *self;
        Callback::request(move |server, _: ()| callback(server))
    }
}