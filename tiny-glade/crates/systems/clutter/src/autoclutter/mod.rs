//! autoclutter 模块（B 主基线；AutoClutterState 布局与支持方法）。
//!
//! 布局证据（clutter-placement-analysis.md）：
//! - +0x20/+0x28：Vec<AutoClutterCandidate>（stride 0x98；add_region 更新时产出区域候选）
//! - +0x30/+0x38/+0x40：Vec<u16>（候选待处理索引；cap/data/len）
//! - +0x48/+0x50/+0x58：Vec<AutoClutterSpawn>（stride 0x68；birdnest/lantern/tree/
//!   process_candidates/on_windows 共同写；+0x58 为写入计数）
//! - +0x60/+0x68/+0x70/+0x78：hashbrown<ClutterRegionId, AutoClutterRegion>
//!   （control bytes / bucket mask / count / insert counter；条目 stride 0x48）
//! - +0x90：byte epoch（写进区域 +0x44）
//!
//! 方法（O）：add_region 0x140ee3fd0、mark_dirty 0x140ee48c0、remove_region_from_lazy_grid
//! 0x140ee4a70、flush_candidates_pending_removal 0x140ee4bf0、register_required_components
//! 0x140ee4ee0。

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

/// 杂物种类（判别值部分确认：鸟巢=4、灯笼/树植被=5、on_windows 入口字节 6）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClutterType {
    Birdnest = 4,
    LanternTreeVegetation = 5,
    ForcedOnWindow = 6,
    Unknown(u8),
}

impl ClutterType {
    pub fn from_byte(b: u8) -> Self {
        match b {
            4 => ClutterType::Birdnest,
            5 => ClutterType::LanternTreeVegetation,
            6 => ClutterType::ForcedOnWindow,
            other => ClutterType::Unknown(other),
        }
    }
    pub fn to_byte(self) -> u8 {
        match self {
            ClutterType::Birdnest => 4,
            ClutterType::LanternTreeVegetation => 5,
            ClutterType::ForcedOnWindow => 6,
            ClutterType::Unknown(b) => b,
        }
    }
}

/// 候选索引（u16；区域条目 +0x42，process_candidates 有序候选下标）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClutterCandidateIndex(pub u16);

/// 区域 id（hashbrown 键；hash_one<&GlobalId>，内容含 GlobalId）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClutterRegionId(pub u64);

/// 自动杂物区域（hashbrown 条目 stride 0x48）。
#[derive(Clone, Debug)]
pub struct AutoClutterRegion {
    pub key: ClutterRegionId,
    /// +0x08..+0x40：中心/基向量（等价比较逐 float/byte）。
    pub center: Vec3,
    pub basis: [Vec2; 2],
    /// +0x42：候选索引。
    pub candidate_index: ClutterCandidateIndex,
    /// +0x44：epoch（= AutoClutterState+0x90 副本）。
    pub epoch: u8,
}

impl AutoClutterRegion {
    /// 逐字段等价比较（add_region 内联：逐 float/byte 比较）。
    pub fn is_equivalent(&self, other: &AutoClutterRegion) -> bool {
        self.center == other.center
            && self.basis == other.basis
            && self.candidate_index == other.candidate_index
            && self.epoch == other.epoch
    }
}

/// 区域候选（stride 0x98；字段映射见 process_autoclutter_candidates 源→输出表）。
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
    pub forced: bool,
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
    /// +0xc u32（写输出 +0x60 缩放）。
    pub scale: f32,
}

impl AutoClutterCandidate {
    /// `byte@+0x30 == 6` → 强制接受（process_candidates / on_windows 同判据）。
    pub fn forced(&self) -> bool {
        self.forced
    }
}

/// 输出候选 +0x08 载荷：鸟巢为 **Arc 强计数域**（F5：draft 明注），lantern/tree 为
/// prefab 名 String（0x143042d28 "lantern" / 0x143042d91 "tree"）。
#[derive(Clone, Debug, PartialEq)]
pub enum SpawnShared {
    /// Arc 强计数（birdnest 输出 +0x08；process 0x4 分支 LOCK 递增）。
    ArcStrong(u64),
    /// prefab 名（lantern/tree 输出 +0x08）。
    Prefab(String),
}

/// 输出候选（stride 0x68；字段写入点见 birdnest/lantern/tree/on_windows/process）。
#[derive(Clone, Debug)]
pub struct AutoClutterSpawn {
    /// +0x00：type（birdnest=4、lantern/tree/on_windows=5）。
    pub kind: u8,
    /// +0x08：Arc 强计数（birdnest）或 prefab 名（lantern/tree）。
    pub shared: SpawnShared,
    /// +0x18：常量（1/2）。
    pub f18: u32,
    /// +0x1c：放置键（calculate_hash 结果）。
    pub key: u64,
    /// +0x30：byte 标记（=5；process 侧 ==6 强制标记）。
    pub marker: u8,
    /// +0x38：位置（Vec3 打包）。
    pub position: Vec3,
    /// +0x40：宿主 id（墙/窗/装饰物 id）。
    pub host_id: u64,
    /// +0x48。
    pub f48: u64,
    /// +0x50。
    pub f50: u64,
    /// +0x54 u32。
    pub f54: u32,
    /// +0x5c u32。
    pub f5c: u32,
    /// +0x60：缩放（birdnest/lantern/tree 写 1.0/0.6）。
    pub scale: f32,
}

