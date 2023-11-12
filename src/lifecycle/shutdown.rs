use crate::Connection;
use crate::connection::Callback;

pub(crate) struct Shutdown<T: 'static>
    (pub(crate) fn(&mut Connection<T>));

impl<T> Shutdown<T> {

    pub(crate) const METHOD: &'static str = "shutdown";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Shutdown(callback) = *self;
        Callback::request(move |connection, _: ()| callback(connection))
    }
}