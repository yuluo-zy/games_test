//! autoclutter 模块（B 主基线；AutoClutterState 布局与支持方法）。
//!
//! 布局证据（fresh 反编译 2026-10-17，--worker shared；0x140ee3fd0/48c0/4a70/4bf0）：
//! - `AutoClutterState`：+0x08/+0x10 懒网格 Vec（cell 记录 stride 0x18）；+0x20/+0x28
//!   `Vec<AutoClutterCandidate>`（stride 0x98）；+0x30/+0x38/+0x40 `Vec<u16>` 待处理索引；
//!   +0x48/+0x50/+0x58 `Vec<AutoClutterSpawn>`（stride 0x68）；+0x60/+0x68/+0x70/+0x78
//!   hashbrown 区域表；+0x90 epoch。二进制 Vec 字段序与安全 Rust 标准序不一致
//!   （pending=(cap,data,len)、懒网格/候选=(data,len)、输出=(data,cap,len)）——类型合同
//!   分歧（U），本重建以语义等价字段承载。
//! - 区域表条目 stride 0x48：key u64 @+0；`AutoClutterRegion` 载荷 @+0x8..+0x47
//!   （kind u32 @+0、u64 @+8/+0x10、7×f32 @+0x18..+0x30、i32 @+0x34、u8 @+0x38/+0x39、
//!   u16 candidate_index @+0x3a（哨兵 0xffff）、u8 epoch @+0x3c）。
//!   `is_equivalent`（O）逐比较 7×f32（IEEE 序，NaN 不相等）+ i32 + 2×u8；不比较
//!   kind/f08/shared/candidate_index/epoch。
//!
//! 方法（O）：add_region 0x140ee3fd0、mark_dirty 0x140ee48c0、remove_region_from_lazy_grid
//! 0x140ee4a70、flush_candidates_pending_removal 0x140ee4bf0、register_required_components
//! 0x140ee4ee0、set_current_tick L164、remove_stale_regions L233、add_region_to_lazy_grid L252、
//! reset_autoclutter_for_new_frame L531。

pub mod add_autoclutter_around_gates;
pub mod add_autoclutter_around_windows;
pub mod add_autoclutter_on_windows;
pub mod add_birdnests;
pub mod add_lantern_water_setdresssing;
pub mod add_preplaced_autoclutter;
pub mod add_tree_vegetation_setdressing;
pub mod autoclutter_animation;
pub mod load_clutter_meshes;
pub mod manage_autoclutter_entities;
pub mod populate_autoclutter_regions;
pub mod process_autoclutter_candidates;
pub mod read_bush_positions_from_gpu;

use bevy_ecs::component::Component;
use bevy_ecs::resource::Resource;
use glam::{Vec2, Vec3};
use hashbrown::HashMap;

/// 世界格标识（DirtyTiles / 懒网格键；`GlobalId`）。
pub type GlobalId = u64;

/// 杂物种类（判别值部分确认：门/预置=3、鸟巢=4、灯笼/树植被=5、on_windows 入口字节 6）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClutterType {
    GateOrPreplaced = 3,
    Birdnest = 4,
    LanternTreeVegetation = 5,
    ForcedOnWindow = 6,
    Unknown(u8),
}

impl ClutterType {
    pub fn from_byte(b: u8) -> Self {
        match b {
            3 => ClutterType::GateOrPreplaced,
            4 => ClutterType::Birdnest,
            5 => ClutterType::LanternTreeVegetation,
            6 => ClutterType::ForcedOnWindow,
            other => ClutterType::Unknown(other),
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            ClutterType::GateOrPreplaced => 3,
            ClutterType::Birdnest => 4,
            ClutterType::LanternTreeVegetation => 5,
            ClutterType::ForcedOnWindow => 6,
            ClutterType::Unknown(b) => b,
        }
    }
}

/// 候选索引（u16 包装；区域条目 +0x42，process 有序候选 u16 下标）。
/// 哨兵：`>= 0xfffe` 视为 None；mark_dirty 写 0xffff 作"已消费"标记。
/// O：add_region/flush 判 `u16 < 0xfffe` 才有效；gates 区域初值 0xffff。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClutterCandidateIndex(pub u16);

