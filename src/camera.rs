use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

use cxx::SharedPtr;

use crate::camera_configuration::*;
use crate::camera_manager::CameraManager;
use crate::errors::*;
use crate::ffi;
use crate::frame_buffer_allocator::FrameBufferAllocator;
use crate::request::Request;

pub use crate::ffi::StreamRole;

// TODO: On drop, do release/stop?

pub struct Camera {
    /// Used to ensure that the ffi::Camera outlives the ffi::CameraManager.
    #[allow(unused)]
    pub(crate) manager: Arc<CameraManager>,

    pub(crate) raw: SharedPtr<ffi::Camera>,
}

// NOTE: Most shared logic is stored in private methods here. They should be
// exposed as public methods in the appropriate state specific structs if they
// are valid to be called in that state.
impl Camera {
    pub fn id(&self) -> String {
        self.raw.as_ref().unwrap().id().to_string()
    }

    fn get_mut(&self) -> Pin<&mut ffi::Camera> {
        unsafe {
            Pin::<&mut ffi::Camera>::new_unchecked(
                &mut *(core::mem::transmute::<_, u64>(self.raw.as_ref().unwrap())
                    as *mut ffi::Camera),
            )
        }
    }

    fn acquire(&self) -> Result<()> {
        ok_if_zero(self.get_mut().acquire())
    }

    fn release(&self) -> Result<()> {
        ok_if_zero(self.get_mut().release())
    }

    fn generate_configuration(
        self: &Arc<Self>,
        stream_roles: &[StreamRole],
    ) -> Option<CameraConfiguration> {
        let raw = ffi::generate_camera_configuration(self.get_mut(), stream_roles);
        if raw.is_null() {
            return None;
        }

        Some(CameraConfiguration::new(self.clone(), raw))
    }

    // Only allowed for Configured and Running cameras
    //
    // TODO: What should we do with requests that are still hanging when the camera
    // is stopped.
    fn create_request(self: &Arc<Self>, cookie: u64) -> Request {
        let raw = self.get_mut().createRequest(cookie);
        assert!(!raw.is_null());
        Request::new(self.clone(), raw)
    }

    // Only allocated in the Running state.
    fn queue_request(&self, request: &mut Request) -> Result<()> {
        ok_if_zero(unsafe {
            self.get_mut()
                .queueRequest(request.raw.as_mut().unwrap().get_unchecked_mut())
        })
    }
}

/// A reference to a camera which may be acquired for exclusive access.
pub struct AvailableCamera {
    camera: Arc<Camera>,
}

impl Deref for AvailableCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        &self.camera
    }
}

impl AvailableCamera {
    pub(crate) fn new(camera: Arc<Camera>) -> Self {
        Self { camera }
    }

    pub fn acquire(self) -> Result<AcquiredCamera> {
        self.camera.acquire()?;
        Ok(AcquiredCamera {
            camera: self.camera,
        })
    }
}

pub struct AcquiredCamera {
    camera: Arc<Camera>,
}

impl Deref for AcquiredCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        &self.camera
    }
}

impl AcquiredCamera {
    pub fn release(self) -> Result<AvailableCamera> {
        self.camera.release()?;
        Ok(AvailableCamera {
            camera: self.camera,
        })
    }

    pub fn generate_configuration(
        &self,
        stream_roles: &[StreamRole],
    ) -> Option<CameraConfiguration> {
        self.camera.generate_configuration(stream_roles)
    }

    pub fn configure(self, config: &mut CameraConfiguration) -> Result<ConfiguredCamera> {
        ok_if_zero(unsafe {
            self.camera
                .get_mut()
                .configure(config.raw.as_mut().unwrap().get_unchecked_mut())
        })?;

        Ok(ConfiguredCamera {
            camera: self.camera,
        })
    }
}

pub struct ConfiguredCamera {
    camera: Arc<Camera>,
}

impl Deref for ConfiguredCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        &self.camera
    }
}

impl ConfiguredCamera {
    pub fn new_frame_buffer_allocator(&self) -> FrameBufferAllocator {
        let raw = ffi::new_frame_buffer_allocator(self.camera.raw.clone());
        assert!(!raw.is_null());

        FrameBufferAllocator::new(self.camera.clone(), raw)
    }

    pub fn create_request(&self, cookie: u64) -> Request {
        self.camera.create_request(cookie)
    }

    pub fn start(self) -> Result<RunningCamera> {
        ok_if_zero(unsafe { self.camera.get_mut().start(core::ptr::null_mut()) })?;
        Ok(RunningCamera {
            camera: self.camera,
        })
    }
}

pub struct RunningCamera {
    camera: Arc<Camera>,
}

impl Deref for RunningCamera {
    type Target = Camera;

    fn deref(&self) -> &Camera {
        &self.camera
    }
}

impl RunningCamera {
    pub fn queue_request(&self, request: &mut Request) -> Result<()> {
        self.camera.queue_request(request)
    }
}
