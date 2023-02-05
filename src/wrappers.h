#pragma once

#include <memory>

#include "libcamera/libcamera.h"
#include "rust/cxx.h"

namespace libcamera {
class RequestCompleteSlot;
}

// Rust types and generated code. Must be after the pure C++ imports.
#include "libcamera/src/ffi.rs.h"

namespace libcamera {

std::unique_ptr<CameraManager> new_camera_manager();

rust::Vec<CameraPtr> list_cameras(const CameraManager &camera_manager);

std::unique_ptr<CameraConfiguration> generate_camera_configuration(
    Camera &camera, rust::Slice<const ::StreamRole> stream_roles);

::CameraConfigurationStatus validate_camera_config(CameraConfiguration &config);

std::unique_ptr<FrameBufferAllocator> new_frame_buffer_allocator(
    std::shared_ptr<Camera> camera);

rust::String stream_config_to_string(const StreamConfiguration &config);

unsigned int stream_config_buffer_count(const StreamConfiguration &config);
void stream_config_set_buffer_count(StreamConfiguration &config,
                                    unsigned int value);

rust::Vec<FrameBufferPtr> get_allocated_frame_buffers(
    const FrameBufferAllocator &allocator, Stream *stream);

rust::Vec<::FrameBufferPlane> frame_buffer_planes(const FrameBuffer &buffer);

::FrameMetadata frame_buffer_metadata(const FrameBuffer &buffer);

rust::String pixel_format_to_string(const PixelFormat &format);

::RequestStatus request_status(const Request &request);

class RequestCompleteSlot {
 public:
  RequestCompleteSlot(
      Signal<Request *> *signal,
      rust::Fn<void(const RequestCompleteContext &, const Request &)> handler,
      rust::Box<::RequestCompleteContext> context)
      : signal_(signal),
        handler_(std::move(handler)),
        context_(std::move(context)) {
    signal_->connect(this, &RequestCompleteSlot::signaled);
  }

  ~RequestCompleteSlot() {
    signal_->disconnect(this, &RequestCompleteSlot::signaled);
  }

 private:
  void signaled(Request *request) { (*handler_)(*context_, *request); }

  Signal<Request *> *signal_;
  rust::Fn<void(const RequestCompleteContext &, const Request &)> handler_;
  rust::Box<::RequestCompleteContext> context_;
};

std::unique_ptr<RequestCompleteSlot> camera_connect_request_completed(
    Camera &camera,
    rust::Fn<void(const RequestCompleteContext &, const Request &)> handler,
    rust::Box<::RequestCompleteContext> context);

}  // namespace libcamera
