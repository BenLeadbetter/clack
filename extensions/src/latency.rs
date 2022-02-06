use clack_common::extensions::*;
use clack_host::wrapper::HostWrapper;
use clap_sys::ext::latency::{clap_host_latency, clap_plugin_latency, CLAP_EXT_LATENCY};

#[repr(C)]
pub struct PluginLatency {
    inner: clap_plugin_latency,
}

unsafe impl Extension for PluginLatency {
    const IDENTIFIER: &'static [u8] = CLAP_EXT_LATENCY;
    type ExtensionType = PluginExtension;
}

#[repr(C)]
pub struct HostLatency {
    inner: clap_host_latency,
}

unsafe impl Extension for HostLatency {
    const IDENTIFIER: &'static [u8] = CLAP_EXT_LATENCY;
    type ExtensionType = HostExtension;
}

#[cfg(feature = "clack-host")]
const _: () = {
    use clack_host::host::PluginHoster;
    use clack_host::plugin::PluginMainThread;
    use clap_sys::host::clap_host;

    impl PluginLatency {
        #[inline]
        pub fn get(&self, plugin: &mut PluginMainThread) -> u32 {
            if let Some(get) = self.inner.get {
                unsafe { get(plugin.as_raw()) }
            } else {
                0
            }
        }
    }

    pub trait HostLatencyImpl {
        fn changed(&mut self);
    }

    impl<'a, H: PluginHoster<'a> + HostLatencyImpl> ExtensionImplementation<H> for HostLatency {
        const IMPLEMENTATION: &'static Self = &HostLatency {
            inner: clap_host_latency {
                changed: Some(changed::<H>),
            },
        };
    }

    unsafe extern "C" fn changed<'a, H: PluginHoster<'a> + HostLatencyImpl>(
        host: *const clap_host,
    ) {
        HostWrapper::<H>::handle(host, |host| {
            host.main_thread().as_mut().changed();
            Ok(())
        });
    }
};

#[cfg(feature = "clack-plugin")]
const _: () = {
    use clack_plugin::host::HostMainThreadHandle;
    use clack_plugin::plugin::wrapper::PluginWrapper;
    use clack_plugin::plugin::Plugin;
    use clap_sys::plugin::clap_plugin;

    impl HostLatency {
        #[inline]
        pub fn changed(&self, host: &mut HostMainThreadHandle) {
            if let Some(changed) = self.inner.changed {
                unsafe { changed(host.shared().as_raw()) }
            }
        }
    }

    pub trait PluginLatencyImpl {
        fn get(&mut self) -> u32;
    }

    impl<'a, P: Plugin<'a>> ExtensionImplementation<P> for PluginLatency
    where
        P::MainThread: PluginLatencyImpl,
    {
        const IMPLEMENTATION: &'static Self = &PluginLatency {
            inner: clap_plugin_latency {
                get: Some(get::<P>),
            },
        };
    }

    unsafe extern "C" fn get<'a, P: Plugin<'a>>(plugin: *const clap_plugin) -> u32
    where
        P::MainThread: PluginLatencyImpl,
    {
        PluginWrapper::<P>::handle(plugin, |plugin| Ok(plugin.main_thread().as_mut().get()))
            .unwrap_or(0)
    }
};
