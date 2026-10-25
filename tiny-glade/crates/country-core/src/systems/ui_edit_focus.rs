//! `systems/ui_edit_focus.rs` — GeoSource → EditSubject 映射（B 原版函数体还原）。
//!
//! 证据（2026-10-17 fresh，B patched EXE，sha256 cf048e27…6323）：
//! - `geo_src_to_edit_subject` 0x140977f50（PDB 函数长度 0xA2；Ghidra 函数边界
//!   因跳表未跟随而把后续函数内联进来，172KB 反编译为伪影——本模块以
//!   radare2 原始指令 + 跳表字节为准；见
//!   `recovery/agent-team/work/unlock-geo-src-raycast/r2.result.json`）。
//! - `geo_src_to_edit_subject_w_deco_clutter` 0x140978000（长度 0x1CB）。
//! - 调用点：`ui_place_tree` 0x1422692e4、`hint_focused_edit_hint` 0x14227e086、
//!   `hint_roof_shape` 0x14228512e、`hint_clutter_focused_edit_hint` 0x141c3dcb1。
//!
//! ## 输入（GeoSource）
//!
//! 两个函数都只读一个 24 字节值（调用方把 `RaycastWorldResult.handle[0..3]`
//! 原样拷到栈上再传指针；地形命中由 `terrain_result` 写首字节 `0xB`）。
//! 指令：`movzx r8d, byte ptr [rdx]` → 跳表 `movsxd r8, dword ptr [r9 + r8*4]`。
//! 跳表 17 项（0..=16），基址 0x142ab747c（plain）/ 0x142ab74c0（_w_deco），
//! int32 相对偏移，高 2 字节恒 `0xfdec`。
//!
//! ## 输出（EditSubject）
//!
//! `mode_manager::EditSubject` 原生判别 0..=5 与机器码写面逐项一致：
//! - tag 写 `dword [rcx]`；`Wall(u64)`/`Roof(u64)` 载荷走 `[rcx+8]`；
//! - `DecoClutter{decorator_id:u32}`/`StairPoint{decorator_id:u32}` 载荷走 `[rcx+4]`。
//!
//! 跳表解码（两表逐项相同，只有 tag 4/5 在 `_w_deco` 中改走 deco 分支）：
//!
//! | GeoSource tag | plain | `_w_deco_clutter` |
//! |---|---|---|
//! | 0,1,8,9 | `Wall(u64 @+8)` | 同左 |
//! | 2 | `Roof(u64 @+8)` | 同左 |
//! | 3,12,15,16 | `unreachable!` | 同左 |
//! | 4,5 | `None` | decorator 解析（见下） |
//! | 6,7,10,11 | `None` | 同左 |
//! | 13,14 | `StairPoint{id:u32 @+4}` | 同左 |
//!
//! ## `_w_deco_clutter` 的 tag 4/5 分支（0x140978030..0x140978175）
//!
//! 1. `test r8,r8; je → None`：第三参数（`&DecoratorStorage`）为空即 None；
//! 2. `DecoratorStorage::get` 0x140a1c2c0，地址取 `{[+8] disc, [+0x10] payload}`
//!    （disc 0=Wall(WallId)/1=Terrain/2=Stair，与 `DecoratorStorageAddr` 一致）；
//! 3. `[Decorators+0x48] == 0`（内嵌 decorators map items 计数为 0）→ None；
//! 4. 在内嵌 map（条目 stride 0x50，键 u32）中查 `[+4]` 的 decorator_id；
//!    未命中 → None；
//! 5. 命中条目的 `DecoratorInfo.ty`（记录 +0x46，即 hash 桶 +0x4E）：
//!    `al = ty.wrapping_add(0xC8)`；
//!    - `al == 7`（ty == 0x3F）→ `StairPoint{decorator_id}`（esi=4）；
//!    - `al == 6`（ty == 0x3E）或 `al >= 8`（ty ≤0x37 或 ≥0x40）→
//!      `DecoClutter{decorator_id}`（esi=3）；
//!    - 其余（ty 0x38..=0x3D，窗口/箭孔/烟囱/灯笼/旗）→ `None`。
//!      `cmovb eax,ecx; cmp al,7; je …; cmp eax,6; jne epilogue` 逐句即上式。
//!
//! ## 登记（I/OS，不臆造）
//!
//! - 原版 tag 只有 0..=16；越界字节在原版是跳表越界读（UB）。Rust 侧
//!   `unreachable!`，属安全边界。
//! - tag 3/12/15/16 的 panic 参数含一个 `Display` 实参，原消息串在
//!   `unreachable!` 前缀后为 `ui_edit_focus shouldn't include  into their
//!   raycasts`（注意 `include` 与 `into` 之间两个空格 = 空 Display）。Rust 侧
//!   复刻该静态文本；Display 变体名未恢复。
//! - 第三参数空指针门用 `Option<&DecoratorStorage>` 表达（Rust 引用非空）。
//! - decorator ty == 62（0x3E）在 `DecoratorType::from_native_tag` 中无变体，
//!   但机器码归 `DecoClutter`，本实现按机器码。

