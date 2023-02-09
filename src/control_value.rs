use std::pin::Pin;

use paste::paste;

use crate::{ffi, Size};

pub use ffi::Rectangle;

// TODO: For consistency, use the C++ toString when debugging this?

// TODO: Remove this?
#[derive(Debug, Clone)]
pub enum ControlValue {
    None,
    Primitive(ControlPrimitiveValue),
    Array(ControlArrayValue),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum ControlPrimitiveValue {
    Bool(bool),
    Byte(u8),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Rectangle(Rectangle),
    Size(Size),
    String(String),
}

#[derive(Debug, Clone)]
pub enum ControlArrayValue {
    Bool(Vec<bool>),
    Byte(Vec<u8>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    Float(Vec<f32>),
    Rectangle(Vec<Rectangle>),
    Size(Vec<Size>),
    String(Vec<String>),
}

impl ControlValue {
    pub fn is_none(&self) -> bool {
        if let ControlValue::None = self {
            true
        } else {
            false
        }
    }

    pub(crate) fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
        match self {
            ControlValue::None => todo!(),
            ControlValue::Primitive(p) => match p {
                ControlPrimitiveValue::Bool(v) => value.set_bool(v),
                ControlPrimitiveValue::Byte(_) => todo!(),
                ControlPrimitiveValue::Int32(_) => todo!(),
                ControlPrimitiveValue::Int64(_) => todo!(),
                ControlPrimitiveValue::Float(_) => todo!(),
                ControlPrimitiveValue::Rectangle(_) => todo!(),
                ControlPrimitiveValue::Size(_) => todo!(),
                ControlPrimitiveValue::String(_) => todo!(),
                //
            },
            ControlValue::Array(_) => todo!(),
            ControlValue::Unknown => todo!(),
        }
    }
}

impl From<&ffi::ControlValue> for ControlValue {
    fn from(value: &ffi::ControlValue) -> Self {
        if value.isArray() {
            use ControlArrayValue::*;

            ControlValue::Array(match value.typ() {
                ffi::ControlType::ControlTypeNone => return ControlValue::None,
                ffi::ControlType::ControlTypeBool => {
                    Bool(ffi::control_value_get_bool_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeByte => {
                    Byte(ffi::control_value_get_byte_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeInteger32 => {
                    Int32(ffi::control_value_get_i32_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeInteger64 => {
                    Int64(ffi::control_value_get_i64_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeFloat => {
                    Float(ffi::control_value_get_float_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeString => {
                    String(ffi::control_value_get_string_array(value))
                }
                ffi::ControlType::ControlTypeRectangle => {
                    Rectangle(ffi::control_value_get_rectangle_array(value).to_vec())
                }
                ffi::ControlType::ControlTypeSize => {
                    Size(ffi::control_value_get_size_array(value).to_vec())
                }
                _ => return ControlValue::Unknown,
            })
        } else {
            use ControlPrimitiveValue::*;

            ControlValue::Primitive(match value.typ() {
                // ffi::ControlType::ControlTypeNone => return ControlValue::None,
                // ffi::ControlType::ControlTypeBool =>
                // Bool(FromRawControlValue::from_value(value)),
                // ffi::ControlType::ControlTypeByte =>
                // Byte(FromRawControlValue::from_value(value)),
                // ffi::ControlType::ControlTypeInteger32 => {
                //     Int32(FromRawControlValue::from_value(value))
                // }
                // ffi::ControlType::ControlTypeInteger64 => {
                //     Int64(FromRawControlValue::from_value(value))
                // }
                // ffi::ControlType::ControlTypeFloat =>
                // Float(FromRawControlValue::from_value(value)),
                // ffi::ControlType::ControlTypeString =>
                // String(ffi::control_value_get_string(value)),
                // ffi::ControlType::ControlTypeRectangle => {
                //     Rectangle(FromRawControlValue::from_value(value))
                // }
                // ffi::ControlType::ControlTypeSize =>
                // Size(FromRawControlValue::from_value(value)),
                _ => {
                    return ControlValue::Unknown;
                }
            })
        }
    }
}

// TODO: Move the type checking to the rust side so we have clearer panic
// behaiovr.
pub trait FromRawControlValue<'a> {
    type Target;

    fn from_value(value: &'a ffi::ControlValue) -> Self::Target;
}

pub trait AssignToRawControlValue {
    fn assign_to(&self, value: Pin<&mut ffi::ControlValue>);
}

/*
Note: get_type() functions are safe in the FFI because ControlValue::get() in C++ asserts that the type is correct.
*/

// control_value_get_bool_array

// TODO: Consider not using standard types like AsRef and From as that may hide
// the fact that these can fail.
macro_rules! impl_control_value_type {
    ($typ:ident) => {
        impl_control_value_type!($typ, $typ);
    };
    ($typ:ident, $ffi_typ:ident) => {
        paste! {
            impl<'a> FromRawControlValue<'a> for $typ {
                type Target = Self;

                fn from_value(value: &'a ffi::ControlValue) -> Self {
                    value.[<get_ $ffi_typ>]()
                }
            }

            impl<'a> FromRawControlValue<'a> for [$typ] {
                type Target = &'a Self;

                fn from_value(value: &'a ffi::ControlValue) -> &'a Self {
                    ffi::[<control_value_get_ $ffi_typ _array>](value)
                }
            }

            impl AssignToRawControlValue for $typ {
                fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
                    value.[<set_ $ffi_typ>](self);
                }
            }

            impl AssignToRawControlValue for [$typ] {
                fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
                   ffi::[<control_value_set_ $ffi_typ _array>](value, self);
                }
            }

            impl<const LEN: usize> AssignToRawControlValue for [$typ; LEN] {
                fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
                   ffi::[<control_value_set_ $ffi_typ _array>](value, &self[..]);
                }
            }
        }
    };
}

impl_control_value_type!(bool);
impl_control_value_type!(u8, byte);
impl_control_value_type!(i32);
impl_control_value_type!(i64);
impl_control_value_type!(f32, float);
impl_control_value_type!(Rectangle, rectangle);
impl_control_value_type!(Size, size);

impl AssignToRawControlValue for String {
    fn assign_to(&self, value: Pin<&mut ffi::ControlValue>) {
        ffi::control_value_set_string(value, self);
    }
}

/*

Two types of values:

Setting:
    &T, Pin<&mut ffi::ControlValue>


*/
