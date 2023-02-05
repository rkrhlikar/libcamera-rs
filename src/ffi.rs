pub use self::ffi::*;

pub struct RequestCompleteContext {
    pub handler: Box<dyn Fn(&Request)>,
}

#[cxx::bridge]
mod ffi {
    /// A mirror of libcamera::StreamRole
    enum StreamRole {
        Raw,
        StillCapture,
        VideoRecording,
        Viewfinder,
    }

    /// A mirror of libcamera::FrameBuffer::Plane
    #[derive(Debug, Clone, Copy)]
    struct FrameBufferPlane {
        fd: u32,
        offset: u32,
        length: u32,
    }

    #[derive(Debug, Clone, Copy)]
    enum FrameStatus {
        FrameSuccess,
        FrameError,
        FrameCancelled,
    }

    #[derive(Debug, Clone)]
    struct FramePlaneMetadata {
        bytes_used: u32,
    }

    #[derive(Debug, Clone)]
    struct FrameMetadata {
        status: FrameStatus,
        sequence: u32,
        timestamp: u64,
        planes: Vec<FramePlaneMetadata>,
    }

    // Wrapper to work around https://github.com/dtolnay/cxx/issues/741
    struct CameraPtr {
        camera: SharedPtr<Camera>,
    }

    struct FrameBufferPtr {
        buffer: *mut FrameBuffer,
    }

    #[derive(Debug, Clone, Copy)]
    enum RequestStatus {
        RequestPending,
        RequestComplete,
        RequestCancelled,
    }

    enum RequestReuseFlag {
        Default = 0,
        ReuseBuffers = 1,
    }

    #[derive(Debug)]
    enum CameraConfigurationStatus {
        Valid,
        Adjusted,
        Invalid,
    }

    extern "Rust" {
        type RequestCompleteContext;
    }

    #[namespace = "libcamera"]
    unsafe extern "C++" {
        include!("libcamera/src/wrappers.h");

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

        fn request_status(request: &Request) -> RequestStatus;

        /*
        void reuse(ReuseFlag flags = Default);

        ControlList &controls() { return *controls_; }
        ControlList &metadata() { return *metadata_; }
        const BufferMap &buffers() const { return bufferMap_; }
        int addBuffer(const Stream *stream, FrameBuffer *buffer,
                      std::unique_ptr<Fence> fence = nullptr);
        FrameBuffer *findBuffer(const Stream *stream) const;

        Status status() const { return status_; }

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

        fn validate_camera_config(
            config: Pin<&mut CameraConfiguration>,
        ) -> CameraConfigurationStatus;

        //////////////////////////////////////

        type StreamConfiguration;

        fn stream_config_set_buffer_count(config: Pin<&mut StreamConfiguration>, value: u32);

        fn stream_config_to_string(config: &StreamConfiguration) -> String;

        fn stream(self: &StreamConfiguration) -> *mut Stream;

        //////////////////////////////////////

        type Stream;

        //////////////////////////////////////

        type PixelFormat;

        fn pixel_format_to_string(format: &PixelFormat) -> String;

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
