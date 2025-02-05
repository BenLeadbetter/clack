use clack_common::extensions::{Extension, HostExtensionSide};
use clap_sys::ext::event_registry::{clap_host_event_registry, CLAP_EXT_EVENT_REGISTRY};
use std::ffi::CStr;

#[repr(C)]
pub struct HostEventRegistry {
    inner: clap_host_event_registry,
}

unsafe impl Extension for HostEventRegistry {
    const IDENTIFIER: &'static CStr = CLAP_EXT_EVENT_REGISTRY;
    type ExtensionSide = HostExtensionSide;
}

#[cfg(feature = "clack-plugin")]
const _: () = {
    use clack_common::events::spaces::{EventSpace, EventSpaceId};
    use clack_plugin::host::HostMainThreadHandle;

    impl HostEventRegistry {
        pub fn query<'a, S: EventSpace<'a>>(
            &self,
            host: &HostMainThreadHandle,
        ) -> Option<EventSpaceId<S>> {
            let mut out = u16::MAX;
            let success =
                unsafe { (self.inner.query?)(host.shared().as_raw(), S::NAME.as_ptr(), &mut out) };

            if !success {
                return None;
            };

            unsafe { Some(EventSpaceId::new(out)?.into_unchecked()) }
        }
    }
};

#[cfg(feature = "clack-host")]
mod host {
    use super::*;
    use clack_common::events::spaces::{EventSpace, EventSpaceId};
    use clack_host::extensions::prelude::*;
    use std::os::raw::c_char;

    /// Host implementation of an event registry
    ///
    /// # Safety
    ///
    /// The implementation of the [`query`](HostEventRegistryImpl) method must return stable, unique
    /// event space ids.
    pub unsafe trait HostEventRegistryImpl {
        fn query(&self, space_name: &CStr) -> Option<EventSpaceId>;

        #[inline]
        fn query_type<'a, S: EventSpace<'a>>(&self) -> Option<EventSpaceId<S>> {
            unsafe { self.query(S::NAME).map(|i| i.into_unchecked()) }
        }
    }

    impl<H: Host> ExtensionImplementation<H> for HostEventRegistry
    where
        for<'a> <H as Host>::MainThread<'a>: HostEventRegistryImpl,
    {
        const IMPLEMENTATION: &'static Self = &HostEventRegistry {
            inner: clap_host_event_registry {
                query: Some(query::<H>),
            },
        };
    }

    unsafe extern "C" fn query<H: Host>(
        host: *const clap_host,
        space_name: *const c_char,
        space_id: *mut u16,
    ) -> bool
    where
        for<'a> <H as Host>::MainThread<'a>: HostEventRegistryImpl,
    {
        HostWrapper::<H>::handle(host, |host| {
            let space_name = CStr::from_ptr(space_name);

            let result = host.main_thread().as_ref().query(space_name);
            *space_id = EventSpaceId::optional_id(&result);

            Ok(result.is_some())
        })
        .unwrap_or(false)
    }
}

#[cfg(feature = "clack-host")]
pub use host::*;
