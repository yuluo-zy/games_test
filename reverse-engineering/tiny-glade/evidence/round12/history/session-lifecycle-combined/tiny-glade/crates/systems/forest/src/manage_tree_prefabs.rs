//! manage_tree_prefabs.rs — 森林树管理主系统（B `manage_tree_prefabs` 0x141bf16c0，O）。
//!
//! 每帧两阶段（O，fresh 反编译 2026-10-17 + reviewer 独立复核 2026-10-17）：
//! - **阶段 1（候选评估）**：遍历放置记录（原 stride **0x50**），对每个放置点判定
//!   `keep`：
//!   1. 越界 `|x| > 65.0 || |z| > 65.0`（常量 `0x142a83814 = 65.0`）→ **keep=1**（O 直接
//!      观察：界外放置保留）；
//!   2. 否则 **6 路** `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（XMM3=130.0、
//!      `[RSP+0x20]=130.0`，O）：
//!      - **前 4 路求和** → `clamp(Σ4, 0, 1)` 作**密度项**（O：`ADDSS XMM11..14` +
//!        `MAXSS 0/MINSS 1.0`，0x141bf1d1d-3c）——density>0 → keep=0；
//!      - **后 2 路排除项**（water/garden，具体栅格归属 U）：任一 `> 0` → keep=0；
//!   3. 否则 **rock mask 单次** `TerrainRockMaskSsbo::sample_at_world_xz`（0x1409ea800）
//!      布尔返回（O：`SETA SIL; OR SIL, AL`，0x141bf1d46-4a）——rock → keep=0；
//!   4. 缓冲位（O：`[RBP+0x108]`，来自构造事件迭代 SETBE，0x141bf1b0f）OR 入跳过条件：
//!      **形状/墙段判定仅在「存在构造形状 && density==0 && rock==0」时运行**；
//!   5. 否则 `is_too_close_to_shape`（点形状 `dist < shape_radius+7.0`；矩形
//!      `dist(closest_point_on_perimeter) < 7.0`，常量 `0x142a7cd48=7.0`）→ 过近 → keep=0；
//!   6. 否则 `Circle2d::new(center, 7.0)` + `WallSpatialHash::query_with_circle_callback`
//!      （单态 0x1422785d0；回调 closure_env_0 内逐墙段 `is_too_close_to_shape` 翻转 keep）。
//!   通过则 push `{tree_id u64, keep u8}`（16B 条目）到 `MemCache`。
//! - **阶段 2（执行）**：`utils::rng::init_rng()` 后遍历候选（16B 条目，`{Entity, flag}`）：
//!   - **门控位来源（RE-ANALYSIS §6.1 已补证）**：`placements 资源 +0x18` 处理状态字节
//!     ——`*(char*)(lVar24+0x18)==1`（lVar24=放置资源 Vec 表头）为后续帧门控
//!     （非"事件存在性"）；首帧为 0 → 读记录 `+0x1c` f32（预置随机值）；
//!     帧末 `*(char*)(lVar24+0x18)=1`。当前 country_core `InteractableForestPlacements`
//!     无该字段，由 canonical `InteractableForestPlacements::processed` 承载，会话卸载复位。
//!   - 后续帧（+0x18==1）：`fastrand::f32()` → `r*0.75+(1-r)*0.3`、scale、flag32=0、
//!     jitter=0；首帧：读记录 `+0x1c` f32（jitter）、scale=0、flag32=1.0；
//!   - keep≠0：实体未生成（`Entity::generation==0`）→ `spawn_empty` + `insert
//!     InteractableForestTree` + `insert TreeAmbience`（位置 {x, `TerrainHeightsData::
//!     sample_at_world_xz` 高度+jitter*10.0, z}；高度采样仅当 `-65≤x,z<65`）+ 原子自增
//!     引用计数 + `pack_2x16f_uint` + 写 `SpawnPrefabCmd`（stride 0x50）+ `record[+0x40]=
//!     实体`；实体已生成且 `Query<&TreeMarkedForDeletion>::contains` → `remove::
//!     <TreeMarkedForDeletion>`（warn）；
//!   - keep==0：实体已生成且未标记 → `insert::<TreeMarkedForDeletion>`（panic）；
//!   - 帧末 `placements[+0x18]=1`。
//!
//! 常量：`65.0`（界）/ `7.0`（树-形间距）/ `0.3..0.75`（随机缩放）/ `10.0`（高度抖动）/
//! `130.0`（栅格尺度）。
//!
//! ## ⚠ 与当前 country_core 类型的分歧（未实现面，详见 RESTORE.md）
//!
//! 原放置记录为 **0x50B** `{Arc<String> prefab @0, u64 打包 pos @0x10, f32 高 @0x14,
//! f32 z @0x18, u32 prefab 索引 @0x1c, …实体 @0x40, u8 有树标志 @0x44}`；当前 country_core
//! `InteractableForestPlacements` 仅 `{ records: Vec<InteractableForestTree> }`（0x20B 记录），
//! 无实体回写位、无 init 标志（`+0x18`）、无 `+0x44` 有树标志。`ConstructTimberframeCmd` 载荷
//! 真实为墙形状（ShapeParameters 0x28B），当前承载为 `shape: Vec<u8>`；本实现按长度
//! ≥0x28 解析前 28 B 为 `ShapeParameters` 并**已落地构造形状邻近判定**（点/矩形两路，
//! 2026-10-17 fresh 0x141bf1d6a-0x141bf1dea 重建）。`SpawnPrefabCmd` 真实 stride 0x50
//! 载荷，当前为 `{_gap}`。以下实现还原**控制流与判定分支**，凡依赖上述字段的语句以可承载
//! 字段落地并显式标注分歧，不伪造成真实布局。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::components::TreeMarkedForDeletion;
use country_core::systems::audio::tree_audio::TreeAmbience;
use country_core::resources::events::ConstructTimberframeCmd;
use country_core::resources::terrain_height::{TerrainHeightsData, TerrainHeightSsbo};
use country_core::resources::terrain_rock_mask::{sample_at_world_xz, TerrainRockMaskSsbo};
use country_core::resources::time::GameTime;
use country_core::resources::tree_storage::InteractableForestPlacements;
use country_core::resources::world_raster_defs::{GardenRaster, WaterRaster};
use country_core::systems::paths::PathRastersResources;
use country_core::systems::prefab::SpawnPrefabCmd;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::wall::valid_enclosure::{Shape, ShapeParameters};
use country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash;
use utils::geometry::Rectangle2d;

