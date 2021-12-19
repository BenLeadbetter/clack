use crate::process::audio::Audio;
use clap_sys::process::clap_process;

#[repr(C)]
pub struct Process {
    inner: clap_process,
}

impl Process {
    #[inline]
    pub(crate) unsafe fn from_raw<'a>(raw: *const clap_process) -> (&'a Process, Audio<'a>) {
        // SAFETY: Process is repr(C) and is guaranteed to have the same memory representation
        let process: &Process = &*(raw as *const _);
        (process, Audio::from_raw(&*raw))
    }

    #[inline]
    pub fn frames_count(&self) -> u32 {
        self.inner.frames_count
    }

    #[inline]
    pub fn steady_time(&self) -> u64 {
        self.inner.steady_time
    }
}

pub mod audio;