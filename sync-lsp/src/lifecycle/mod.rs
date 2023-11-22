use crate::{Server, TypeProvider};
use crate::connection::Callback;

pub(super) use self::exit::Exit;
pub(super) use self::initialize::Initialize;
pub(super) use self::initialized::Initialized;
pub(super) use self::shutdown::Shutdown;
pub(super) use self::cancel::Cancel;

pub(crate) mod initialize;
mod initialized;
mod shutdown;
mod exit;
mod cancel;

pub(super) struct LifecycleService<T: TypeProvider> {
    pub(super) initialize: Initialize<T>,
    pub(super) initialized: Initialized<T>,
    pub(super) shutdown: Shutdown<T>,
    pub(super) exit: Exit<T>,
    pub(super) cancel: Cancel<T>
}

impl<T: TypeProvider> LifecycleService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Server<T>>> {
        match method {
            Initialize::<T>::METHOD => Some(self.initialize.callback()),
            Initialized::<T>::METHOD => Some(self.initialized.callback()),
            Shutdown::<T>::METHOD => Some(self.shutdown.callback()),
            Exit::<T>::METHOD => Some(self.exit.callback()),
            Cancel::<T>::METHOD => Some(self.cancel.callback()),
            _ => None
        }
    }
}