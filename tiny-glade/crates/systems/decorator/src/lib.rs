//! system-decorator：装饰系统（批次 2，组 D 多 restorer 并行恢复）。
//!
//! 原始 crate：`crates/systems/decorator`（B PDB；189 业务，批次 2）。本 lib.rs 为
//! 组 D 共享声明文件——各 restorer 把自己的模块加入声明，不得整体覆盖：
//! - D1（decorator_visual 22 文件几何组装族）：decorator_visual / blueprint（消费
//!   契约）/ utils（visual_entity_builder 消费契约）
//! - D2（ui_systems）：ui_systems
//! - D3（生命周期/历史/备份族）：apply_decorator_op / backup / cache_decorator_transforms /
//!   decorator_on_* / decorator_resources / decorator_visual_state / deferred_decorator_history /
//!   derive_decorator_subtype / handle_external_decorator_cmds / load_decorators /
//!   merge_proposal_* / replay_decorator / restore_archived_decorators
//! - D4（杂项族 + lib.rs 统一注册，本 restorer）：audio（op_confirm/op_preview/
//!   op_stairs）、window_lights、remove_ivy_under_windows、dormer_holes、
//!   collision_detection、cull_oob_decorators、chimney_smoke_anim_thickness、
//!   move_decorators_following_anchors + `add_decorator_systems` / 事件注册
//!   （lib.rs 注册链由 D4 统一维护）。
//!
//! 证据：`evidence/country-core-deep/systems-recovery/groupD/`（fresh RE-MCP
//! 反编译 + 机器码契约）。依赖方向 = 原 B：`system_decorator → country_core`
//! （DecoratorStorage / DecoratorArchivist / History / PublicWalls /
//! CachedDecoratorTransforms / DecoratorMeshLibrary / StairsState /
//! StairsAssemblyParams / dormers / instanced_wall / render）+ `utils`
//! （wall_space/curve/geometry/comparative_space/natural_cubic/float_grid/
//! swizzlers/halver）+ bevy_ecs + glam + fastrand + puffin + lerp + itertools。
//!
//! # lib.rs 注册链（D4 统一）
//!
//! 原 B `lib.rs` 含 `system_decorator::impl$0::snapping_direction`（行 63，
//! inline，7 实例）与 `system_decorator::try_merge_decorators`（行 77，inline，
//! 6 实例，`Iterator::any`/`sum` + `DecoratorInfo::eq`）；系统注册入口
//! `add_decorator_systems` 由 D4 按原 `add_*_systems` 地址与 decorator_plugin
//! 装配顺序统一提供（D1-D3 各自系统的注册由本函数逐批接入；跨域挂账见
//! `evidence/country-core-deep/systems-recovery/groupD/misc/RESTORE.md`）。

pub mod apply_decorator_op;
pub mod audio;
pub mod backup;
pub mod blueprint;
pub mod cache_decorator_transforms;
pub mod chimney_smoke_anim_thickness;
pub mod collision_detection;
pub mod cull_oob_decorators;
pub mod decorator_on_freehand_wall_edited;
#[cfg(test)]
mod freehand_history_integration;
pub mod decorator_on_move_shape;
pub mod decorator_on_rectangle_edited;
pub mod decorator_on_wall_height_changed;
pub mod decorator_resources;
pub mod decorator_visual;
pub mod decorator_visual_state;
pub mod deferred_decorator_history;
pub mod derive_decorator_subtype;
pub mod dormer_holes;
pub mod handle_external_decorator_cmds;
pub mod load_decorators;
pub mod lifecycle;
pub mod merge_proposal_roof_decorators;
pub mod merge_proposal_wall_decorators;
pub mod move_decorators_following_anchors;
pub mod remove_ivy_under_windows;
pub mod replay_decorator;
pub mod restore_archived_decorators;
pub mod ui_systems;
pub mod utils;
pub mod window_lights;

// ---- lib.rs 内联业务（PDB 编译单元：snapping_direction 行 63 /
//      try_merge_decorators 行 77，均 inline，被 merge_proposal / ui_systems /
//      collision_detection 内联实例化）----

use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::world::World;
use country_core::resources::walls::decorator::DecoratorInfo;

