//! Original W21/W27 wall batches and their canonical visibility owners.
use bevy_ecs::world::World;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    geometry::instanced_wall::{BrickTransformSsbo, BrickDrawListSsbo,
        BrickPrevVisibilityBitmaskSsbo, BrickNextVisibilityBitmaskSsbo, InstancedWall},
    render::rg_helpers::clear_rg_buffer,
    resources::walls::WallHighlight,
    systems::render_loop::{compute_passes::{generate_hzb::Hzb,
        gpu_generate_draw_lists::{self as draws, WallDrawBatches, WallSsbos}},
        indirect_params::DrawIndexedIndirectBuffer},
};
use crate::{render_bridge::CoreGraphFrame, scene_compute::with_env};

struct GameWallSsbos;
impl WallSsbos for GameWallSsbos {
    type BrickTransform = BrickTransformSsbo;
    type BrickPrevVisibility = BrickPrevVisibilityBitmaskSsbo;
    type BrickNextVisibility = BrickNextVisibilityBitmaskSsbo;
    type BrickDrawList = BrickDrawListSsbo;
    type DrawCommands = DrawIndexedIndirectBuffer;
}

fn missing(name: &str) -> rhapsody::Error {
    rhapsody::Error::invalid_usage(format!("original wall draw lists: missing {name}"))
}

fn batches(world: &World) -> rhapsody::Result<WallDrawBatches> {
    let storage = world.get_resource::<AssetSsboLibrary>().ok_or_else(|| missing("AssetSsboLibrary"))?
        .sparse_draw_view::<BrickTransformSsbo>()
        .ok_or_else(|| missing("BrickTransformSsbo"))?;
    let highlight = world.get_resource::<WallHighlight>().ok_or_else(|| missing("WallHighlight"))?;
    let highlighted_wall = (highlight.ty != 0).then_some(highlight.wall_entity).flatten()
        .and_then(|entity| world.get::<InstancedWall<BrickTransformSsbo>>(entity));
    if let Some(wall) = highlighted_wall {
        let allocation = wall.allocation().ok_or_else(|| missing("highlighted wall sparse allocation"))?;
        let highlighted = storage.allocation_to_draw_multi_instanced(allocation);
        let normal = storage.to_draw_multi_instanced_excluding(allocation);
        Ok(WallDrawBatches { normal, highlighted })
    } else {
        Ok(WallDrawBatches { normal: storage.to_draw_multi_instanced(), highlighted: Vec::new() })
    }
}

fn clear_visibility<T: country_core::asset_libraries::ssbo_library::SsboType>(
    frame: &mut CoreGraphFrame<'_>, world: &World,
) -> rhapsody::Result<()> {
    let library = world.get_resource::<AssetSsboLibrary>().ok_or_else(|| missing("AssetSsboLibrary"))?;
    let buffer = library.get_rg_buffer::<T>(&mut frame.resource_graph());
    let resource = u32::try_from(buffer.resource)
        .map_err(|_| rhapsody::Error::invalid_usage("wall visibility graph handle exceeds u32"))?;
    frame.with_compute(|compute| { clear_rg_buffer(compute, resource); Ok(()) })
}

/// @14098aaf0/1961B. Clear Next and declare normal/highlighted ranges.
/// Selection filters the read-only view without mutating shared visibility.
pub fn generate_wall_draw_lists(
    frame: &mut CoreGraphFrame<'_>, world: &mut World,
) -> rhapsody::Result<()> {
    clear_visibility::<BrickNextVisibilityBitmaskSsbo>(frame, world)?;
    let batches = batches(world)?;
    with_env(frame, world, |env| {
        let pass = draws::generate_wall_draw_lists::<GameWallSsbos>(env, 17, batches);
        env.record(pass); Ok(())
    })
}

/// @14098b2a0/3257B. Reset only the original command counts, collect real
/// batches, clear Prev, and read Next/write Prev for the second culling round.
pub fn generate_wall_draw_lists_pass2(
    frame: &mut CoreGraphFrame<'_>, world: &mut World, hzb: &Hzb,
) -> rhapsody::Result<()> {
    with_env(frame, world, |env| {
        let reset = draws::reset_wall_pass2_inst_counts::<GameWallSsbos>(env, 17);
        env.record(reset); Ok(())
    })?;
    let batches = batches(world)?;
    clear_visibility::<BrickPrevVisibilityBitmaskSsbo>(frame, world)?;
    with_env(frame, world, |env| {
        let pass = draws::wall_draw_lists_pass2::<GameWallSsbos>(
            env, &hzb.texture, hzb.half_extent, 17, batches);
        env.record(pass); Ok(())
    })
}


/// Render-thread extraction: what this module reads from the World.
pub fn render_extract(m: &mut impl country_core::render::extract::RenderExtract) {
    m.asset_library::<AssetSsboLibrary>();
    m.resource::<WallHighlight>();
    m.component_journaled::<InstancedWall<BrickTransformSsbo>>();
}