impl ClutterCandidateIndex {
    /// None 哨兵（O：gates 写 0xffff）。
    pub const NONE: Self = Self(0xffff);
    /// `new`（PDB L341）。
    pub const fn new(value: u16) -> Self {
        Self(value)
    }
    /// `get`（PDB L347；O：`< 0xfffe` 才有效）。
    pub fn get(self) -> Option<u16> {
        (self.0 < 0xfffe).then_some(self.0)
    }
    /// `is_none`（PDB L355）。
    pub fn is_none(self) -> bool {
        self.0 >= 0xfffe
    }
}

/// 区域 id（hashbrown 键；hash_one<&GlobalId>）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClutterRegionId(pub u64);

/// 自动杂物区域（区域表值；二进制载荷布局见模块文档）。
/// 本重建以 key/center/basis 承载载荷几何（近似），candidate_index/epoch 为状态字段。
#[derive(Clone, Debug)]
pub struct AutoClutterRegion {
    /// +0x00：种类（3=门/预置、4=鸟巢、5=灯笼/树；add_region 判别==4 时释放 Arc）。
    pub kind: u32,
    /// +0x08..：区域键（hashbrown 表键；载荷内 +0x08 为 u64 段）。
    pub key: ClutterRegionId,
    /// +0x10..：中心（3×f32；二进制 +0x18.. 近似）。
    pub center: Vec3,
    /// +0x1c..：基向量（4×f32；二进制 +0x24.. 近似）。
    pub basis: [Vec2; 2],
    /// +0x34：i32（is_equivalent 逐比较）。
    pub f34: i32,
    /// +0x38/+0x39：2×u8（is_equivalent 逐比较）。
    pub b38: u8,
    pub b39: u8,
    /// +0x42：候选索引。
    pub candidate_index: ClutterCandidateIndex,
    /// +0x44：epoch（= AutoClutterState+0x90 副本）。
    pub epoch: u8,
}

impl Default for AutoClutterRegion {
    fn default() -> Self {
        Self {
            kind: 3,
            key: ClutterRegionId(0),
            center: Vec3::ZERO,
            basis: [Vec2::ZERO; 2],
            f34: 0,
            b38: 0,
            b39: 0,
            candidate_index: ClutterCandidateIndex::NONE,
            epoch: 0,
        }
    }
}

impl AutoClutterRegion {
    /// `is_equivalent`（0x140ee3fd0 内联；L408）：逐 7×f32（IEEE 序相等，NaN 不相等）+
    /// i32 + 2×u8 比较；**不**比较 kind/candidate_index/epoch（O）。
    pub fn is_equivalent(&self, other: &AutoClutterRegion) -> bool {
        float_ieq(self.center.x, other.center.x)
            && float_ieq(self.center.y, other.center.y)
            && float_ieq(self.center.z, other.center.z)
            && float_ieq(self.basis[0].x, other.basis[0].x)
            && float_ieq(self.basis[0].y, other.basis[0].y)
            && float_ieq(self.basis[1].x, other.basis[1].x)
            && float_ieq(self.basis[1].y, other.basis[1].y)
            && self.f34 == other.f34
            && self.b38 == other.b38
            && self.b39 == other.b39
    }

    /// `rectangle2d`（L521；O add_region 内联 `local_dc = d*a + c*b; local_e0 = c*a - d*b`
    /// → x0y → Rectangle2d::new → aabb）。精确 center/size/axis 组合为 I（U）；本重建以
    /// 中心 XZ + 基向量归一化轴近似。
    pub fn rectangle2d(&self) -> utils::geometry::rectangle::Rectangle2d {
        // O：点积/叉积（d*a+c*b、c*a-d*b）→ x0y；此处用 center/basis 近似组合（U）。
        let center = Vec2::new(self.center.x, self.center.z);
        let axis = self.basis[0].normalize_or_zero();
        if axis == Vec2::ZERO {
            utils::geometry::rectangle::Rectangle2d::new_axis_aligned(center, Vec2::splat(1.0))
        } else {
            utils::geometry::rectangle::Rectangle2d::new(center, Vec2::splat(1.0), axis)
        }
    }
}

