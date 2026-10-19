//! B `ui_systems/ui_change_stair_height.rs` — 楼梯高度调整（0x1407adea0，O）。
//!
//! 门控（O）：`dispatcher.state==4 && payload==0` 且 `dispatcher.ty=='?'`
//! （`[*gate+0x5c]=='?'` 观察）；`is_active` 检查；拖动相位<2 → 复位；
//! `lookup_storage` + `decorator_info`（0x140a1c3e0/0x140a1c5f0）→ 按目的 tag
//! 跳表 `0x142a832b0` 分派。
//!
//! **跳表修正（2026-10-19 fresh capstone，O）**：表基址 = `0x142a832b0`，
//! 索引公式 `(tag-2)<3 ? tag-1 : 0`：
//! - idx0（tag 0/1，墙）→ `0x1407ae26f` = `unreachable!()`
//!   （"internal error: entered unreachable code" 0x142a83270）。
//! - idx1（tag 2，屋顶）→ `0x1407adfe2`：`PublicWalls::get` +
//!   `PublicWallState::flat_roof_y`（0x140b2a6e0）+ `CachedDecoratorTransforms::
//!   position`（0x140a1a9f0）→ `SwizzlerXVY::xvy` 得世界位置。
//! - idx2（tag 3，地形）→ `0x1407ae060`：`DecoratorInfo+0x00` 前 8B = Vec2(x,z)，
//!   `TerrainHeightFloatGrid::bilinear_sample_at_world_xz`（0x140257ba0）→ xvy。
//! - idx3（tag 4，楼梯）→ `0x1407ae26f` = `unreachable!()`。
//! 旧注释把处理器地址记为 `0x1407adba7/0x1407ad998/0x1407ad91a` 且把 2/3
//! 语义对调，已按原字节纠正（`evidence/round11/contracts/dec-decorator-CONTRACT.md`
//! §2.0 的目标地址有 0x400 偏移笔误，以本文件 capstone 复核为准）。
//!
//! 滑块（O，`Slider3d::update<ref<UiWidgetContext>, closure_env$0>` 0x141276b20）：
//! - 相位 = `(dragged-2)<3 ? dragged-2 : 3`；0=悬停、1=拖中、2/3=释放。
//! - 拖中：线段 origin = 当前世界位置、direction = `(0,1,0)`
//!   （0x1429345b0 = {0.0f,1.0f}），`offset = proj.y - 当前高度`。
//! - 释放：`new = clamp(proj.y - offset, 0.0, 100.0)`（区间常量
//!   0x142a831a0 = {0.0f,100.0f}），构造原地 change op（`+0x10` 哨兵 3）→
//!   `apply_decorator_op`（0x141276e30，8 参）。
//! - op.info 尾部（O）：`u30 = 1 | (height_bits<<32)`、`u38=0`、`u40=0`、
//!   `rank=2`、`u45=0x40`。
//!
//! **core 模型适配（登记）**：core 的 `DecoratorExtraInfo` 编码把载荷放在
//! `dst_payload/u18/u20`（`extra_disc` = `dst_payload` 高位字节，章节已登记为
//! 偏移偏离）。因此高度既写原生 `u30` 低 dword，也同步 core 载荷首 f32，
//! 保证 `DecoratorInfo::extra().as_stair()` 消费者可见。
use bevy_ecs::prelude::*;
use country_core::resources::mouse_terrain_raycast::CursorPositionSS;
use country_core::resources::terrain_height::TerrainHeightsData;
use country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms;
use country_core::resources::walls::decorator::{
    DecoratorExtraInfo, DecoratorId, DecoratorInfo, DeferredDecoratorOpEvents, StairExtra,
};
use country_core::resources::walls::decorator_archivist::DecoratorArchivist;
use country_core::resources::walls::decorator_storage::{DecoratorStorage, DecoratorStorageAddr};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::window::WindowSize;
use country_core::systems::camera_rig::main_camera::MainCamera;
use country_core::systems::ui::resources::slider3d::{Slider3dMatrices, Slider3dState};
use country_core::systems::ui::resources::ui_state::{UiState, WidgetId};
use glam::{Vec2, Vec3};

