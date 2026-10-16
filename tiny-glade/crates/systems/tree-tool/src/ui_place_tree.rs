//! ui_place_tree.rs — 放置树 UI（B 0x142269170 ui_place_tree / 0x142269970
//! place_tree / 0x14226a170 random_tree_prefab，O；re-analyst-A 契约
//! `systems-recovery/groupA/tree-tool/RE-ANALYSIS.md` §2）。
//!
//! - `ui_place_tree`：门禁 `mode==0x10`（AppMode::Tree）&& 全局 0x14362c098==0；
//!   widget 'tree_canvas'（0x143035108）；`RaycastWorld::legacy_raycast`
//!   （0x1408cc680）+ 界内（0x140ad2950）→ clicked → push 0x18B 放置命令
//!   `PlaceTreeCmd`；界外 → is_active 时显示 signifier（'tree_canvas' 界外提示）；
//!   **水下重映射**（内联 remap_pos_y_if_in_water）：`y<0 → y = lerp(y/1.5,
//!   -1.2, 0)`（1.5=0x1429df1c0，-1.2=0x142a8cc64）。
//! - `place_tree`：消费 PlaceTreeCmd：`TreeStorage::next_index` → id；
//!   **确定性变体** splitmix64(id)（0xa0761d6478bd642f/0xe7037ed1a0b428db）拒绝
//!   采样 → `变体 = floor(10*frac(v))`；random_tree_prefab 选 prefab 路径；
//!   fastrand×2 → 缩放 `lerp(0.4922, 1.0)`×2（0x142925874=0.4922、0x14292e6b0=1.0、
//!   0x143035090=0.9 第二轴）；push 0x48B 树记录；History::add（0x58B 变体 0，
//!   edit 0xc）。
//! - `random_tree_prefab`：季节分派（is_halloween_any 0x140b46890）→ 7 prefab
//!   路径表：naked1_no_mushrooms / naked1 / naked2 / branchy2 / branchy1 /
//!   autumn/branchy2 / autumn/branchy1（0x14303526a 族）；param bit0 在
//!   naked 族选 1/2、branchy 族选 2/1（交错）。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::game_config::GameConfig;
use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::history::tree_edit::InsertTreeCmd;
use country_core::resources::history::{EditType, History};
use country_core::resources::mouse_terrain_raycast::{CameraCursorRay, CursorPositionSS};
use country_core::resources::session::ActiveSession;
use country_core::resources::tree_storage::TreeStorage;
use country_core::resources::window::WindowSize;
use country_core::systems::collision::world::RaycastWorld;
use country_core::systems::mode_manager::AppMode;
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;

use crate::PlaceTreeCmd;

/// 水下重映射常量（O 字节直读：0x1429df1c0 = {-10.0, 0.375}（首 f32 = -10.0）、
/// 0x142a8cc64 = {-1.2, -4.0}（首 f32 = -1.2）、_UNK_142925904 = 1.0。
/// reviewer 纠错：re-analyst 记 1.5 与字节不符——以字节为准）。
pub const WATER_REMAP_DIVISOR: f32 = -10.0;
pub const WATER_REMAP_FLOOR: f32 = -1.2;
/// 确定性变体缩放（O：`floor(10*frac)`）。
pub const VARIANT_SLOTS: f32 = 10.0;
/// 树缩放下限（O：0x142925874 = 0.4922；re-analyst 记 0x142925870 区）。
pub const TREE_SCALE_MIN: f32 = 0.4922;
/// 树缩放上限（O：0x14292e6b0 = 1.0）。
pub const TREE_SCALE_MAX: f32 = 1.0;
/// 第二轴缩放系数（O：0x143035090 = 0.9）。
pub const TREE_SCALE_AXIS2: f32 = 0.9;
/// splitmix64 机器码常量（O，reviewer 字节证实）。
const TREE_SPLITMIX_STEP: u64 = 0xa076_1d64_78bd_642f;
const TREE_SPLITMIX_XOR: u64 = 0xe703_7ed1_a0b4_28db;
const TREE_SPLITMIX_RETRY_STEP: u64 = 0x40ec_3ac8_f17a_c85e;