/// 世界边界（O：0x142a83814 = 65.0）。
pub const WORLD_BOUND: f32 = 65.0;
/// 树-形状最小间距（O：0x142a7cd48 = 7.0）。
pub const TREE_SHAPE_GAP: f32 = 7.0;
/// 新树缩放下限/上限（O：`r*0.75 + (1-r)*0.3`，常量 0x1429edb90 = 0.75 / 0x1429258d8 = 0.3）。
pub const TREE_SCALE_MIN: f32 = 0.3;
pub const TREE_SCALE_MAX: f32 = 0.75;
/// 高度抖动幅度（O：0x142a95db0 高字 = 10.0）。
pub const HEIGHT_JITTER: f32 = 10.0;
/// 栅格采样尺度（O：0x142a7b658 = 130.0）。
pub const RASTER_SCALE: f32 = 130.0;
/// 密度钳制上界（O：0x142925904 = 1.0）。
pub const DENSITY_MAX: f32 = 1.0;

/// 候选缓冲（O：行 33 `impl$0::default`；阶段 1 逐条 push `{tree_id u64 @0, keep u8 @8}`，
/// 16B 条目）。
#[derive(Default)]
pub struct MemCache {
    /// 候选条目（tree_id/entity u64 + keep u8，16B；语义 O）。
    pub candidates: Vec<(u64, u8)>,
}

/// `is_too_close_to_shape`（源行 211，内联；O：宿主 0x141bf1d6a–0x141bf1dea 墙邻近判定区）。
///
/// 原内联体两路：
/// - 点形状（`shape.kind & 1 == 0`）：`dist = |tree_pos - shape_pos|`，
///   过近 = `dist < shape_radius + 7.0`；
/// - 矩形形状（`shape.kind & 1 == 1`）：`dist = |tree_pos - closest_point_on_perimeter|`，
///   过近 = `dist < 7.0`（矩形自身含延展，附加间距即 7.0）。
///
/// 比较算子：`UCOMISS threshold,dist; SETA; XOR 1` → `threshold <= dist` 为 keep，即
/// `dist < threshold` 为过近（O，reviewer 独立复核）。
///
/// ⚠ 形状类型与 `+7.0` 为常量边距还是墙半厚补偿未定（U）；矩形分派
/// （`Rectangle2d::closest_point_on_perimeter` 0x140c9ae20）当前 country_core 无
/// Rectangle2d/Shape 合同——此处实现点族，矩形分派登记待补证。
pub fn is_too_close_to_shape(tree_pos: Vec2, shape_pos: Vec2, shape_radius: f32) -> bool {
    let dist = tree_pos.distance(shape_pos);
    dist < shape_radius + TREE_SHAPE_GAP
}

