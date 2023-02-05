use std::sync::Arc;
use std::sync::Mutex;

use cxx::UniquePtr;

use crate::camera::Camera;
use crate::errors::*;
use crate::ffi;
use crate::frame_buffer::FrameBufferRef;

/// Allocates FrameBuffers for storing camera response data.
///
/// NOTE: Compared to the C++ API, we do not allow explicitly freeing buffers
/// associated with a specific stream. Exposing this would add the risk of
/// freeing buffers still associated with a request. Instead all buffers in the
/// FrameBufferAllocator will be freed at once when all references to them are
/// dropped. If you need to create more buffers earlier than than, prefer to use
/// a different FrameBufferAllocator instance.
pub struct FrameBufferAllocator {
    inner: Arc<FrameBufferAllocatorInner>,
}

pub(crate) struct FrameBufferAllocatorInner {
    #[allow(unused)]
    camera: Arc<Camera>,

    raw: Mutex<UniquePtr<ffi::FrameBufferAllocator>>,
}

impl FrameBufferAllocator {
    pub(crate) fn new(camera: Arc<Camera>, raw: UniquePtr<ffi::FrameBufferAllocator>) -> Self {
        Self {
            inner: Arc::new(FrameBufferAllocatorInner {
                camera,
                raw: Mutex::new(raw),
            }),
        }
    }

    /// Returns the number of buffers allocated.
    pub fn allocate(&mut self, stream: &mut ffi::Stream) -> Result<usize> {
        // NOTE: libcamera will return EBUSY if buffers have already been allocated for
        // the given stream so old buffers don't be overriden/freed.
        let n = to_result(unsafe {
            self.inner
                .raw
                .lock()
                .unwrap()
                .as_mut()
                .unwrap()
                .allocate(stream)
        })? as usize;

        Ok(n)
    }

    /// TODO: stream references need to be bound to a Camera.
    pub fn buffers<'a>(&'a self, stream: &mut ffi::Stream) -> Vec<FrameBufferRef> {
        let s = stream as *mut ffi::Stream;

        unsafe {
            ffi::get_allocated_frame_buffers(
                self.inner.raw.lock().unwrap().as_ref().unwrap(),
                &mut *s,
            )
            .into_iter()
            .map(|v| FrameBufferRef {
                allocator: self.inner.clone(),
                stream: core::mem::transmute(s),
                raw: &mut *v.buffer,
            })
            .collect()
        }
    }
}