use crate::resources::walls::WallId;
use crate::resources::walls::decorator::DecoratorId;
use crate::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use crate::systems::mode_manager::{AppMode, EditSubject};

/// 命中结果前 24 字节（`RaycastWorldResult.handle` / `ColliderRegistryEntry`
/// 头部，或地形 `0xB` 哨兵），即原版 `GeoSource` 的字节投影。
///
/// 只按字节读取，不假设 Rust 枚举重排后的 ABI；raw 布局：
/// `[0] tag`、`[4..8] u32`、`[8..24] u64 ×2`。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GeoSource(pub [u8; 24]);

impl GeoSource {
    /// 从 `RaycastWorldResult.handle` 构造（原版调用方逐字节拷贝同源）。
    pub fn from_hit_handle(handle: [u64; 3]) -> Self {
        let mut bytes = [0u8; 24];
        for (i, word) in handle.iter().enumerate() {
            bytes[i * 8..i * 8 + 8].copy_from_slice(&word.to_le_bytes());
        }
        Self(bytes)
    }

    /// `+0x00`：跳表索引（原版 `movzx r8d, byte ptr [rdx]`）。
    #[inline]
    pub fn tag(self) -> u8 {
        self.0[0]
    }

    /// `+0x04`：u32 载荷（decorator_id / stair point id）。
    #[inline]
    pub fn payload_u32(self) -> u32 {
        u32::from_le_bytes(self.0[4..8].try_into().unwrap())
    }

    /// `+0x08`：qword 载荷（WallId / RoofId；deco 分支为存储地址判别）。
    #[inline]
    pub fn payload_u64(self) -> u64 {
        u64::from_le_bytes(self.0[8..16].try_into().unwrap())
    }

    /// `+0x10`：qword 载荷（deco 分支存储地址 payload）。
    #[inline]
    pub fn payload_u64_hi(self) -> u64 {
        u64::from_le_bytes(self.0[16..24].try_into().unwrap())
    }

    /// `{[+8] disc, [+0x10] payload}` → `DecoratorStorageAddr`
    /// （0=Wall(WallId)/1=Terrain/2=Stair，见 `decorator_storage.rs` 布局）。
    fn storage_addr(self) -> Option<DecoratorStorageAddr> {
        match self.payload_u64() {
            0 => Some(DecoratorStorageAddr::Wall(WallId(self.payload_u64_hi()))),
            1 => Some(DecoratorStorageAddr::Terrain),
            2 => Some(DecoratorStorageAddr::Stair),
            _ => None,
        }
    }
}

/// `geo_src_to_edit_subject` 0x140977f50 的 tag 4/5 分支：原版直接返回
/// `EditSubject::None`（跳表项 = 公共尾声，eax 保持入口的 0）。
#[inline]
fn plain_none() -> EditSubject {
    EditSubject::None
}

/// tag 3/12/15/16 的不可达分支（原版跳到 0x140977f9e / 0x140978177 构 panic）。
#[cold]
#[inline(never)]
fn geo_src_unreachable() -> ! {
    unreachable!("ui_edit_focus shouldn't include  into their raycasts")
}