use crate::apply_decorator_op::{apply_decorator_op, DecoratorOp};
use crate::deferred_decorator_history::DeferredDecoratorEditHistory;
use crate::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher;
use crate::ui_systems::MoveDecorationInteractionState;

/// 高度滑块区间（O：0x142a831a0 = {0.0f, 100.0f}）。
pub const STAIR_HEIGHT_RANGE: [f32; 2] = [0.0, 100.0];

/// 高度载荷在 `u30` 更新判别（O：`info.u30 低 32 位 == 1`）。
pub const STAIR_HEIGHT_UPDATE_TAG: u64 = 1;

/// 每系统滑块状态（O：`Slider3d` 状态 + 拖中偏移 f32@+0x18）。
#[derive(Clone, Copy, Debug)]
pub struct StairHeightSliderState {
    /// 线段起点（世界）。
    pub origin: Vec3,
    /// 线段方向（拖中写 `(0,1,0)`；O 常量 0x1429345b0）。
    pub direction: Vec3,
    /// 拖中记录的 `proj.y - 当前高度`（O：`*(float*)(state+0x18)`）。
    pub offset: f32,
}

impl Default for StairHeightSliderState {
    fn default() -> Self {
        // 与 core Slider3dState::default 一致；拖中帧会立即覆盖。
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::X,
            offset: 0.0,
        }
    }
}

impl StairHeightSliderState {
    /// `Slider3dState::project_cursor_onto_line`（0x140a9bd40，core O）。
    fn project(&self, cursor: Vec2, window: &WindowSize, camera: &MainCamera) -> Vec3 {
        let inverse_projection = camera.read_matrix(0x40);
        let inverse_view = camera.read_matrix(0xc0);
        // `project_onto_ray` 语义：`view_to_clip * (world_to_view * p)`。
        // Slider3dMatrices.second = view_to_clip、.first = world_to_view。
        let matrices = Slider3dMatrices {
            second: inverse_projection.inverse(),
            gap: [0; 0x40],
            first: inverse_view.inverse(),
        };
        Slider3dState {
            origin: self.origin,
            direction: self.direction,
        }
        .project_cursor_onto_line(cursor, window.as_dims(), &matrices)
    }
}

/// B 0x1407adea0（O 结构）：楼梯高度调整主系统。
#[allow(clippy::too_many_arguments)]
pub fn ui_change_stair_height<C: Send + Sync + 'static>(
    ui: Res<UiState>,
    dispatcher: Res<DecoratorAffordanceDispatcher>,
    transforms: Res<CachedDecoratorTransforms>,
    walls: Res<PublicWalls>,
    terrain: Res<TerrainHeightsData>,
    cursor: Res<CursorPositionSS>,
    window: Res<WindowSize>,
    camera: Res<MainCamera>,
    mut storage: ResMut<DecoratorStorage>,
    mut archivist: ResMut<DecoratorArchivist<C>>,
    mut history: ResMut<DeferredDecoratorEditHistory>,
    events: Res<DeferredDecoratorOpEvents>,
    mut interaction: ResMut<MoveDecorationInteractionState>,
    mut slider: Local<StairHeightSliderState>,
) {
    if !(dispatcher.gated(4) && dispatcher.ty == b'?') {
        return;
    }
    let widget = WidgetId(dispatcher.payload);
    if !ui.widget_context(widget).is_active() {
        return;
    }
    let dragged = ui.widget_context(widget).dragged();
    if dragged < 2 {
        interaction.reset_interaction();
    }
    let decorator_id = DecoratorId(dispatcher.payload as u32);
    let Some((addr, info)) = lookup_info(&storage, decorator_id) else {
        panic!("ui_change_stair_height: no decorator info (ui_change_stair_height.rs:40)");
    };
    let info = *info;

    // O：跳表分派（tag 2 屋顶 / tag 3 地形；0/1/4 与其它 = unreachable）。
    let world_pos = match info.dst_tag() {
        2 => roof_height_position(&walls, &transforms, &addr, decorator_id),
        3 => terrain_height_position(&terrain, &info),
        _ => unreachable!(),
    };

    // O：`as_stair`（0x1408394b0）首个 f32 = 当前高度。
    let current_height = stair_height_of(&info);
    let phase = if dragged.wrapping_sub(2) < 3 {
        dragged - 2
    } else {
        3
    };
    match phase {
        // 释放（O：bVar9∈{2,3} → 构造 op + apply）。
        2 | 3 => {
            let projected = slider.project(cursor.0, &window, &camera);
            let new_height = (projected.y - slider.offset)
                .clamp(STAIR_HEIGHT_RANGE[0], STAIR_HEIGHT_RANGE[1]);
            let op = build_stair_height_op(addr, decorator_id, new_height);
            apply_decorator_op(
                &op,
                u8::from(dragged >= 2),
                addr,
                &mut storage,
                Some(&mut *archivist),
                Some(&mut *history),
                &events,
            );
            interaction.reset_interaction();
        }
        // 拖中（O：origin/方向初始化 + offset = proj.y - 当前高度）。
        1 => {
            slider.origin = world_pos;
            slider.direction = Vec3::Y;
            let projected = slider.project(cursor.0, &window, &camera);
            slider.offset = projected.y - current_height;
        }
        // 悬停（O：仅项目投影，不写 op）。
        _ => {}
    }
}

