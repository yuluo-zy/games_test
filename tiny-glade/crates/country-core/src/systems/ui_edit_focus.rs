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

/// `country_core::systems::ui_edit_focus::ToolSelectionCancelled`（PDB 事件类型；
/// `ui_edit_focus` 0x1409771f0 的 EventWriter 之一）。消费者只判断"本帧有无未读事件"：
/// `hint_deselect_tool` 0x1422853c0、`hint_stairs` 0x142285a20、`exponential_back_off`
/// 0x142284800 的读者从不读取载荷，因此这里不声明任何字段。生产者（`ui_edit_focus` 的
/// Wall/Roof/DecoClutter/StairPoint 分支，经 3 张跳表分派）尚未恢复，当前没有代码发送它。
#[derive(bevy_ecs::event::Event, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ToolSelectionCancelled;

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

/// `country_core::systems::ui_edit_focus::TerrainFocusModeOrigin`（PDB `Local`，1 字节）。
/// 原版每帧在 0x14097742c 写 `mode == 10`（`Terrain` 工具），在 0x140977b3c / 0x140977c68 读。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TerrainFocusModeOrigin(pub bool);

/// 把原版 `RequestAppModeChange` 实例里的 `{tag u32 @+0, u32 @+4, u64 @+8}` 还原成 `AppMode`
/// （`0x140977ac5` / `0x140977c74` / `0x140977cde` 三处写入点共用同一布局）。
fn app_mode_from_native(tag: u32, id32: u32, id64: u64) -> AppMode {
    match tag {
        0 => AppMode::Editing(EditSubject::None),
        1 => AppMode::Editing(EditSubject::Wall(id64)),
        2 => AppMode::Editing(EditSubject::Roof(id64)),
        3 => AppMode::Editing(EditSubject::DecoClutter { decorator_id: id32 }),
        4 => AppMode::Editing(EditSubject::StairPoint { decorator_id: id32 }),
        5 => AppMode::Editing(EditSubject::Terrain(id32)),
        0xa => AppMode::Terrain,
        other => unreachable!("RequestAppModeChange tag {other}"),
    }
}