/// `country_core::systems::ui_edit_focus::geo_src_to_edit_subject`（0x140977f50）。
///
/// 逐 tag 与跳表 0x142ab747c 一致。新函数（非 pub(in)）供 onboarding /
/// tree-tool 侧调用；原版是私有 helper，这里放宽可见性只为跨 crate 接线。
pub fn geo_src_to_edit_subject(src: GeoSource) -> EditSubject {
    match src.tag() {
        // 跳表 idx 0/1/8/9 → 0x140977f6b：`mov rax,[rdx+8]; mov [rcx+8],rax; eax=1`
        0 | 1 | 8 | 9 => EditSubject::Wall(src.payload_u64()),
        // idx 2 → 0x140977f87：同拷贝，eax=2
        2 => EditSubject::Roof(src.payload_u64()),
        // idx 3/12/15/16 → 0x140977f9e（unreachable）
        3 | 12 | 15 | 16 => geo_src_unreachable(),
        // idx 4/5 → 公共尾声，eax=0
        4 | 5 => plain_none(),
        // idx 6/7/10/11 → 公共尾声，eax=0（含地形哨兵 0xB）
        6 | 7 | 10 | 11 => EditSubject::None,
        // idx 13/14 → 0x140977f7a：`mov eax,[rdx+4]; mov [rcx+4],eax; eax=4`
        13 | 14 => EditSubject::StairPoint {
            decorator_id: src.payload_u32(),
        },
        // 原版只有 17 项跳表；越界字节在原版是 UB。
        _ => geo_src_unreachable(),
    }
}

/// `country_core::systems::ui_edit_focus::geo_src_to_edit_subject_w_deco_clutter`
/// （0x140978000）。
///
/// 与 [`geo_src_to_edit_subject`] 的唯一差异是 tag 4/5 走 decorator 解析
/// （0x140978030..0x140978175）；其余 tag 跳表目标逐项相同。
/// `storage` 对应原版第三寄存器参数（`test r8,r8` 空指针门 → `None`）。
pub fn geo_src_to_edit_subject_w_deco_clutter(
    src: GeoSource,
    storage: Option<&DecoratorStorage>,
) -> EditSubject {
    match src.tag() {
        0 | 1 | 8 | 9 => EditSubject::Wall(src.payload_u64()),
        2 => EditSubject::Roof(src.payload_u64()),
        3 | 12 | 15 | 16 => geo_src_unreachable(),
        // idx 4/5 → 0x140978030
        4 | 5 => deco_clutter_subject(src, storage),
        6 | 7 | 10 | 11 => EditSubject::None,
        13 | 14 => EditSubject::StairPoint {
            decorator_id: src.payload_u32(),
        },
        _ => geo_src_unreachable(),
    }
}

/// tag 4/5 的 decorator 解析（0x140978030..0x140978175）。
fn deco_clutter_subject(src: GeoSource, storage: Option<&DecoratorStorage>) -> EditSubject {
    // 0x140978030 `test r8,r8; je → None`
    let Some(storage) = storage else {
        return EditSubject::None;
    };
    // 0x140978054 `DecoratorStorage::get`（0x140a1c2c0）
    let Some(addr) = src.storage_addr() else {
        // 原版地址判别是 Rust 枚举域 {0,1,2}；非法判别不在原版语义内。
        return EditSubject::None;
    };
    let Some(decos) = storage.get(addr) else {
        return EditSubject::None;
    };
    // 0x140978062 `cmp qword [rax+0x48], 0; je → None`（内嵌 map items == 0）
    if decos.decorators().is_empty() {
        return EditSubject::None;
    }
    // 0x140978070..0x1409780ef：hashbrown 查 `[+4]` 键。
    let decorator_id = src.payload_u32();
    let Some(info) = decos.get(DecoratorId(decorator_id)) else {
        return EditSubject::None;
    };
    // 0x14097813d..0x140978175：ty(+0x46) → EditSubject 判别。
    let al = info.ty.wrapping_add(0xc8);
    if al == 7 {
        // 0x14097816d：esi=4
        EditSubject::StairPoint { decorator_id }
    } else if al == 6 || al >= 8 {
        // 0x140978163：esi=3
        EditSubject::DecoClutter { decorator_id }
    } else {
        // al 0..=5（ty 0x38..=0x3D）→ 公共尾声，esi=0
        EditSubject::None
    }
}