fn lookup_info<'a>(
    storage: &'a DecoratorStorage,
    id: DecoratorId,
) -> Option<(DecoratorStorageAddr, &'a DecoratorInfo)> {
    let addr = storage.lookup_storage(id)?;
    let info = storage.get(addr)?.get(id)?;
    Some((addr, info))
}

/// 屋顶处理器（0x1407adfe2，O）：`as_wall`（0x140a1c240，非墙 panic）→
/// `PublicWalls::get`（未命中 panic）→ `flat_roof_y` → 缓存位置 → xvy。
pub fn roof_height_position(
    walls: &PublicWalls,
    transforms: &CachedDecoratorTransforms,
    addr: &DecoratorStorageAddr,
    id: DecoratorId,
) -> Vec3 {
    let wall = addr.as_wall();
    let state = walls
        .get(wall)
        .unwrap_or_else(|| panic!("ui_change_stair_height: missing public wall state"));
    let y = state.flat_roof_y();
    let position = transforms.position(id.0);
    // SwizzlerXVY::xvy：(x,z) + y → (x,y,z)。
    Vec3::new(position.x, y, position.z)
}

/// 地形处理器（0x1407ae060，O）：`DecoratorInfo+0x00` 前 8B = Vec2(x,z)，
/// 对地形网格做双线性采样（0x140257ba0）。
pub fn terrain_height_position(terrain: &TerrainHeightsData, info: &DecoratorInfo) -> Vec3 {
    let x = f32::from_bits(info.dst_wall as u32);
    let z = f32::from_bits((info.dst_wall >> 32) as u32);
    let y = terrain.sample_at_world_xz(x, z);
    Vec3::new(x, y, z)
}

/// 当前楼梯高度（O：`as_stair` 返回载荷首个 f32；core 编码在
/// `dst_payload/u18/u20`，见文件头适配说明）。
pub fn stair_height_of(info: &DecoratorInfo) -> f32 {
    let extra = info.extra().as_stair();
    f32::from_le_bytes(extra.0[0..4].try_into().unwrap())
}

/// 构造原地高度 change op（O：`+0x10` 哨兵 3；`u30 = 1 | height<<32`；
/// `rank=2`、`u45=0x40`）。同时同步 core 载荷首 f32（见文件头）。
pub fn build_stair_height_op(
    addr: DecoratorStorageAddr,
    id: DecoratorId,
    height: f32,
) -> DecoratorOp {
    let (disc, payload) = addr_disc_payload(addr);
    DecoratorOp {
        kind: disc,
        info: DecoratorInfo {
            // 原生 op.info.dst_wall = 存储地址载荷；地形装饰的位置另由
            // 既有记录承载（op.info 未填字段在原版为未定义读面）。
            dst_wall: payload,
            // O：原生第二定位器判别 = 3（目标即源，哨兵）。
            dst_secondary: 3,
            // core 载荷编码：StairExtra 首 f32 = 新高度（disc 字节为 0）。
            dst_payload: u64::from(height.to_bits()),
            decorator_id: id.0,
            u30: STAIR_HEIGHT_UPDATE_TAG | (u64::from(height.to_bits()) << 32),
            rank: 2,
            u45: 0x40,
            ..DecoratorInfo::default()
        },
        extra: [0; 0x20],
        // 原生 `+0x10 == 3` → 原地更新。
        dst: None,
    }
}

