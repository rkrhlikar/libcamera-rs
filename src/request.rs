use std::sync::Arc;

use cxx::UniquePtr;

use crate::camera::Camera;
use crate::errors::*;
use crate::ffi;
use crate::frame_buffer::FrameBufferRef;

pub use ffi::RequestStatus;

/// NOTE: When this is dropped, the libcamers C++ code will cancel the request
/// in the destructor.
pub struct Request {
    #[allow(unused)]
    camera: Arc<Camera>,

    pub(crate) raw: UniquePtr<ffi::Request>,
}

impl Request {
    pub(crate) fn new(camera: Arc<Camera>, raw: UniquePtr<ffi::Request>) -> Self {
        Self { camera, raw }
    }

    /// TODO: Ensure frame buffers are only assigned to one request.
    pub fn add_buffer(&mut self, buffer: &mut FrameBufferRef) -> Result<()> {
        ok_if_zero(unsafe {
            self.raw
                .as_mut()
                .unwrap()
                .addBuffer(buffer.stream, buffer.raw, UniquePtr::null())
        })
    }

    pub fn status(&self) -> RequestStatus {
        ffi::request_status(self.raw.as_ref().unwrap())
    }

    pub fn cookie(&self) -> u64 {
        self.raw.cookie()
    }

    pub fn sequence(&self) -> u32 {
        self.raw.sequence()
    }
}
