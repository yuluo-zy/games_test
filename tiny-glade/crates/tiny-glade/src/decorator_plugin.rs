//! B `crates/tiny-glade/src/decorator_plugin.rs`（装饰物插件注册 / glade 卸载）。
//!
//! 恢复依据（Ghidra shared 2026-10-17）：
//! - `impl$0::build` `0x140342c10`（10,365 B，113 KB 反编译）
//! - `decorators_on_glade_unload` `0x140347090`（5,395 B，40 KB 反编译）
//!
//! `impl$0::build` 是 `DecoratorPlugin` 的 Bevy 注册面：事件/资源插入 + 多组
//! `add_systems`（`Update`/`PostUpdate`），反编译体 113 KB，包含大量
//! `system_decorator::*` 系统函数指针与 `GameSystemSet` 排序。逐注册项判等
//! 需要：
//! 1. 从反编译 CALL 的泛型签名还原每组 `(系统函数, schedule, set, run_if)`;
//! 2. 核对 `system-decorator` 中各系统函数签名是否一致（D1–D4 恢复批次仍在
//!    推进，`add_decorator_systems` 只接了部分系统）。
//!
//! 当前不做：用 `add_decorator_systems` 整体转发会改变原注册顺序与集合，
//! 属「callee 转发错」，故登记未实现而不是近似接线。
//!
//! `decorators_on_glade_unload` 是卸载时的真实业务系统（装饰物/碰撞/可视化
//! 实体清理），40 KB 反编译，未逐语句重建。

/// `tiny_glade::decorator_plugin::impl$0::build` `0x140342c10` —— 未实现。
pub fn build() {
    todo!(
        "未实现: DecoratorPlugin::build 0x140342c10 —— 113KB 注册面，需逐组还原 \
         (系统, schedule, set, run_if) 并与 system-decorator 签名核对"
    );
}

/// `tiny_glade::decorator_plugin::decorators_on_glade_unload` `0x140347090` —— 未实现。
pub fn decorators_on_glade_unload() {
    todo!("未实现: decorators_on_glade_unload 0x140347090 —— 40KB 卸载清理业务，未逐语句重建")
}