/// B `country_core::systems::ui_edit_focus::ui_edit_focus` @0x1409771f0，3424B，17 个 SystemParam
/// （PDB 参数表见 `reports/module-recovery-2026-10-23/m01-system-params-from-pdb.md`；
/// 第 14..17 个在原版被 `tuple$<..>` 打包）。逐段对应 `uef.asm`：
///
/// 1. **校验当前被编辑对象仍存在**（0x140977250..0x140977426）：
///    `Editing(Wall|Roof)`：`PublicWalls::get`（内联断言 `w.simplified_curve_2d.is_valid()`），
///    缺失 → `RequestAppModeChange(Editing(None))` 并返回；
///    `Editing(DecoClutter|StairPoint)`：仅当 `History` **未在记录**（`[history+0x30] == i64::MIN`
///    的 niche）时查 `DecoratorStorage::lookup_decorator_info`，缺失 → 同上并返回。
/// 2. `TerrainFocusModeOrigin = (mode == Terrain)`（每帧）；`UiNavState.focus` 为 None 且
///    `digital[18]`（ui_cancel）`just_pressed` → `RequestAppModeChange(None)`，当前模式判别 >= 6
///    再写 `ToolSelectionCancelled`，返回。
/// 3. 模式判别 >= 6（工具模式）：`digital[1]`（right_click）`just_pressed`；`Terrain` 且悬停地形
///    笔画（`hovered.tag & 1`）→ 进入 4；否则非 `Photomode(0x13)` → `RequestAppModeChange(None)`
///    + `ToolSelectionCancelled`；返回。
/// 4. 判别 < 6，或 Terrain 悬停：注册 `edit_focus_canvas` 控件；right_click `just_pressed`
///    → 对象选取（`raycast_simple` mask 0x3f43，tag 0x0c 写 `EvRightClickedAutoClutter`，
///    地形比几何近且悬停 tag==1 → 选地形）→ `RequestAppModeChange`（点同一对象 = 取消选中，见
///    `edit_focus_toggle_native_tag`）+ 选中地形时 `NotifyHintSystemTerrain{tag:2}`。
/// 5. 判别 < 6：画布被点击（`clicked()`）→ `raycast(mask 0xfb)`，未命中或命中不可聚焦的 tag
///    （6/7/10/11，或解析为 None 的装饰物）→ `RequestAppModeChange(None)`；随后 `Editing(Wall|Roof)`
///    每帧写 `OutlineWallCmd{wall, strength 1.0, kind 2}`。
#[allow(clippy::too_many_arguments)]
pub fn ui_edit_focus(
    mode: bevy_ecs::prelude::Res<AppMode>,
    ui: bevy_ecs::prelude::Res<crate::systems::ui::resources::ui_state::UiState>,
    navigation: bevy_ecs::prelude::Res<crate::systems::ui::resources::widget_params::UiNavState>,
    actions: bevy_ecs::prelude::Res<crate::input::action_set::ActionSetStates>,
    mut request: bevy_ecs::event::EventWriter<crate::systems::mode_manager::RequestAppModeChange>,
    mut cancelled: bevy_ecs::event::EventWriter<ToolSelectionCancelled>,
    mut outline: bevy_ecs::event::EventWriter<crate::systems::wall::structure_outline::OutlineWallCmd>,
    camera: bevy_ecs::prelude::Res<crate::resources::mouse_terrain_raycast::CameraCursorRay>,
    raycast: bevy_ecs::prelude::Res<crate::systems::collision::world::RaycastWorld>,
    walls: bevy_ecs::prelude::ResMut<crate::resources::walls::public_walls::PublicWalls>,
    decorators: bevy_ecs::prelude::Res<DecoratorStorage>,
    mut autoclutter: bevy_ecs::event::EventWriter<
        crate::systems::ui_clutter::rmb_autoclutter::EvRightClickedAutoClutter,
    >,
    history: bevy_ecs::prelude::Res<crate::resources::history::History>,
    (hovered, cursor, mut hint_terrain, mut origin): (
        bevy_ecs::prelude::Res<crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,
        bevy_ecs::prelude::Res<crate::resources::cursor_terrain_raycast::CursorTerrainRaycast>,
        bevy_ecs::event::EventWriter<crate::resources::onboarding::NotifyHintSystemTerrain>,
        bevy_ecs::system::Local<TerrainFocusModeOrigin>,
    ),
) {
    use crate::systems::collision::world::RAYCAST_WORLD_RESULT_MISS;
    use crate::systems::mode_manager::RequestAppModeChange;
    use crate::systems::ui::resources::widget_params::{UiSounds, WidgetParams, WidgetVolumeShape};
    use crate::systems::wall::structure_outline::OutlineWallCmd;

    let none = AppMode::Editing(EditSubject::None);
    let tag = mode.native_tag();

    // 1. 0x140977250..0x140977426。
    match *mode {
        AppMode::Editing(EditSubject::Wall(id)) | AppMode::Editing(EditSubject::Roof(id)) => {
            // 0x1409772d9：`PublicWalls::get`（内联断言 simplified_curve_2d.is_valid()）。
            if walls.get(WallId(id)).is_none() {
                request.write(RequestAppModeChange(none));
                return;
            }
        }
        AppMode::Editing(EditSubject::DecoClutter { decorator_id })
        | AppMode::Editing(EditSubject::StairPoint { decorator_id }) => {
            // 0x140977281：`cmp 0,[history+0x30]; jno` —— 仅 niche（未在记录）时校验。
            if !history.is_recording()
                && decorators
                    .lookup_decorator_info(DecoratorId(decorator_id))
                    .is_none()
            {
                request.write(RequestAppModeChange(none));
                return;
            }
        }
        _ => {}
    }

    // 2. 0x140977429。
    origin.0 = tag == 10;
    if navigation.focus.is_none() && actions.digital[CANCEL_SLOT].just_pressed() {
        request.write(RequestAppModeChange(none));
        if tag >= 6 {
            cancelled.write(ToolSelectionCancelled);
        }
        return;
    }

    let right_click = actions.digital[RIGHT_CLICK_SLOT];

    // 3. 工具模式（0x1409776ae）。
    if tag >= 6 {
        if tag == 10 && terrain_focus_input_enabled(*mode, &navigation, &actions, &hovered) {
            // T2[just_pressed] + `test byte [hovered],1` → 0x140977467。
        } else {
            if right_click.just_pressed() && tag != 0x13 {
                request.write(RequestAppModeChange(none));
                cancelled.write(ToolSelectionCancelled);
            }
            return;
        }
    }

    // 4. 0x140977467：画布控件，随后按 right_click 相位（T4）分派。
    let canvas = ui.add(
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
    if right_click.just_pressed() {
        focus_attempt(
            &mode,
            &camera,
            &raycast,
            &walls,
            &decorators,
            &hovered,
            &cursor,
            &origin,
            &mut request,
            &mut autoclutter,
            &mut hint_terrain,
        );
    }

    // 5. 0x140977536：只有判别 < 6 才处理点击。
    if tag >= 6 {
        return;
    }
    if canvas.clicked() {
        // 0x14097758b..0x140977593：`Rapier::raycast`（max_toi = f32::MAX, mask 0xfb）。
        let hit = raycast.raycast(camera.origin, camera.dir, 0xfb);
        if hit.discriminant == RAYCAST_WORLD_RESULT_MISS {
            request.write(RequestAppModeChange(none));
        } else {
            // stack90 跳表 0x142ab7438，下标 = 句柄首字节。
            match hit.handle[0] as u8 {
                0 | 1 | 2 | 8 | 9 | 13 | 14 => {}
                6 | 7 | 10 | 11 => {
                    request.write(RequestAppModeChange(none));
                }
                4 | 5 => {
                    // 0x140977e11：`DecoratorStorage::decorator_info` 缺失或 ty 0x38..=0x3d → None。
                    if geo_src_to_edit_subject_w_deco_clutter(
                        GeoSource::from_hit_handle(hit.handle),
                        Some(&decorators),
                    ) == EditSubject::None
                    {
                        request.write(RequestAppModeChange(none));
                    }
                }
                3 | 12 | 15 | 16 => geo_src_unreachable(),
                other => unreachable!("RaycastWorldResult handle tag {other}"),
            }
        }
    }
    // 0x140977640：被编辑的是墙或屋顶时每帧写轮廓命令。
    if let AppMode::Editing(EditSubject::Wall(id)) | AppMode::Editing(EditSubject::Roof(id)) = *mode {
        outline.write(OutlineWallCmd {
            wall: WallId(id),
            strength: 1.0,
            kind: 2,
        });
    }
}

/// `digital[18]` = `ui_cancel`、`digital[1]` = `right_click`（`DIGITAL_NAMES`）；原版读
/// `ActionSetStates+0x90` / `+0x08`。
const CANCEL_SLOT: usize = 18;
const RIGHT_CLICK_SLOT: usize = 1;

/// 0x14097770d..0x140977d75：right_click 按下那一帧的对象选取与模式请求。
#[allow(clippy::too_many_arguments)]
fn focus_attempt(
    mode: &AppMode,
    camera: &crate::resources::mouse_terrain_raycast::CameraCursorRay,
    raycast: &crate::systems::collision::world::RaycastWorld,
    walls: &crate::resources::walls::public_walls::PublicWalls,
    decorators: &DecoratorStorage,
    hovered: &crate::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId,
    cursor: &crate::resources::cursor_terrain_raycast::CursorTerrainRaycast,
    origin: &TerrainFocusModeOrigin,
    request: &mut bevy_ecs::event::EventWriter<crate::systems::mode_manager::RequestAppModeChange>,
    autoclutter: &mut bevy_ecs::event::EventWriter<
        crate::systems::ui_clutter::rmb_autoclutter::EvRightClickedAutoClutter,
    >,
    hint_terrain: &mut bevy_ecs::event::EventWriter<
        crate::resources::onboarding::NotifyHintSystemTerrain,
    >,
) {
    use crate::systems::mode_manager::RequestAppModeChange;

    let geometry = raycast.raycast_simple(camera.origin, camera.dir, 0x3f43);
    // `ebx`（候选类型）与 `r10d` / `r11`（u32 / u64 载荷）。
    let (mut kind, mut id32, mut id64) = (0u32, 0u32, 0u64);
    if !geometry.is_miss() {
        if geometry.handle[0] as u8 == 0x0c {
            // 0x1409777fc：自动杂物，payload = 句柄 +8 的 u64；候选类型保持 None。
            autoclutter.write(
                crate::systems::ui_clutter::rmb_autoclutter::EvRightClickedAutoClutter {
                    entity: geometry.handle[1],
                },
            );
        } else {
            // 0x14097796f：GeoSource 跳表 0x142ab73dc（含 4/5 的装饰物解析）。
            match geo_src_to_edit_subject_w_deco_clutter(
                GeoSource::from_hit_handle(geometry.handle),
                Some(decorators),
            ) {
                EditSubject::None => {}
                EditSubject::Wall(id) => (kind, id64) = (1, id),
                EditSubject::Roof(id) => (kind, id64) = (2, id),
                EditSubject::DecoClutter { decorator_id } => (kind, id32) = (3, decorator_id),
                EditSubject::StairPoint { decorator_id } => (kind, id32) = (4, decorator_id),
                EditSubject::Terrain(id) => (kind, id32) = (5, id),
            }
        }
    }
    // 0x140977a75：地形比几何近且悬停 tag == 1 → 候选改为地形。
    if let Some(stroke) = terrain_focus_target(camera, cursor, hovered, &geometry) {
        kind = 5;
        id32 = stroke;
    }

    // r15d：当前模式判别；Terrain 工具模式下原版在 0x14097772d 把它清零。
    let current = mode.native_tag();
    let current_kind = if current == 10 { 0 } else { current };
    let (cur32, cur64) = match *mode {
        AppMode::Editing(EditSubject::Wall(id)) | AppMode::Editing(EditSubject::Roof(id)) => (0, id),
        AppMode::Editing(EditSubject::DecoClutter { decorator_id })
        | AppMode::Editing(EditSubject::StairPoint { decorator_id }) => (decorator_id, 0),
        AppMode::Editing(EditSubject::Terrain(id)) => (id, 0),
        _ => (0, 0),
    };

    if current_kind == kind {
        // r15eq 跳表 0x142ab7420：同类型时比较 id。
        let same = match current_kind {
            0 => true,
            1 | 2 => cur64 == id64,
            3 | 4 | 5 => cur32 == id32,
            other => unreachable!("current_kind {other}"),
        };
        if same {
            let tag = if current_kind == 5 && origin.0 { 0xa } else { 0 };
            request.write(RequestAppModeChange(app_mode_from_native(tag, id32, id64)));
            return;
        }
    }

    // 0x140977b4d：类型或 id 不同。
    let wall_candidate_missing = (kind == 1 || kind == 2) && walls.get(WallId(id64)).is_none();
    let mut kind_after = kind;
    let new_tag = if wall_candidate_missing {
        // 0x140977c22：候选墙已不存在 → None（当前为地形焦点时按 origin 回到 Terrain）。
        kind_after = 0;
        if current_kind == 5 && origin.0 { 0xa } else { 0 }
    } else if current_kind == 5 && kind == 0 {
        // 0x140977c60..0x140977c71：当前在地形焦点且候选为 None。
        if origin.0 { 0xa } else { 0 }
    } else {
        kind
    };
    request.write(RequestAppModeChange(app_mode_from_native(new_tag, id32, id64)));
    // 0x140977d08：候选是地形 → NotifyHintSystemTerrain 的 byte2。
    if kind_after == 5 {
        hint_terrain.write(crate::resources::onboarding::NotifyHintSystemTerrain { tag: 2 });
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
        use crate::resources::gui::TimeOfDayScreen;
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
        world.insert_resource(crate::resources::history::History::<crate::resources::history::MaybePendingSnapshotId>::empty_glade());
        insert_edit_focus_resources(&mut world);
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
        mode_manager::configure_mode_manager::<crate::resources::history::EditType, crate::resources::history::MaybePendingSnapshotId>(&mut world, &mut schedule, false);
        schedule.add_systems(ui_edit_focus.before(mode_manager::mode_manager::<crate::resources::history::EditType, crate::resources::history::MaybePendingSnapshotId>));
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

    /// Resources the full 17-parameter system needs besides the ones each test sets itself.
    fn insert_edit_focus_resources(world: &mut World) {
        use crate::systems::wall::structure_outline::OutlineWallCmd;
        world.init_resource::<crate::resources::walls::public_walls::PublicWalls>();
        world.init_resource::<DecoratorStorage>();
        world.init_resource::<Events<ToolSelectionCancelled>>();
        world.init_resource::<Events<OutlineWallCmd>>();
        world.init_resource::<Events<crate::systems::ui_clutter::rmb_autoclutter::EvRightClickedAutoClutter>>();
        world.init_resource::<Events<RequestAppModeChange>>();
        world.init_resource::<Events<NotifyHintSystemTerrain>>();
    }

    struct Rig {
        world: World,
        schedule: Schedule,
    }
    impl Rig {
        fn new(mode: AppMode) -> Self {
            let mut world = World::new();
            world.insert_resource(mode);
            world.insert_resource(MainActionSet::default());
            world.insert_resource(UiNavState::default());
            world.insert_resource(UiState::at_startup());
            world.insert_resource(CameraCursorRay { origin: Vec3::ZERO, dir: Vec3::Z });
            world.insert_resource(RaycastWorld::new());
            world.insert_resource(HoveredTerrainStrokeId { tag: 0, id: -1 });
            world.insert_resource(CursorTerrainRaycast::default());
            world.insert_resource(crate::resources::history::History::<crate::resources::history::MaybePendingSnapshotId>::empty_glade());
            insert_edit_focus_resources(&mut world);
            let mut schedule = Schedule::default();
            schedule.add_systems(ui_edit_focus);
            Self { world, schedule }
        }
        fn run(&mut self) {
            self.schedule.run(&mut self.world);
        }
        fn press(&mut self, slot: usize) {
            self.world.resource_mut::<MainActionSet>().digital[slot].phase = DigitalActionPhase::JustPressed;
        }
        fn requests(&mut self) -> Vec<AppMode> {
            self.world.resource_mut::<Events<RequestAppModeChange>>().drain().map(|r| r.0).collect()
        }
        fn cancelled(&mut self) -> usize {
            self.world.resource_mut::<Events<ToolSelectionCancelled>>().drain().count()
        }
    }
    const NONE: AppMode = AppMode::Editing(EditSubject::None);

    /// 1. 被编辑的墙/屋顶已不存在 → Editing(None) 并立即返回（后续取消键也不会再写事件）。
    #[test]
    fn missing_wall_or_roof_subject_requests_none_and_returns() {
        for mode in [AppMode::Editing(EditSubject::Wall(7)), AppMode::Editing(EditSubject::Roof(7))] {
            let mut rig = Rig::new(mode);
            rig.press(CANCEL_SLOT);
            rig.run();
            assert_eq!(rig.requests(), [NONE], "{mode:?}");
            assert_eq!(rig.cancelled(), 0, "提前返回，没有走取消键分支");
        }
    }

    /// 1. 装饰物类焦点：仅在 History 未记录时才校验；缺失 → None。
    #[test]
    fn missing_decorator_subject_requests_none_unless_history_is_recording() {
        for mode in [
            AppMode::Editing(EditSubject::DecoClutter { decorator_id: 5 }),
            AppMode::Editing(EditSubject::StairPoint { decorator_id: 5 }),
        ] {
            let mut rig = Rig::new(mode);
            rig.run();
            assert_eq!(rig.requests(), [NONE], "{mode:?}");
        }
    }

    /// 2. ui_cancel（digital[18]）且无 UI 焦点：编辑模式只写 Editing(None)；工具模式再写
    /// ToolSelectionCancelled；有 UI 焦点时整条被跳过。
    #[test]
    fn ui_cancel_leaves_edit_mode_and_cancels_tools() {
        let mut rig = Rig::new(AppMode::Editing(EditSubject::None));
        rig.press(CANCEL_SLOT);
        rig.run();
        assert_eq!(rig.requests(), [NONE]);
        assert_eq!(rig.cancelled(), 0);

        let mut rig = Rig::new(AppMode::Path);
        rig.press(CANCEL_SLOT);
        rig.run();
        assert_eq!(rig.requests(), [NONE]);
        assert_eq!(rig.cancelled(), 1);

        let mut rig = Rig::new(AppMode::Path);
        rig.press(CANCEL_SLOT);
        rig.world.resource_mut::<UiNavState>().focus = crate::systems::ui::resources::widget_params::UiFocus::TOOLBAR;
        rig.run();
        assert!(rig.requests().is_empty(), "UI 持有焦点时取消键不属于世界");
    }

    /// Repeating 相位按 `timer <= 0` 才算 just_pressed（FUN_14093fa90 的 ucomiss 分支）。
    #[test]
    fn repeating_cancel_needs_a_non_positive_timer() {
        let mut rig = Rig::new(AppMode::Path);
        rig.world.resource_mut::<MainActionSet>().digital[CANCEL_SLOT] =
            crate::input::action_set::DigitalActionState { phase: DigitalActionPhase::Repeating, timer_seconds: 0.1 };
        rig.run();
        assert!(rig.requests().is_empty());
        rig.world.resource_mut::<MainActionSet>().digital[CANCEL_SLOT].timer_seconds = 0.0;
        rig.run();
        assert_eq!(rig.requests(), [NONE]);
    }

    /// 3. 工具模式 right_click：取消工具（Photomode 例外，Terrain 无悬停笔画时同样取消）。
    #[test]
    fn right_click_in_a_tool_mode_cancels_it_except_photomode() {
        for (mode, cancels) in [
            (AppMode::Path, true),
            (AppMode::Terrain, true),
            (AppMode::Water, true),
            (AppMode::Photomode, false),
        ] {
            let mut rig = Rig::new(mode);
            rig.press(RIGHT_CLICK_SLOT);
            rig.run();
            assert_eq!(rig.requests(), if cancels { vec![NONE] } else { vec![] }, "{mode:?}");
            assert_eq!(rig.cancelled(), usize::from(cancels), "{mode:?}");
        }
        // 没按 right_click：什么也不发。
        let mut rig = Rig::new(AppMode::Path);
        rig.run();
        assert!(rig.requests().is_empty());
    }

    /// 4. 编辑模式下 right_click 打到空处，当前焦点是 None → 同类型同 id，仍写 Editing(None)。
    /// 当前焦点是墙而点到空处 → 候选类型 None，写 Editing(None)。
    #[test]
    fn right_click_on_nothing_requests_none() {
        let mut rig = Rig::new(NONE);
        rig.press(RIGHT_CLICK_SLOT);
        rig.run();
        assert_eq!(rig.requests(), [NONE]);
        assert_eq!(rig.world.resource::<Events<NotifyHintSystemTerrain>>().len(), 0);
    }

    /// 编辑模式每帧注册画布；Wall/Roof 焦点每帧写 OutlineWallCmd{wall, 1.0, kind 2}，且需要墙存在。
    #[test]
    fn edit_modes_register_the_canvas_every_frame() {
        let mut rig = Rig::new(NONE);
        rig.run();
        let ui = rig.world.resource::<UiState>();
        let widget = ui.widget(WidgetId::from_name("edit_focus_canvas")).unwrap();
        assert_eq!(widget.volume.tag, 5);
        // 工具模式不注册（除非 Terrain 悬停笔画）。
        let mut rig = Rig::new(AppMode::Path);
        rig.run();
        assert!(rig.world.resource::<UiState>().widget(WidgetId::from_name("edit_focus_canvas")).is_none());
    }
}
