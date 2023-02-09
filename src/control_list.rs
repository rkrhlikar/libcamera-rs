use std::fmt::Debug;
use std::pin::Pin;

use crate::control::Control;
use crate::control_value::{AssignToRawControlValue, ControlValue, FromRawControlValue};
use crate::ffi;

/*
Note:
- Valid ControlLists should always have a valid idMap and infoMap

*/

#[repr(transparent)]
pub struct ControlList {
    raw: ffi::ControlList,
}

impl ControlList {
    /*
    pub fn get(&self, id: &ffi::ControlId) -> Option<ControlValue> {
        if !self.raw.contains(id.id()) {
            return None;
        }

        Some(ControlValue::from(self.raw.get(id.id())))
    }
    */

    pub fn get<'a, T: FromRawControlValue<'a>>(&'a self, control: Control<T>) -> Option<T::Target> {
        if !self.raw.contains(control.id()) {
            return None;
        }

        Some(T::from_value(self.raw.get(control.id())))
    }

    pub fn set<T: AssignToRawControlValue, V: Into<T>>(&mut self, control: Control<T>, value: V) {
        let mut raw_value = ffi::new_control_value();
        value.into().assign_to(raw_value.as_mut().unwrap());

        let p = unsafe { Pin::new_unchecked(&mut self.raw) };
        p.set(control.id(), &raw_value);
    }

    /*
    pub fn set(&mut self, id: &ffi::ControlId, value: &ControlValue) {
        // TODO: Check for a type match?

        // NOTE: We assume that libcamera will behave ok if the control is not defined
        // for the camera.

        let mut native_value = ffi::new_control_value();
        value.assign_to(native_value.as_mut().unwrap());

        let p = unsafe { Pin::new_unchecked(&mut self.raw) };
        p.set(id.id(), &native_value);
    }
    */
}

impl<'a> From<&'a ffi::ControlList> for &'a ControlList {
    fn from(value: &'a ffi::ControlList) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl<'a> From<&'a mut ffi::ControlList> for &'a mut ControlList {
    fn from(value: &'a mut ffi::ControlList) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Debug for ControlList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = ffi::control_list_entries(&self.raw);

        // TODO: Debup this logic.
        let id_map = {
            let p = self.raw.idMap();
            assert!(p != core::ptr::null());
            unsafe { &*p }
        };

        let mut s = f.debug_struct("ControlList");

        for entry in entries {
            let field_name = if id_map.contains(&entry.key) {
                unsafe { &**id_map.at(&entry.key) }.name().to_string()
            } else {
                format!("Unknown({})", entry.key)
            };

            s.field(&field_name, &ffi::control_value_to_string(entry.value));
        }

        s.finish()
    }
}