/// 存储地址 → 原生 `(判别, 载荷)`。
pub fn addr_disc_payload(addr: DecoratorStorageAddr) -> (u64, u64) {
    match addr {
        DecoratorStorageAddr::Wall(w) => (0, w.0),
        DecoratorStorageAddr::Terrain => (1, 0),
        DecoratorStorageAddr::Stair => (2, 0),
    }
}

/// 把新高度同步进 core 的 `DecoratorExtraInfo` 编码（见文件头适配说明）。
/// 保留既有载荷其余字节（楼梯边 id 等）。
pub fn sync_core_stair_extra(entry: &mut DecoratorInfo, height: f32) {
    let mut raw: [u8; 20] = entry.extra().as_stair().0;
    raw[0..4].copy_from_slice(&height.to_le_bytes());
    entry.set_extra(DecoratorExtraInfo::Stair(StairExtra(raw)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::resources::walls::WallId;

    fn terrain_info(x: f32, z: f32) -> DecoratorInfo {
        let packed = u64::from(x.to_bits()) | (u64::from(z.to_bits()) << 32);
        DecoratorInfo {
            dst_wall: packed,
            u20: 3 << 16, // dst_tag = 3（地形）
            ..DecoratorInfo::default()
        }
    }

    #[test]
    fn terrain_handler_reads_packed_dst_wall_xz() {
        let info = terrain_info(12.5, -7.25);
        assert_eq!(info.dst_tag(), 3);
        let x = f32::from_bits(info.dst_wall as u32);
        let z = f32::from_bits((info.dst_wall >> 32) as u32);
        assert_eq!((x, z), (12.5, -7.25));
    }

    #[test]
    fn height_op_is_in_place_change_with_height_in_u30_high_word() {
        let op = build_stair_height_op(DecoratorStorageAddr::Wall(WallId(42)), DecoratorId(7), 3.5);
        assert_eq!(op.kind, 0, "wall storage change kind = disc 0");
        assert_eq!(op.dst, None, "native +0x10 sentinel 3");
        assert_eq!(op.info.dst_secondary, 3);
        assert_eq!(op.info.decorator_id, 7);
        assert_eq!(op.info.u30 as u32, 1);
        assert_eq!((op.info.u30 >> 32) as u32, 3.5f32.to_bits());
        assert_eq!(op.info.rank, 2);
        assert_eq!(op.info.u45, 0x40);
        // core 载荷编码同步：首 f32 可被 extra().as_stair() 读回。
        assert_eq!(stair_height_of(&op.info), 3.5);
    }

    #[test]
    fn sync_core_stair_extra_preserves_edge_ids() {
        let mut raw = [0u8; 20];
        raw[0..4].copy_from_slice(&1.0f32.to_le_bytes());
        raw[8..12].copy_from_slice(&11u32.to_le_bytes());
        raw[12..16].copy_from_slice(&22u32.to_le_bytes());
        let mut info = DecoratorInfo::default();
        info.set_extra(DecoratorExtraInfo::Stair(StairExtra(raw)));
        sync_core_stair_extra(&mut info, 6.25);
        let out = info.extra().as_stair().0;
        assert_eq!(f32::from_le_bytes(out[0..4].try_into().unwrap()), 6.25);
        assert_eq!(u32::from_le_bytes(out[8..12].try_into().unwrap()), 11);
        assert_eq!(u32::from_le_bytes(out[12..16].try_into().unwrap()), 22);
    }

    #[test]
    fn terrain_addr_maps_to_disc_one() {
        assert_eq!(addr_disc_payload(DecoratorStorageAddr::Terrain), (1, 0));
        assert_eq!(addr_disc_payload(DecoratorStorageAddr::Stair), (2, 0));
        assert_eq!(
            addr_disc_payload(DecoratorStorageAddr::Wall(WallId(9))),
            (0, 9)
        );
    }
}
