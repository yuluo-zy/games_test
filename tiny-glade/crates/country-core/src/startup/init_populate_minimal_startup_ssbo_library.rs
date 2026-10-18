//! Original loading Startup's two canonical SSBO owners.
//!
//! B MinimalStartupPlugin: 140b1ac90 -> add_systems<Startup>@140b65f80
//! -> five-tuple@1413fb240 -> config@1408ae5b0. Its name slice is
//! `country_core::resources::init_populate_minimal_startup_ssbo_library`.
//! Full run_unsafe@1408ac4a0 calls TextPrim@1408ac544 then ShapeStream@1408ac553.
//! Source placement here follows the existing startup module organization;
//! the original consumer semantics and canonical element identities are reused.
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::{device::GraphicsDeviceResource, ssbo::rhapsody_boundary::GraphicsDevice},
    systems::render_loop::draw_passes::ui_2d_pass::{ShapeStreamSsbo, TextPrimSsbo},
};

/// The original minimal subset, not gameplay's full 72-entry producer.
/// Both add_new calls forward None to the actual typed factory: no zero upload.
/// Duplicate registration and device failures keep the library's real errors.
pub fn init_populate_minimal_startup_ssbo_library(
    device: &dyn GraphicsDevice,
    library: &AssetSsboLibrary,
) {
    library.add_new::<TextPrimSsbo>(device);
    library.add_new::<ShapeStreamSsbo>(device);
}

/// Native ECS inputs are the loading App's own device and SSBO library.
pub fn minimal_startup_ssbo_system(
    device: bevy_ecs::prelude::Res<GraphicsDeviceResource>,
    library: bevy_ecs::prelude::Res<AssetSsboLibrary>,
) {
    init_populate_minimal_startup_ssbo_library(device.device(), &library);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;

    #[test]
    fn minimal_loading_startup_creates_only_two_ordered_canonical_owners_without_upload() {
        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        init_populate_minimal_startup_ssbo_library(&device, &library);
        let calls = device.created.borrow();
        assert_eq!(calls.len(), 2);
        for (call, name, size) in [
            (&calls[0], "TextPrim", 0x600000),
            (&calls[1], "ShapeStream", 0x400000),
        ] {
            assert_eq!(call.1, name);
            assert_eq!((call.0.size, call.0.usage, call.0.memory, call.0.alignment),
                (size, 0x22, 1, 4));
            assert!(call.2.is_none(), "minimal Startup must not fabricate initial contents");
        }
        assert_eq!(library.get::<TextPrimSsbo>().unwrap().buffer.byte_size, 0x600000);
        assert_eq!(library.get::<ShapeStreamSsbo>().unwrap().buffer.byte_size, 0x400000);
        assert_eq!(library.snapshot_graph_leases().len(), 2);
    }
}
