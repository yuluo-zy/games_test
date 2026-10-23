//! B 0x140aabdd0. At most one pair of CPU copies per frame; expiration is a
//! two-frame software delay; actual completion is polled before CPU publication.
pub use crate::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
};
use bevy_ecs::prelude::*;

#[derive(Default)]
pub struct DelayedUpdateWallsMsgBuffer(pub Vec<u32>);
#[derive(Default)]
pub struct TerrainReadbackScratch {
    authority: Vec<u16>,
    visual: Vec<u16>,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct OnTerrainChange;

pub fn delayed_terrain_changed_msg(
    library: Res<AssetSsboLibrary>,
    mut readback: ResMut<ScheduledTerrainReadback>,
    mut delayed: Local<DelayedUpdateWallsMsgBuffer>,
    mut scratch: Local<TerrainReadbackScratch>,
    mut changed: EventWriter<OnTerrainChange>,
    mut authority: ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
    mut visual: ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>,
) {
    let mut copied = false;
    for delay in &mut delayed.0 {
        *delay = delay.wrapping_sub(1);
        if *delay == 0 && !copied {
            let height = library
                .get::<TerrainHeightSsbo>()
                .expect("TerrainHeightSsbo is missing");
            let authority_ready = height.try_cpu_read(|samples| {
                scratch.authority.clear();
                scratch.authority.extend_from_slice(bytemuck::cast_slice(samples));
            }).expect("terrain height readback failed").is_some();
            let height = library
                .get::<VisualTerrainHeightSsbo>()
                .expect("VisualTerrainHeightSsbo is missing");
            let visual_ready = authority_ready && height.try_cpu_read(|samples| {
                scratch.visual.clear();
                scratch.visual.extend_from_slice(bytemuck::cast_slice(samples));
            }).expect("visual terrain height readback failed").is_some();
            if !visual_ready {
                // Keep the due request live: the old fixed delay is not a fence.
                *delay = 1;
                continue;
            }
            authority.copy_from_gpu(&scratch.authority);
            // Original sends between the two copies, regardless of pixel diff.
            changed.write(OnTerrainChange);
            visual.copy_from_gpu(&scratch.visual);
            copied = true;
        }
    }
    if std::mem::take(&mut readback.copy_issued) {
        delayed.0.push(2);
    }
    delayed.0.retain(|delay| *delay != 0);
}
