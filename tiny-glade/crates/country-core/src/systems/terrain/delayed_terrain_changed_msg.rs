//! B 0x140aabdd0. At most one pair of CPU copies per frame; expiration is a
//! two-frame software delay. The heights come from the render thread's
//! published readbacks (render::gpu_readback), never from the live mapping:
//! a due request waits until a copy from the issuing frame (or later) arrives.
pub use crate::systems::render_loop::compute_passes::read_terrain_data::ScheduledTerrainReadback;
use crate::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::gpu_readback::GpuReadbacks,
    resources::terrain_height::{TerrainHeightSsbo, TerrainHeightsData, VisualTerrainHeightSsbo},
};
use bevy_ecs::prelude::*;

/// Pending copies: remaining software delay and the render frame whose GPU
/// copy the request was issued in (the published data must be at least that new).
#[derive(Default)]
pub struct DelayedUpdateWallsMsgBuffer(pub Vec<(u32, u64)>);
#[derive(Event, Clone, Copy, Debug)]
pub struct OnTerrainChange;

pub fn delayed_terrain_changed_msg(
    library: Res<AssetSsboLibrary>,
    mut readback: ResMut<ScheduledTerrainReadback>,
    mut delayed: Local<DelayedUpdateWallsMsgBuffer>,
    mut changed: EventWriter<OnTerrainChange>,
    mut authority: ResMut<TerrainHeightsData<TerrainHeightSsbo>>,
    mut visual: ResMut<TerrainHeightsData<VisualTerrainHeightSsbo>>,
    readbacks: Option<Res<GpuReadbacks>>,
) {
    let readbacks = readbacks.as_deref();
    let mut copied = false;
    for (delay, after_frame) in &mut delayed.0 {
        *delay = delay.wrapping_sub(1);
        if *delay == 0 && !copied {
            library.get::<TerrainHeightSsbo>().expect("TerrainHeightSsbo is missing");
            library.get::<VisualTerrainHeightSsbo>().expect("VisualTerrainHeightSsbo is missing");
            let after = *after_frame;
            fn fresh<T>(view: Option<crate::render::gpu_readback::ReadbackView<'_, T>>, after: u64) -> Option<&[T]> {
                view.filter(|view| view.frame >= after).map(|view| view.data)
            }
            let (Some(heights), Some(visual_heights)) = (
                fresh(readbacks.and_then(GpuReadbacks::get::<TerrainHeightSsbo>), after),
                fresh(readbacks.and_then(GpuReadbacks::get::<VisualTerrainHeightSsbo>), after),
            ) else {
                // Keep the due request live: the old fixed delay is not a fence.
                *delay = 1;
                continue;
            };
            authority.copy_from_gpu(bytemuck::cast_slice(heights));
            // Original sends between the two copies, regardless of pixel diff.
            changed.write(OnTerrainChange);
            visual.copy_from_gpu(bytemuck::cast_slice(visual_heights));
            copied = true;
        }
    }
    if std::mem::take(&mut readback.copy_issued) {
        // copy_issued arrives with the submission acknowledgment of the frame
        // that issued the GPU copy; submitted_frame is that frame's number.
        let frame = readbacks.map_or(0, GpuReadbacks::submitted_frame);
        delayed.0.push((2, frame));
    }
    delayed.0.retain(|(delay, _)| *delay != 0);
}
