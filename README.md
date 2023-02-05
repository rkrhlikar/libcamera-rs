# Safe Rust Bindings for ibcamera

This is still under development, but is able to open a camera and send/receive requests/responses.

## Safety

The raw libcamera APIs require a lot of careful management of memory ownership in order to use
correctly. To avoid exposing this to Rust users, we internally keep dependencies alive through
`Arc` references to them. For example, the `Camera` struct contains an `Arc<CameraManager>` to
ensure that no `Camera`s exist after the `CameraManager` has been shutdown. More specific
implementation notes are provided below.
