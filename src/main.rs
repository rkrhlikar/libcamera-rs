use std::num::NonZeroUsize;

use libcamera::Result;
use nix::sys::mman::*;

unsafe fn mmap_plane(plane: libcamera::FrameBufferPlane) -> Result<&'static [u8]> {
    let mem = mmap(
        None,
        NonZeroUsize::new(plane.length as usize).unwrap(),
        ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
        MapFlags::MAP_SHARED,
        plane.fd as i32,
        plane.offset as nix::libc::off_t,
    )?;

    Ok(core::slice::from_raw_parts(
        core::mem::transmute(mem),
        plane.length as usize,
    ))
}

fn main() -> Result<()> {
    let manager = libcamera::CameraManager::create()?;

    let mut cameras = manager.cameras();

    println!("Num Cameras: {}", cameras.len());

    if cameras.len() == 0 {
        return Ok(());
    }

    // TOOD: Ignore ones on Pi that contain "/usb"
    let camera = cameras.pop().unwrap();
    println!("Id: {}", camera.id());

    let camera = camera.acquire()?;
    println!("Acquired!");

    let mut config = camera
        .generate_configuration(&[libcamera::StreamRole::Viewfinder])
        .unwrap();
    assert_eq!(config.stream_configs_len(), 1);

    // Only allocate one buffer per stream.
    config.stream_config_mut(0).set_buffer_count(1);

    assert_eq!(
        config.validate(),
        libcamera::CameraConfigurationStatus::Valid
    );

    let camera = camera.configure(&mut config)?;
    println!("Configured!");

    let mut frame_buffer_allocator = camera.new_frame_buffer_allocator();

    let stream_config = config.stream_config(0);
    println!("Stream: {}", stream_config.to_string());

    let mut frame_buffer = {
        let stream = stream_config.stream().unwrap();
        frame_buffer_allocator.allocate(stream)?;

        // We only requested that one buffer be generated.
        let mut frame_buffers = frame_buffer_allocator.buffers(stream);
        assert_eq!(frame_buffers.len(), 1);

        frame_buffers.pop().unwrap()
    };

    let mut request = camera.create_request(0);
    request.add_buffer(&mut frame_buffer)?;

    let mut memory_buffers = vec![];

    let mut current_segment: Option<libcamera::FrameBufferPlane> = None;
    for plane in frame_buffer.planes() {
        if let Some(p) = &mut current_segment {
            if p.fd == plane.fd && p.offset + p.length == plane.offset {
                p.length += plane.length;
                continue;
            }

            memory_buffers.push(unsafe { mmap_plane(p.clone()) }?);
        }

        current_segment = Some(plane);
    }

    if let Some(p) = current_segment {
        memory_buffers.push(unsafe { mmap_plane(p) }?);
    }

    let camera = camera.start()?;

    camera.queue_request(&mut request)?;

    println!("{:?}", request.status());

    std::thread::sleep(std::time::Duration::from_secs(4));

    println!("{:?}", request.status());

    std::fs::write("image.bin", &memory_buffers[0][..]).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}
