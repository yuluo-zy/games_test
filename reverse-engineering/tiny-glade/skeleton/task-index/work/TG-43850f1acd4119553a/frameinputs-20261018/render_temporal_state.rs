//! Candidate RenderTemporalState contract, original B startup + per-frame.
use bevy_ecs::resource::Resource;
use glam::{Mat4, Vec2};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameCameraMatrices {
    pub projection: Mat4,
    pub inverse_projection: Mat4,
    pub view: Mat4,
    pub inverse_view: Mat4,
}

#[repr(C)]
#[derive(Resource, Clone, Copy, Debug)]
pub struct RenderTemporalState {
    pub camera: FrameCameraMatrices,
    pub previous_jitter: Vec2,
    pub frame_index: u32,
}

impl RenderTemporalState {
    /// Native run_game_thread @14059ef47..efbd: actual camera, jitter=0,index=0.
    /// No identity-camera Default is introduced.
    pub fn new(camera: FrameCameraMatrices) -> Self {
        Self {
            camera,
            previous_jitter: Vec2::ZERO,
            frame_index: 0,
        }
    }

    /// Original GameLoop::simulate_and_render_frame line738 @1404de63a,
    /// immediately after render@1404de5bc returns. Not called by prepare.
    pub fn frame_render_returned(&mut self) {
        self.frame_index = self.frame_index.wrapping_add(1);
    }

    /// Original GameLoop line651 @1404ddc19: generation/extent rebuild.
    pub fn render_resources_rebuilt(&mut self) {
        self.frame_index = 0;
    }
}

const _: () = assert!(core::mem::size_of::<FrameCameraMatrices>() == 0x100);
const _: () = assert!(core::mem::offset_of!(RenderTemporalState, previous_jitter) == 0x100);
const _: () = assert!(core::mem::offset_of!(RenderTemporalState, frame_index) == 0x108);
const _: () = assert!(core::mem::size_of::<RenderTemporalState>() == 0x110);
