use std::fmt::{write, Debug};

use crate::ffi;

pub use ffi::PixelFormat;

impl ToString for PixelFormat {
    fn to_string(&self) -> String {
        ffi::pixel_format_to_string(self)
    }
}

impl From<String> for PixelFormat {
    fn from(s: String) -> Self {
        ffi::pixel_format_from_string(s)
    }
}

impl Debug for PixelFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
