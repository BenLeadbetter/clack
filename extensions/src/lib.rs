#![doc(html_logo_url = "https://raw.githubusercontent.com/prokopyl/clack/main/logo.svg")]

#[cfg(feature = "audio-ports")]
pub mod audio_ports;
#[cfg(feature = "event-registry")]
pub mod event_registry;
#[cfg(feature = "gui")]
pub mod gui;
#[cfg(feature = "latency")]
pub mod latency;
#[cfg(feature = "log")]
pub mod log;
#[cfg(feature = "note-ports")]
pub mod note_ports;
#[cfg(feature = "params")]
pub mod params;
#[cfg(feature = "state")]
pub mod state;
#[cfg(feature = "thread-check")]
pub mod thread_check;

pub(crate) mod utils;