/// float 序相等（O：`a != b || NAN(a) || NAN(b)` 的反义，即 IEEE `oeq`）。
#[inline]
fn float_ieq(a: f32, b: f32) -> bool {
    a == b && !a.is_nan() && !b.is_nan()
}

/// 区域候选（stride 0x98；字段映射见 process 源→输出表）。
#[derive(Clone, Debug)]
pub struct AutoClutterCandidate {
    /// +0x00：种类判别（process 按 3/4/5 clone 共享字段）。
    pub kind: u8,
    /// +0x08：共享引用（Arc 强计数等；process 0x4 分支强计数++、0x5 分支裸指针语义）。
    pub shared: u64,
    /// +0x10：键/哈希。
    pub key: u64,
    /// +0x18：id/计数。
    pub f18: u64,
    /// +0x20。
    pub f20: u64,
    /// +0x28。
    pub f28: u64,
    /// +0x30：byte（==6 强制接受路径）+ 上 3B。
    pub forced: u8,
    /// +0x34 u32。
    pub f34: u32,
    /// +0x38。
    pub f38: u64,
    /// +0x40。
    pub f40: u64,
    /// +0x48。
    pub f48: u64,
    /// +0x50 u32。
    pub f50: u32,
    /// +0x54 u64。
    pub f54: u64,
    /// +0x5c u32。
    pub f5c: u32,
    /// +0x60 u32（缩放位）。
    pub f60: u32,
    /// +0x68..+0x88：Rectangle3d（overlaps_rectangle3d 输入；+0x68 center、+0x74 size、
    /// +0x80 axis）。
    pub rect: utils::geometry::rectangle::Rectangle3d,
    /// +0x88：键（flush swap_remove 换入后查区域表）。
    pub region_key: u64,
}

impl Default for AutoClutterCandidate {
    fn default() -> Self {
        Self {
            kind: 0,
            shared: 0,
            key: 0,
            f18: 0,
            f20: 0,
            f28: 0,
            forced: 0,
            f34: 0,
            f38: 0,
            f40: 0,
            f48: 0,
            f50: 0,
            f54: 0,
            f5c: 0,
            f60: 0,
            rect: utils::geometry::rectangle::Rectangle3d::new(
                glam::Vec3::ZERO,
                glam::Vec3::ZERO,
                glam::Vec2::X,
            ),
            region_key: 0,
        }
    }
}

impl AutoClutterCandidate {
    /// `byte@+0x30 == 6` → 强制接受（process_candidates / on_windows 同判据）。
    pub fn forced(&self) -> bool {
        self.forced == 6
    }
}

/// Actual metadata selected from the canonical loader for this frame's output.
/// B output+38 is a borrowed ClutterMesh reference. Copying only its consumed
/// values avoids a native pointer or a second asset/resource owner in Rust.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResolvedClutterMesh {
    pub name_hash: country_core::name_hash::NameHash,
    pub radius_xz: f32,
    pub native_group: u8,
    pub solid_texture: Option<country_core::components::drawable::NaniSolidTexture>,
}

impl From<&country_core::resources::clutter_meshes::ClutterMesh> for ResolvedClutterMesh {
    fn from(mesh: &country_core::resources::clutter_meshes::ClutterMesh) -> Self {
        Self {
            // Both NameHash types wrap the same unseeded XXH3-64 value.
            name_hash: country_core::name_hash::NameHash(mesh.name_hash.0),
            radius_xz: mesh.radius_xz,
            native_group: mesh.native_group,
            solid_texture: mesh.solid_texture,
        }
    }
}

