use nyanko_engine::graphics::shader::{fs, vs, Shader};
use vulkano::device::Device;
use vulkano::device::DeviceCreateInfo;
use vulkano::device::QueueCreateInfo;
use vulkano::instance::Instance;

#[test]
fn test_shader_compilation() {
    // Create instance
    let instance = Instance::new(Default::default()).unwrap();

    // Get physical device
    let physical = instance
        .enumerate_physical_devices()
        .unwrap()
        .next()
        .unwrap();

    // Create device
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .unwrap();

    let (device, _) = Device::new(
        physical,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            ..Default::default()
        },
    )
    .unwrap();

    // Test vertex shader
    let vert = vs::load(device.clone()).unwrap();
    assert!(vert.entry_point("main").is_some());

    // Test fragment shader
    let frag = fs::load(device.clone()).unwrap();
    assert!(frag.entry_point("main").is_some());
}
