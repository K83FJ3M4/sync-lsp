use crate::Connection;
use crate::connection::Callback;

pub(crate) struct Exit<T: 'static>
    (pub(crate) fn(&mut Connection<T>));

impl<T> Exit<T> {

    pub(crate) const METHOD: &'static str = "exit";
    
    pub(crate) fn callback(&self) -> Callback<Connection<T>> {
        let Exit(callback) = *self;
        Callback::notification(move |connection, _: ()| callback(connection))
    }
}