/// Rust semantic adapter, not a native source-enum/ABI representation.
#[derive(Clone, Debug, PartialEq)]
pub enum SpawnShared {
    /// Legacy reconstruction erased the actual Arc owner. A number alone is
    /// not an Arc; the manager rejects this unresolved input before ECS writes.
    ArcStrong(u64),
    /// Legacy writers store hash salts here and have lost their selected mesh.
    Prefab(String),
    /// Actual producer-resolved metadata. Color is the original shutter payload.
    Mesh {
        metadata: ResolvedClutterMesh,
        color: Option<country_core::resources::color_id::ColorId>,
    },
}

/// One per-frame output value. Native records have stride0x68, but this Rust
/// representation does not promise those offsets. The Window producer uses
/// key=original+40, position=+48 xyz, marker=source selector+30, host_id=wall+20,
/// and direction bits=+54/+58/+5c. `Mesh` carries the original+38 dependency.
#[derive(Clone, Debug)]
pub struct AutoClutterSpawn {
    pub kind: u8,
    pub shared: SpawnShared,
    pub f18: u32,
    pub key: u64,
    pub marker: u8,
    pub position: Vec3,
    pub host_id: u64,
    pub f48: u64,
    pub f50: u64,
    pub f54: u32,
    pub f58: u32,
    pub f5c: u32,
    pub scale: f32,
}

impl Default for AutoClutterSpawn {
    fn default() -> Self {
        Self {
            kind: 0,
            shared: SpawnShared::ArcStrong(0),
            f18: 1,
            key: 0,
            marker: 5,
            position: Vec3::ZERO,
            host_id: 0,
            f48: 0,
            f50: 0,
            f54: 0,
            f58: 0,
            f5c: 0,
            scale: 1.0,
        }
    }
}

/// B system_clutter::autoclutter::WindowShutter: quaternion plus side byte.
/// The manager and actual animation share this one ECS component.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct WindowShutter {
    pub base_rotation: glam::Quat,
    pub side: bool,
}

/// 自动杂物状态资源（布局见模块文档）。
/// 二进制布局与安全 Rust `Vec` 字段序不一致（pending 为 cap/data/len、懒网格/候选为
/// data/len、输出为 data/cap/len）——类型合同分歧（U），本重建以语义等价字段承载。
#[derive(Resource, Debug, Default)]
pub struct AutoClutterState {
    /// +0x08/+0x10：懒网格（cell → GlobalId 集；mark_dirty 消费）。
    pub lazy_grid: Vec<Vec<GlobalId>>,
    /// +0x20/+0x28：区域候选（stride 0x98）。
    pub region_candidates: Vec<AutoClutterCandidate>,
    /// +0x30/+0x38/+0x40：候选待处理 u16 索引。
    pub pending_indices: Vec<u16>,
    /// +0x48/+0x50/+0x58：输出候选（stride 0x68）。
    pub spawn_candidates: Vec<AutoClutterSpawn>,
    /// +0x60..：区域哈希表。
    pub regions: HashMap<ClutterRegionId, AutoClutterRegion>,
    /// +0x90：epoch。
    pub epoch: u8,
}

impl AutoClutterState {
    /// `set_current_tick`（L164）。
    /// **评审 FIX #8（未实现-需重做）**：PDB 注解该系统签名为
    /// `void(*)(Res<GameTime>, ResMut<AutoClutterState>, ResMut<InterShapeBrickStitches>)`
    /// （skeleton mod.rs L3689）——即读取 GameTime 并同时改 AutoClutterState 与
    /// InterShapeBrickStitches 的独立系统；epoch 来源与 stitch 交互 U。现为方法桩，
    /// 未还原系统签名与宿主交互。
    pub fn set_current_tick(&mut self) {
        // O：tick 由宿主查询索引推进；epoch 逐帧推进（remove_stale_regions 判定）。
        self.epoch = self.epoch.wrapping_add(1);
    }