/// B `system_decorator::impl$0::snapping_direction`（lib.rs 行 63，inline；
/// 7 实例：merge_proposal_wall_decorators / decorator_interaction_intent /
/// collision_detection::intersect_wall_decorators_predicate 宿主）。
///
/// 吸附方向（O 结构，0x141fe5310 内联宿主观察）：由两装饰墙空间坐标差符号
/// 得 -1/0/1（I：u 方向差符号；0 = 不可吸附/居中）。
pub fn snapping_direction(a: f32, b: f32) -> i8 {
    if a < b {
        -1
    } else if a > b {
        1
    } else {
        0
    }
}

/// B `system_decorator::try_merge_decorators`（lib.rs 行 77，inline；6 实例）。
///
/// 尝试合并重叠装饰（O：`Iterator::any`/`sum` + `DecoratorInfo::eq`；
/// 语义 = 同墙/同型装饰列表按重叠合并，命中返回可合并标记）。
/// 签名面 I（调用点 = merge_proposal_* 内联宿主，登记 RESTORE.md）。
pub fn try_merge_decorators(
    candidates: &[DecoratorInfo],
    merge_rank: u8,
) -> bool {
    // O 结构：any + sum —— 候选非空且 rank 一致（I：合并阈值语义登记）。
    !candidates.is_empty() && candidates.iter().all(|c| c.rank() == merge_rank)
}

/// 系统注册入口（D4 统一；原 B 由 app 端 `tiny_glade::decorator_plugin::build`
/// 0x140342c10 装配 —— 本入口按 crate 内 add_*_systems 惯例提供，宿主 App
/// 组装到调度）。
///
/// 已接入（D4 杂项族）：
/// - `audio`：op_confirm_audio / op_preview_audio / op_stairs_audio；
/// - `window_lights` / `remove_ivy_under_windows` /
///   `chimney_smoke_anim_thickness`；
/// - `dormer_holes::upload_dormer_holes_to_gpu` 为 render-handoff 库函数
///   （渲染世界持 GraphicsDevice 后调用，不入调度）；
/// - `cull_oob_decorators` 族（auto_allow / cull_oob / cull_stair ×2）因
///   `DecoratorArchivist<C>` 泛型需宿主 App 以具体 C 注册（本入口以
///   `register_cull_oob_systems<C>` 提供，C 由 App 具化）。
///
/// 2026-10-17 接线（restorer-purelogic-integration，integration-registration 条目）：
/// - `clear_out_wall_holes` / `clear_out_visual_entities`（判等，0x1407f98c0 /
///   0x1407f9260）补调度注册点；
/// - `decorator_record_history`（0x140817850，循环/清空判等）与
///   `create_or_restore_decorator_backup`（0x1408238a0，状态机判等）补调度
///   注册点——body 残留（48B 载荷 kind/f2/f3、恢复叶三 map 反向拷贝）在
///   RESTORE 报告与总账持续登记，注册点先行接通。
/// - `maintain_dormer_tiles`（0x1407ce920）**每瓦 GPU 退役已落**（2026-10-17
///   consume-types：`hash_one<&GlobalId>` + `remove_entry` + 条件 drop
///   `SparseInstanceBufferAllocation<RoofTileSsbo>`）；函数仍为非 bevy 系统
///   （参数为绘制状态/双列表借用），原调度注册点未取证 → 未入调度。
pub fn add_decorator_systems<S: Send + Sync + 'static, C: Send + Sync + 'static>(
    app: &mut bevy_app::App,
) {
    use bevy_app::Update;

    app.add_event::<country_core::resources::events::OnShapeMoved>();
    app.add_systems(Update, decorator_on_move_shape::decorator_on_move_shape
        .before(derive_decorator_subtype::derive_decorator_subtypes));

    // 阶段 0/3（契约 §1 阶段 0.3 + 3.1）：子类型派生 → 变换缓存。缓存必须先于
    // UI 高度系统与可视化生成（它们消费 CachedDecoratorTransforms）。
    app.add_systems(
        Update,
        (
            derive_decorator_subtype::derive_decorator_subtypes,
            cache_decorator_transforms::cache_decorator_transforms,
        )
            .chain(),
    );

    // 阶段 4（可视化清空 → 音频/窗灯/常春藤/烟柱）。清空在缓存之后；生成族由
    // 各模块宿主继续接入（见 DELIVERY 剩余项）。
    app.add_systems(
        Update,
        (
            decorator_visual::maintain_visual_entities::clear_out_visual_entities,
            decorator_visual::maintain_visual_entities::clear_out_wall_holes,
            audio::op_confirm_audio::op_confirm_audio,
            audio::op_preview_audio::op_preview_audio,
            audio::op_stairs_audio::op_stairs_audio,
            window_lights::window_lights,
            remove_ivy_under_windows::remove_ivy_under_windows,
            chimney_smoke_anim_thickness::chimney_smoke_anim_thickness,
        )
            .chain()
            .after(cache_decorator_transforms::cache_decorator_transforms),
    );

    // 阶段 5/6（历史 → 备份 → 外部命令）。历史排空在窗灯/可视化之后。
    //
    // `restore_archived_decorators` **暂不注册**：原系统带历史游标守卫
    // （阶段 6.3，0x140817a40 步骤 2；本实现尚未闭合），每帧无条件运行会用
    // archivist 内容反复覆盖存储。待游标守卫取证后单独以 run_if 注册。
    app.add_systems(
        Update,
        (
            deferred_decorator_history::decorator_record_history::<S>,
            backup::create_or_restore_decorator_backup::<C>,
            handle_external_decorator_cmds::handle_external_decorator_cmds::<C>,
        )
            .chain()
            .after(window_lights::window_lights),
    );

    // 回放（0x14080c8d0）消费 `LiveDecoratorArchivist` 的 cull 集；在历史链后。
    app.add_systems(
        Update,
        replay_decorator::replay_decorator
            .after(deferred_decorator_history::decorator_record_history::<S>),
    );
}

