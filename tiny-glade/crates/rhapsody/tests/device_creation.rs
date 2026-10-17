//! Device creation uses the same negotiated capabilities for every entry point.
use rhapsody::raw::init::GraphicsInstance;

#[test]
fn legacy_selection_uses_the_common_device_negotiation() {
    let Ok(graphics) = GraphicsInstance::new() else {
        return;
    };
    let Ok(devices) = graphics.available_for_surface(None, Default::default()) else {
        return;
    };
    match devices.create_device(&graphics) {
        Ok(device) => {
            assert!(
                device
                    .caps()
                    .enabled()
                    .contains(rhapsody::device::Feature::TimelineSemaphore)
            );
        }
        Err(error) => panic!("默认设备创建失败：{error}"),
    }
}
