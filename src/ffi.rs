use cxx::{type_id, ExternType};

use crate::bindings;

pub use self::ffi::*;

pub struct RequestCompleteContext {
    pub handler: Box<dyn Fn(&Request) + Send + Sync + 'static>,
}

unsafe impl ExternType for bindings::libcamera_StreamRole {
    type Id = type_id!("libcamera::StreamRole");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_Request_Status {
    type Id = type_id!("libcamera::RequestStatus");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_Request_ReuseFlag {
    type Id = type_id!("libcamera::RequestReuseFlag");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_FrameMetadata_Status {
    type Id = type_id!("libcamera::FrameStatus");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_FrameMetadata_Plane {
    type Id = type_id!("libcamera::FramePlaneMetadata");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_CameraConfiguration_Status {
    type Id = type_id!("libcamera::CameraConfigurationStatus");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_PixelFormat {
    type Id = type_id!("libcamera::PixelFormat");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_Size {
    type Id = type_id!("libcamera::Size");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for bindings::libcamera_SizeRange {
    type Id = type_id!("libcamera::SizeRange");
    type Kind = cxx::kind::Trivial;
}

// libcamera_CameraConfiguration_Status

#[cxx::bridge]
mod ffi {
    /// A mirror of libcamera::FrameBuffer::Plane
    #[derive(Debug, Clone, Copy)]
    struct FrameBufferPlane {
        fd: u32,
        offset: u32,
        length: u32,
    }

    #[derive(Debug, Clone)]
    struct FrameMetadata {
        status: FrameStatus,
        sequence: u32,
        timestamp: u64,
        planes: Vec<FramePlaneMetadataWrap>,
    }

    // Wrapper to work around https://github.com/dtolnay/cxx/issues/741
    struct CameraPtr {
        camera: SharedPtr<Camera>,
    }

    struct FrameBufferPtr {
        buffer: *mut FrameBuffer,
    }

    #[derive(Debug, Clone)]
    struct FramePlaneMetadataWrap {
        inner: FramePlaneMetadata,
    }

    struct StreamPtr {
        stream: *mut Stream,
    }

    struct PixelFormatWrap {
        value: PixelFormat,
    }

    struct SizeWrap {
        value: Size,
    }

    extern "Rust" {
        type RequestCompleteContext;
    }

    #[namespace = "libcamera"]
    unsafe extern "C++" {
        include!("libcamera/src/ffi.h");

        // Keep in sync with ExternType definitions above.
        type StreamRole = crate::bindings::libcamera_StreamRole;
        type RequestStatus = crate::bindings::libcamera_Request_Status;
        type RequestReuseFlag = crate::bindings::libcamera_Request_ReuseFlag;
        type FrameStatus = crate::bindings::libcamera_FrameMetadata_Status;
        type FramePlaneMetadata = crate::bindings::libcamera_FrameMetadata_Plane;
        type CameraConfigurationStatus = crate::bindings::libcamera_CameraConfiguration_Status;
        type PixelFormat = crate::bindings::libcamera_PixelFormat;
        type Size = crate::bindings::libcamera_Size;
        type SizeRange = crate::bindings::libcamera_SizeRange;

        //////////////////////////////////////

        type CameraManager;

        fn new_camera_manager() -> UniquePtr<CameraManager>;

        fn start(self: Pin<&mut CameraManager>) -> i32;

        fn stop(self: Pin<&mut CameraManager>);

        fn list_cameras(camera_manager: &CameraManager) -> Vec<CameraPtr>;

        //////////////////////////////////////

        type Camera;

        fn id(self: &Camera) -> &CxxString;

        /// Thread safe
        fn acquire(self: Pin<&mut Camera>) -> i32;

        fn release(self: Pin<&mut Camera>) -> i32;

        // Thread safe
        fn generate_camera_configuration(
            camera: Pin<&mut Camera>,
            stream_roles: &[StreamRole],
        ) -> UniquePtr<CameraConfiguration>;

        unsafe fn start(self: Pin<&mut Camera>, control_list: *const ControlList) -> i32;

        unsafe fn configure(self: Pin<&mut Camera>, config: *mut CameraConfiguration) -> i32;

        /// Thread safe
        fn createRequest(self: Pin<&mut Camera>, cookie: u64) -> UniquePtr<Request>;

        /// Thread safe
        unsafe fn queueRequest(self: Pin<&mut Camera>, request: *mut Request) -> i32;

        /// Thread safe
        type RequestCompleteSlot;
        fn camera_connect_request_completed(
            camera: Pin<&mut Camera>,
            handler: fn(&RequestCompleteContext, &Request),
            context: Box<RequestCompleteContext>,
        ) -> UniquePtr<RequestCompleteSlot>;

        fn camera_streams(camera: &Camera) -> Vec<StreamPtr>;

        unsafe fn camera_contains_stream(camera: &Camera, stream: *mut Stream) -> bool;

        //////////////////////////////////////

        type Request;

        fn sequence(self: &Request) -> u32;
        fn cookie(self: &Request) -> u64;

        // May return a negative error.
        unsafe fn addBuffer(
            self: Pin<&mut Request>,
            stream: *const Stream,
            buffer: *mut FrameBuffer,
            fence: UniquePtr<Fence>,
        ) -> i32;

        fn status(self: &Request) -> RequestStatus;

        fn reuse(self: Pin<&mut Request>, flags: RequestReuseFlag);

        /*

        ControlList &controls() { return *controls_; }
        ControlList &metadata() { return *metadata_; }
        const BufferMap &buffers() const { return bufferMap_; }
        int addBuffer(const Stream *stream, FrameBuffer *buffer,
                      std::unique_ptr<Fence> fence = nullptr);
        FrameBuffer *findBuffer(const Stream *stream) const;

        bool hasPendingBuffers() const;

        std::string toString() const;

        */

        //////////////////////////////////////

        type Fence;

        //////////////////////////////////////

        type CameraConfiguration;

        fn at(self: &CameraConfiguration, index: u32) -> &StreamConfiguration;

        #[rust_name = "at_mut"]
        fn at(self: Pin<&mut CameraConfiguration>, index: u32) -> Pin<&mut StreamConfiguration>;

        fn size(self: &CameraConfiguration) -> usize;

        fn validate(self: Pin<&mut CameraConfiguration>) -> CameraConfigurationStatus;

        //////////////////////////////////////

        type StreamConfiguration;

        fn stream_config_pixel_format(config: &StreamConfiguration) -> PixelFormat;
        fn stream_config_set_pixel_format(
            config: Pin<&mut StreamConfiguration>,
            value: PixelFormat,
        );

        fn stream_config_size(config: &StreamConfiguration) -> Size;
        fn stream_config_set_size(config: Pin<&mut StreamConfiguration>, value: Size);

        fn stream_config_stride(config: &StreamConfiguration) -> u32;
        fn stream_config_set_stride(config: Pin<&mut StreamConfiguration>, value: u32);

        fn stream_config_frame_size(config: &StreamConfiguration) -> u32;
        fn stream_config_set_frame_size(config: Pin<&mut StreamConfiguration>, value: u32);

        fn stream_config_buffer_count(config: &StreamConfiguration) -> u32;
        fn stream_config_set_buffer_count(config: Pin<&mut StreamConfiguration>, value: u32);

        fn stream_config_to_string(config: &StreamConfiguration) -> String;

        fn stream(self: &StreamConfiguration) -> *mut Stream;

        fn formats(self: &StreamConfiguration) -> &StreamFormats;

        //////////////////////////////////////

        type Stream;

        //////////////////////////////////////

        fn pixel_format_to_string(format: &PixelFormat) -> String;

        //////////////////////////////////////

        type StreamFormats;

        fn stream_formats_pixelformats(stream_formats: &StreamFormats) -> Vec<PixelFormatWrap>;

        fn stream_formats_sizes(
            stream_formats: &StreamFormats,
            pixelformat: &PixelFormat,
        ) -> Vec<SizeWrap>;

        fn range(self: &StreamFormats, pixelformat: &PixelFormat) -> SizeRange;

        //////////////////////////////////////

        type ControlList;

        // ControlId { id(), name(), type() }

        //////////////////////////////////////

        type FrameBuffer;

        fn frame_buffer_planes(buffer: &FrameBuffer) -> Vec<FrameBufferPlane>;

        fn frame_buffer_metadata(buffer: &FrameBuffer) -> FrameMetadata;

        //////////////////////////////////////

        type FrameBufferAllocator;

        fn new_frame_buffer_allocator(camera: SharedPtr<Camera>)
            -> UniquePtr<FrameBufferAllocator>;

        unsafe fn allocate(self: Pin<&mut FrameBufferAllocator>, stream: *mut Stream) -> i32;

        unsafe fn free(self: Pin<&mut FrameBufferAllocator>, stream: *mut Stream) -> i32;

        unsafe fn get_allocated_frame_buffers(
            allocator: &FrameBufferAllocator,
            stream: *mut Stream,
        ) -> Vec<FrameBufferPtr>;

        //////////////////////////////////////

    }
}
