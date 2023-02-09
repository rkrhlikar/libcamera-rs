#include "ffi.h"

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
    Camera &camera, rust::Slice<const StreamRole> stream_roles) {
  StreamRoles roles;
  for (const auto &role : stream_roles) {
    roles.push_back(role);
  }

  auto config = camera.generateConfiguration(roles);

  return config;
}

std::unique_ptr<FrameBufferAllocator> new_frame_buffer_allocator(
    std::shared_ptr<Camera> camera) {
  return std::make_unique<FrameBufferAllocator>(camera);
}

rust::String stream_config_to_string(const StreamConfiguration &config) {
  return rust::String(config.toString());
}

PixelFormat stream_config_pixel_format(const StreamConfiguration &config) {
  return config.pixelFormat;
}
void stream_config_set_pixel_format(StreamConfiguration &config,
                                    PixelFormat value) {
  config.pixelFormat = value;
}

Size stream_config_size(const StreamConfiguration &config) {
  return config.size;
}
void stream_config_set_size(StreamConfiguration &config, Size value) {
  config.size = value;
}

unsigned int stream_config_stride(const StreamConfiguration &config) {
  return config.stride;
}
void stream_config_set_stride(StreamConfiguration &config, unsigned int value) {
  config.stride = value;
}

unsigned int stream_config_frame_size(const StreamConfiguration &config) {
  return config.frameSize;
}
void stream_config_set_frame_size(StreamConfiguration &config,
                                  unsigned int value) {
  config.frameSize = value;
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

  ::FrameMetadata out{.status = meta.status,
                      .sequence = meta.sequence,
                      .timestamp = meta.timestamp,
                      .planes = rust::Vec<::FramePlaneMetadataWrap>()};

  for (const auto &plane : meta.planes()) {
    out.planes.push_back({.inner = plane});
  }

  return out;
}

rust::String pixel_format_to_string(const PixelFormat &format) {
  return rust::String(format.toString());
}

rust::Vec<PixelFormatWrap> stream_formats_pixelformats(
    const StreamFormats &stream_formats) {
  auto value = stream_formats.pixelformats();
  rust::Vec<PixelFormatWrap> out;
  for (const auto &v : value) {
    out.push_back({.value = v});
  }
  return out;
}
rust::Vec<SizeWrap> stream_formats_sizes(const StreamFormats &stream_formats,
                                         const PixelFormat &pixelformat) {
  auto value = stream_formats.sizes(pixelformat);
  rust::Vec<SizeWrap> out;
  for (const auto &v : value) {
    out.push_back({.value = v});
  }
  return out;
}

std::unique_ptr<RequestCompleteSlot> camera_connect_request_completed(
    Camera &camera,
    rust::Fn<void(const RequestCompleteContext &, const Request &)> handler,
    rust::Box<::RequestCompleteContext> context) {
  auto slot = std::make_unique<RequestCompleteSlot>(
      &camera.requestCompleted, std::move(handler), std::move(context));

  return slot;
}

rust::Vec<::StreamPtr> camera_streams(const Camera &camera) {
  rust::Vec<::StreamPtr> out;
  for (auto stream : camera.streams()) {
    out.push_back({.stream = stream});
  }
  return out;
}

bool camera_contains_stream(const Camera &camera, Stream *stream) {
  return camera.streams().contains(stream);
}

rust::String request_to_string(const Request &request) {
  return rust::String(request.toString());
}

rust::Vec<::ControlInfoMapEntry> control_info_map_entries(
    const ControlInfoMap &map) {
  rust::Vec<::ControlInfoMapEntry> out;
  for (const auto &[key, value] : map) {
    out.push_back(::ControlInfoMapEntry{.key = *key, .value = value});
  }

  return out;
}

rust::String control_value_get_string(const ControlValue &value) {
  return rust::String(value.get<std::string>());
}

void control_value_set_string(ControlValue &value, const rust::String &s) {
  value.set<std::string>(std::string(s));
}

rust::Vec<rust::String> control_value_get_string_array(
    const ControlValue &value) {
  rust::Vec<rust::String> out;
  for (const auto &v : value.get<Span<const std::string>>()) {
    out.push_back(v);
  }

  return out;
}

rust::Vec<::ControlListEntry> control_list_entries(const ControlList &list) {
  rust::Vec<::ControlListEntry> out;
  for (const auto &[key, value] : list) {
    out.push_back(::ControlListEntry{.key = key, .value = value});
  }
  return out;
}

rust::String control_value_to_string(const ControlValue &value) {
  return rust::String(value.toString());
}

rust::String control_info_to_string(const ControlInfo &info) {
  return rust::String(info.toString());
}

}  // namespace libcamera
