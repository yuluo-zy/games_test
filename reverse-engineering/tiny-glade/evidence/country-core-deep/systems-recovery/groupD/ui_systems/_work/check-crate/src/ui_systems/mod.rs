//! B `crates/systems/decorator/src/ui_systems/` — 装饰交互 UI 系统族（D2 restorer tg_d2）。
//!
//! 证据：`evidence/country-core-deep/systems-recovery/groupD/ui_systems/`
//! （decomp/decomp-0x14*.txt 全部业务 VA fresh 反编译 + 机器码手解）。
//! 记号：O=机器码证实 / I=推断（注明依据）/ U=未恢复，不臆造。
//!
//! 共同交互骨架（O，各系统门控一致）：
//! - **门控**：`DecoratorAffordanceDispatcher.state`（u64@+0；clear 写 {6,0}）——
//!   ui_rotate=3、ui_change_stair_height=4、ui_change_chimney_height=4、
//!   continue_stairs∈{4,11(带 '?')}、stairs_reset∈{4,5,11}、inactive=6→clear；
//! - **激活检查**：`UiWidgetContext::is_active`（0x14092e680，core O）；
//! - **拖动相位**：`dragged`（0x14092df10）字节 {0,1}=未拖、2=悬停、3=拖中、4=释放
//!   （相位映射 I；`bVar20=(c-2)<3?c-2:3` 观察）；释放相位写 op 记录；
//! - **复位写入**：`*[state+0x10] ← [state+0x1c]` + `[state+0x48] = 1`
//!   （[`MoveDecorationInteractionState::reset_interaction`]）；
//! - **signifier**：`UiSignifierStream::write`（0x140b2cf90，core O）。装饰域记录
//!   48B = {tag=1, kind=1, name_hash@0x10, id@0x18, pos@0x20, scale@0x28}，
//!   写拷贝 40B（5×qword）；core `UiAppendSignifier` 40B 布局承载
//!   {tag, kind, position, scale, name}，装饰域把 name 槽当 id/编码（I，登记）；
//! - **op 队列**：`DeferredDecoratorEditHistory`（Vec<0x28B>@+0x20 + 计数器@+0x40），
//!   记录 {disc@0, kind@8, payload}；apply 见 [`crate::apply_decorator_op`]。
pub mod calculate_decorator_dst;
pub mod calculate_decorator_grab_offset;
pub mod decorator_interaction_intent;
pub mod handle_splitting_roof_decorators;
pub mod handle_splitting_wall_decorators;
pub mod move_ops;
pub mod ui_change_chimney_height;
pub mod ui_change_stair_height;
pub mod ui_decorator_affordance_dispatch;
pub mod ui_focus_stairs;
pub mod ui_inactive;
pub mod ui_move_decorator;
pub mod ui_place_decorator;
pub mod ui_rotate_lantern;
pub mod ui_stairs_misc;
pub mod ui_stairs_viz;

use bevy_ecs::prelude::*;
use country_core::resources::walls::decorator::DecoratorId;

pub use ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
pub use ui_move_decorator::RaycastResources;

/// B `MoveDecorationInteractionState`（lib.rs 符号；I 级布局，字段语义 U）。
///
/// O 面写入（各系统复位）：`*[+0x10] ← [+0x1c]`（u32 拷贝经 +0x10 指针槽）与
/// `[*+0x0] + 0x48 = 1`（内层激活标记字节）。指针槽由宿主（输入/游标接线）提供；
/// 本 crate 以 Option 槽承载该写入模式，宿主侧接线登记待取证。
#[derive(Resource, Default)]
pub struct MoveDecorationInteractionState {
    /// +0x10：u32 槽指针（宿主提供；None = 未接线）。
    pub slot: Option<u32>,
    /// +0x1c：待写值。
    pub value: u32,
    /// 内层激活标记（`[*+0x0]+0x48`，I）。
    pub active: bool,
}

impl MoveDecorationInteractionState {
    /// 机器码复位模式（O 面；指针槽语义 U）：`*[+0x10]=[+0x1c]; [*+0x0]+0x48=1`。
    pub fn reset_interaction(&mut self) {
        if let Some(slot) = &mut self.slot {
            *slot = self.value;
        }
        self.active = true;
    }

    /// 失效标记（I：0x1408fe880 `[内层+0x10] = 3`）。
    pub fn mark_invalid(&mut self) {
        self.active = false;
    }
}

/// B `DecoratorIdGen`-adjacent 交互上下文构建（I）：ui_systems 各系统以
/// 活动装饰 id（dispatcher/backup +0x10 观察）构造 WidgetContext 并查
/// `UiWidgetContext::is_active`（0x14092e680）。
pub fn decorator_widget_id(id: DecoratorId) -> u64 {
    u64::from(id.0)
}

/// 注册装饰 UI 交互系统（宿主调度组装；阶段归属按原 B 登记 I）。
pub fn register_ui_systems(app: &mut bevy_app::App) {
    app.init_resource::<DecoratorAffordanceDispatcher>();
    app.init_resource::<MoveDecorationInteractionState>();
    app.init_resource::<crate::backup::DecoratorBackup>();
    app.init_resource::<crate::deferred_decorator_history::DeferredDecoratorEditHistory>();
    app.init_resource::<crate::deferred_decorator_history::DeferredDecoratorOpQueue>();
    app.init_resource::<crate::blueprint::DecoratorBlueprints>();
    app.add_event::<ui_place_decorator::EvInitPlaceDecoratorInteractionState>();
    app.add_systems(
        bevy_app::Update,
        (
            ui_inactive::ui_decorator_inactive,
        ui_stairs_viz::visualize_ui_stairs,
        ui_stairs_viz::insert_point_ui,
        ui_stairs_misc::stairs_reset_for_new_frame,
        ui_stairs_misc::continue_stairs,
        ui_focus_stairs::ui_focus_stairs,
        ui_decorator_affordance_dispatch::ui_dispatch_decorator_affordance,
        ui_move_decorator::ui_move_decorator,
        ui_place_decorator::ui_place_decorator,
        ui_rotate_lantern::ui_rotate_decorator,
        ui_change_stair_height::ui_change_stair_height,
        ui_change_chimney_height::ui_change_chimney_height,
    ));
}
