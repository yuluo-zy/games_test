//! B full init_populate_ssbo_library@140909e80 (2540 bytes).
//! Original function lived in resources; current startup module owns this ECS
//! producer. Preserve registration order, per-type constructors and actual
//! mapped cursor event. Canonical type/factory corrections are shared with the resource owners;
//! this producer contains no surrogate registrations or skipped entries.

use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::{device::GraphicsDeviceResource, ssbo::rhapsody_boundary::GraphicsDevice},
    resources::mouse_capture::CustomCursorSsboCreatedCmd,
};
use bevy_ecs::prelude::{EventWriter, Res};

/// Original ECS inputs: Res<Device>, Res<Library>, EventWriter<CreatedCmd>.
/// Installed only in the original gameplay PreStartup tuple, once per App.
pub fn init_populate_ssbo_library(
    device: Res<GraphicsDeviceResource>,
    library: Res<AssetSsboLibrary>,
    mut created: EventWriter<CustomCursorSsboCreatedCmd>,
) {
    created.write(populate_ssbo_library(&library, device.device()));
}

/// Callable same-library producer. Failure propagates through each original
/// factory's existing panic/error contract; cursor publication occurs only
/// after all preceding real registrations and a valid actual mapping.
pub fn populate_ssbo_library(
    library: &AssetSsboLibrary,
    device: &dyn GraphicsDevice,
) -> CustomCursorSsboCreatedCmd {
    // 1: native call@140909eb7, add_new@0x1412bf670.
    library.add_new::<crate::geometry::instanced_wall::BrickTransformSsbo>(device);
    // 2: native call@140909ec7, add_new@0x1412c3050.
    library.add_new::<crate::geometry::instanced_wall::BrickDrawListSsbo>(device);
    // 3: native call@140909ed7, add_new@0x1412bc6f0.
    library.add_new::<crate::geometry::instanced_wall::BrickPrevVisibilityBitmaskSsbo>(device);
    // 4: native call@140909ee7, add_new@0x1412c3d00.
    library.add_new::<crate::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo>(device);
    // 5: native call@140909ef7, add_new@0x1412c05d0.
    library.add_new::<crate::render::normals_debug::NormalDebugSsbo>(device);
    // 6: native call@140909f07, add_new@0x1412c4c00.
    library.add_new::<crate::resources::compute_mouse_depth_value::MouseDepthValueSsbo>(device);
    // 7: native call@140909f17, add_new@0x1412c6f30.
    library.add_new::<crate::resources::world_raster_defs::PathRasterStagingSsbo>(device);
    // 8: native call@140909f27, add_new@0x1412bf3f0.
    library.add_new::<crate::resources::world_raster_defs::Cobblestone1RasterStagingSsbo>(device);
    // 9: native call@140909f37, add_new@0x1412c4e80.
    library.add_new::<crate::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>(device);
    // 10: native call@140909f47, add_new@0x1412bdaf0.
    library.add_new::<crate::resources::world_raster_defs::Cobblestone3RasterStagingSsbo>(device);
    // 11: native call@140909f57, add_new@0x1412bfba0.
    library.add_new::<crate::resources::roof_tile_ssbo::RoofTileSsbo>(device);
    // 12: native call@140909f67, add_new@0x1412bdd70.
    library.add_new::<crate::resources::roof_tile_ssbo::RoofSpireSsbo>(device);
    // 13: native call@140909f77, add_new@0x1412c6a30.
    library.add_new::<crate::resources::terrain_height::TerrainHeightSsbo>(device);
    // 14: native call@140909f87, add_new@0x1412c5b00.
    library.add_new::<crate::resources::terrain_height::VisualTerrainHeightSsbo>(device);
    // 15: native call@140909f97, add_new@0x1412be9f0.
    library.add_new::<crate::resources::terrain_rock_mask::TerrainRockMaskSsbo>(device);
    // 16: native call@140909fa7, add_new@0x1412c2650.
    library.add_new::<crate::resources::roofs::roof_buffers::RoofSupportBeamSsbo>(device);
    // 17: native call@140909fb7, add_new@0x1412c6000.
    library.add_new::<crate::resources::roofs::roof_buffers::WoodenPlankSsbo>(device);
    // 18: native call@140909fc7, add_new@0x1412c0850.
    library.add_new::<crate::systems::audio::terrain_stairs_audio::StairsCounterSsbo>(device);
    // 19: native call@140909fd7, add_new@0x1412c0ad0.
    library.add_new::<crate::systems::render_loop::resources::critters::ButterflySsbo>(device);
    // 20: native call@140909fe7, add_new@0x1412c1c50.
    library.add_new::<crate::systems::render_loop::resources::critters::FireflySsbo>(device);
    // 21: native call@140909ff7, add_new@0x1412bc470.
    library
        .add_new::<crate::systems::render_loop::resources::critters::FireflyVisibilitySsbo>(device);
    // 22: native call@14090a007, add_new@0x1412bc970.
    library.add_new::<crate::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo>(device);
    // 23: native call@14090a017, add_new@0x1412bf920.
    library.add_new::<crate::systems::ivy::ivy_send_to_gpu_leaf::IvyFlowerSsbo>(device);
    // 24: native call@14090a027, add_new@0x1412c28d0.
    library.add_new::<crate::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo>(device);
    // 25: native call@14090a037, add_new@0x1412bf170.
    library.add_new::<crate::resources::walls::dormers::DormerHoleSsbo>(device);
    // 26: native call@14090a047, add_new@0x1412c3f80.
    library.add_new::<crate::resources::walls::dormers::DormerHoleCountPerCellSsbo>(device);
    // 27: native call@14090a057, add_new@0x1412c2150.
    library.add_new::<crate::systems::render_loop::indirect_params::DispatchIndirectBuffer>(device);
    // 28: native call@14090a067, add_new@0x1412c6530.
    library
        .add_new::<crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>(device);
    // 29: native call@14090a077, add_new@0x1412be270.
    library.add_new::<crate::systems::render_loop::indirect_params::DrawIndirectBuffer>(device);
    // 30: native call@14090a087, add_new@0x1412bd5f0.
    library.add_new::<crate::systems::render_loop::indirect_params::AtomicCounterBuffer>(device);
    // 31: native call@14090a097, add_new@0x1412bec70.
    library
        .add_new::<crate::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo>(
            device,
        );
    // 32: native call@14090a0a7, add_new@0x1412bcbf0.
    library.add_new::<crate::systems::render_loop::compute_passes::dof_autofocus::DofFocusReadbackSsbo>(device);
    // 33: native call@14090a0b7, add_new@0x1412c19d0.
    library
        .add_new::<crate::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo>(device);
    // 34: native call@14090a0c7, add_new@0x1412bfe50.
    library.add_new::<crate::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo>(device);
    // 35: native call@14090a0d7, add_new@0x1412c37d0.
    library.add_new::<crate::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>(
        device,
    );
    // 36: native call@14090a0e7, add_new@0x1412c3550.
    library.add_new::<crate::resources::rt::RtTrianglesSsbo>(device);
    // 37: native call@14090a0f7, add_new@0x1412c00d0.
    library.add_new::<crate::resources::rt::RtTriangleMetaSsbo>(device);
    // 38: native call@14090a107, add_new@0x1412c0d50.
    library.add_new::<crate::resources::rt::RtBvhSsbo>(device);
    // 39: native call@14090a117, add_new@0x1412c5880.
    library
        .add_new::<crate::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>(
            device,
        );
    // 40: native call@14090a127, add_new@0x1412c67b0.
    library
        .add_new::<crate::systems::render_loop::compute_passes::leaf_particles::LeafParticleSsbo>(
            device,
        );
    // 41: native call@14090a137, add_new@0x1412c62b0.
    library.add_new::<crate::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo>(device);
    // 42: native call@14090a147, add_new@0x1412bd370.
    library
        .add_new::<crate::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo>(
            device,
        );
    // 43: native call@14090a157, add_new@0x1412c2b50.
    library.add_new::<crate::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo>(device);
    // 44: native call@14090a167, add_new@0x1412be4f0.
    library
        .add_new::<crate::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo>(device);
    // 45: native call@14090a177, add_new@0x1412c5100.
    library.add_new::<crate::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo>(device);
    // 46: native call@14090a187, add_new@0x1412c0fd0.
    library.add_new::<crate::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo>(device);
    // 47: native call@14090a197, add_new@0x1412c32d0.
    library
        .add_new::<crate::systems::render_loop::compute_passes::compute_water::LilypadSsbo>(device);
    // 48: native call@14090a1a7, add_new@0x1412c4700.
    library
        .add_new::<crate::systems::render_loop::compute_passes::compute_water::LilypadFlowerSsbo>(
            device,
        );
    // 49: native call@14090a1b7, add_new@0x1412c5d80.
    library.add_new::<crate::systems::render_loop::compute_passes::compute_water::ReedSsbo>(device);
    // Current unique Lantern definition; no parallel original-path type.
    // 50: native call@14090a1c7, add_new@0x1412c14d0.
    library.add_new::<crate::resources::render_ssbo::LanternSsbo>(device);
    // 51: native call@14090a1d7, add_new@0x1412bce70.
    library
        .add_new::<crate::systems::render_loop::compute_passes::compute_garden::BushSsbo>(device);
    // 52: native call@14090a1e7, add_new@0x1412c1250.
    library
        .add_new::<crate::systems::render_loop::compute_passes::compute_garden::FlowerSsbo>(device);
    // 53: native call@14090a1f7, add_new@0x1412c4480.
    library.add_new::<crate::systems::render_loop::compute_passes::compute_garden::Flower03Ssbo>(
        device,
    );
    // 54: native call@14090a207, add_new@0x1412c0350.
    library.add_new::<crate::systems::render_loop::compute_passes::rasterize_buildings::GroundedBuildingsSsbo>(device);
    // 55: native call@14090a217, add_new@0x1412c23d0.
    library.add_new::<crate::systems::render_loop::compute_passes::spawn_boats::BoatSsbo>(device);
    // 56: native call@14090a227, add_new@0x1412c4980.
    library
        .add_new::<crate::systems::render_loop::compute_passes::compute_garden::GardenStoneSsbo>(
            device,
        );
    // 57: native call@14090a237, add_new@0x1412be770.
    library.add_new::<crate::resources::world_raster_defs::WaterRasterStagingSsbo>(device);
    // 58: native call@14090a247, add_new@0x1412c6cb0.
    library.add_new::<crate::resources::world_raster_defs::GardenRasterStagingSsbo>(device);
    // 59: native call@14090a257, add_new@0x1412c2dd0.
    library.add_new::<crate::resources::water_mask::WaterMaskSsbo>(device);
    // 60: native call@14090a267, add_new@0x1412c1750.
    library.add_new::<crate::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo>(
        device,
    );
    // 61: native call@14090a277, add_new@0x1412c5600.
    library.add_new::<crate::resources::garden_mask::GardenMaskSsbo>(device);
    // 62: native call@14090a287, add_new@0x1412bd0f0.
    library.add_new::<crate::render::nani::buffers::NaniDrawListLengthSsbo>(device);
    // 63: native call@14090a297, add_new@0x1412bdff0.
    library.add_new::<crate::render::nani::buffers::NaniDrawListOffsetDefaults>(device);
    // 64: native call@14090a2a7, add_new@0x1412c5380.
    library.add_new::<crate::render::nani::buffers::NaniInstanceIndirectionAllocatorSsbo>(device);
    // 65: native call@14090a2b7, add_new@0x1412bd870.
    library.add_new::<crate::render::nani::buffers::NaniFilteredInstanceAllocatorSsbo>(device);
    // 66: native call@14090a2c7, add_new@0x1412c1ed0.
    library.add_new::<crate::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo>(device);
    // 67: native call@14090a2d7, add_new@0x1412c4200.
    library.add_new::<crate::resources::water::RippleSourceSsbo>(device);
    // 68: native call@14090a2e7, add_new@0x1412c3a50.
    library.add_new::<crate::resources::sparkle::SparkleSsbo>(device);

    // The three original add_new_with_data calls are separated from the fixed
    // Nani registrations (62..65) by BushCpu/RippleSource/Sparkle (66..68).
    use crate::render::nani::buffers::{
        NaniDrawListSsbo, NaniInstanceIndirectionSsbo, NaniListDrawCmdOffsets,
        nani_calculate_draw_list_params,
    };
    let params = nani_calculate_draw_list_params();
    // 69: call@14090a435, zero20B records[params+0x64].
    let draw_lists = vec![NaniDrawListSsbo::default(); params.total_max_instances as usize];
    library.add_new_with_data(device, &draw_lists);
    // 70: call@14090a470, original params second Vec, 108 u32 offsets.
    let offsets: Vec<_> = params
        .max_instance_offsets
        .iter()
        .map(|&offset| NaniListDrawCmdOffsets([offset]))
        .collect();
    library.add_new_with_data(device, &offsets);
    // 71: call@14090a506, zero u32 records[params+0x60].
    let indirection =
        vec![NaniInstanceIndirectionSsbo::default(); params.total_cmd_offset as usize];
    library.add_new_with_data(device, &indirection);
    // 72: call@14090a532, then original lookup/downcast and real map check.
    // The existing helper registers precisely this one canonical type and
    // returns an actual NonNull mapped address, retaining the library owner.
    crate::systems::render_loop::draw_passes::custom_cursor_pass::create_custom_cursor_ssbo(
        library, device,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::ssbo::tests::MockDevice;
    use crate::systems::render_loop::draw_passes::ui_2d_pass::{ShapeStreamSsbo, TextPrimSsbo};

    #[test]
    fn full_original_producer_registers_all_canonical_entries_and_returns_mapped_cursor() {
        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        let created = populate_ssbo_library(&library, &device);
        assert_eq!(library.snapshot_graph_leases().len(), 72);
        assert_eq!(created.0.as_ptr() as usize % 4, 0);
        assert_eq!(library.get_buffer::<TextPrimSsbo>().unwrap().byte_size, 0x600000);
        assert_eq!(library.get_buffer::<ShapeStreamSsbo>().unwrap().byte_size, 0x400000);
        assert_eq!(library.get_buffer::<crate::systems::audio::terrain_stairs_audio::StairsCounterSsbo>().unwrap().byte_size, 4);
        assert_eq!(library.get_buffer::<crate::resources::walls::dormers::DormerHoleSsbo>().unwrap().byte_size, 0x83d60);
        assert_eq!(library.get_buffer::<crate::resources::walls::dormers::DormerHoleCountPerCellSsbo>().unwrap().byte_size, 0x384);
        let calls = device.created.borrow();
        assert!(calls.iter().all(|(desc, _, _)| desc.size > 0), "no zero-sized placeholder buffers");
        eprintln!("original full72: {} physical buffer creates, {} total bytes",
            calls.len(), calls.iter().map(|(desc, _, _)| desc.size as u64).sum::<u64>());
    }
}
