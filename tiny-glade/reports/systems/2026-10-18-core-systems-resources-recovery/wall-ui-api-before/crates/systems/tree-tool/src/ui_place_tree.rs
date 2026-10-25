//! ui_place_tree.rs — 放置树 UI 与命令（B 0x142269170 ui_place_tree /
//! 0x142269970 place_tree / 0x14226a170 random_tree_prefab，O；2026-10-17 fresh
//! 反编译 + 反汇编复核）。
//!
//! ## ui_place_tree（0x142269170）— 重建（判等面 + 1 处 core 缺口）
//! - 门禁 `AppMode == Tree(0x10)` && `DEV_OVERRIDE_IS_DEMO_MODE == 0`。
//! - `WidgetParams::new(volume{tag=5, f32::MAX}, extra=[0,0])` →
//!   `occlude_by_world(false)`（机器码 `XOR R8D,R8D`）→ `.sounds(UiSounds::empty())`
//!   → `UiState::add("tree_canvas")`（实读 0x143035108 长度 11）。
//! - `RaycastWorld::legacy_raycast` → 命中码（+0x30）!= 2 时
//!   `geo_src_to_edit_subject`；subject != None 早退。
//! - `CursorTerrainRaycast::position()` + `GladeBorder::is_pos_inside`：
//!   界外 → `DisplayOutOfBorder`；clicked 再 `FeedbackInputOutOfBorder`。
//!   界内 → `is_active()` 时写指示符 → `clicked()` 时再取 position →
//!   水下 y 重映射 → `PlaceTreeCmd{position}`。
//! - 指示符（O）：label = `format!("decorator_{}", CursorPositionSS)`（字面量
//!   "decorator_" @0x143035140；参数 1 项 display）；20 B 记录
//!   `{1,1,cursor_ss 位型, WindowSize::as_vec2().y, GameConfig.ui.icon_size_mult}`；
//!   `SignifierFactory::new(label, record, 0x143035160)` + `UiSignifierStream::write`。
//!   core `SignifierFactory::new(payload, inner, marker)` + `write_factory` 逐 5 qword
//!   承载；唤醒闭包/记录所有权未建模（U）。
//!
//! ## place_tree（0x142269970）— 重建 + InsertTreeCmd 0x48 缺口
//! `next_index` → `fastrand::f32` r1 → splitmix(id) 变体 → 事件位置 →
//! `fastrand::f32` r2 → `random_tree_prefab(id 低字节, …)` →
//! scale=`r1*0.9+(1-r1)*0.5`、rotation=`r2*360+(1-r2)*0.4922`（0x142925874，
//! 旧实现误用 0.0）→ `InsertTreeCmd`（0x48 槽）+ `History::add`（TreeEdit Place）。
//!
//! ## random_tree_prefab（0x14226a170）
//! selector = **树 id 低字节**（调用点 RCX=next_index，函数内只用 `&1`）；
//! 门控依次 `SessionLoadReadiness::deny_load` / `SessionMeta::is_halloween_any`
//! / `BackupSaveFileKind::is_corrupted` / `DragState::is_started`。本 crate 以
//! 布尔入参承载这些宿主资源读取面（签名 I；place_tree 调用处非 halloween 的
//! 资源身份未接线 → 登记）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::history::tree_edit::InsertTreeCmd;
use country_core::resources::history::{EditType, History};
use country_core::resources::session::ActiveSession;
use country_core::resources::tree_storage::TreeStorage;
use country_core::systems::collision::world::{RAYCAST_WORLD_RESULT_MISS, RaycastWorld};
use country_core::systems::mode_manager::{AppMode, EditSubject};
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{
    UiSounds, WidgetParams, WidgetVolumeShape,
};
use country_core::systems::ui::signifiers::SignifierFactory;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui_edit_focus::{GeoSource, geo_src_to_edit_subject};

use crate::PlaceTreeCmd;

/// 进程全局 `_ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE...E` @0x14362c098
/// （O：ui_place_tree 0x142269170 行 51-52、ui_delete_tree 0x141bd2510 行 61
/// `!=0 → return`）。非 0 时禁用户工具。本 crate 侧建模为编译期常量；
/// 真实值由宿主进程写入（非 0 早退），接线时由宿主覆盖。
pub const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;

