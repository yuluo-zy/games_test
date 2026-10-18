//! `tiny_glade::rollercoaster` — decorative rollercoaster cart simulation.
//!
//! Recovered VAs: `rollercoaster_update` `0x1403b7f20`,
//! `create_cart_chain` `0x1403be1a0`,
//! `CoasterAudioInstance::new` `0x1403b6880`.
//!
//! `CoasterAudioInstance::new` is fully recovered:
//! ```text
//! if volume < 0.03 { return None }
//! match audio_core.instantiate_audio() {
//!     Ok(instance) => {
//!         instance.set_volume(volume);
//!         for (name, value) in parameters { instance.set_parameter(name, value); }
//!         Some(instance)
//!     }
//!     Err(error) => { log error; None }
//! }
//! ```
//!
//! `rollercoaster_update` and `create_cart_chain` are large
//! spline/physics bodies (26 KB+ decompilation each). Their control flow is not
//! yet reconstructed statement-by-statement; the reasons and blocking data are
//! recorded in `reports/tinyglade/B/RESTORE.md` (`未实现-需重做`).

use anyhow::Result;

/// Audio backend used by the coaster (country-core `AudioCore` in B).
pub trait CoasterAudio {
    type Instance;
    fn instantiate_audio(&mut self) -> Result<Self::Instance>;
    fn set_volume(instance: &mut Self::Instance, volume: f32);
    fn set_parameter(instance: &mut Self::Instance, name: &str, value: f32);
}

/// B `tiny_glade::rollercoaster::CoasterAudioInstance`.
pub struct CoasterAudioInstance<A: CoasterAudio> {
    pub instance: A::Instance,
    pub volume: f32,
}

impl<A: CoasterAudio> CoasterAudioInstance<A> {
    /// Minimum audible volume (`0.03` in the machine code).
    pub const MIN_VOLUME: f32 = 0.03;

    /// B `CoasterAudioInstance::new` (`0x1403b6880`).
    pub fn new(
        audio: &mut A,
        volume: f32,
        parameters: &[(&str, f32)],
    ) -> Option<CoasterAudioInstance<A>> {
        if volume < Self::MIN_VOLUME {
            return None;
        }
        match audio.instantiate_audio() {
            Ok(mut instance) => {
                A::set_volume(&mut instance, volume);
                for (name, value) in parameters {
                    A::set_parameter(&mut instance, name, *value);
                }
                Some(Self { instance, volume })
            }
            Err(error) => {
                log::error!("failed to instantiate coaster audio: {error:#}");
                None
            }
        }
    }
}

/// One cart in the chain.
#[derive(Clone, Debug, Default)]
pub struct Cart {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub speed: f32,
    pub distance_along_curve: f32,
}

/// Coaster runtime state.
#[derive(Debug, Default, bevy_ecs::resource::Resource)]
pub struct RollercoasterState {
    pub carts: Vec<Cart>,
    pub enabled: bool,
}

impl RollercoasterState {
    /// B `RollercoasterState::new` (inlined into the plugin setup).
    pub fn new() -> Self {
        Self::default()
    }
}

/// B `rollercoaster_update` (`0x1403b7f20`) — 未实现-需重做.
///
/// Blocking points: the fresh decompilation is a 26 KB inlined body with
/// spline sampling, per-cart orientation quaternions, audio parameter updates
/// and render instance writes; the curve/instance field layouts are owned by
/// `country-core` and `rhapsody` and are not closed in the current workspace.
pub fn rollercoaster_update(_state: &mut RollercoasterState, _delta_seconds: f32) {
    todo!(
        "未实现-需重做: rollercoaster_update 0x1403b7f20 —— 26KB 样条/朝向/实例写入体，\
         曲线与 render instance 字段布局未闭合"
    );
}

/// B `create_cart_chain` (`0x1403be1a0`) — 未实现-需重做.
///
/// Blocking points: cart spacing is derived from curve arc length and a
/// per-cart transform stack; the recovered constants depend on the curve
/// representation and cart mesh extents (not present in the workspace).
pub fn create_cart_chain(_curve: &[ [f32; 3] ], _cart_count: usize) -> Vec<Cart> {
    todo!(
        "未实现-需重做: create_cart_chain 0x1403be1a0 —— 弧长/间距/变换栈未逐语句还原"
    );
}

/// B plugin `build` — registration glue.
pub fn build(app: &mut bevy_app::App) {
    app.init_resource::<RollercoasterState>();
}
