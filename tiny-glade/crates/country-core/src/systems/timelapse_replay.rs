//! B timelapse control event used by the actual history entry consumer.
//! No playback consumer is claimed by this type declaration.

/// Original history_process arg8 writes EventId only, stride8, at
/// 140af5df8..5e37 when clearing/replacing pending history commands.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InterruptTimelapse;