/// 水下重映射常量（O：ui_place_tree 0x142269170 行 152-154：`y<0 →
/// (y/-2.5)*-1.2 + (1-y/-2.5)*0.0`）。字节直读 `DIVSS XMM0,-2.5` /
/// `MULSS XMM0,-1.2`。
pub const WATER_REMAP_DIVISOR: f32 = -2.5;
pub const WATER_REMAP_TARGET: f32 = -1.2;

/// 树缩放 lerp 区间（O：place_tree 0x142269970 `r*0.9+(1-r)*0.5`；
/// 常量 0x143035090 = {0.9, 360.0}、0x142925870 = {0.5, 0.4922}）。
pub const TREE_SCALE_MIN: f32 = 0.5;
pub const TREE_SCALE_MAX: f32 = 0.9;
/// 旋转上限（O：`r*360 + (1-r)*0.4922`；下限 0x142925874 = 0.49220002，
/// 旧实现误记 0.0）。
pub const TREE_ROTATION_MAX: f32 = 360.0;
/// 旋转下限（O：0x142925874 f32 位型 0x3efc0fc1 = 0.49220002）。
pub const TREE_ROTATION_MIN: f32 = 0.492_200_02;

/// splitmix64 机器码常量（O：place_tree 反汇编 0x142269b80-0x142269bde；
/// 拒绝阈 `(u32)(10*mixed) < 6` 重采，接受后返回 `(10*(u32)mixed) >> 32`）。
const TREE_SPLITMIX_STEP: u64 = 0xa076_1d64_78bd_642f;
const TREE_SPLITMIX_XOR: u64 = 0xe703_7ed1_a0b4_28db;
const TREE_SPLITMIX_RETRY_STEP: u64 = 0x40ec_3ac8_f17a_c85e;

/// 树 prefab 路径表（O：0x1430351d0 区字符串直读，2026-10-17）：
/// branchy2(22) / branchy1(22) / autumn/branchy2(29) / autumn/branchy1(29) /
/// naked1(26) / naked2(26) / naked1_no_mushrooms(39)。
pub const TREE_PREFAB_BRANCHY: [&str; 2] = ["clearing/tree/branchy2", "clearing/tree/branchy1"];
pub const TREE_PREFAB_AUTUMN: [&str; 2] = [
    "clearing/tree/autumn/branchy2",
    "clearing/tree/autumn/branchy1",
];
pub const TREE_PREFAB_NAKED: [&str; 2] =
    ["clearing/naked_tree/naked1", "clearing/naked_tree/naked2"];
pub const TREE_PREFAB_NO_MUSHROOMS: &str = "clearing/naked_tree/naked1_no_mushrooms";

/// `random_tree_prefab` @0x14226a170（O，2026-10-17 fresh 反编译 + 反汇编）。
///
/// 反汇编门控结构（bit = `selector & 1`，selector = 树 id 低字节）：
/// - `is_corrupted && bit==0` → naked1_no_mushrooms（0x14303526a, 39）
/// - `(deny_load || is_halloween_any) && bit==0` → naked1（0x143035236, 26）
/// - 其余 → `DragState::is_started()` 真 → autumn/branchy、假 → branchy；
///   bit==0 → `*2`、bit!=0 → `*1`。
/// 原生 4 个门控为宿主资源/全局读取（非函数入参）；本函数以布尔参数承载该面
/// （签名 I，见模块文档）。
pub fn random_tree_prefab(
    selector: u8,
    deny_load: bool,
    is_halloween: bool,
    is_corrupted: bool,
    drag_started: bool,
) -> &'static str {
    let bit = selector & 1;
    if is_corrupted && bit == 0 {
        return TREE_PREFAB_NO_MUSHROOMS;
    }
    if (deny_load || is_halloween) && bit == 0 {
        return TREE_PREFAB_NAKED[0];
    }
    let table = if drag_started {
        TREE_PREFAB_AUTUMN
    } else {
        TREE_PREFAB_BRANCHY
    };
    table[bit as usize]
}

