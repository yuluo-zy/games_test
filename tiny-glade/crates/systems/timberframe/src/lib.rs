//! timberframe — 木框架系统（restorer-wave3-timber-roof，2026-10-17 梁写路径落库）。
//!
//! 原 crate（B skeleton）：`crates/systems/timberframe/`（4 文件；符号 =
//! 业务 3 / 胶水 1）。
//! - `impl$0::add_timberframe`（0x140630a20）：**glue-legitimate**（bevy App 注册壳）。
//! - `timberframe_beam_count_change_audio`（0x1421b6610）：重建（主体判等）。
//! - `construct_timberframe_circle`（0x1421b67a0）：重建 + 梁写路径。
//! - `construct_timberframe_rectangle`（0x141a41ba0）：重建 + 梁写路径。
//! - `timberframe_on_glade_unload`（行 50，内联）：glue（谷地卸载清理）。
//!
//! ## 梁实例写路径（2026-10-17 wave3 落库）
//!
//! country-core 已提供 `WoodenPlankSsbo`（SparseSsbo/SsboType）与 `AssetSsboLibrary`
//! 的 `get_mut`；本 crate 现经 `AssetSsboLibrary` 取 `SparseInstanceBuffer<WoodenPlankSsbo>`
//! 的持久 `SparseInstanceBufferAllocation`，逐梁 `SparseInstanceBufferWriter::push`。
//! 分歧登记（诚实）：`RenderInstance` 记录（`impl$1::from` 中间态）字段序 I 未落库——
//! 梁实例按 core `WoodenPlankSsbo {position,size,rotation}` 写（位置/截面/计数判等；
//! 朝向经 utils 镜像 `orientation_mat3`/`rotation_from_normal`，最终 `Quat`）。

pub mod timberframe_beam_count_change_audio;
pub mod timberframe_circle;
pub mod timberframe_rectangle;

use bevy_ecs::prelude::*;

use country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation;
use country_core::resources::roofs::roof_buffers::WoodenPlankSsbo;

use crate::timberframe_beam_count_change_audio::timberframe_beam_count_change_audio;
use crate::timberframe_circle::construct_timberframe_circle;
use crate::timberframe_rectangle::construct_timberframe_rectangle;

/// 梁实例缓冲（O：`TimberframeInstanceBuffer` 槽 = `{gate@0, allocation@8}`，allocation 为
/// `WoodenPlankSsbo` 的持久 `SparseInstanceBufferAllocation`）。
///
/// 矩形构造（0x141a41ba0）槽空时经 `AssetSsboLibrary::get<WoodenPlankSsbo>` 创建 allocation
/// 存入；圆形构造（0x1421b67a0）以**只读** Res 面门控（槽空 → `unwrap_failed` panic）。
/// `None` = 槽空（gate==0）。
#[derive(Resource)]
pub struct TimberframeInstanceBuffer {
    /// 槽 allocation（None = gate==0 槽空）。
    pub allocation: Option<SparseInstanceBufferAllocation<WoodenPlankSsbo>>,
}

impl Default for TimberframeInstanceBuffer {
    fn default() -> Self {
        Self { allocation: None }
    }
}

/// `timberframe::impl$0::add_timberframe` @0x140630a20（O 面：App 注册壳——insert_resource×2 +
/// add_event + chain_ignore_deferred + in_set + add_systems(Update) + OnExit 卸载；bevy 内部
/// 注册环不恢复）。glue-legitimate。
///
/// 分歧点（GAP，U）：`in_set` 的具体 `SystemSet` 标签 U（机器码 intern 一个 `dyn SystemSet`）；
/// `OnExit<AppLoadingState>` 依赖 country-core 未提供的 `AppLoadingState` 类型与 bevy_state ——
/// 本处暴露 `timberframe_on_glade_unload` 供宿主在对应 OnExit 调度接线（GAP）。
/// FIX-4：原 add_timberframe **不注册** `PlayTimberframeSound` 事件（country-core `add_audio_events`
/// 已注册）——本函数仅注册 `ConstructTimberframeCmd`。
pub fn register_timberframe_events(world: &mut World) {
    use country_core::resources::events::ConstructTimberframeCmd;
    world.init_resource::<TimberframeInstanceBuffer>();
    world.init_resource::<crate::timberframe_beam_count_change_audio::TimberframeBeamCount>();
    world.init_resource::<Events<ConstructTimberframeCmd>>();
}

/// 注册 CPU 系统（O：add_timberframe 的 add_systems(Update) 面；chain A→B→C）。
/// `in_set` 标签 U；本处以 `chain_ignore_deferred` 表达链式接线（I）。
pub fn add_timberframe_cpu_systems(schedule: &mut Schedule) {
    schedule.add_systems(
        (
            construct_timberframe_rectangle,
            construct_timberframe_circle,
            timberframe_beam_count_change_audio,
        )
            .chain_ignore_deferred(),
    );
}

/// `timberframe_on_glade_unload`（行 50，内联实例；glue）：谷地卸载时清理梁实例缓冲
/// （O：`ResMut<TimberframeInstanceBuffer>` + deref_mut + set_changed；清理语义 I——
/// 释放 allocation 把其 chunk 归还缓冲 waste）。
/// 注册面：`Schedules::add_systems(OnExit<AppLoadingState>, ...)`（bevy 胶水；宿主接线 GAP）。
pub fn timberframe_on_glade_unload(mut buffer: ResMut<TimberframeInstanceBuffer>) {
    buffer.allocation = None;
}

/// `add_timberframe`（0x140630a20，O 面）。glue-legitimate。
pub fn build(world: &mut World, schedule: &mut Schedule) {
    register_timberframe_events(world);
    add_timberframe_cpu_systems(schedule);
}
