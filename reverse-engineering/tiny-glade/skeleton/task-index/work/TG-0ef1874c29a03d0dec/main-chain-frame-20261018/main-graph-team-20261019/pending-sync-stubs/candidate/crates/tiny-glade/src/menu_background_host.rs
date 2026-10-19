//! Temporary compile seam for a source file awaiting synchronization.
//! No menu texture owner, frame or successful result is fabricated here.
use bevy_app::App;
use bevy_ecs::world::World;
use country_core::render::pass_builder::framebuffer_binding::AttachmentTexture;
use crate::{render_bridge::CoreGraphFrame, render_host::HostTables};

const PENDING: &str = "ORIGINAL_IMPLEMENTATION_PENDING_SYNC: menu_background_host.rs";

pub fn install(_app: &mut App, _hosts: HostTables) {
    panic!("{PENDING}: install");
}

pub fn render(
    _frame: &mut CoreGraphFrame<'_>, _world: &mut World, _target: AttachmentTexture,
) -> rhapsody::Result<()> {
    Err(rhapsody::Error::unsupported(PENDING))
}