/// 确定性树变体（O：place_tree 反汇编 0x142269b80-0x142269be0）：
/// `z = seed + STEP`；`a = z ^ XOR`；`mixed = hi^lo((a as u128)*(z as u128))`；
/// `p = (u32)mixed * 10`；拒绝采样 `(u32)p < 6` 时 `z ← seed + RETRY_STEP`
/// 后每轮 `z += STEP`；通过后 **变体 = p >> 32**（∈[0,9]）。
fn tree_variant(seed: u64) -> u32 {
    let mut z = seed.wrapping_add(TREE_SPLITMIX_STEP);
    let mut in_reject = false;
    loop {
        let a = z ^ TREE_SPLITMIX_XOR;
        let product = (a as u128) * (z as u128);
        let mixed = ((product >> 64) as u64) ^ (product as u64);
        let p = (mixed as u32) as u64 * 10;
        if (p as u32) >= 6 {
            return (p >> 32) as u32;
        }
        if !in_reject {
            z = seed.wrapping_add(TREE_SPLITMIX_RETRY_STEP);
            in_reject = true;
        } else {
            z = z.wrapping_add(TREE_SPLITMIX_STEP);
        }
    }
}

/// 水下 y 重映射（O：ui_place_tree 内联 `remap_pos_y_if_in_water`）：
/// `y' = (y / -2.5) * -1.2 + (1.0 - y / -2.5) * 0.0`。
pub fn remap_pos_y_if_in_water(y: f32) -> f32 {
    if y < 0.0 {
        let a = y / WATER_REMAP_DIVISOR;
        a * WATER_REMAP_TARGET + (1.0 - a) * 0.0
    } else {
        y
    }
}

/// legacy_raycast miss 判断（O 调用点 0x1422692ba：`cmp dword [rbp+0xf8],2`，
/// 即结果 **+0x30 槽位型** == 2）。
///
/// native `RaycastWorld::legacy_raycast` 在 miss 时把 u32 2 的位型写入 +0x30，
/// 故 `hit.toi.to_bits() == 2` 逐位等价。非 native-physics 壳返回
/// `RaycastWorldResult::miss()`（toi 未写位型、判别 +0x34 == 2），此分支作
/// shell seam 一并视为 miss（登记 I；native 下两条件恒等价）。
fn legacy_raycast_miss(hit: &country_core::systems::collision::world::RaycastWorldResult) -> bool {
    hit.toi.to_bits() == RAYCAST_WORLD_RESULT_MISS || hit.is_miss()
}

/// `ui_place_tree` @0x142269170（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_tree(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    cursor: Res<CursorTerrainRaycast>,
    mut place_cmds: EventWriter<PlaceTreeCmd>,
    raycast_world: Res<RaycastWorld>,
    camera_ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    cursor_ss: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    window: Res<country_core::resources::window::WindowSize>,
    config: Res<country_core::game_config::GameConfig>,
    mut border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
) {
    // 门禁（O：`*AppMode==0x10 && DEMO==0`）。
    if DEV_OVERRIDE_IS_DEMO_MODE || mode.native_tag() != AppMode::Tree.native_tag() {
        return;
    }
    // O：widget（WidgetParams::new → occlude_by_world(false) → sounds(empty) → add）。
    // volume 字节：tag@0 = 5、f32@4 = f32::MAX（size.y 原生未写，U）。
    let volume = WidgetVolumeShape {
        tag: 5,
        layer: 0,
        size: Vec2::new(f32::MAX, 0.0),
    };
    let ctx = ui.add(
        "tree_canvas",
        WidgetParams::new(volume, [0; 2])
            .occlude_by_world(false)
            .sounds(UiSounds::empty()),
    );
    // O：legacy_raycast（0x1408cc680）+ geo_src_to_edit_subject（0x140977f50）；
    // 命中且 subject != None → 早退。
    let hit = raycast_world.legacy_raycast(camera_ray.origin, camera_ray.dir);
    if !legacy_raycast_miss(&hit) {
        let subject = geo_src_to_edit_subject(GeoSource::from_hit_handle(hit.handle));
        if !matches!(subject, EditSubject::None) {
            return;
        }
    }
    let position = cursor.position();
    let center = Vec2::new(position.x, position.z);
    if !border.0.is_pos_inside(center) {
        // O：界外 → DisplayOutOfBorder；clicked 时再 FeedbackInputOutOfBorder。
        border.1.write(DisplayOutOfBorder);
        if !ctx.clicked() {
            return;
        }
        border.2.write(FeedbackInputOutOfBorder);
        return;
    }
    // O：界内且 is_active → 指示符。
    // label = format!("decorator_{}", CursorPositionSS)；记录 20 B
    // {1,1,cursor_ss 位型, window.as_vec2().y, config.ui.icon_size_mult}；
    // SignifierFactory::new(label, record, 0x143035160) + write（逐 5 qword）。
    if ctx.is_active() {
        let ss = cursor_ss.0;
        let label = format!("decorator_{ss}");
        let mut record = [0u8; 0x20];
        record[0..8].copy_from_slice(&1u64.to_le_bytes());
        record[8..16].copy_from_slice(&1u64.to_le_bytes());
        record[16..20].copy_from_slice(&ss.x.to_le_bytes());
        record[20..24].copy_from_slice(&ss.y.to_le_bytes());
        record[24..28].copy_from_slice(&window.as_vec2().y.to_le_bytes());
        record[28..32].copy_from_slice(&config.ui.icon_size_mult.to_le_bytes());
        // 原生把 20 B 记录的所有权交给 signifier 记录（`inner`）；core 记录只存
        // 裸指针，故以 leak 保持载荷存活（所有权/唤醒登记 U）。
        let inner = Box::leak(Box::new(record)) as *const u8;
        let factory = SignifierFactory::new(label.as_bytes(), inner, 0x1430_3516_0);
        signifiers.write_factory(&factory);
    }
    if !ctx.clicked() {
        return;
    }
    // O：clicked → position()（第二次采样）→ 水下重映射 → PlaceTreeCmd。
    let mut pos = cursor.position();
    if pos.y < 0.0 {
        pos.y = remap_pos_y_if_in_water(pos.y);
    }
    place_cmds.write(PlaceTreeCmd { position: pos });
}

