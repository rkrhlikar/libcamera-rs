use std::sync::Arc;

use crate::ffi;
use crate::frame_buffer_allocator::FrameBufferAllocatorInner;

pub use ffi::{FrameBufferPlane, FrameMetadata, FramePlaneMetadata, FrameStatus};

/// Reference to a FrameBuffer owned by a FrameBufferAllocator.
pub struct FrameBufferRef {
    #[allow(unused)]
    pub(crate) allocator: Arc<FrameBufferAllocatorInner>,
    pub(crate) stream: &'static ffi::Stream,
    pub(crate) raw: &'static mut ffi::FrameBuffer,
}

unsafe impl Send for FrameBufferRef {}
unsafe impl Sync for FrameBufferRef {}

impl FrameBufferRef {
    pub fn planes(&self) -> Vec<FrameBufferPlane> {
        ffi::frame_buffer_planes(self.raw)
    }

    pub fn metadata(&self) -> FrameMetadata {
        ffi::frame_buffer_metadata(self.raw)
    }
}