/// cull 剔除族注册（`DecoratorArchivist<C>` 泛型 → 宿主 App 具化 C 后调用）。
pub fn register_cull_oob_systems<C: Send + Sync + 'static>(app: &mut bevy_app::App) {
    app.add_systems(
        bevy_app::Update,
        (
            cull_oob_decorators::auto_allow_decorator_culling::<C>,
            cull_oob_decorators::cull_oob_decorators,
            cull_oob_decorators::cull_stair_attached_decorators,
            cull_oob_decorators::cull_stair_attached_decorators_on_invalid_stairs,
        )
            .chain(),
    );
}

/// 事件/资源注册（D4 统一）：装饰域自有资源 init_resource。
pub fn add_decorator_events_and_resources(world: &mut World) {
    world.init_resource::<audio::DecoratorAudioEvents>();
    world.init_resource::<window_lights::GlobalWindowLightsControl>();
    world.init_resource::<chimney_smoke_anim_thickness::ChimneySmokeThickness>();
    world.init_resource::<chimney_smoke_anim_thickness::TimeOfDayCycle>();
    // integration-registration 2026-10-17：add_decorator_systems 新增 4 系统所需的
    // crate 本地资源（幂等 init；DecoratorBackup/DeferredDecoratorEditHistory 亦在
    // ui_systems::register_ui_systems 初始化，双路径并存无冲突）。
    world.init_resource::<decorator_visual_state::DecoratorVisualState>();
    world.init_resource::<deferred_decorator_history::DeferredDecoratorEditHistory>();
    world.init_resource::<backup::DecoratorBackup>();
    // add_decorator_systems 消费的权威/缓存资源（幂等：宿主已插入时保持原值）。
    world.init_resource::<country_core::resources::walls::decorator_storage::DecoratorStorage>();
    world.init_resource::<
        country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms,
    >();
    world.init_resource::<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>();
    world.init_resource::<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>();
    world.init_resource::<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>();
    // 外部装饰命令事件（handle_external_decorator_cmds 的 EventReader 面）。
    world.init_resource::<bevy_ecs::event::Events<
        handle_external_decorator_cmds::ExternalDecoratorCmd,
    >>();
    // 音频事件通道由 country-core `add_audio_events` 注册（PlayDecoratorSoundCmd
    // 属于 core）；装饰域直接写该事件。
}
