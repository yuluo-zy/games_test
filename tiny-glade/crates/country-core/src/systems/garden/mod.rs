//! B garden drawing and recording, connected to the shared raster and history
//! owners. Original global plugin schedule and pin rendering are not implied.
pub mod draw_garden;
pub mod record_garden_history;
#[cfg(test)]
mod tests;

use bevy_ecs::prelude::*;
use glam::Vec3;
use utils::geometry::Circle2d;

#[derive(Event, Clone, Copy, Debug)]
pub struct GardenDrawEvent {
    pub position: Vec3,
    pub radius: f32,
    pub observed_flag: bool,
}
#[derive(Event, Clone, Copy, Debug)]
pub struct GardenEraseEvent {
    pub position: Vec3,
    pub radius: f32,
    pub observed_flag: bool,
}

/// Garden's known circle/height specialization of ShowPinnedDecoClutter. The
/// full 56-byte event union and its renderer are not restored by this type.
#[derive(Event, Clone, Copy, Debug)]
pub struct GardenPinnedClutterCircle {
    pub circle: Circle2d,
    pub height: f32,
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum GardenCpuStage {
    Draw,
    Record,
}

/// B replay_raster<GardenRasterStagingSsbo, GardenDrawEvent> carries a single
/// draw-event callback, including when replaying an erase stroke. The emitted
/// bool is the replay marker; it must not choose the live erase event channel.
pub fn route_garden_replay_feedback(
    mut feedback: EventReader<crate::systems::history::replay_raster::RasterReplayFeedback>,
    mut draw: EventWriter<GardenDrawEvent>,
) {
    use crate::systems::history::replay_raster::RasterDomain;
    for sample in feedback
        .read()
        .filter(|sample| sample.domain == RasterDomain::Garden)
    {
        draw.write(GardenDrawEvent {
            position: sample.position,
            radius: sample.radius,
            observed_flag: sample.replay,
        });
    }
}

pub fn register_garden_events(world: &mut World) {
    use crate::{
        resources::{
            glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder},
            onboarding::NotifyHintSystemToolWasJustUsed,
        },
        systems::history::{history_process::DebugHistoryMsg, recording::HistoryRecordingFailed},
    };
    world.init_resource::<Events<GardenDrawEvent>>();
    world.init_resource::<Events<GardenEraseEvent>>();
    world.init_resource::<Events<GardenPinnedClutterCircle>>();
    world.init_resource::<Events<record_garden_history::RecordGardenEditToHistoryCmd>>();
    world.init_resource::<Events<DisplayOutOfBorder>>();
    world.init_resource::<Events<FeedbackInputOutOfBorder>>();
    world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
    world.init_resource::<Events<DebugHistoryMsg>>();
    world.init_resource::<Events<HistoryRecordingFailed>>();
    world.init_resource::<Events<crate::systems::history::replay_raster::RasterReplayFeedback>>();
}

/// Runnable local order. Supply real UiState, AppMode, brush/border/cursor,
/// archivist, History, raster, strokes and mapped SSBO resources before running.
pub fn add_garden_cpu_systems<C: Send + Sync + 'static, S: Send + Sync + 'static>(
    schedule: &mut Schedule,
) {
    use crate::systems::history::HistoryCpuStage;
    schedule.configure_sets((GardenCpuStage::Draw, GardenCpuStage::Record).chain());
    schedule.configure_sets(
        GardenCpuStage::Draw
            .after(HistoryCpuStage::Pop)
            .before(HistoryCpuStage::Record),
    );
    schedule.add_systems(draw_garden::draw_garden::<C>.in_set(GardenCpuStage::Draw));
    schedule.add_systems(route_garden_replay_feedback.in_set(GardenCpuStage::Draw));
    schedule.add_systems(
        record_garden_history::record_garden_history::<S>
            .in_set(GardenCpuStage::Record)
            .in_set(HistoryCpuStage::Record),
    );
}
