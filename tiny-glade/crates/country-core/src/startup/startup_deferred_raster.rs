//! B startup_deferred_raster @0x140a97a30 (310 B), original four material shaders.
//! GraphicsDevice/ShaderWatch adaptation uses the existing ShaderBackend; it
//! does not create another library, recover a separate driver, or choose a stage.
use crate::{
    asset_libraries::shader_library::AssetShaderLibrary, render::shader::boundary::ShaderBackend,
    systems::render_loop::draw_passes::deferred_raster::DeferredRasterResources,
};

/// Declaration order and destination material index are separately preserved.
pub const SHADERS: [(usize, &str, &str); 4] = [
    (
        1,
        "/deferred/grass_light_sun_sky.raster.hlsl",
        "grass light sun sky",
    ),
    (
        3,
        "/deferred/canopy_light_sun_sky.raster.hlsl",
        "canopy light sun sky",
    ),
    (
        0,
        "/deferred/solid_light_sun_sky.raster.hlsl",
        "solid light sun sky",
    ),
    (
        2,
        "/deferred/plant_light_sun_sky.raster.hlsl",
        "plant light sun sky",
    ),
];

/// Mutates the existing per-App resource. The native consumer writes a slot
/// after each successful load, and does not modify Forward's slot 4.
pub fn startup_deferred_raster(
    library: &AssetShaderLibrary,
    backend: &dyn ShaderBackend,
    resources: &mut DeferredRasterResources,
) {
    for (index, path, name) in SHADERS {
        resources.shaders[index] = Some(super::load_shaders::load_shader_into_library(
            library, backend, path, name,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::test_device::TestDevice;

    #[test]
    fn original_deferred_startup_registers_four_ordered_materials_and_preserves_forward() {
        for keep_forward in [false, true] {
            let library = AssetShaderLibrary::new();
            let device = TestDevice::default();
            let mut resources = DeferredRasterResources::default();
            assert!(resources.shaders.iter().all(Option::is_none));
            let forward = keep_forward.then(|| {
                crate::startup::load_shaders::load_shader_into_library(
                    &library,
                    &device,
                    "/test/forward.raster.hlsl",
                    "test forward",
                )
            });
            resources.shaders[4] = forward;
            startup_deferred_raster(&library, &device, &mut resources);
            assert_eq!(resources.shaders[4], forward);
            assert_eq!(library.raster_shaders.len(), 4 + usize::from(keep_forward));
            let mut declaration_ids = Vec::new();
            for (index, path, name) in SHADERS {
                let handle = resources.shaders[index].expect("actual registered material handle");
                assert_eq!(library.get_raster_handle_by_name(name), Some(handle));
                let program = library.get_raster(handle).expect("program in same library");
                assert_eq!(program.name, name);
                assert_eq!(program.desc.path, path);
                assert!(program.desc.entry.is_empty());
                assert!(program.desc.defines.is_empty());
                // Existing TestDevice records each requested artifact path in
                // the compiled payload. Both original variants must be loaded.
                for variant in &program.variants {
                    assert_eq!(variant.compiled.0, path.as_bytes());
                }
                declaration_ids.push(handle.id().raw());
            }
            // The real current library assigns increasing IDs. Interleaved
            // tests may leave gaps, so no global ID values are assumed.
            assert!(declaration_ids.windows(2).all(|ids| ids[0] < ids[1]));
        }
    }
}
