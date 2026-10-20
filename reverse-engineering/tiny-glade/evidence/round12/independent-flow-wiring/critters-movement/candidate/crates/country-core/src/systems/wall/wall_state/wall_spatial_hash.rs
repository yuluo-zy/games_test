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

    /// B 0x140aa5a10 (snapping_to_segments): AABB query returning, per segment
    /// whose cell lies within the clamped world bounds, **the segment's closest
    /// point to the AABB center** as `(candidate point, wall)` pairs. The
    /// consumer (`snapping_to_segments::closest_segment_to_snap_to`) computes
    /// each proposal from a WallSegment's closest point to the query reference
    /// (fresh 0x140b16500 closure: `get_approx_coord_from_pos<Vec3,
    /// Thresholded3D>` + `lerp(point[i], point[i+1], t)`); since segments are
    /// straight edges of the simplified curve, the closest point on the edge
    /// equals the native snap point. Points outside the strict (-65,65) grid
    /// are clamped to the border cells, matching the world->grid clamp.
    /// fresh 反编译：`get_cells_from_aabb` 后 **`sort_unstable_by` 按距离比较器
    /// 排序（复用 query_with_circle 比较器）** + 相邻同墙去重（keep 最近）；消费者
    /// 依赖最近优先。Rust 此前直接遍历 cell 收集段端点（非最近点），已重建为最近点。
    pub fn query_with_aabb_clamped(&self, min: [f32; 2], max: [f32; 2]) -> Vec<([f32; 2], WallId)> {
        use crate::resources::walls::spatial_index::GRID_SIDE;
        use crate::resources::walls::spatial_index::grid_coord_to_cell_index;
        use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
        let clamp_to_cell = |p: [f32; 2]| {
            let g = world_space_to_grid_coord_unclamped(p);
            let g = [
                g[0].clamp(0, GRID_SIDE as i32 - 1),
                g[1].clamp(0, GRID_SIDE as i32 - 1),
            ];
            grid_coord_to_cell_index(g) as usize
        };
        let lo = clamp_to_cell(min);
        let hi = clamp_to_cell(max);
        let (min_x, max_x) = (lo % GRID_SIDE, hi % GRID_SIDE);
        let (min_z, max_z) = (lo / GRID_SIDE, hi / GRID_SIDE);
        // 排序/最近点参考点 = AABB 中心（fresh 0x140aa5a10 复用圆查询比较器）。
        let center = [(min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5];
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for z in min_z..=max_z {
            for x in min_x..=max_x {
                let cell = z * GRID_SIDE + x;
                for segment in self.segments_in_cell(cell) {
                    // 每段取到参考点的最近点（fresh 消费端 SnapProposal 语义）。
                    let closest = segment_closest_point_to(segment, center);
                    // [f32;2] is not Eq/Hash; dedup on bit patterns.
                    let key = ([closest[0].to_bits(), closest[1].to_bits()], segment.wall);
                    if seen.insert(key) {
                        out.push((closest, segment.wall));
                    }
                }
            }
        }
        // 原按距离比较器排序（最近优先），再相邻同墙去重（keep 最近）。
        out.sort_by(|a, b| {
            endpoint_distance2_to(center, a.0).total_cmp(&endpoint_distance2_to(center, b.0))
        });
        let mut seen_walls = HashSet::new();
        let mut deduped = Vec::with_capacity(out.len());
        for item in out {
            if seen_walls.insert(item.1) {
                deduped.push(item);
            }
        }
        deduped
    }

    /// `query_with_circle`（O：0x140aa5940 + 0x1425defd0/0x1425df320/0x1422785d0
    /// 单态族——树域与墙共享的圆查询）。fresh 反编译 0x140aa5940：`get_cells_from_circle`
    /// 收集 WallSegment 指针后 **`sort_unstable_by`（smallsort/ipnsort，圆查询比较器）**
    /// + **相邻同墙去重（keep 最近，按 +0x30 wall id）**；半径筛在回调单态内
    /// （0x1422785d0 等：`d <= radius` 才调用用户回调）。Rust 此前直接遍历 cell
    /// 无序回调——补排序 + 同墙去重（与 query_with_aabb_clamped 重建一致）。
    /// 森林候选评估与树放置共用（间距 7.0 规则）。
    pub fn query_with_circle<F: FnMut(&WallSpatialSegment, f32)>(
        &self,
        center: glam::Vec2,
        radius: f32,
        mut on_hit: F,
    ) {
        let min = [center.x - radius, center.y - radius];
        let max = [center.x + radius, center.y + radius];
        use crate::resources::walls::spatial_index::GRID_SIDE;
        use crate::resources::walls::spatial_index::grid_coord_to_cell_index;
        use crate::resources::walls::spatial_index::world_space_to_grid_coord_unclamped;
        let clamp_to_cell = |p: [f32; 2]| {
            let g = world_space_to_grid_coord_unclamped(p);
            let g = [
                g[0].clamp(0, GRID_SIDE as i32 - 1),
                g[1].clamp(0, GRID_SIDE as i32 - 1),
            ];
            grid_coord_to_cell_index(g) as usize
        };
        let lo = clamp_to_cell(min);
        let hi = clamp_to_cell(max);
        let mut candidates: Vec<(&WallSpatialSegment, f32)> = Vec::new();
        for z in lo / GRID_SIDE..=hi / GRID_SIDE {
            for x in lo % GRID_SIDE..=hi % GRID_SIDE {
                let cell = z * GRID_SIDE + x;
                for segment in self.segments_in_cell(cell) {
                    let d = segment_distance_to_point(segment, center);
                    candidates.push((segment, d));
                }
            }
        }
        // 原按圆心距离排序（最近优先），相邻同墙去重 keep 最近；半径筛随后（同
        // 0x1422785d0 回调单态：d <= radius 才回调）。同墙最近段在半径外则该墙
        // 不回调（与原生 dedup-then-filter 顺序一致）。
        candidates.sort_by(|a, b| a.1.total_cmp(&b.1));
        let mut seen_walls = HashSet::new();
        for (segment, d) in candidates {
            if seen_walls.insert(segment.wall) && d <= radius {
                on_hit(segment, d);
            }
        }
    }

    /// Original B1425defd0 specialization used by converted preplaced trees.
    /// Its callback rejects any occupied cell, without the different public
    /// segment-distance/sort/dedup policy of query_with_circle.
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
    /// `sort_unstable_by`（smallsort/ipnsort，query_with_circle 比较器）+ 相邻
    /// 同墙去重（keep 最近）。与 query_with_circle 同骨架。
    pub fn query_with_segment(
        &self,
        a: [f32; 2],
        b: [f32; 2],
        radius: f32,
    ) -> Vec<&WallSpatialSegment> {
        let mut candidates: Vec<&WallSpatialSegment> = Vec::new();
        let reference = [(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5];
        for &cell in &segment_cells(a, b, radius) {
            for segment in self.segments_in_cell(cell) {
                candidates.push(segment);
            }
        }
        candidates.sort_by(|x, y| {
            segment_distance2_to_point(x, reference)
                .total_cmp(&segment_distance2_to_point(y, reference))
        });
        // 相邻同墙去重（排序后同墙相邻，keep 最近）。
        let mut seen_walls = HashSet::new();
        candidates.retain(|segment| seen_walls.insert(segment.wall));
        candidates
    }

    /// B 0x140aa5ba0 (native-only variant): query with a Rectangle2d.
    /// fresh 反编译：`get_cells_from_rectangle2d`（0x1409ef210）收集覆盖格 →
    /// 排序 + 相邻同墙去重。与 query_with_circle 同骨架。
    pub fn query_with_rectangle2d(&self, rect: Rectangle2d) -> Vec<&WallSpatialSegment> {
        let mut candidates: Vec<&WallSpatialSegment> = Vec::new();
        let reference = [rect.center.x, rect.center.y];
        for &cell in &rectangle_cells(rect) {
            for segment in self.segments_in_cell(cell) {
                candidates.push(segment);
            }
        }
        candidates.sort_by(|x, y| {
            segment_distance2_to_point(x, reference)
                .total_cmp(&segment_distance2_to_point(y, reference))
        });
        let mut seen_walls = HashSet::new();
        candidates.retain(|segment| seen_walls.insert(segment.wall));
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

/// 点到线段距离（圆查询筛选用；语义 I——机器码为 segment 最近点距离判定）。
fn segment_distance_to_point(segment: &WallSpatialSegment, point: glam::Vec2) -> f32 {
    let a = glam::Vec2::new(segment.endpoints[0][0], segment.endpoints[0][1]);
    let b = glam::Vec2::new(segment.endpoints[1][0], segment.endpoints[1][1]);
    let ab = b - a;
    let t = ((point - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    (a + ab * t - point).length()
}

/// 端点到参考点距离平方（query_with_aabb_clamped 排序键，fresh 复用圆查询比较器）。
fn endpoint_distance2_to(reference: [f32; 2], endpoint: [f32; 2]) -> f32 {
    let dx = endpoint[0] - reference[0];
    let dy = endpoint[1] - reference[1];
    dx * dx + dy * dy
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

/// 点到线段最近距离平方（query_with_segment/rectangle2d 排序键；圆查询比较器
/// 语义复用——机器码以 segment 最近点距离判定）。
fn segment_distance2_to_point(segment: &WallSpatialSegment, point: [f32; 2]) -> f32 {
    let a = glam::Vec2::new(segment.endpoints[0][0], segment.endpoints[0][1]);
    let b = glam::Vec2::new(segment.endpoints[1][0], segment.endpoints[1][1]);
    let p = glam::Vec2::new(point[0], point[1]);
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_squared()).clamp(0.0, 1.0);
    let nearest = a + ab * t;
    let dx = nearest.x - p.x;
    let dy = nearest.y - p.y;
    dx * dx + dy * dy
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