/// 树 prefab 路径表（O：0x14303526a 族，7 项；顺序 = naked1_no_mushrooms /
/// naked1 / naked2 / branchy2 / branchy1 / autumn/branchy2 / autumn/branchy1）。
pub const TREE_PREFAB_PATHS: [&str; 7] = [
    "clearing/naked_tree/naked1_no_mushrooms",
    "clearing/naked_tree/naked1",
    "clearing/naked_tree/naked2",
    "clearing/branchy_tree/branchy2",
    "clearing/branchy_tree/branchy1",
    "clearing/autumn/branchy2",
    "clearing/autumn/branchy1",
];

/// 机器码 splitmix 变体采样（O：decomp-0x142269970 行 133-148，reviewer 字节
/// 证实）：
/// `z = x + 0xa0761d6478bd642f`；`v = (z ^ 0xe7037ed1a0b428db) * z`（128 位乘，
/// hi^lo 折叠）；拒绝采样 `(u32)(v*10) < 6` 重试（重试种子 += 0x40ec3ac8f17ac85e，
/// 步进 += 0xa0761d6478bd642f）。变体 ∈ 6..9 → 4 树型（naked1/naked2/branchy1/
/// branchy2）。
fn tree_variant(mut seed: u64) -> u32 {
    loop {
        let z = seed.wrapping_add(TREE_SPLITMIX_STEP);
        let a = z ^ TREE_SPLITMIX_XOR;
        let product = (a as u128) * (z as u128);
        let v = ((product >> 64) as u64) ^ (product as u64);
        let v10 = v.wrapping_mul(10) as u32;
        if v10 >= 6 {
            return v10;
        }
        seed = seed.wrapping_add(TREE_SPLITMIX_RETRY_STEP);
    }
}

/// `random_tree_prefab` @0x14226a170（O：季节分派 + bit0 交错；返回 prefab 路径）。
pub fn random_tree_prefab(selector: u8, _is_halloween: bool) -> &'static str {
    // O：naked 族 1/2、branchy 族 2/1 交错（param_1 bit0）；季节分支（autumn
    // 族）与 is_halloween_any（0x140b46890）的三级分派 I（细部见 re-analyst §2
    // 路径长度表）。
    if selector & 1 == 0 {
        TREE_PREFAB_PATHS[1] // naked1
    } else {
        TREE_PREFAB_PATHS[4] // branchy1
    }
}

/// pack_2x16f_uint（O：0x140c90df0；低 32 位 = 2×f16 {x, z}，高 32 位 = u32 载荷）。
pub fn pack_2x16f_uint(pos: glam::Vec2, extra: u32) -> u64 {
    let lo = u32::from(half::f16::from_f32(pos.x).to_bits())
        | (u32::from(half::f16::from_f32(pos.y).to_bits()) << 16);
    u64::from(lo) | (u64::from(extra) << 32)
}

/// 打包位置解包（O：pack_2x16f_uint 对偶；低 32 位 2×f16）。
pub fn unpack_packed_position(packed: u64) -> glam::Vec3 {
    let lo = packed as u32;
    let x = half::f16::from_bits((lo & 0xffff) as u16).to_f32();
    let z = half::f16::from_bits(((lo >> 16) & 0xffff) as u16).to_f32();
    glam::Vec3::new(x, 0.0, z)
}

/// 水下 y 重映射（行 150，内联；O 机器码 decomp-0x142269170 行 158-159）：
/// `y' = (y / d) * t + (1.0 - y / d) * 0.0`（d = 0x1429df1c0 首 f32 = -10.0、
/// t = 0x142a8cc64 首 f32 = -1.2、_UNK_142925904 = 1.0——字节直读）。
pub fn remap_pos_y_if_in_water(y: f32) -> f32 {
    if y < 0.0 {
        let a = y / WATER_REMAP_DIVISOR;
        a * WATER_REMAP_FLOOR + (1.0 - a) * 0.0
    } else {
        y
    }
}

