//! B wall spatial membership and segment data. Standard containers replace only
//! dependency storage; endpoint projection and cell coverage retain game policy.

use bevy_ecs::prelude::*;
use hashbrown::{HashMap, HashSet};
use utils::geometry::Rectangle2d;

use crate::resources::walls::decorator_subtype::placement::nearest_segment;
use crate::resources::walls::spatial_index::CELL_COUNT;
use crate::resources::walls::spatial_index::GRID_SIDE;
use crate::resources::walls::spatial_index::WALL_SEGMENT_RADIUS;
use crate::resources::walls::spatial_index::WORLD_SIZE;
use crate::resources::walls::spatial_index::segment_cells;
use crate::resources::walls::{
    WallId,
    public_walls::{OnWallChanged, OnWallDeleted, PublicWallState, PublicWalls},
};
use crate::utils::dirty_tiles::DirtyTiles;

/// The 48-byte game payload at add_segment 0x140aa5dd0 is two Vec2 endpoints,
/// three pairs of floats, and WallId. The separate u32 segment key is generated
/// by the spatial owner. This typed representation does not emulate map ABI.
#[derive(Clone, Copy, Debug)]
pub struct WallSpatialSegment {
    pub endpoints: [[f32; 2]; 2],
    pub source_u: [f32; 2],
    pub relative_heights: [f32; 2],
    pub terrain_heights: [f32; 2],
    pub wall: WallId,
}

impl WallSpatialSegment {
    /// 0x140aa50e0（fresh disasm 2026-10-17，25 指令）：
    /// `t = clamp01(dot(p − a, b − a) / max(|b − a|², 1e-20))`，
    /// 返回 `(1 − t)·h0 + t·h1`。
    /// **偏移**：原生读 `WallSegment+0x18` 的两 f32；本结构按已证布局把 +0x18
    /// 映射到 `relative_heights`（`ivy_grower::wall_segment_from_spatial` 同锚：
    /// `h0@+0x18`）。PDB 名称为 terrain_height，与 +0x18 语义的最终确认登记
    /// （add_segment 形参顺序未逐槽取证）。
    /// SSE 有序语义：`MAXSS 0` / `MINSS 1` 在未排序（含 NaN）时取源操作数。
    pub fn terrain_height_closest_to_ws_pos(&self, p: [f32; 2]) -> f32 {
        let a = self.endpoints[0];
        let b = self.endpoints[1];
        let d = [b[0] - a[0], b[1] - a[1]];
        let q = [p[0] - a[0], p[1] - a[1]];
        let dot = q[0] * d[0] + q[1] * d[1];
        let length_squared = d[0] * d[0] + d[1] * d[1];
        // MAXSS XMM1, [1e-20]：dst>src ? dst : src。
        let denominator = if length_squared > 1e-20 {
            length_squared
        } else {
            1e-20
        };
        let ratio = dot / denominator;
        // MAXSS 0, ratio → dst>src ? dst : src。
        let t = if 0.0 > ratio { 0.0 } else { ratio };
        // MINSS 1, t → dst<src ? dst : src。
        let t = if 1.0 < t { 1.0 } else { t };
        let (h0, h1) = (self.relative_heights[0], self.relative_heights[1]);
        (1.0 - t) * h0 + t * h1
    }
}

#[derive(Resource, Debug)]
pub struct WallSpatialHash {
    cells: Vec<Vec<u32>>,
    segments: HashMap<u32, WallSpatialSegment>,
    wall_segments: HashMap<WallId, Vec<u32>>,
    segment_cells: HashMap<u32, Vec<usize>>,
    next_segment_id: u32,
}

impl Default for WallSpatialHash {
    fn default() -> Self {
        Self {
            cells: vec![Vec::new(); CELL_COUNT],
            segments: HashMap::new(),
            wall_segments: HashMap::new(),
            segment_cells: HashMap::new(),
            next_segment_id: 0,
        }
    }
}

