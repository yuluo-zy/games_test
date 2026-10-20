//! `ui_focus_decoclutter.rs` — deco-clutter 聚焦（B 0x1409a92c0）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/1409a92c0，54 KB）。逐语句对照：
//! - **门 `*(int*)*in_RCX != 3 → return`**（EditSubject::DecoClutter），
//!   `((int*)*in_RCX)[1]` = decorator id；
//! - `*(longlong*)(lVar21+0x78)==0 → return`；DecoratorId → d2s 哈希探测 →
//!   `DecoratorStorage::get`（0x140a1c2c0）→ **null → return；`result[+0x48]==0 → return`**；
//! - 内嵌 map 探测（0x50 stride）取 `DecoratorInfo`：**`ty = bucket+0x4e`（= info+0x46）**；
//!   `d = ty-0x38`，`d<8 ∧ d!=6 → return`（即接受 ty<0x38、≥0x40、==0x3e）；
//! - `clutter_mesh_ty_data()` → 足迹 Rectangle3d；`set_size`（类型 0x22..0x2a 表
//!   `&DAT_142abdee8/+0xc/+0x30`）；`set_orientation_xz` → `translate` →
//!   `DebugRectangle3d::show`；
//! - 多路事件（每路门 = `widget.native_tag != 0 ∧ widget id 匹配 ∧ interaction+0x28 != 2`
//!   = `UiWidgetContext::is_active`）：
//!   - 点击 widget（phase==3）→ `RequestAppModeChange` + `CameraMatrices::xz_projective_rotation`
//!     + `PlayDecoratorSoundCmd`；
//!   - rotate widget active → `RequestRotateDecorator`；move widget active →
//!     `RequestMoveClutter`；另有多处 `UiState::add` + `SignifierFactory::new` +
//!     `push<UiAppendSignifier>`。
//!
//! crate 的 UiState 只有单一 interaction 上下文（宿主 UI 名队列未接入），故各路事件以
//! [`DecoClutterFocusUi`] 适配门控（I）；几何/相机/焦点状态机依赖宿主实体，登记待接。
//! **早期 Rust 的单个 `DecoClutterFocused` 事件系编造，已删。**
use bevy_ecs::prelude::*;

use crate::{
    resources::walls::decorator::DecoratorId,
    resources::walls::decorator_storage::DecoratorStorage,
    systems::{
        audio::PlayDecoratorSoundCmd,
        mode_manager::{AppMode, EditSubject, RequestAppModeChange},
        ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream},
    },
};

/// deco-clutter 聚焦状态资源（I：门 = 判别；`decorator_id` = 聚焦目标）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DecoClutterFocusState {
    /// 状态机门（O：首 u32 判别；`!= 3` → return）。
    pub tag: u32,
    /// 聚焦的 DecoratorId（O：`((int*)*in_RCX)[1]`）。
    pub decorator_id: u32,
}

/// 宿主 UI 门控（I：UiState 多 widget 交互面；每条对应 `UiWidgetContext::is_active`
/// + phase 判别。宿主 UI 名队列接入后由真实 widget id 替代）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct DecoClutterFocusUi {
    /// 点击 widget phase（O：`[+0x19] == 3` 才写 RequestAppModeChange/PlayDecoratorSoundCmd）。
    pub click_phase: u8,
    /// rotate widget 激活（O：`is_active`）。
    pub rotate_active: bool,
    /// move widget 激活（O：`is_active`）。
    pub move_active: bool,
    /// 聚焦位置（宿主实体世界位置；`PlayDecoratorSoundCmd` 载荷）。
    pub position: glam::Vec3,
}

