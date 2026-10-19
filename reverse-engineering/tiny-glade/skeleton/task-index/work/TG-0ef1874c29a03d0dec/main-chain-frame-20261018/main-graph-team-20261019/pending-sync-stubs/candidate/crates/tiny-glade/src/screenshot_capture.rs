//! Temporary compile seam for a source file awaiting synchronization.
//! No request queue, texture capture, completion callback or ready flag is made.
use bevy_app::App;
use bevy_ecs::world::World;
use country_core::render::pass_builder::pass_resource_access::RgTexture;
use rhapsody::rg::{GraphId, SessionFrameReceipt};
use crate::{render_bridge::{CoreGraphFrame, CoreRenderBridge}, screenshot::ScreenshotRequest};

const PENDING: &str = "ORIGINAL_IMPLEMENTATION_PENDING_SYNC: screenshot_capture.rs";
fn pending<T>() -> rhapsody::Result<T> { Err(rhapsody::Error::unsupported(PENDING)) }

pub fn install(_app: &mut App) { panic!("{PENDING}: install"); }
pub fn installed(_world: &World) -> bool { panic!("{PENDING}: installed"); }
pub fn enqueue(_world: &mut World, _request: ScreenshotRequest) -> rhapsody::Result<()> { pending() }
pub fn active_capture_purpose(_world: &World) -> u8 { panic!("{PENDING}: active_capture_purpose"); }
pub fn has_requests(_world: &World) -> bool { panic!("{PENDING}: has_requests"); }
pub fn opportunity(
    _frame: &mut CoreGraphFrame<'_>, _world: &mut World, _purpose: u8,
    _target: RgTexture, _sample_count: u8,
) -> rhapsody::Result<()> { pending() }
pub fn submitted(
    _world: &mut World, _bridge: &mut CoreRenderBridge,
    _graph: GraphId, _receipt: &SessionFrameReceipt,
) -> rhapsody::Result<()> { pending() }
pub fn cancel_unsubmitted(_world: &mut World, _graph: GraphId) {
    panic!("{PENDING}: cancel_unsubmitted");
}
pub fn finish(_world: &mut World) -> rhapsody::Result<()> { pending() }
pub fn shutdown(_world: &mut World) -> rhapsody::Result<()> { pending() }
