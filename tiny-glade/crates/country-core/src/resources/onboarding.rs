//! Payload-free tool-use notification observed in RasterDrawResources.
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug)]
pub struct NotifyHintSystemToolWasJustUsed;