impl WallSpatialHash {
    pub fn remove_wall_id(&mut self, wall: WallId) {
        if let Some(ids) = self.wall_segments.remove(&wall) {
            for id in ids {
                if let Some(cells) = self.segment_cells.remove(&id) {
                    for cell in cells {
                        self.cells[cell].retain(|old| *old != id);
                    }
                }
                self.segments.remove(&id);
            }
        }
    }

    /// 0x140aa5dd0 increments +0x58 before inserting into every covered bucket,
    /// including a segment whose geometry lies outside the strict world bounds.
    pub fn add_segment(&mut self, segment: WallSpatialSegment) -> u32 {
        let id = self.next_segment_id;
        self.next_segment_id = self.next_segment_id.wrapping_add(1);
        let cells = segment_cells(
            segment.endpoints[0],
            segment.endpoints[1],
            WALL_SEGMENT_RADIUS,
        );
        for &cell in &cells {
            self.cells[cell].push(id);
        }
        self.wall_segments.entry(segment.wall).or_default().push(id);
        self.segment_cells.insert(id, cells);
        self.segments.insert(id, segment);
        id
    }

    pub fn segments_for_wall(&self, wall: WallId) -> impl Iterator<Item = &WallSpatialSegment> {
        self.wall_segments
            .get(&wall)
            .into_iter()
            .flatten()
            .filter_map(|id| self.segments.get(id))
    }
    /// B query140aa5730/273B: only the point's current cell, no clamping,
    /// radius, sorting, wall deduplication or whole-wall expansion. Float to
    /// i32 uses native Rust saturation (NaN ->0); -65 is included, +65 outside.
    pub fn segments_at_world_xz(&self, position: [f32; 2]) -> impl Iterator<Item = &WallSpatialSegment> {
        let cell = crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped(position);
        let valid = cell[0] >= 0 && cell[1] >= 0
            && cell[0] < GRID_SIDE as i32 && cell[1] < GRID_SIDE as i32;
        valid.then_some(cell[1].wrapping_mul(GRID_SIDE as i32).wrapping_add(cell[0]) as usize)
            .into_iter().flat_map(move |index| self.segments_in_cell(index))
    }

    pub fn segments_in_cell(&self, cell: usize) -> impl Iterator<Item = &WallSpatialSegment> {
        self.cells
            .get(cell)
            .into_iter()
            .flatten()
            .filter_map(|id| self.segments.get(id))
    }
    pub fn candidates(&self, dirty: &DirtyTiles) -> Vec<WallId> {
        let mut seen = HashSet::new();
        let mut result = Vec::new();
        for cell in dirty.tile_indices() {
            for segment in self.segments_in_cell(cell) {
                if seen.insert(segment.wall) {
                    result.push(segment.wall);
                }
            }
        }
        result
    }

