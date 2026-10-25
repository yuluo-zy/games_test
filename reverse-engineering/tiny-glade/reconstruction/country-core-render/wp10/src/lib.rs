//! WP10 — country-core vegetation + critters content domain（B 版本）
//!
//! 恢复范围（业务逻辑→骨架实现，差分仅作裁决）：
//!   - `resources::tree_storage`     树/植被实例存储（TreeStorage 等）
//!   - `resources::critters`         小动物 CPU 状态（Critter/Firefly/Butterfly/Duck/
//!                                   Wren/BirdSpots/Sheep + sheep::control 快照）
//!   - `resources::vegetation`       garden/grass/植被 CPU 数据（帧 pass 消费的生产者）
//!   - `resources::render_loop_critters`  critters GPU SSBO（FireflySsbo 等）
//!   - `systems::critters`           小动物系统（shared/firefly/butterfly/wren/boids/
//!                                   duck/fish/spawn）
//!   - `systems::tree_tool`          树工具系统（process_tree_cmds/despawn/hide/move）
//!   - `systems::forest`             森林系统（tree_animation/manage_tree_prefabs）
//!   - `systems::garden`             花园系统（draw_garden/record_garden_history）
//!
//! 方法：PDB 骨架（enriched/structures）+ Ghidra 反编译（见 evidence/）+ 已有
//! `reconstruction/critters`/`reconstruction/tree-tool` 候选（reva/radare2 反汇编核心，
//! 按 B 骨架名核对复用）。全部标记 implemented_unverified：无 runtime/GPU 验收；合同是
//! 当前可审阅依据，不因编译通过声称与原作等价。
//!
//! CJK 文档注释使用对齐续行（`- ` 列表换行）；clippy 文档缩进 lint 与本仓库格式约定冲突，
//! 在此统一允许。

#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

pub mod components;
pub mod resources;
pub mod systems;
pub mod typed_gap;

/// 依赖复用边界（不还原依赖实现，仅声明版本锚点；与 `reconstruction/Cargo.lock` 基线一致）。
#[doc(hidden)]
pub mod dependency_anchors {
    // bevy_ecs 0.16.0 / glam 0.29.3 / hashbrown 0.14.5 / serde 1.0.200 /
    // serde_json 1.0.85 / perchance 0.4.0（仅语义复用，见 resources::critters::CritterRng）。
}
