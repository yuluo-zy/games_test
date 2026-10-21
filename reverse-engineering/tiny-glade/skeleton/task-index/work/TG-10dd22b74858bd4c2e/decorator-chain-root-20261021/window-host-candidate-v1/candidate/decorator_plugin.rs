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

/// `tiny_glade::decorator_plugin::impl$0::build` `0x140342c10`。
///
/// 原 113KB 反编译是逐组 `(系统, schedule, set, run_if)` 注册表；本轮先接通
/// crate 内已恢复的三组注册入口，并给出契约 §1 的阶段顺序（由
/// `system_decorator::add_decorator_systems` 内的 `.chain()/.after()` 保证）：
/// 装载/派生 → 变换缓存 → 可视化清空/音频/窗灯 → 历史/备份/归档恢复 →
/// 外部命令 → 回放；cull 族在历史之后注册。
///
/// 仍未逐项还原原注册表的 schedule/set/run_if 与缺失生成族（见 DEC DELIVERY）。
pub fn build(app: &mut bevy_app::App) {
    use country_core::resources::history::MaybePendingSnapshotId;
    // 与原 cull 族同一 `DecoratorArchivist<C>` 载荷（tiny-glade 启用
    // native-physics → LiveDecoratorArchivist 即此二元组）。
    type CollisionPayload = (
        country_core::systems::collision::world::OffsetColliderTransform,
        country_core::systems::collision::world::RaycastMesh,
    );

    system_decorator::add_decorator_events_and_resources(app.world_mut());
    // 装饰归档 owner（原 DecoratorPlugin @140343132/13d 安装；幂等）。
    app.world_mut()
        .init_resource::<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    system_decorator::add_decorator_systems::<MaybePendingSnapshotId, CollisionPayload>(app);
    system_decorator::register_cull_oob_systems::<CollisionPayload>(app);
    system_decorator::ui_systems::register_ui_systems::<CollisionPayload>(app);
}

/// `tiny_glade::decorator_plugin::decorators_on_glade_unload` `0x140347090` —— 未实现。
pub fn decorators_on_glade_unload() {
    todo!("未实现: decorators_on_glade_unload 0x140347090 —— 40KB 卸载清理业务，未逐语句重建")
}

/// Genuine startup owners, independent of incomplete edit/UI system adapters.
/// No updater, replay, or UI leaf is mounted by this resource constructor.
pub fn build_startup_resources(app: &mut bevy_app::App) {
    system_decorator::add_decorator_events_and_resources(app.world_mut());
    system_decorator::ui_systems::register_ui_resources(app);
    app.init_resource::<country_core::resources::walls::decorator_archivist::LiveDecoratorArchivist>();
    app.add_event::<system_decorator::handle_external_decorator_cmds::ExternalDecoratorCmd>();
    app.init_resource::<system_decorator::blueprint::PrevDecoratorBlueprints>();
    app.init_resource::<country_core::systems::collision::WindowAutoClutterCandidates>();
}

/// 原装饰物帧链的已恢复成员：set10清墙洞，set25缓存、派生、生成与发布。
/// 所有正式资产、请求和缓存producer必须已由同App启动链安装后再调用。
/// validate/编辑回滚等其余成员仍须独立恢复；此处只注册真实可调用系统。
pub fn build_recovered_visual_systems(app: &mut bevy_app::App) {
    use bevy_ecs::schedule::IntoScheduleConfigs;
    use country_core::systems::{GameSystemSet, collision::world::flush_rapier_colliders};
    use system_decorator::{
        blueprint::{clear_blueprints, copy_blueprints_to_prev,
            instantiate_blueprint_colliders, instantiate_blueprints},
        cache_decorator_transforms::cache_decorator_transforms,
        derive_decorator_subtype::derive_decorator_subtypes,
        decorator_visual::{
            cottage_corner_window::generate_cottage_corner_windows,
            cottage_wall_window::generate_cottage_wall_windows,
            decorator_clutter::generate_decorator_clutter,
            maintain_visual_entities::{clear_out_visual_entities, clear_out_wall_holes},
        },
    };
    // 原140344e0f/e12：墙洞双缓冲清帧属于set10，不与瞬态实体清理混淆。
    app.add_systems(bevy_app::Update, clear_out_wall_holes.in_set(GameSystemSet(10)));
    // 原外19成员chain：cache#2先于derive#4，clear tuple#5先于generator tuple#6。
    // 原15个生成器中这里只挂完整恢复的三项；tuple内部没有人为添加相邻顺序。
    // outer chain保留自动deferred同步，full/copy的原内部链才ignore_deferred。
    app.add_systems(bevy_app::Update, (
        cache_decorator_transforms,
        derive_decorator_subtypes,
        (clear_blueprints, clear_out_visual_entities),
        (generate_cottage_wall_windows, generate_cottage_corner_windows, generate_decorator_clutter),
        instantiate_blueprint_colliders,
        flush_rapier_colliders,
        (instantiate_blueprints, copy_blueprints_to_prev).chain_ignore_deferred(),
    ).chain()
        .in_set(GameSystemSet(25))
        .before(country_core::resources::walls::decorator::unleash_decorator_op_events));
    // 原member16再次冲刷同一RaycastWorld，处在member15通知发布之后、member18 UI之前。
    app.add_systems(bevy_app::Update, flush_rapier_colliders
        .in_set(GameSystemSet(25))
        .after(country_core::resources::walls::decorator::unleash_decorator_op_events)
        .before(system_decorator::ui_systems::ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance));
}
