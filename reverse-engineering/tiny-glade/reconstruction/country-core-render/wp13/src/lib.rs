//! WP13 — country-core session / prefab / history / save 域（B 版本）
//!
//! 恢复范围（业务逻辑→骨架实现，差分仅作裁决）：
//!   - `resources::session`     ActiveSession/GladeType/NextSession/SessionId/
//!                              SessionMeta/SavedSession(List) 会话状态
//!   - `resources::prefab`      PrefabLibrary = HashMap<PrefabName, Prefab> 目录
//!   - `resources::history`     S05 权威历史缓冲：History/HistoryEditChain/
//!                              EditType/MultiEdit/HistoryMutationToken/快照记录
//!   - `systems::history`       history_process（撤销/重做驱动）/history_recording
//!   - `systems::save`          存档：save_game/load_game/snapshot/save_thread/
//!                              save_backup_list/shadow_file/load_completion_checker
//!
//! 方法：PDB 骨架（enriched/structures）+ Ghidra 反编译 + 原机器码字节核验（见
//! evidence/）。复用既有 `reconstruction/history`（tg-history 观察模型）、
//! `reconstruction/crates/tg-history`、task-index 候选（history_chain /
//! HistoryCmd::interrupt / GladeType tag），按 B 骨架名核对。全部标记
//! implemented_unverified：无 runtime 验收；合同是当前可审阅依据。
//!
//! CJK 文档注释使用对齐续行；clippy 文档缩进 lint 与本仓库格式约定冲突，
//! 在此统一允许。

#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

pub mod resources;
pub mod systems;
pub mod typed_gap;

#[cfg(test)]
mod tests;

/// 依赖复用边界（不还原依赖实现，仅声明版本锚点；与原 `reconstruction/Cargo.lock`
/// 基线一致）。
#[doc(hidden)]
pub mod dependency_anchors {
    // bevy_ecs 0.16.0 / uuid 1.15.1 / hashbrown 0.14.5 / serde 1.0.200 /
    // serde_json 1.0.85 / fastrand 1.8.0 / anyhow（语义复用）。
}
