use crate::Connection;
use crate::connection::Callback;

pub(super) use self::exit::Exit;
pub(super) use self::initialize::Initialize;
pub(super) use self::initialized::Initialized;
pub(super) use self::shutdown::Shutdown;

pub(crate) mod initialize;
mod initialized;
mod shutdown;
mod exit;

pub(super) struct LifecycleService<T: 'static> {
    pub(super) initialize: Initialize<T>,
    pub(super) initialized: Initialized<T>,
    pub(super) shutdown: Shutdown<T>,
    pub(super) exit: Exit<T>
}

impl<T> LifecycleService<T> {
    pub(super) fn resolve(&self, method: &str) -> Option<Callback<Connection<T>>> {
        match method {
            Initialize::<T>::METHOD => Some(self.initialize.callback()),
            Initialized::<T>::METHOD => Some(self.initialized.callback()),
            Shutdown::<T>::METHOD => Some(self.shutdown.callback()),
            Exit::<T>::METHOD => Some(self.exit.callback()),
            _ => None
        }
    }
}