    /// `add_region`（0x140ee3fd0）：hash_one<&GlobalId> → 区域表（PDB `HashMap::entry`）：
    /// - 命中且 is_equivalent → 仅刷新 epoch + flush；区域载荷判别==4 → drop Arc。
    /// - 命中不等价 → remove 旧区域（iter_world_space_rect_cells<remove_region_from_lazy_grid>）
    ///   + add 新区域（<add_region_to_lazy_grid>）；旧候选索引 <0xfffe → push pending；
    ///   旧判别==4 → Arc 释放；覆盖字段+epoch；flush。
    /// - 未命中 → 必要时 reserve_rehash；add 新区域；写新条目（键/载荷/epoch；+0x78++）；flush。
    /// 本重建以 get/insert 表达 entry 语义（entry 借用与 self 方法互斥，行为等价）。
    pub fn add_region(&mut self, key: ClutterRegionId, region: &AutoClutterRegion) {
        let old = self.regions.get(&key).cloned();
        match old {
            Some(old) => {
                if old.is_equivalent(region) {
                    // (b) 等价：仅刷新 epoch + flush；区域载荷判别==4 则 drop Arc。
                    self.regions.get_mut(&key).expect("occupied").epoch = self.epoch;
                    self.flush_candidates_pending_removal();
                    if region.kind == 4 {
                        // **U 登记缝**：O 对 region 载荷 +0x10 Arc<[Vec3]> LOCK 递减、归零
                        // drop_slow——真实副作用；因 Arc payload 类型合同 U，此处不执行，
                        // 明确登记而非静默（评审 FIX #5）。
                        let _ = region.b38;
                    }
                } else {
                    // (a) 命中不等价：移除旧区域 + 加入新区域。
                    let old_rect = old.rectangle2d();
                    let new_rect = region.rectangle2d();
                    self.remove_region_from_lazy_grid_rect(old_rect, key.0);
                    self.add_region_to_lazy_grid_rect(new_rect, key.0);
                    // 旧条目候选索引 < 0xfffe → push pending。
                    if let Some(ci) = old.candidate_index.get() {
                        self.push_pending(ci);
                    }
                    // **U 登记缝**：旧判别==4 → 旧载荷 Arc LOCK 递减、归零 drop_slow
                    // （O）；因 Arc payload 合同 U，此处登记不执行（评审 FIX #5）。
                    if old.kind == 4 {
                        let _ = old.b38;
                    }
                    // 覆盖字段 + epoch。
                    let mut new_region = region.clone();
                    new_region.epoch = self.epoch;
                    self.regions.insert(key, new_region);
                    self.flush_candidates_pending_removal();
                }
            }
            None => {
                // 未命中：reserve_rehash（区域表 +0x70==0 时）。
                if self.regions.is_empty() {
                    self.regions.reserve(1);
                }
                self.add_region_to_lazy_grid_rect(region.rectangle2d(), key.0);
                let mut new_region = region.clone();
                new_region.epoch = self.epoch;
                self.regions.insert(key, new_region);
                self.flush_candidates_pending_removal();
            }
        }
    }

    /// 懒网格加区域（iter_world_space_rect_cells<add_region_to_lazy_grid::closure_env_0>
    /// 语义：矩形覆盖的每格插入区域 **GlobalId**（区域键；O：闭包写键，非占位 0）。
    fn add_region_to_lazy_grid_rect(
        &mut self,
        rect: utils::geometry::rectangle::Rectangle2d,
        key: GlobalId,
    ) {
        for cell in world_rect_cells_from_aabb(rect.aabb()) {
            while self.lazy_grid.len() <= cell {
                self.lazy_grid.push(Vec::new());
            }
            let bucket = &mut self.lazy_grid[cell];
            if !bucket.contains(&key) {
                bucket.push(key);
            }
        }
    }

    /// 懒网格移除区域（iter_world_space_rect_cells<remove_region_from_lazy_grid::closure_env_0>
    /// 语义：矩形覆盖的每格删除区域 GlobalId）。
    fn remove_region_from_lazy_grid_rect(
        &mut self,
        rect: utils::geometry::rectangle::Rectangle2d,
        key: GlobalId,
    ) {
        for cell in world_rect_cells_from_aabb(rect.aabb()) {
            if let Some(bucket) = self.lazy_grid.get_mut(cell) {
                bucket.retain(|g| *g != key);
            }
        }
    }