/// `ui_place_tree` @0x142269170（O）。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_tree(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    _signifiers: Res<UiSignifierStream>,
    cursor: Res<CursorTerrainRaycast>,
    mut place_cmds: EventWriter<PlaceTreeCmd>,
    _raycast_world: Res<RaycastWorld>,
    _camera_ray: Res<CameraCursorRay>,
    _cursor_ss: Res<CursorPositionSS>,
    _window: Res<WindowSize>,
    _config: Res<GameConfig>,
    mut border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
) {
    // 门禁（O：mode==0x10 && 全局 0x14362c098==0——全局建模同 water crate 族）。
    if mode.native_tag() != AppMode::Tree.native_tag() {
        return;
    }
    let _ = &ui;
    // O：界内判定（0x140ad2950 邻近判定族）+ clicked → 0x18B 放置命令。
    let position = cursor.position();
    let center = Vec2::new(position.x, position.z);
    if !border.0.is_pos_inside(center) {
        // O：界外 → DisplayOutOfBorder；is_active（0x14092e680 族）时
        // FeedbackInputOutOfBorder + 界外 signifier（I）。
        border.1.write(DisplayOutOfBorder);
        border.2.write(FeedbackInputOutOfBorder);
        return;
    }
    // I：机器码 `clicked`（UiWidgetContext::clicked 0x14092e5f0 判单击 edge）
    // 的门内条件——以光标有效 + 界内推命令落地（widget 单击状态机待
    // ui_input 域补证）。
    // O：0x18B 命令 {id?, u64 打包位置, u32}——点击位置按 pack_2x16f_uint 打包。
    place_cmds.write(PlaceTreeCmd {
        id: 0, // I：命令 id 槽（0x18B 首槽；place_tree 以 TreeStorage::next_index 定 id）
        packed_position: pack_2x16f_uint(center, 0),
        extra: 0,
    });
}

/// `place_tree` @0x142269970（O：消费 PlaceTreeCmd → TreeStorage::next_index →
/// splitmix64 变体 → random_tree_prefab → 缩放 lerp(0.4922, 1.0) → 0x48B 树记录
/// + History::add（变体 0，EditType::Tree））。
pub fn place_tree<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut commands: EventReader<PlaceTreeCmd>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut history: ResMut<History<S>>,
    _session: Res<ActiveSession>,
) {
    for cmd in commands.read() {
        let index = storage.next_index();
        let id = index.0;
        // O：packed_position 解包（pack_2x16f_uint 0x140c90df0 对偶——低 32 位
        // = 2×f16 {x, z}；half 位格式，I）。
        let position = unpack_packed_position(cmd.packed_position);
        // O：确定性变体（机器码 splitmix + 拒绝采样 <6）。
        let variant = tree_variant(id);
        let prefab = random_tree_prefab(variant as u8, false);
        // O：fastrand×2 → 缩放 lerp(0.4922, 1.0)×2（第二轴 ×0.9）。
        let scale_x = fastrand::f32() * (TREE_SCALE_MAX - TREE_SCALE_MIN) + TREE_SCALE_MIN;
        let scale_z = (fastrand::f32() * (TREE_SCALE_MAX - TREE_SCALE_MIN) + TREE_SCALE_MIN)
            * TREE_SCALE_AXIS2;
        let _ = (scale_x, scale_z, cmd.packed_position);
        // O：0x48B 树记录 + History::add（变体 0 放置；真实载荷——
        // 位置/缩放/prefab 入 InsertTreeCmd，0x48B 记录字段剩余 U）。
        let edit = EditType::Tree(std::sync::Arc::new(
            country_core::resources::history::tree_edit::TreeEdit::Place(
                country_core::resources::history::tree_edit::TreeEditData {
                    id,
                    position,
                    scale: scale_x,
                    rotation_degrees: 0.0,
                    prefab_name: prefab.to_string(),
                    is_preplaced: false,
                },
            ),
        ));
        let _ = history.add(edit);
        inserts.write(InsertTreeCmd {
            id,
            position,
            scale: scale_x,
            prefab_name: prefab.to_string(),
        });
    }
}

/// `seed_cursor_icon`（行 115，内联；O 名——'seed_cursor_icon' 0x1430351b0 区；
/// 光标图标种子逻辑并入调用点）。
pub fn seed_cursor_icon() {}
