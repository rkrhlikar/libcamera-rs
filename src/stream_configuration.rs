use std::pin::Pin;

use crate::ffi;

pub use ffi::Stream;

/// Wrapper around a ffi::StreamConfiguration value. Note that because CXX only
/// understands C++ structs as opaque values with no defined size, this struct
/// will never be instantiated. You will only ever get references to it.
#[repr(transparent)]
pub struct StreamConfigurationOpaque {
    config: ffi::StreamConfiguration,
}

impl StreamConfigurationOpaque {
    pub fn set_buffer_count(&mut self, value: u32) {
        ffi::stream_config_set_buffer_count(unsafe { Pin::new_unchecked(&mut self.config) }, value)
    }

    /// NOTE: Streams will only be after the configuration is used to configure
    /// a camera.
    pub fn stream(&self) -> Option<&mut Stream> {
        let stream = self.config.stream();
        if stream != core::ptr::null_mut() {
            Some(unsafe { &mut *stream })
        } else {
            None
        }
    }
}

impl ToString for StreamConfigurationOpaque {
    fn to_string(&self) -> String {
        ffi::stream_config_to_string(&self.config)
    }
}