/// 自动杂物对象（实体组件；manage 用其键建集合）。
#[derive(Component, Clone, Debug)]
pub struct AutoClutterObject {
    /// 键（u64；键类型含 GlobalId）。
    pub key: u64,
}

/// 自动杂物状态资源（布局见模块文档）。
#[derive(Resource, Default, Debug)]
pub struct AutoClutterState {
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
    /// 懒网格（0x140ee48c0 的 Vec<0x18 记录>；每格 GlobalId 列表；
    /// mark_dirty 消费；布局 U）。
    pub lazy_grid: Vec<Vec<u64>>,
}

impl AutoClutterState {
    /// `add_region`（0x140ee3fd0）：hash key→查区域表；命中且等价 → 仅刷 lazy grid +
    /// push u16 索引；命中不等价 → remove 旧格 + add 新格 + 覆盖 + flush；
    /// 未命中 → 插入新条目（reserve_rehash）→ flush。
    pub fn add_region(&mut self, key: ClutterRegionId, region: &AutoClutterRegion) {
        let equivalent = self.regions.get(&key).map_or(false, |e| e.is_equivalent(region));
        if equivalent {
            // 仅刷新 lazy grid + push 候选索引。
            self.pending_indices.push(region.candidate_index.0);
            return;
        }
        if self.regions.contains_key(&key) {
            // 覆盖字段 + flush 候选。
            self.remove_region_from_lazy_grid(&key);
            let mut region = region.clone();
            region.epoch = self.epoch;
            self.regions.insert(key, region);
            self.pending_indices.push(self.pending_indices.len() as u16);
            self.flush_candidates_pending_removal();
        } else {
            let mut region = region.clone();
            region.epoch = self.epoch;
            self.regions.insert(key, region);
            self.flush_candidates_pending_removal();
        }
    }

    /// `mark_dirty`（0x140ee48c0）：把脏格内全部 GlobalId 对应的区域候选索引
    /// 推入 pending 并标记区域候选索引 = 0xffff（已消费）。
    /// 证据（反编译）：`if (len <= idx) panic_bounds_check`（lazy grid Vec<0x18 记录>）；
    /// `hash_one<&GlobalId>(state@+0x80)` 查区域表（控制字节@+0x60、掩码@+0x68）；
    /// 命中 → `u16@+0x42 < 0xfffe` → push pending（+0x38 数据/+0x40 长度）+ 写 0xffff；
    /// 未命中 → panic（unreachable）。
    pub fn mark_dirty(&mut self, cell_index: usize) {
        let Some(globals) = self.lazy_grid.get(cell_index) else {
            return;
        };
        for global in globals {
            let key = ClutterRegionId(*global);
            let Some(region) = self.regions.get_mut(&key) else {
                // O：未命中 → unreachable panic（懒网格与区域表同步不变量）。
                panic!("autoclutter mark_dirty: region missing for lazy grid entry");
            };
            let ci = region.candidate_index.0;
            if ci < 0xfffe {
                self.pending_indices.push(ci);
            }
            // O：条目 +0x42 = 0xffff（已消费标记）。
            region.candidate_index = ClutterCandidateIndex(0xffff);
        }
    }

    /// `remove_region_from_lazy_grid`（0x140ee4a70）。
    pub fn remove_region_from_lazy_grid(&mut self, key: &ClutterRegionId) {
        self.regions.remove(key);
    }

    /// `flush_candidates_pending_removal`（0x140ee4bf0）：排序 pending → 逐条
    /// `Vec::remove`（memmove 右移）→ kind==4 时 Arc 强计数释放（LOCK 递减，
    /// 归零释放）→ 维护区域表 candidate_index 一致性（被移除索引后的条目减 1）。
    /// 证据（反编译）：`< 0x15 用 insertion_sort_shift_left 否则 driftsort`；
    /// `func_0x142923c10` memmove（0x98 stride）→ len-1；`(int)copy == 4 → LOCK;
    /// arc--; ==0 → 释放`；区域条目 u16 与候选下标一致不变量断言。
    pub fn flush_candidates_pending_removal(&mut self) {
        let mut pending = std::mem::take(&mut self.pending_indices);
        pending.sort_unstable();
        for &removed in &pending {
            let removed = removed as usize;
            if removed >= self.region_candidates.len() {
                continue;
            }
            let candidate = self.region_candidates.remove(removed);
            // kind==4 → Arc 强计数释放（O：LOCK/UNLOCK 递减；此处为共享载荷释放缝）。
            if candidate.kind == 4 {
                let _ = candidate.shared;
            }
            // 维护区域表候选索引一致性（被移除索引后的条目整体前移 1）。
            for region in self.regions.values_mut() {
                let ci = region.candidate_index.0;
                if ci != 0xffff && ci > removed as u16 {
                    region.candidate_index = ClutterCandidateIndex(ci - 1);
                }
            }
        }
    }

    /// `reset_autoclutter_for_new_frame`（mod.rs L531 顶层函数）。
    pub fn reset_autoclutter_for_new_frame(&mut self) {
        self.spawn_candidates.clear();
    }
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