    /// 候选索引入队（O：len==cap → grow_one<u16>；data[len]=idx；len++）。
    fn push_pending(&mut self, idx: u16) {
        if self.pending_indices.len() == self.pending_indices.capacity() {
            self.pending_indices.reserve(1);
        }
        self.pending_indices.push(idx);
    }

    /// `mark_dirty`（0x140ee48c0）：懒网格 cell 的每个 GlobalId → hash_one<&GlobalId> 查
    /// 区域表；命中 → 条目候选索引 <0xfffe → push pending + 写 0xffff（已消费）；
    /// 未命中**或表空** → unwrap_failed panic（懒网格与区域表同步不变量）。
    /// O：越界 cell 索引 → panic_bounds_check（非静默返回）。
    pub fn mark_dirty(&mut self, cell_index: usize) {
        assert!(
            cell_index < self.lazy_grid.len(),
            "autoclutter mark_dirty: bounds check failed"
        );
        let globals = self.lazy_grid[cell_index].clone();
        let mut pending = Vec::new();
        for global in globals {
            let key = ClutterRegionId(global);
            // O：`+0x78 != 0` 才查；未命中/表空 → unwrap_failed（无静默跳过）。
            let region = self
                .regions
                .get_mut(&key)
                .expect("autoclutter mark_dirty: region missing for lazy grid entry");
            let ci = region.candidate_index;
            if ci.get().is_some() {
                pending.push(ci.0);
            }
            // O：条目 +0x42 = 0xffff（已消费标记）。
            region.candidate_index = ClutterCandidateIndex::NONE;
        }
        self.pending_indices.extend(pending);
    }

    /// `remove_region_from_lazy_grid`（0x140ee4a70）：x0y + Rectangle2d::new/aabb +
    /// iter_world_space_rect_cells<remove_region_from_lazy_grid::closure_env_0>。
    /// 删除键 = 该区域 GlobalId（区域表条目仍在时取其矩形）。
    pub fn remove_region_from_lazy_grid(&mut self, key: &ClutterRegionId) {
        if let Some(region) = self.regions.get(key) {
            let rect = region.rectangle2d();
            self.remove_region_from_lazy_grid_rect(rect, key.0);
        }
    }

    /// `flush_candidates_pending_removal`（0x140ee4bf0）：pending 升序（<0x15 用
    /// insertion_sort_shift_left 否则 driftsort_main，closure$0 比较器）→ 逐索引：
    /// `Vec::swap_remove`（memmove 末元素进槽、len-1）；判别==4 → Arc 释放；若新长度!=被删
    /// 索引 → 取换入候选 key（@+0x88）查区域表，断言条目候选索引==Some(新长度) 后写新索引
    /// （否则 assert_failed/unwrap_failed）；末尾 +0x40=0。
    pub fn flush_candidates_pending_removal(&mut self) {
        let n = self.pending_indices.len();
        if n == 0 {
            return;
        }
        // O：**降序**稳定排序（高索引先删，swap_remove 才不破坏低索引）。内联插入排序
        // 逐行判向：`if (v[i-1] < v[i])` 进入移位、`while (v[j-1] < key)` 右移 →
        // [3,5]→[5,3]。原升序 + swap_remove 会删错候选甚至 swap_remove assert 崩
        // （例 pending=[2,5]、len=6：先删 2 后删 5 → len(5)<=idx(5) panic）。
        self.pending_indices.sort_by(|a, b| b.cmp(a));
        for &idx_u16 in &self.pending_indices {
            let idx = idx_u16 as usize;
            // O：`swap_remove::assert_failed`（len <= idx）。
            assert!(
                self.region_candidates.len() > idx,
                "flush_candidates_pending_removal: swap_remove bounds"
            );
            let removed = self.region_candidates.swap_remove(idx);
            // **U 登记缝**：判别==4 → Arc 强计数 LOCK 递减、归零 drop_slow（O 真实副作用；
            // Arc payload 合同 U，登记不执行——评审 FIX #5）。
            if removed.kind == 4 {
                let _ = removed.shared;
            }
            if self.region_candidates.len() != idx {
                // O：被换入候选的 key（+0x88）→ 区域表修正 +0x42。
                let moved_key = self.region_candidates[idx].region_key;
                if !self.regions.is_empty() {
                    let region = self
                        .regions
                        .get_mut(&ClutterRegionId(moved_key))
                        .expect("flush_candidates_pending_removal: region missing");
                    // O：`assert_failed<Option<usize>, Option<usize>>`：
                    // 条目候选索引必须 == Some(新长度)。
                    assert!(
                        region.candidate_index.get().is_some()
                            && region.candidate_index.0 as usize == self.region_candidates.len(),
                        "flush_candidates_pending_removal: candidate index invariant"
                    );
                    region.candidate_index = ClutterCandidateIndex(idx_u16);
                }
            }
        }
        // O：+0x40 = 0。
        self.pending_indices.clear();
    }