    /// 原 140aa5a10：按格收集完整段，按段 ID 排序并消除跨格重复。
    /// WallId 在原段 +0x28，排序键 +0x30 是段 ID，不能按墙去重：
    /// 同一墙的不同段承载不同方向，形状吸附的法向依赖它们。
    pub fn query_segments_with_aabb_clamped(
        &self,
        min: [f32; 2],
        max: [f32; 2],
    ) -> impl ExactSizeIterator<Item = &WallSpatialSegment> + '_ {
        use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
        // 原调用先把 AABB 裁到世界边界，再由 get_cells_from_aabb 遍历。
        // 不把整个越界区间移动到边格，也不颠倒反向区间。
        let half = WORLD_SIZE * 0.5;
        let lo = min.map(|v| v.max(-half));
        let hi = max.map(|v| v.min(half));
        let lo = world_space_to_grid_coord_unclamped(lo).map(|v| v.max(0));
        let hi = world_space_to_grid_coord_unclamped(hi).map(|v| v.min(GRID_SIDE as i32 - 1));
        let mut ids = Vec::new();
        for z in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                ids.extend_from_slice(&self.cells[z as usize * GRID_SIDE + x as usize]);
            }
        }
        // 标准库排序替代容器依赖；相同段 ID 总是指向同一条段，稳定性不可观察。
        ids.sort_unstable();
        ids.dedup();
        // 返回拥有 ID 缓冲区的借用迭代器，不再额外分配一个引用 Vec。
        ids.into_iter().map(|id| &self.segments[&id])
    }

    /// 保留现有点查询调用面；形状业务使用上面的完整段，独立计算 SnapProposal。
    pub fn query_with_aabb_clamped(&self, min: [f32; 2], max: [f32; 2]) -> Vec<([f32; 2], WallId)> {
        let center = [(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5];
        self.query_segments_with_aabb_clamped(min, max).into_iter()
            .map(|segment| (segment_closest_point_to(segment, center), segment.wall))
            .collect()
    }

    /// Original B1425defd0 specialization used by converted preplaced trees.
    /// Its callback rejects any occupied cell, without the different public
    /// wall-id sort/dedup policy of the 0x140aa5940 family.
    pub fn has_occupied_cells_in_circle(&self, center: glam::Vec2, radius: f32) -> bool {
        use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
        let half = WORLD_SIZE * 0.5;
        let lo = [center.x - radius, center.y - radius].map(|v| v.max(-half));
        let hi = [center.x + radius, center.y + radius].map(|v| v.min(half));
        assert!(
            lo[0] >= -half && lo[1] >= -half,
            "original circle lower bounds"
        );
        assert!(
            hi[0] <= half && hi[1] <= half,
            "original circle upper bounds"
        );
        let cell_size = WORLD_SIZE / GRID_SIDE as f32;
        assert!(cell_size > 0.5, "original wall spatial cell size");
        let lo = world_space_to_grid_coord_unclamped(lo).map(|v| v.max(0));
        let hi = world_space_to_grid_coord_unclamped(hi).map(|v| v.min(GRID_SIDE as i32 - 1));
        let expansion = (cell_size * 0.5) * f32::from_bits(0x3fb5_04f3);
        for z in lo[1]..=hi[1] {
            for x in lo[0]..=hi[0] {
                let middle = |i: i32| {
                    let t = i as f32 / GRID_SIDE as f32;
                    (t * half - (1.0 - t) * half) + cell_size * 0.5
                };
                let dx = middle(x) - center.x;
                let dz = middle(z) - center.y;
                let distance = (dz * dz + dx * dx).sqrt();
                // Native UCOMISS/JBE excludes equality and unordered input.
                if radius + expansion > distance {
                    let index = z as usize * GRID_SIDE + x as usize;
                    if !self.cells[index].is_empty() {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// B 0x140aa5850 (native-only variant): query with a world-space segment.
    /// fresh 反编译：`get_cells_from_segment`（0x1409eff80）收集覆盖格 →
    /// 按墙 id（`WallSegment+0x30` 的 u32）排序 + 相邻同墙去重（每墙只保留排序后首段，
    /// 无距离排序、无半径筛）。与 0x140aa5940 / 0x140aa5ba0 同骨架。
    pub fn query_with_segment(
        &self,
        a: [f32; 2],
        b: [f32; 2],
        radius: f32,
    ) -> Vec<&WallSpatialSegment> {
        let mut candidates: Vec<&WallSpatialSegment> = Vec::new();
        for &cell in &segment_cells(a, b, radius) {
            for segment in self.segments_in_cell(cell) {
                candidates.push(segment);
            }
        }
        sort_and_dedup_by_wall_id(&mut candidates);
        candidates
    }

    /// B 0x140aa5ba0 (native-only variant): query with a Rectangle2d.
    /// fresh 反编译：`get_cells_from_rectangle2d`（0x1409ef210）收集覆盖格 →
    /// 按墙 id（`WallSegment+0x30` 的 u32）排序 + 相邻同墙去重。与 0x140aa5940 同骨架。
    pub fn query_with_rectangle2d(&self, rect: Rectangle2d) -> Vec<&WallSpatialSegment> {
        let mut candidates: Vec<&WallSpatialSegment> = Vec::new();
        for &cell in &rectangle_cells(rect) {
            for segment in self.segments_in_cell(cell) {
                candidates.push(segment);
            }
        }
        sort_and_dedup_by_wall_id(&mut candidates);
        candidates
    }

    /// B 0x140aa5c70 (native-only): world-space center of a cell index.
    /// fresh 反编译：`dims==0 → panic_const_rem_by_zero`；index/dims 拆分
    /// （col = index % dims.x, row = index / dims.x）；中心换算
    /// `(coord + 0.5) * cell - 0.5 * extent`。dims 从 +0x48 读（64），
    /// extent 从 +0x50 读（130），与 shared building modules GRID_SIDE/WORLD_SIZE 一致。
    pub fn cell_center(&self, index: usize) -> [f32; 2] {
        let dims_x = GRID_SIDE as f32;
        if dims_x == 0.0 {
            panic!("attempt to divide by zero");
        }
        let col = index % GRID_SIDE;
        let row = index / GRID_SIDE;
        let cell = WORLD_SIZE / dims_x;
        let center = |coord: usize| (coord as f32 + 0.5) * cell - WORLD_SIZE * 0.5;
        [center(col), center(row)]
    }

    /// B 0x140aa6600 (native-only, 19B): world AABB from extent.
    /// fresh 反编译：读 +0x50 extent（打包两 f32）→ 输出
    /// `{-extent/2, -extent.y/2, +extent.x/2, +extent.y/2}`。extent=130 →
    /// `{-65, -65, 65, 65}`。
    pub fn bounds_ws(&self) -> [f32; 4] {
        let half = WORLD_SIZE * 0.5;
        [-half, -half, half, half]
    }

    pub fn replace_wall(&mut self, id: WallId, state: &PublicWallState) {
        state.assert_valid();
        self.remove_wall_id(id);
        let source = state.simplified_curve_2d();
        assert_eq!(source.points.len(), source.points_u.len());
        let mapped = &state.geometry.outline;
        let relative = &state.geometry.relative_heights;
        assert_eq!(mapped.len(), relative.len());
        let project = |point: [f32; 2]| {
            let (index, t) = nearest_segment(mapped, [point[0], 0.0, point[1]])
                .expect("invalid terrain-mapped wall curve");
            assert!((0.0..=1.0).contains(&t));
            let height = (relative[index + 1] - relative[index]) * t + relative[index];
            let terrain_y = t * mapped[index + 1][1] + (1.0 - t) * mapped[index][1];
            (height, terrain_y)
        };
        for (i, endpoints) in source.points.windows(2).enumerate() {
            let a = project(endpoints[0]);
            let b = project(endpoints[1]);
            self.add_segment(WallSpatialSegment {
                endpoints: [endpoints[0], endpoints[1]],
                source_u: [source.points_u[i], source.points_u[i + 1]],
                relative_heights: [a.0, b.0],
                terrain_heights: [a.1, b.1],
                wall: id,
            });
        }
    }
}

pub fn spatial_hash_update(
    mut changed: EventReader<OnWallChanged>,
    mut deleted: EventReader<OnWallDeleted>,
    public: Res<PublicWalls>,
    mut spatial: ResMut<WallSpatialHash>,
) {
    for &OnWallChanged(id) in changed.read() {
        let Some(state) = public.get(id) else {
            // 0x140d4a330 is GlobalLogger::log (S_PUB32), not a panic helper.
            log::warn!(target: "country_core::systems::wall::wall_state::spatial_hash_update",
                "SpatialQuery received OnWallChanged with wall index {}", id.0);
            continue;
        };
        spatial.replace_wall(id, state);
    }
    for &OnWallDeleted(id) in deleted.read() {
        spatial.remove_wall_id(id);
    }
}

/// 旧圆/矩形查询调用面的按墙聚合适配器，尚不作为原段查询已恢复的证据。
/// 原 141338ac3 实际读段 +0x30 ID；已恢复的 AABB 原查询不经过本适配器。
pub(crate) fn sort_and_dedup_by_wall_id(candidates: &mut Vec<&WallSpatialSegment>) {
    candidates.sort_by_key(|segment| segment.wall.0 as u32);
    candidates.dedup_by_key(|segment| segment.wall.0 as u32);
}

/// 段上到参考点的最近点（fresh 0x140aa5a10 消费端 SnapProposal 语义：段为简化
/// 曲线直边，最近点即原生 `get_approx_coord_from_pos` 的 snap 点）。
fn segment_closest_point_to(segment: &WallSpatialSegment, point: [f32; 2]) -> [f32; 2] {
    let a = glam::Vec2::new(segment.endpoints[0][0], segment.endpoints[0][1]);
    let b = glam::Vec2::new(segment.endpoints[1][0], segment.endpoints[1][1]);
    let p = glam::Vec2::new(point[0], point[1]);
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let nearest = a + ab * t;
    [nearest.x, nearest.y]
}

/// 矩形覆盖格（query_with_rectangle2d 用）。
///
/// 0x1409ef210（fresh 反编译 2026-10-17）：查询矩形 `Rectangle2d::aabb`（轴对齐
/// 包围盒）→ 网格坐标区间（`min=max(min,0)`、`max=min(max,dims-1)`，不反向钳制，
/// 区间空即无格）→ **逐格构造格矩形 `Rectangle2d::new` + `aabb` +
/// `Aabb2::intersects(查询 AABB)` 才算命中**。Rust 此前只按包围盒铺格（未逐格
/// intersects），已补逐格筛。
fn rectangle_cells(rect: Rectangle2d) -> Vec<usize> {
    use crate::resources::walls::spatial_index::GRID_SIDE;
    use crate::resources::walls::spatial_index::grid_coord_to_cell_index;
    use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
    use utils::geometry::Aabb2;
    let bounds = rect.aabb();
    let corners = [
        glam::Vec2::new(bounds.min.x, bounds.min.y),
        glam::Vec2::new(bounds.max.x, bounds.min.y),
        glam::Vec2::new(bounds.min.x, bounds.max.y),
        glam::Vec2::new(bounds.max.x, bounds.max.y),
    ];
    let mut min = [i32::MAX, i32::MAX];
    let mut max = [i32::MIN, i32::MIN];
    for c in corners {
        let g = world_space_to_grid_coord_unclamped([c.x, c.y]);
        min[0] = min[0].min(g[0]);
        min[1] = min[1].min(g[1]);
        max[0] = max[0].max(g[0]);
        max[1] = max[1].max(g[1]);
    }
    // 原生：min 只做 `max(0)`，max 只做 `min(dims-1)`；越界矩形的空区间保持空。
    let min = [min[0].max(0), min[1].max(0)];
    let max = [
        max[0].min(GRID_SIDE as i32 - 1),
        max[1].min(GRID_SIDE as i32 - 1),
    ];
    if min[0] > max[0] || min[1] > max[1] {
        return Vec::new();
    }
    let cell = WORLD_SIZE / GRID_SIDE as f32;
    let half = cell * 0.5;
    let mut out = Vec::new();
    for z in min[1]..=max[1] {
        for x in min[0]..=max[0] {
            let center = [
                (x as f32 + 0.5) * cell - WORLD_SIZE * 0.5,
                (z as f32 + 0.5) * cell - WORLD_SIZE * 0.5,
            ];
            let cell_bounds = Aabb2 {
                min: glam::Vec2::new(center[0] - half, center[1] - half),
                max: glam::Vec2::new(center[0] + half, center[1] + half),
            };
            if cell_bounds.intersects(bounds) {
                out.push(grid_coord_to_cell_index([x, z]) as usize);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn segment(heights: [f32; 2]) -> WallSpatialSegment {
        WallSpatialSegment {
            endpoints: [[0.0, 0.0], [10.0, 0.0]],
            source_u: [0.0, 1.0],
            relative_heights: heights,
            terrain_heights: [100.0, 200.0],
            wall: WallId(1),
        }
    }

    #[test]
    fn bounded_raw_query_preserves_same_wall_segments_orders_ids_and_unloads() {
        let mut spatial=WallSpatialHash::default();
        for i in 0..100 {
            spatial.add_segment(WallSpatialSegment{endpoints:[[0.,0.],[1.,i as f32*0.001]],source_u:[0.,1.],relative_heights:[0.;2],terrain_heights:[0.;2],wall:WallId(42)});
        }
        for (lo,hi) in [([-2.,-2.],[2.,2.]),([-100.,-100.],[100.,100.])] {
            let rows:Vec<_>=spatial.query_segments_with_aabb_clamped(lo,hi).collect();
            assert_eq!(rows.len(),100,"同墙多段保留，仅去跨格重复的同一段");
            for (i,row) in rows.iter().enumerate(){assert_eq!(row.endpoints[1][1].to_bits(),(i as f32*0.001).to_bits());}
        }
        assert!(spatial.query_segments_with_aabb_clamped([70.,70.],[80.,80.]).len()==0);
        assert!(spatial.query_segments_with_aabb_clamped([-80.,-80.],[-70.,-70.]).len()==0);
        assert!(spatial.query_segments_with_aabb_clamped([5.,5.],[-5.,-5.]).len()==0);
        spatial.remove_wall_id(WallId(42));assert!(spatial.query_segments_with_aabb_clamped([-2.,-2.],[2.,2.]).len()==0);
    }

    #[test]
    fn terrain_height_closest_to_ws_pos_interpolates_and_clamps() {
        let seg = segment([1.0, 3.0]);
        assert!((seg.terrain_height_closest_to_ws_pos([5.0, 2.0]) - 2.0).abs() < 1e-6);
        assert!((seg.terrain_height_closest_to_ws_pos([-5.0, 0.0]) - 1.0).abs() < 1e-6);
        assert!((seg.terrain_height_closest_to_ws_pos([20.0, 0.0]) - 3.0).abs() < 1e-6);
        // 零长段：t = 0（MAXSS/MINSS 源操作数语义），返回 h0。
        let zero = WallSpatialSegment {
            endpoints: [[0.0, 0.0], [0.0, 0.0]],
            ..segment([1.0, 3.0])
        };
        assert_eq!(zero.terrain_height_closest_to_ws_pos([1.0, 1.0]), 1.0);
    }

    #[test]
    fn rectangle_cells_empty_when_rect_lies_outside_world() {
        let rect = Rectangle2d {
            axis: glam::Vec2::X,
            center: glam::Vec2::new(-100.0, 0.0),
            size: glam::Vec2::splat(2.0),
        };
        assert!(rectangle_cells(rect).is_empty());
    }

    #[test]
    fn rectangle_cells_fills_cell_range_with_intersection_filter() {
        let rect = Rectangle2d {
            axis: glam::Vec2::X,
            center: glam::Vec2::ZERO,
            size: glam::Vec2::splat(2.0),
        };
        // 格宽 130/64 = 2.03125；±1 覆盖 31、32 两行/列。
        assert_eq!(
            rectangle_cells(rect),
            vec![31 * 64 + 31, 31 * 64 + 32, 32 * 64 + 31, 32 * 64 + 32]
        );
    }
}

#[cfg(test)]
mod original_occupied_circle_fixtures {
    use super::*;
    #[test]
    fn original_b_837_byte_callback_boundary_vectors() {
        // Obtained by running untouched original B1425defd0, including its
        // original geometry/grid helpers, against independently built cells.
        // Full168-case oracle and binary SHA: round12/preplaced-tree-chain/
        // original-wall-fixtures.json. These are oracle outputs, not roundtrips.
        for (center, radius, occupied, reject) in [
            ([0., 0.], 7., None, false),
            ([0., 0.], 7., Some([32, 32]), true),
            ([0., 0.], 7., Some([36, 32]), false),
            ([65., 65.], 7., Some([63, 63]), true),
            ([-65., -65.], 7., Some([0, 0]), true),
            ([72., 72.], 7., Some([63, 63]), false),
            ([80., 80.], 7., Some([63, 63]), false),
            ([0., 0.], 0., Some([32, 32]), false),
        ] {
            let mut hash = WallSpatialHash::default();
            if let Some([x, z]) = occupied {
                // The callback reads cell occupancy only, even with no
                // geometry record associated with the stored key.
                hash.cells[z * GRID_SIDE + x].push(7);
            }
            assert_eq!(
                hash.has_occupied_cells_in_circle(glam::Vec2::from_array(center), radius),
                reject,
                "center={center:?}, radius={radius}, occupied={occupied:?}"
            );
        }
    }
}