/// `place_tree` @0x142269970（O 面：消费 PlaceTreeCmd → next_index → fastrand
/// r1 → splitmix 变体 → fastrand r2 → random_tree_prefab → scale/rotation →
/// InsertTreeCmd + History::add）。
///
/// The actual Insert payload retains rotation, placement variant and the
/// preplaced flag. Native EventInstance+0x40 is EventId, not tree height.
pub fn place_tree<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut commands: EventReader<PlaceTreeCmd>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut history: ResMut<History<S>>,
    session: Res<ActiveSession>,
) {
    for cmd in commands.read() {
        let id = storage.next_index().0;
        // O：r1（scale 随机）在 splitmix 之前采样，r2（rotation）在其后。
        let r1 = fastrand::f32();
        let variant = tree_variant(id);
        let r2 = fastrand::f32();
        // O：selector = 树 id 低字节（调用点 RCX=next_index；函数内只用 &1）。
        // deny_load/is_corrupted/drag_started 的宿主资源读取面未接线（登记）。
        let prefab = random_tree_prefab(
            id as u8,
            false,
            session.meta.is_halloween_any(),
            false,
            false,
        );
        // O：scale = r1*0.9+(1-r1)*0.5；rotation = r2*360+(1-r2)*0.4922。
        let scale = r1 * TREE_SCALE_MAX + (1.0 - r1) * TREE_SCALE_MIN;
        let rotation = r2 * TREE_ROTATION_MAX + (1.0 - r2) * TREE_ROTATION_MIN;
        // O：History::add（TreeEdit Place，disc u32@+0x10=0）。
        let edit = EditType::Tree(std::sync::Arc::new(
            country_core::resources::history::tree_edit::TreeEdit::Place(
                country_core::resources::history::tree_edit::TreeEditData {
                    storage_index: Some(country_core::resources::tree_storage::TreeStorageIndex(
                        id,
                    )),
                    id: variant as i32,
                    position: cmd.position,
                    previous_position: glam::Vec3::ZERO,
                    scale,
                    rotation_degrees: rotation,
                    prefab_name: prefab.to_string(),
                    is_preplaced: false,
                },
            ),
        ));
        let _ = history.add(edit);
        // Actual Insert business payload, preserving its separate placement ID.
        inserts.write(InsertTreeCmd {
            id,
            rotation_degrees: rotation,
            variant: variant as i32,
            is_preplaced: false,
            position: cmd.position,
            scale,
            prefab_name: prefab.to_string(),
        });
    }
}

/// `seed_cursor_icon`（行 115，内联；O 名——光标图标种子逻辑并入调用点，
/// 本 crate 无独立业务面）。
pub fn seed_cursor_icon() {}