/// 构造事件载荷 → `ShapeParameters`（0x28B；当前 `ConstructTimberframeCmd.shape` 为
/// `Vec<u8>` typed gap，按长度 ≥ 0x28 解析前 28 B）。
fn construct_shape_of(cmd: &ConstructTimberframeCmd) -> Option<ShapeParameters> {
    let bytes = cmd.shape.as_slice();
    if bytes.len() < 28 {
        return None;
    }
    let mut record = [0u8; 28];
    record.copy_from_slice(&bytes[..28]);
    Some(ShapeParameters::from_observed(record))
}

/// 构造形状邻近判定（`is_too_close_to_shape` 内联体，fresh 0x141bf1d6a-0x141bf1dea）：
/// - **bit0==0**（点形状）：`center` 固定取记录 +4/+8、`radius` +0xc（二进制对任意
///   偶数 tag 均按固定偏移读，不按 `tag*8`）；过近门 `radius+7.0 > dist`；
/// - **bit0==1**（矩形）：`dist(tree, closest_point_on_perimeter) < 7.0`。core 以
///   `Shape::Rectangle(rect).get_closest_u_from_pos` + `rect.get_pos_at_u(u)` 承接
///   0x140c9ae20 的最近点（等价分解）。
///
/// 比较算子 `UCOMISS threshold,dist; SETA; XOR 1`：keep=`threshold<=dist`，即
/// **过近 = `threshold > dist`**；unordered（NaN）时 SETA=0 → 不过近。
fn is_too_close_to_constructed_shape(tree_pos: Vec2, shape: &ShapeParameters) -> bool {
    let b = shape.bytes();
    let le = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    if shape.tag() & 1 == 0 {
        let center = Vec2::new(le(4), le(8));
        let threshold = le(0xc) + TREE_SHAPE_GAP;
        threshold > tree_pos.distance(center)
    } else {
        let rect = Rectangle2d {
            axis: Vec2::new(le(4), le(8)),
            center: Vec2::new(le(0xc), le(0x10)),
            size: Vec2::new(le(0x14), le(0x18)),
        };
        let u = Shape::Rectangle(rect).get_closest_u_from_pos(tree_pos);
        let closest = rect.get_pos_at_u(u);
        TREE_SHAPE_GAP > tree_pos.distance(closest)
    }
}

