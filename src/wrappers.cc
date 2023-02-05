#include "wrappers.h"

namespace libcamera {

std::unique_ptr<CameraManager> new_camera_manager() {
  return std::make_unique<CameraManager>();
}

rust::Vec<CameraPtr> list_cameras(const CameraManager &camera_manager) {
  auto cameras = camera_manager.cameras();

  rust::Vec<CameraPtr> out;
  out.reserve(cameras.size());
  for (const auto &camera : cameras) {
    out.push_back(CameraPtr{.camera = camera});
  }

  return out;
}

std::unique_ptr<CameraConfiguration> generate_camera_configuration(
    Camera &camera, rust::Slice<const ::StreamRole> stream_roles) {
  StreamRoles roles;
  for (const auto &role_rs : stream_roles) {
    roles.push_back(static_cast<StreamRole>(role_rs));
  }

  auto config = camera.generateConfiguration(roles);

  return config;
}

::CameraConfigurationStatus validate_camera_config(
    CameraConfiguration &config) {
  return static_cast<::CameraConfigurationStatus>(config.validate());
}

std::unique_ptr<FrameBufferAllocator> new_frame_buffer_allocator(
    std::shared_ptr<Camera> camera) {
  return std::make_unique<FrameBufferAllocator>(camera);
}

rust::String stream_config_to_string(const StreamConfiguration &config) {
  return rust::String(config.toString());
}

unsigned int stream_config_buffer_count(const StreamConfiguration &config) {
  return config.bufferCount;
}

void stream_config_set_buffer_count(StreamConfiguration &config,
                                    unsigned int value) {
  config.bufferCount = value;
}

rust::Vec<FrameBufferPtr> get_allocated_frame_buffers(
    const FrameBufferAllocator &allocator, Stream *stream) {
  const auto &buffers = allocator.buffers(stream);

  rust::Vec<FrameBufferPtr> out;
  out.reserve(buffers.size());

  for (const auto &buffer : buffers) {
    out.push_back(FrameBufferPtr{.buffer = buffer.get()});
  }

  return out;
}

rust::Vec<::FrameBufferPlane> frame_buffer_planes(const FrameBuffer &buffer) {
  rust::Vec<::FrameBufferPlane> out;

  for (const auto &plane : buffer.planes()) {
    out.push_back(
        ::FrameBufferPlane{.fd = static_cast<unsigned int>(plane.fd.get()),
                           .offset = plane.offset,
                           .length = plane.length});
  }

  return out;
}

::FrameMetadata frame_buffer_metadata(const FrameBuffer &buffer) {
  const auto &meta = buffer.metadata();

  ::FrameMetadata out{.status = static_cast<::FrameStatus>(meta.status),
                      .sequence = meta.sequence,
                      .timestamp = meta.timestamp,
                      .planes = rust::Vec<::FramePlaneMetadata>()};

  for (const auto &plane : meta.planes()) {
    out.planes.push_back(::FramePlaneMetadata{.bytes_used = plane.bytesused});
  }

  return out;
}

rust::String pixel_format_to_string(const PixelFormat &format) {
  return rust::String(format.toString());
}

::RequestStatus request_status(const Request &request) {
  return static_cast<::RequestStatus>(request.status());
}

std::unique_ptr<RequestCompleteSlot> camera_connect_request_completed(
    Camera &camera,
    rust::Fn<void(const RequestCompleteContext &, const Request &)> handler,
    rust::Box<::RequestCompleteContext> context) {
  auto slot = std::make_unique<RequestCompleteSlot>(
      &camera.requestCompleted, std::move(handler), std::move(context));

  return slot;
}

}  // namespace libcamera
