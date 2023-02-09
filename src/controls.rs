use std::pin::Pin;

use crate::ffi;
use crate::{AssignToRawControlValue, Control, FromRawControlValue, Rectangle, Size};

/// WARNING: This macro is unsafe and must be scoped only to this crate.
macro_rules! control {
    ($name:ident, $t:ty, stable) => {
        control!($name, $t, $crate::bindings::controls::$name);
    };
    ($name:ident, $t:ty, draft) => {
        control!($name, $t, $crate::bindings::controls::draft::$name);
    };
    ($name:ident, $t:ty, $extern_var:expr) => {
        pub const $name: $crate::Control<$t> =
            unsafe { $crate::Control::new(|| ::core::mem::transmute(&$extern_var)) };
    };
}

// Keep scoped to this crate only.
macro_rules! control_enum {
    ($name:ident $t:ty { $($case:ident = $val:expr,)* }) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name {
            value: $t,
        }

        impl $name {
            $(
                pub const $case: Self = Self::new($val);
            )*

            const fn new(value: $t) -> Self {
                Self { value }
            }
        }

        impl<'a> FromRawControlValue<'a> for $name {
            type Target = Self;

            fn from_value(value: &'a ffi::ControlValue) -> Self {
                Self::new(<$t as FromRawControlValue>::from_value(value))
            }
        }

        impl AssignToRawControlValue for $name {
            fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
                <$t>::assign_to(&self.value, value)
            }
        }

        // TODO: also need stringification
    };
}

include!(concat!(env!("OUT_DIR"), "/controls.rs"));