/// `manage_tree_prefabs` @0x141bf16c0（O；14 参面按 BASELINE/PDB 泛型拼写）。
#[allow(clippy::too_many_arguments)]
pub fn manage_tree_prefabs<C: Send + Sync + 'static>(
    _signifiers: Res<UiSignifierStream>,
    ssbo_library: Res<AssetSsboLibrary>,
    mut placements: ResMut<InteractableForestPlacements>,
    mut commands: Commands,
    mut spawns: EventWriter<SpawnPrefabCmd>,
    marked: Query<&TreeMarkedForDeletion>,
    mut constructs: EventReader<ConstructTimberframeCmd>,
    walls: Res<WallSpatialHash>,
    _time: Res<GameTime>,
    heights: Res<TerrainHeightsData<TerrainHeightSsbo>>,
    path_rasters: PathRastersResources,
    water: Res<WaterRaster>,
    garden: Res<GardenRaster>,
    mut cache: Local<MemCache>,
) {
    cache.candidates.clear();

    // ---------------------------------------------------------------- 阶段 1（O）
    // 逐放置记录评估 keep。⚠ 原记录 stride 0x50、实体/init 字段位在 country_core 缺失
    // （模块头），此处以 `record.position`（Vec3 = x, 高, z）与 `record.id` 承载位置与标识。
    //
    // 构造形状源（fresh 0x141bf1e50-0x141bf1ee0，decoded 2026-10-17）：事件双缓冲各取
    // **末条** 0x28B 记录（新缓冲优先，空则旧缓冲），即 `EventReader::read().last()`
    // （reader 游标 vs 原始缓冲的细微差异登记 I）。仅当「存在构造形状 && density==0 &&
    // rock==0」时运行形状邻近判定（`((empty2&&empty1)|blocked)!=1`）。
    let last_shape = constructs.read().last();
    let has_construct_shape = last_shape.is_some();
    for record in placements.records.iter() {
        let pos = Vec2::new(record.position.x, record.position.z);
        let keep = if pos.x.abs() > WORLD_BOUND || pos.y.abs() > WORLD_BOUND {
            // O：界外（|x|>65 || |z|>65）→ keep=1（保留；界外放置也保留语义）。
            true
        } else {
            // O：6 路 `bilinear_sample_at_world_xz`。
            //   前 4 路（path/cobblestone ×4 密度项）求和 → clamp[0,1]；
            //   后 2 路（water/garden 排除项）任一 >0 → keep=0。
            let mut density = 0.0f32;
            for grid in path_rasters.as_float_grid() {
                density += grid.bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
            }
            let water_level = (**water)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
            let garden_level = (**garden)
                .as_ref()
                .bilinear_sample_at_world_xz(pos.x, pos.y, RASTER_SCALE, RASTER_SCALE);
            if water_level > 0.0 || garden_level > 0.0 {
                // O：排除项任一 > 0 → keep=0（水上/花园淘汰）。
                false
            } else {
                let density = density.clamp(0.0, DENSITY_MAX);
                // O：rock mask 单次布尔（0x1409ea800 返回位；`SETA SIL; OR SIL,AL`）。
                let rock = rock_mask_bool(ssbo_library.as_ref(), pos);
                if density > 0.0 || rock {
                    // O：密度>0 | rock → keep=0（`bVar32 != 0 → bVar33 = 0`）。
                    false
                } else {
                    // O（0x141bf1d20-0x141bf1d44 核）：`bVar32 = 0.0<fVar50|AL`（密度/岩石
                    // 判定）后 `bVar33 = bVar32^1`；形状邻近判定仅在「存在构造形状 &&
                    // density==0 && rock==0」时运行（`((A&&B)|bVar32)!=1`，A&&B=形状
                    // 双缓冲空），随后 `if (bVar33 != 0)` 才运行墙圆查询
                    // `Circle2d::new(x,z,7.0)` + `query_with_circle_callback`（回调逐段
                    // `is_too_close_to_shape` 翻转 keep）。
                    //
                    // 形状源 = 本帧构造形状（最后一条 `ConstructTimberframeCmd`
                    // ShapeParameters 0x28B 记录；fresh 0x141bf1d6a-0x141bf1dea）。
                    let mut blocked = false;
                    if has_construct_shape {
                        if let Some(shape) = last_shape.and_then(construct_shape_of) {
                            blocked = is_too_close_to_constructed_shape(pos, &shape);
                        }
                    }
                    // O：形状判定若已过近，二进制跳过墙查询（bVar33==0 → 不进入
                    // query_with_circle_callback）。
                    if !blocked {
                        walls.query_with_circle(pos, TREE_SHAPE_GAP, |segment, _distance| {
                            // O：closure_env_0 逐墙段形状判定；墙段以中点 + 半径（半厚 U，
                            // 以 0.0 落地）经 `is_too_close_to_shape` 判定。
                            let mid = glam::Vec2::new(
                                (segment.endpoints[0][0] + segment.endpoints[1][0]) * 0.5,
                                (segment.endpoints[0][1] + segment.endpoints[1][1]) * 0.5,
                            );
                            if is_too_close_to_shape(pos, mid, 0.0) {
                                blocked = true;
                            }
                        });
                    }
                    !blocked
                }
            }
        };
        // O：push {tree_id(u64), keep(u8)} 到 MemCache（16B 条目）。
        cache.candidates.push((record.id, keep as u8));
    }

    // ---------------------------------------------------------------- 阶段 2（O）
    // `utils::rng::init_rng()` 初始化 RNG（O；真实调用带寄存器 seed，seed 值 U——以 0 落地）。
    let _rng = utils::rng::init_rng(0);
    let first_frame = !placements.processed;

    for (i, (tree_id, keep)) in cache.candidates.iter().copied().enumerate() {
        // O：init 标志（placements +0x18；canonical processed）决定随机参数来源：
        //   后续帧：`r = fastrand::f32()`，`scale = r*0.75 + (1-r)*0.3`，jitter = 0；
        //   首帧：读放置记录 `+0x1c` f32（预置随机值，此处以 record._gap 低位承载，分歧 U）。
        let (scale, flag32, jitter) = if !first_frame {
            let r = fastrand::f32();
            (r * TREE_SCALE_MAX + (1.0 - r) * TREE_SCALE_MIN, 0u32, 0.0f32)
        } else {
            let jitter = placements
                .records
                .get(i)
                .map(|r| r._gap as f32)
                .unwrap_or(0.0);
            (0.0f32, 1.0f32.to_bits(), jitter)
        };

        if keep != 0 {
            // O（keep=1）：实体未生成（`id >> 32 == 0`）→ spawn；已生成且带
            // `TreeMarkedForDeletion` → remove 标记。
            if tree_id >> 32 == 0 {
                // O：`Commands::spawn_empty()` → `insert::<InteractableForestTree>` +
                // `insert::<TreeAmbience>`。
                // ⚠ InteractableForestTree 组件真实 stride 0x18 {+0x8 幅度, +0xc 进度, +0x10
                // 计时器}；当前 country_core 为 {id, position, scale, _gap}——以可用字段落地：
                // id = 候选标识、scale = 随机缩放、position = 放置位置（布局分歧 U）。
                let pos = placements.records.get(i).map(|r| r.position);
                let entity = commands
                    .spawn_empty()
                    .insert(country_core::resources::tree_storage::InteractableForestTree {
                        id: tree_id,
                        position: pos.unwrap_or_else(|| glam::Vec3::new(0.0, 0.0, 0.0)),
                        scale,
                        _gap: u64::from(flag32),
                    })
                    .id();
                // O：`insert::<TreeAmbience>`（位置 = {x, 地形采样高度 + jitter*10.0, z}）。
                // ⚠ 高度采样仅当 `(-65 <= x < 65) && (-65 <= z < 65)`（O 原指令条件，
                // `-65.0 <= x && x < 65.0 && -65.0 <= z && z < 65.0`）；`jitter*10.0`
                // 仅首帧非零。
                let y = match pos {
                    Some(p)
                        if (-WORLD_BOUND <= p.x && p.x < WORLD_BOUND)
                            && (-WORLD_BOUND <= p.z && p.z < WORLD_BOUND) =>
                    {
                        heights.sample_at_world_xz(p.x, p.z) + jitter * HEIGHT_JITTER
                    }
                    _ => 0.0,
                };
                commands.entity(entity).insert(TreeAmbience {
                    position: glam::Vec3::new(
                        pos.map_or(0.0, |p| p.x),
                        y,
                        pos.map_or(0.0, |p| p.z),
                    ),
                });
                // O：`SpawnPrefabCmd` 真实 0x48 载荷已落 core（prefab/entity/pos/
                // scale/rotation/variant/flags/mode）；本 forest 调用点的字段映射
                // 仍 U（放置记录族未逐字段锚定）→ 先发 Default，登记待取证。
                spawns.write(SpawnPrefabCmd::default());
                // O：真实把新实体写回放置记录 `+0x40`（避免下帧重复生成）；当前 country_core
                // 记录以 `id`（+0x00）承载实体位——偏移分歧 U，生命周期语义等价落地。
                if let Some(record) = placements.records.get_mut(i) {
                    record.id = entity.to_bits();
                }
            } else if marked.contains(Entity::from_bits(tree_id)) {
                // O：`remove::<TreeMarkedForDeletion>`（warn 处理器）——取消删除标记。
                commands
                    .entity(Entity::from_bits(tree_id))
                    .remove::<TreeMarkedForDeletion>();
            }
        } else if tree_id >> 32 != 0 && !marked.contains(Entity::from_bits(tree_id)) {
            // O（keep=0）：实体已生成且未标记 → `insert::<TreeMarkedForDeletion>`（panic 处理器）。
            commands
                .entity(Entity::from_bits(tree_id))
                .insert(TreeMarkedForDeletion);
        }
    }
    placements.processed = true;
}

/// rock mask 布尔（O：`TerrainRockMaskSsbo::sample_at_world_xz` 0x1409ea800 单次调用返回位）。
fn rock_mask_bool(library: &AssetSsboLibrary, pos: Vec2) -> bool {
    let storage = match library.get::<TerrainRockMaskSsbo>() {
        Some(storage) => storage,
        None => return false,
    };
    // ⚠ storage.mapped = `&mut [TerrainRockMaskSsbo]`（u32 newtype，Pod）；`sample_at_world_xz`
    // 取 `&[u32]`，以 bytemuck 按位重解释（分歧 I）。
    let mask: &[u32] = bytemuck::cast_slice(storage.mapped);
    sample_at_world_xz(mask, pos.x, pos.y) != 0
}
