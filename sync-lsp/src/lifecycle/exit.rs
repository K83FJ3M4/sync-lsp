use crate::{Server, TypeProvider};
use crate::connection::Callback;

pub(crate) struct Exit<T: TypeProvider>
    (pub(crate) fn(&mut Server<T>));

impl<T: TypeProvider> Exit<T> {

    pub(crate) const METHOD: &'static str = "exit";
    
    pub(crate) fn callback(&self) -> Callback<Server<T>> {
        let Exit(callback) = *self;
        Callback::notification(move |server, _: ()| callback(server))
    }
}