/// Terrain-tool endpoint of B `ui_edit_focus` @0x1409771f0. The existing
/// GeoSource converters above retain their separate contract. Other selection
/// branches remain with their current owners; this restores the real terrain
/// right-click producer, rather than deriving completion from mode changes.
fn terrain_focus_input_enabled(
    mode: AppMode,
    navigation: &crate::systems::ui::resources::widget_params::UiNavState,
    actions: &crate::input::action_set::MainActionSet,
    hovered: &crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
) -> bool {
    mode == AppMode::Terrain
        && !(navigation.focus.is_none() && actions.digital[18].just_pressed())
        && actions.digital[1].just_pressed()
        && hovered.tag & 1 != 0
}

/// B @1409779bf and @140977a40 sum y²+x² before z². Keep the scalar order:
/// Vec3::distance may choose a different dot-product reduction.
fn focus_distance(point: glam::Vec3, origin: glam::Vec3) -> f32 {
    let y = point.y - origin.y;
    let x = point.x - origin.x;
    let z = point.z - origin.z;
    ((y * y + x * x) + z * z).sqrt()
}

fn terrain_focus_target(
    camera: &crate::resources::mouse_terrain_raycast::CameraCursorRay,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    hovered: &crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
    geometry: &crate::systems::collision::world::RaycastSimpleResult,
) -> Option<u32> {
    let geometry_distance = if geometry.is_miss() {
        f32::MAX
    } else {
        focus_distance(geometry.position, camera.origin)
    };
    // Native position() returns ZERO for an absent cursor; it does not reject
    // that branch. Ordered strict comparison rejects equal distances and NaNs.
    let terrain_distance = focus_distance(cursor.position(), camera.origin);
    (geometry_distance > terrain_distance && hovered.tag == 1).then_some(hovered.id as u32)
}

