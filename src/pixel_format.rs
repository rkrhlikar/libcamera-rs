use std::fmt::{write, Debug};

use crate::ffi;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PixelFormat {
    pub(crate) raw: ffi::PixelFormat,
}

impl ToString for PixelFormat {
    fn to_string(&self) -> String {
        ffi::pixel_format_to_string(&self.raw)
    }
}

impl Debug for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<ffi::PixelFormat> for PixelFormat {
    fn from(value: ffi::PixelFormat) -> Self {
        Self { raw: value }
    }
}

impl From<PixelFormat> for ffi::PixelFormat {
    fn from(value: PixelFormat) -> Self {
        value.raw
    }
}