    /// Clear the actual per-frame output Vec. The system wrapper also updates
    /// epoch and clears stitches, as proven in both original inline hosts.
    pub fn reset_autoclutter_for_new_frame(&mut self) {
        self.spawn_candidates.clear();
    }
}

/// `set_current_tick` 系统包装（PDB L164 注解签名：
/// `void(*)(Res<GameTime>, ResMut<AutoClutterState>, ResMut<InterShapeBrickStitches>)`）。
///
/// **未实现-需重做（U）**：该函数在 B 无独立符号（内联），fresh 反编译不可定点；
/// 仅 epoch 推进可见（`remove_stale_regions` 判据）。与 `InterShapeBrickStitches`
/// 的交互（是否清 tick/stitch 槽）未取证，故不注册、不伪造。宿主接线需 core 侧
/// 提供 `GameTime` tick 字段后按 PDB 签名重建。
pub fn set_current_tick(
    _time: bevy_ecs::system::Res<country_core::resources::time::GameTime>,
    mut state: bevy_ecs::system::ResMut<AutoClutterState>,
    _stitches: bevy_ecs::system::ResMut<
        crate::inter_shape_stitches::stitch_bricks::InterShapeBrickStitches,
    >,
) {
    state.set_current_tick();
}

/// B inline reset in physical hosts140ee5830/833B and141ce1f10/961B:
/// clear outputs, copy GameTime.frame_count low byte to epoch, clear stitches.
/// WindowAutoClutterCandidates is consumed by its separate real drain.
pub fn reset_autoclutter_for_new_frame(
    time: bevy_ecs::system::Res<country_core::resources::time::GameTime>,
    mut state: bevy_ecs::system::ResMut<AutoClutterState>,
    mut stitches: bevy_ecs::system::ResMut<
        crate::inter_shape_stitches::stitch_bricks::InterShapeBrickStitches,
    >,
) {
    state.reset_autoclutter_for_new_frame();
    state.epoch = time.frame_count as u8;
    stitches.0.clear();
}

/// Aabb2 → 网格单元列表（O：`Rectangle2d::aabb` → `iter_world_space_rect_cells` 语义；
/// 经 DirtyTiles::world_rect_cells 的 scale 0x3efc0fc1、bias 32、clamp 63）。
fn world_rect_cells_from_aabb(aabb: utils::geometry::Aabb2) -> Vec<usize> {
    let min = aabb.center() - aabb.half_dims();
    let max = aabb.center() + aabb.half_dims();
    let cells = country_core::utils::dirty_tiles::world_rect_cells([min.x, min.y, max.x, max.y]);
    let mut out = Vec::new();
    for y in cells.minimum[1]..cells.end[1] {
        for x in cells.minimum[0]..cells.end[0] {
            out.push(y as usize * 64 + x as usize);
        }
    }
    out
}

/// 懒网格格单元（read_bush_positions_from_gpu 共用；stride unknown）。
#[derive(Resource, Clone, Debug, Default)]
pub struct LazyGridClutterCell;

