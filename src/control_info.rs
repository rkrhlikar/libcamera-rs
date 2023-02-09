use std::fmt::{write, Debug};

use crate::control_value::ControlValue;
use crate::ffi;

#[repr(transparent)]
pub struct ControlInfo {
    raw: ffi::ControlInfo,
}

impl<'a> From<&'a ffi::ControlInfo> for &'a ControlInfo {
    fn from(value: &'a ffi::ControlInfo) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl ControlInfo {
    pub fn def(&self) -> ControlValue {
        self.raw.def().into()
    }

    pub fn min(&self) -> ControlValue {
        self.raw.min().into()
    }

    pub fn max(&self) -> ControlValue {
        self.raw.max().into()
    }

    pub fn values(&self) -> Vec<ControlValue> {
        self.raw
            .values()
            .iter()
            .map(|v| v.into())
            .collect::<Vec<_>>()
    }
}

impl Debug for ControlInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ffi::control_info_to_string(&self.raw))
    }
}