/// `LastClonedClutter`（原版 `ui_focus_decoclutter::LastClonedClutter`；`ClutterPlugin::build`
/// 在 0x140652754 `insert_resource`）。
///
/// 机器码证据（0x140652731…0x140652754）：初值 = 16 B 常量 `0x14293e0c0`
/// （`00 00 00 00 00 00 00 00 00 00 00 00 00 00 80 3f`，即 (0,0,0,1) = `Quat::IDENTITY`）
/// + 1 B 取自被 ICF 折叠成 `return 0` 的 `UiNav::clone_behavior`（0x140002370）——一个默认值 0 的
/// 单字节枚举/布尔。故它**不是**仅 1 字节：Rust 侧为 `{ Quat, u8 }`（16 B 对齐，共 32 B）。
/// 该单字节的具体枚举类型与名字**未闭合**，以 `u8` 承载；读写者（clone-deco 流程）在其它 crate。
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct LastClonedClutter {
    /// 上次克隆杂物的旋转（初值 `Quat::IDENTITY`）。
    pub rotation: glam::Quat,
    /// 初值 0 的单字节（枚举类型未定型）。
    pub tag: u8,
}

impl Default for LastClonedClutter {
    fn default() -> Self {
        Self { rotation: glam::Quat::IDENTITY, tag: 0 }
    }
}

/// 旋转请求事件（I：原生 `RequestRotateDecorator` 事件布局 U；载荷 = 聚焦 id + 角）。
#[derive(Event, Clone, Copy, Debug, PartialEq)]
pub struct RequestRotateDecorator {
    pub decorator_id: u32,
    pub amount: f32,
}

/// 移动请求事件（I：原生 `RequestMoveClutter` 事件布局 U；载荷 = 聚焦 id + 位移）。
#[derive(Event, Clone, Copy, Debug, Default, PartialEq)]
pub struct RequestMoveClutter {
    pub decorator_id: u32,
    pub delta: glam::Vec2,
}

/// `ui_focus_decoclutter` @0x1409a92c0。
#[allow(clippy::too_many_arguments)]
pub fn ui_focus_decoclutter(
    state: Res<DecoClutterFocusState>,
    ui: Res<DecoClutterFocusUi>,
    storage: Res<DecoratorStorage>,
    mut sound: EventWriter<PlayDecoratorSoundCmd>,
    mut mode_change: EventWriter<RequestAppModeChange>,
    mut rotate: EventWriter<RequestRotateDecorator>,
    mut move_clutter: EventWriter<RequestMoveClutter>,
    mut signifiers: ResMut<UiSignifierStream>,
) {
    // **门 = disc != 3 → return**（O：fresh `*(int*)*in_RCX != 3`）。
    if state.tag != 3 {
        return;
    }
    let id = DecoratorId(state.decorator_id);
    // d2s 探测 + DecoratorStorage::get；null / [+0x48]==0 → return。
    let Some(addr) = storage.lookup_storage(id) else {
        return;
    };
    let Some(info) = storage.decorator_info(id) else {
        return;
    };
    let _ = addr;
    // 类型门（O：`bVar26 = info.ty; d = ty-0x38; d<8 ∧ d!=6 → return`）。
    let d = info.ty.wrapping_sub(0x38);
    if d < 8 && d != 6 {
        return;
    }
    // 点击 widget（phase==3）：RequestAppModeChange + PlayDecoratorSoundCmd。
    // O：0x1409a92c0 确有 `EventWriter::write<RequestAppModeChange>` 调用；载荷 = AppMode。
    // 载荷来源（局部再取证前）按聚焦目标取 Editing(DecoClutter{id})（I，不冒充字符串名）。
    if ui.click_phase == 3 {
        mode_change.write(RequestAppModeChange(AppMode::Editing(
            EditSubject::DecoClutter {
                decorator_id: id.0,
            },
        )));
        sound.write(PlayDecoratorSoundCmd {
            kind: 0,
            position: ui.position,
        });
    }
    // rotate widget active → RequestRotateDecorator。
    if ui.rotate_active {
        rotate.write(RequestRotateDecorator {
            decorator_id: id.0,
            amount: 0.0,
        });
    }
    // move widget active → RequestMoveClutter。
    if ui.move_active {
        move_clutter.write(RequestMoveClutter {
            decorator_id: id.0,
            delta: glam::Vec2::ZERO,
        });
    }
    // `SignifierFactory::new` + `push<UiAppendSignifier>`（O 多处）；名字/载荷由工厂与
    // 宿主实体提供，crate 以聚焦位置 + name 0 承载，登记 I。
    signifiers.write(UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: ui.position,
        scale: 1.0,
        name: 0,
    });
}