/// 世界空间 rect 采样（filter_clutter_object 用；Aabb2::from_center_radius +
/// DirtyTiles::iter_world_space_rect_cells）。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClutterQueryRect {
    pub center: Vec2,
    pub radius: f32,
}

#[cfg(test)]
pub(crate) mod window_test_helpers {
    use super::*;
    use bevy_ecs::prelude::*;
    use country_core::{resources::{clutter_meshes::{ClutterMesh, ClutterMeshes},
        time::GameTime, walls::WallId}, systems::{audio::PlayClutterSoundCmd,
        collision::{WindowAutoClutterCandidate, WindowAutoClutterCandidates, WindowAutoClutterKind}}};
    use glam::Quat;

    // Explicit CPU metadata fixture. No asset/GPU registration or successful
    // whole-App frame is claimed from these named ECS descriptor tests.
    fn metadata(id: u64) -> ClutterMesh {
        ClutterMesh {
            bounds: utils::geometry::rectangle::Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X),
            mesh_name: "unit/window-metadata",
            mesh: None,
            name_hash: utils::name_hash::NameHash(id),
            radius_xz: 1.0,
            native_group: 7,
            solid_texture: None,
            native_flag_46: true,
            native_flag_47: false,
        }
    }

    pub fn textured_meshes(name: &'static str,
        texture: country_core::components::drawable::NaniSolidTexture) -> ClutterMeshes {
        let mut fixed = std::array::from_fn(|index| metadata(index as u64 + 1));
        fixed[27].name_hash = utils::name_hash::NameHash::compute(name);
        fixed[27].solid_texture = Some(texture);
        ClutterMeshes::from_parts(fixed, vec![metadata(1000), metadata(1001)])
    }

    pub fn candidate(key: u64, kind: WindowAutoClutterKind) -> WindowAutoClutterCandidate {
        WindowAutoClutterCandidate {
            kind, key, wall: WallId(0xfedc_ba98_7654_3210),
            rotation: Quat::from_rotation_y(core::f32::consts::FRAC_PI_2),
            translation: Vec3::new(3.0, 7.0, -2.0), scale: Vec3::new(9.0, 8.0, 7.0),
        }
    }

    pub fn world(candidates: Vec<WindowAutoClutterCandidate>) -> World {
        let mut world = World::new();
        world.init_resource::<AutoClutterState>();
        world.insert_resource(ClutterMeshes::from_parts(
            std::array::from_fn(|index| metadata(index as u64 + 1)),
            vec![metadata(1000), metadata(1001)]));
        world.insert_resource(WindowAutoClutterCandidates(candidates));
        world.insert_resource(GameTime { frame_count: 0x12ab, ..Default::default() });
        world.init_resource::<Events<PlayClutterSoundCmd>>();
        world
    }

    #[test]
    fn reset_clears_real_outputs_and_stitches_but_preserves_window_input() {
        use crate::inter_shape_stitches::stitch_bricks::{InterShapeBrickStitches, StairsFloatingBrickParams};
        use country_core::systems::collision::{WindowAutoClutterCandidates, WindowAutoClutterKind};
        let mut world = world(vec![candidate(42, WindowAutoClutterKind::DoorBell)]);
        let mut drain = Schedule::default();
        drain.add_systems(super::add_autoclutter_on_windows::add_autoclutter_on_windows);
        drain.run(&mut world);
        world.resource_mut::<WindowAutoClutterCandidates>().0.push(candidate(43, WindowAutoClutterKind::DoorKrans));
        world.insert_resource(InterShapeBrickStitches(vec![StairsFloatingBrickParams::default()]));
        let mut reset = Schedule::default();
        reset.add_systems(super::reset_autoclutter_for_new_frame);
        reset.run(&mut world);
        assert!(world.resource::<AutoClutterState>().spawn_candidates.is_empty());
        assert_eq!(world.resource::<AutoClutterState>().epoch, 0xab);
        assert!(world.resource::<InterShapeBrickStitches>().0.is_empty());
        assert_eq!(world.resource::<WindowAutoClutterCandidates>().0[0].key, 43);
    }
}
