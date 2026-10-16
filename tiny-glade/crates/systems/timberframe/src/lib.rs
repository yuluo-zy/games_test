//! timberframe — 木框架系统（组 A restorer，2026-10-16）。
//!
//! 原 crate（B skeleton）：`crates/systems/timberframe/`（4 文件；10 符号 =
//! 业务 4 / 内联 1 / 胶水 5）。
//! - `impl$0::add_timberframe`（0x140630a20）：注册壳（insert_resource +
//!   add_systems(chain_ignore_deferred + in_set) + register_event，bevy App 机制）。
//! - `timberframe_beam_count_change_audio`（0x1421b6610）：梁数 +1 → 音频事件。
//! - `construct_timberframe_circle`（0x1421b67a0）：圆顶木框架梁构建。
//! - `construct_timberframe_rectangle`（0x141a41ba0）：矩形木框架梁构建。
//! - `timberframe_on_glade_unload`（行 50，内联）：谷地卸载回调。

pub mod timberframe_beam_count_change_audio;
pub mod timberframe_circle;
pub mod timberframe_rectangle;

use bevy_ecs::prelude::*;

use crate::timberframe_beam_count_change_audio::timberframe_beam_count_change_audio;
use crate::timberframe_circle::construct_timberframe_circle;
use crate::timberframe_rectangle::construct_timberframe_rectangle;

/// 梁实例缓冲（O：BASELINE 参数面 `ResMut<timberframe::TimberframeInstanceBuffer>`；
/// re-analyst-A 契约 §2：梁实例记录 0x20B 条目族，写往
/// `roofs::roof_buffers::WoodenPlankSsbo`（GPU 后置）；CPU 侧本资源承载）。
#[derive(Resource, Clone, Debug, Default)]
pub struct TimberframeInstanceBuffer(pub Vec<crate::timberframe_circle::TimberframePlank>);

/// `timberframe::impl$0::add_timberframe` @0x140630a20（O 面：App 注册壳——
/// insert_resource + 事件注册 + add_systems(chain + in_set)；bevy 内部注册环
/// 不恢复）。
pub fn register_timberframe_events(world: &mut World) {
    use country_core::resources::events::ConstructTimberframeCmd;
    use country_core::systems::audio::PlayTimberframeSound;
    world.init_resource::<TimberframeInstanceBuffer>();
    world.init_resource::<crate::timberframe_beam_count_change_audio::TimberframeBeamCount>();
    world.init_resource::<Events<ConstructTimberframeCmd>>();
    world.init_resource::<Events<PlayTimberframeSound>>();
}

/// 注册 CPU 系统（O：add_timberframe 的 add_systems 面）。
pub fn add_timberframe_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems((
        timberframe_beam_count_change_audio,
        construct_timberframe_circle,
        construct_timberframe_rectangle,
    ));
}

/// `add_timberframe`（0x140630a20，O 面）。
pub fn build(world: &mut World, schedule: &mut Schedule) {
    register_timberframe_events(world);
    add_timberframe_cpu_systems(schedule);
}