/// Uses the real input, UI, collider query and terrain hover resources. Optional
/// host resources defer the system until their actual owners install them;
/// there is no fabricated cursor/raycast/action fallback.
pub fn ui_edit_focus_terrain(
    mode: bevy_ecs::prelude::Res<AppMode>,
    actions: Option<bevy_ecs::prelude::Res<crate::input::action_set::MainActionSet>>,
    ui: Option<bevy_ecs::prelude::Res<crate::systems::ui::resources::ui_state::UiState>>,
    navigation: Option<
        bevy_ecs::prelude::Res<crate::systems::ui::resources::widget_params::UiNavState>,
    >,
    camera: Option<
        bevy_ecs::prelude::Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    >,
    collision: Option<bevy_ecs::prelude::Res<crate::systems::collision::world::RaycastWorld>>,
    hovered: bevy_ecs::prelude::Res<
        crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
    >,
    cursor: Option<
        bevy_ecs::prelude::Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
    >,
    mut requests: bevy_ecs::event::EventWriter<crate::systems::mode_manager::RequestAppModeChange>,
    mut hints: bevy_ecs::event::EventWriter<crate::resources::onboarding::NotifyHintSystemTerrain>,
) {
    use crate::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};
    let (Some(actions), Some(ui), Some(navigation), Some(camera), Some(collision), Some(cursor)) =
        (actions, ui, navigation, camera, collision, cursor)
    else {
        return;
    };
    if !terrain_focus_input_enabled(*mode, &navigation, &actions, &hovered) {
        return;
    }
    // B @140977467..504: actual edit_focus_canvas with volume tag5, max
    // distance in size.x, empty sounds and occlude_by_world byte0.
    ui.add(
        "edit_focus_canvas",
        WidgetParams::new_with_sounds_and_volume(
            UiSounds::empty(),
            WidgetVolumeShape {
                tag: 5,
                layer: 0,
                size: glam::Vec2::new(f32::MAX, 0.0),
                ..Default::default()
            },
        )
        .occlude_by_world(false),
    );
    let geometry = collision.raycast_simple(camera.origin, camera.dir, 0x3f43);
    if let Some(stroke_id) = terrain_focus_target(&camera, &cursor, &hovered, &geometry) {
        // B @140977ce6..d75: request first, then terrain notification byte2.
        requests.write(crate::systems::mode_manager::RequestAppModeChange(
            AppMode::Editing(EditSubject::Terrain(stroke_id)),
        ));
        hints.write(crate::resources::onboarding::NotifyHintSystemTerrain { tag: 2 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::walls::WallId;
    use crate::resources::walls::decorator::{DecoratorId, DecoratorInfo};
    use crate::resources::walls::decorator_storage::{DecoratorStorage, Decorators};

    fn src(tag: u8, u32v: u32, u64v: u64, hi: u64) -> GeoSource {
        let mut b = [0u8; 24];
        b[0] = tag;
        b[4..8].copy_from_slice(&u32v.to_le_bytes());
        b[8..16].copy_from_slice(&u64v.to_le_bytes());
        b[16..24].copy_from_slice(&hi.to_le_bytes());
        GeoSource(b)
    }

    #[test]
    fn plain_table_matches_jump_table() {
        for tag in 0u8..=16 {
            let s = src(tag, 0x11223344, 0x5566778899aabbcc, 0xdeadbeef);
            let got = if matches!(tag, 3 | 12 | 15 | 16) {
                None
            } else {
                Some(geo_src_to_edit_subject(s))
            };
            match tag {
                0 | 1 | 8 | 9 => assert_eq!(got, Some(EditSubject::Wall(0x5566778899aabbcc))),
                2 => assert_eq!(got, Some(EditSubject::Roof(0x5566778899aabbcc))),
                4 | 5 | 6 | 7 | 10 | 11 => assert_eq!(got, Some(EditSubject::None)),
                13 | 14 => assert_eq!(
                    got,
                    Some(EditSubject::StairPoint {
                        decorator_id: 0x11223344
                    })
                ),
                3 | 12 | 15 | 16 => assert_eq!(got, None),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn deco_clutter_table_matches_jump_table() {
        let storage = DecoratorStorage::default();
        for tag in 0u8..=16 {
            let s = src(tag, 1, 0, 0);
            if matches!(tag, 4 | 5) {
                // storage 中无记录 → None（原版同样早退）。
                assert_eq!(
                    geo_src_to_edit_subject_w_deco_clutter(s, Some(&storage)),
                    EditSubject::None
                );
            } else if !matches!(tag, 3 | 12 | 15 | 16) {
                assert_eq!(
                    geo_src_to_edit_subject_w_deco_clutter(s, Some(&storage)),
                    geo_src_to_edit_subject(s)
                );
            }
        }
    }

    fn storage_with(ty: u8) -> (DecoratorStorage, u32) {
        let mut storage = DecoratorStorage::default();
        let addr = DecoratorStorageAddr::Wall(WallId(7));
        let decos = storage.get_or_insert_default(addr);
        let mut info = DecoratorInfo::default();
        info.ty = ty;
        info.decorator_id = 42;
        decos.insert(DecoratorId(42), info);
        (storage, 42)
    }

    #[test]
    fn deco_branch_type_mapping() {
        let cases = [
            (0u8, EditSubject::DecoClutter { decorator_id: 42 }),
            (0x37, EditSubject::DecoClutter { decorator_id: 42 }),
            (0x38, EditSubject::None),
            (0x3d, EditSubject::None),
            (0x3e, EditSubject::DecoClutter { decorator_id: 42 }),
            (0x3f, EditSubject::StairPoint { decorator_id: 42 }),
            (0x40, EditSubject::DecoClutter { decorator_id: 42 }),
        ];
        for (ty, want) in cases {
            let (storage, id) = storage_with(ty);
            let s = src(4, id, 0, 7);
            assert_eq!(
                geo_src_to_edit_subject_w_deco_clutter(s, Some(&storage)),
                want,
                "ty=0x{ty:02x}"
            );
        }
    }

    #[test]
    fn deco_branch_none_gates() {
        let s = src(4, 42, 0, 7);
        assert_eq!(
            geo_src_to_edit_subject_w_deco_clutter(s, None),
            EditSubject::None
        );
        let storage = DecoratorStorage::default();
        assert_eq!(
            geo_src_to_edit_subject_w_deco_clutter(s, Some(&storage)),
            EditSubject::None
        );
        // 地址存在但 id 缺失。
        let (storage, _) = storage_with(0x3f);
        let s2 = src(4, 99, 0, 7);
        assert_eq!(
            geo_src_to_edit_subject_w_deco_clutter(s2, Some(&storage)),
            EditSubject::None
        );
    }

    #[test]
    fn decorators_bucket_lookup_is_by_wall_addr() {
        // dist=1 时 terrain 桶未建 → None（即使 wall 桶有同 id）。
        let (storage, id) = storage_with(0x3f);
        let s = src(4, id, 1, 0);
        assert_eq!(
            geo_src_to_edit_subject_w_deco_clutter(s, Some(&storage)),
            EditSubject::None
        );
    }
}

#[cfg(test)]
mod terrain_focus_producer_tests {
    use super::*;
    use crate::input::action_set::{DigitalActionPhase, DigitalActionState, MainActionSet};
    use crate::resources::{
        cursor_terrain_raycast::CursorTerrainRaycast, mouse_terrain_raycast::CameraCursorRay,
        onboarding::NotifyHintSystemTerrain,
    };
    use crate::systems::ui::resources::{
        ui_state::{UiState, WidgetId},
        widget_params::{UiFocus, UiNavState},
    };
    use crate::systems::{
        collision::world::{RaycastSimpleResult, RaycastWorld},
        mode_manager::{self, RequestAppModeChange},
        terrain::terrain_hover_idx::HoveredTerrainStrokeId,
    };
    use bevy_ecs::prelude::*;
    use glam::Vec3;

    fn phase(n: u32) -> DigitalActionPhase {
        match n {
            0 => DigitalActionPhase::JustPressed,
            1 => DigitalActionPhase::Held,
            2 => DigitalActionPhase::Repeating,
            3 => DigitalActionPhase::JustReleased,
            4 => DigitalActionPhase::Idle,
            _ => panic!("fixture phase"),
        }
    }
    #[test]
    fn terrain_focus_matches_twenty_original_entry_observations() {
        // 0x1409771f0 original entry, 20 resolved dependency cases. The native
        // harness records both real RequestAppModeChange and Terrain byte2.
        // Rows store the original f32 words, including NaN and signed zero.
        let cases: &[([u32; 17], bool)] = &[
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000001, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000002, 0xbf800000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000002, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000002, 0x3f800000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000002, 0x7fc00000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000003, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000004, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000000,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000001, 0x00000000,
                    0x00000000, 0x00000011, 0x00000000, 0x00000000, 0x42c80000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000000, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x3f800000, 0x00000000, 0x00000000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x3f800000, 0x00000000, 0x00000000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x40400000, 0x40800000, 0x40a00000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x40400000, 0x40800000, 0x40c00000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x7fc00000, 0x00000000, 0x00000000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x40400000, 0x40800000, 0x40c00000, 0x7fc00000,
                    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                ],
                false,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x40400000, 0x40800000, 0x40c00000, 0x40400000,
                    0x40800000, 0x40a00000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
            (
                [
                    0x00000000, 0x00000000, 0x00000001, 0x00000001, 0x00000000, 0x00000004,
                    0x00000000, 0x00000006, 0x40400000, 0x40800000, 0x40c00000, 0x80000000,
                    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
                ],
                true,
            ),
        ];
        for (index, (v, expected)) in cases.iter().enumerate() {
            let mut actions = MainActionSet::default();
            actions.digital[1] = DigitalActionState {
                phase: phase(v[0]),
                timer_seconds: f32::from_bits(v[1]),
            };
            actions.digital[18] = DigitalActionState {
                phase: phase(v[5]),
                timer_seconds: f32::from_bits(v[6]),
            };
            let mut navigation = UiNavState::default();
            navigation.focus = UiFocus::new(v[4]);
            let hovered = HoveredTerrainStrokeId {
                tag: u64::from(v[2]),
                id: 17,
            };
            let vec = |s: &[u32]| {
                Vec3::new(
                    f32::from_bits(s[0]),
                    f32::from_bits(s[1]),
                    f32::from_bits(s[2]),
                )
            };
            let cursor = CursorTerrainRaycast {
                tag: v[3],
                pos: vec(&v[11..14]),
                ..Default::default()
            };
            let camera = CameraCursorRay {
                origin: vec(&v[14..17]),
                dir: Vec3::Z,
            };
            let geometry = RaycastSimpleResult {
                handle: [u64::from(v[7]), 0, 0],
                position: vec(&v[8..11]),
                toi: 0.0,
            };
            let actual =
                terrain_focus_input_enabled(AppMode::Terrain, &navigation, &actions, &hovered)
                    .then(|| terrain_focus_target(&camera, &cursor, &hovered, &geometry))
                    .flatten();
            assert_eq!(actual, expected.then_some(17), "original case {index}");
        }
    }

    #[test]
    fn terrain_right_click_uses_real_query_and_request_consumer() {
        use crate::resources::{
            gui::TimeOfDayScreen,
            history::transactions::{HistoryTransactions, ObservedHistoryLedger},
        };
        let mut world = World::new();
        world.insert_resource(AppMode::Terrain);
        let mut actions = MainActionSet::default();
        actions.digital[1].phase = DigitalActionPhase::JustPressed;
        world.insert_resource(actions);
        world.insert_resource(UiNavState::default());
        world.insert_resource(UiState::at_startup());
        world.insert_resource(CameraCursorRay {
            origin: Vec3::ZERO,
            dir: Vec3::Z,
        });
        world.insert_resource(RaycastWorld::new());
        world.insert_resource(HoveredTerrainStrokeId { tag: 1, id: 17 });
        world.insert_resource(CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(3.0, 4.0, 5.0),
            ..Default::default()
        });
        world.init_resource::<Events<NotifyHintSystemTerrain>>();
        world.insert_resource(HistoryTransactions::<(), ()>::from_observed(
            ObservedHistoryLedger {
                entries: vec![],
                cursor: -1,
                undo_anchor: -1,
                resolved_snapshots: vec![],
                observed_count_58: 0,
                mutation_nonce: 0,
            },
        ));
        world.insert_resource(TimeOfDayScreen::default());
        world.insert_resource(mode_manager::ModeManagerUiContext {
            probe: mode_manager::EscMenuProbe {
                hovered: false,
                widget_id: 0,
                hit_tag: 2,
            },
            esc_menu_canvas_id: 55,
        });
        let mut schedule = Schedule::default();
        mode_manager::configure_mode_manager::<(), ()>(&mut world, &mut schedule, false);
        schedule.add_systems(ui_edit_focus_terrain.before(mode_manager::mode_manager::<(), ()>));
        schedule.run(&mut world);
        assert_eq!(
            *world.resource::<AppMode>(),
            AppMode::Editing(EditSubject::Terrain(17))
        );
        let requests = world.resource::<Events<RequestAppModeChange>>();
        assert_eq!(
            requests
                .get_cursor()
                .read(requests)
                .copied()
                .collect::<Vec<_>>(),
            [RequestAppModeChange(AppMode::Editing(
                EditSubject::Terrain(17)
            ))]
        );
        let hints = world.resource::<Events<NotifyHintSystemTerrain>>();
        assert_eq!(
            hints
                .get_cursor()
                .read(hints)
                .map(|e| e.tag)
                .collect::<Vec<_>>(),
            [2]
        );
        let ui = world.resource::<UiState>();
        let widget = ui.widget(WidgetId::from_name("edit_focus_canvas")).unwrap();
        assert_eq!(widget.volume.tag, 5);
        assert_eq!(widget.volume.size.x.to_bits(), f32::MAX.to_bits());
        assert!(!widget.occlude_by_world);
        // A held input in the already focused mode cannot repeat this producer.
        world.resource_mut::<MainActionSet>().digital[1].phase = DigitalActionPhase::Held;
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<NotifyHintSystemTerrain>>().len(), 1);
    }
}
