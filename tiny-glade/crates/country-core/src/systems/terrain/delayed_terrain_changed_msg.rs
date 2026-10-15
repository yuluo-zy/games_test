//! B 0x140aabdd0. At most one pair of CPU copies per frame; expiration is a
//! two-frame software delay, not a fabricated GPU completion fence.
pub use crate::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
};
use bevy_ecs::prelude::*;

#[derive(Default)]
pub struct DelayedUpdateWallsMsgBuffer(pub Vec<u32>);
#[derive(Event, Clone, Copy, Debug)]
pub struct OnTerrainChange;

pub fn delayed_terrain_changed_msg(
    library: Res<AssetSsboLibrary>,
    mut readback: ResMut<ScheduledTerrainReadback>,
    mut delayed: Local<DelayedUpdateWallsMsgBuffer>,
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
            authority.copy_from_gpu(bytemuck::cast_slice(height.mapped));
            // Original sends between the two copies, regardless of pixel diff.
            changed.write(OnTerrainChange);
            let height = library
                .get::<VisualTerrainHeightSsbo>()
                .expect("VisualTerrainHeightSsbo is missing");
            visual.copy_from_gpu(bytemuck::cast_slice(height.mapped));
            copied = true;
        }
    }
    if std::mem::take(&mut readback.copy_issued) {
        delayed.0.push(2);
    }
    delayed.0.retain(|delay| *delay != 0);
}
