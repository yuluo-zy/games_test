//! system-clutter：autoclutter 模块业务恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/clutter-placement-analysis.md（2026-10-13 reva/radare2 精读）。
//! 证据来源：B patched exe（reva worker `clutter-owner`）+ RNE 发行原件（radare2 oracle）。
//! 字段布局为证据；未确认字段保留 `unknown`，不伪造。
//!
//! 依赖复用：bevy_ecs 0.16、glam 0.29.3、fastrand、hashbrown、tabloid、utils 直接复用，
//! 不还原其实现。本文件 = 游戏自有逻辑（system_clutter autoclutter 模块）。

pub mod add_rules;
pub mod manage_autoclutter_entities;
pub mod populate_autoclutter_regions;
pub mod process_autoclutter_candidates;

use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use hashbrown::HashMap;
use std::sync::Arc;

// ---- country_core 类型引用（定义在 country-core，经泛型实例引用）----
/// 世界格标识（DirtyTiles / 懒网格键）。
pub type GlobalId = u64;
/// 门标识（country_core::resources::walls::WallId）。
pub type WallId = u64;

/// 杂物种类（ClutterType，mod.rs L73–92 方法 is_window/is_plant/is_olden/is_window_shutter）。
/// 判别值部分确认：鸟巢记录 type=4、灯笼/树植被 type=5、on_windows 入口字节 6。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClutterType {
    /// 证据：add_birdnests 输出 +0x0=4。
    Birdnest = 4,
    /// 证据：add_lantern/add_tree 输出 +0x0=5。
    LanternTreeVegetation = 5,
    /// 证据：process_candidates / on_windows 判 byte==6 强制接受。
    ForcedOnWindow = 6,
    /// 其余判别值 unknown。
    Unknown(u8),
}

/// 候选索引（mod.rs L341 ClutterCandidateIndex::new / L347 get / L355 is_none；u16）。
/// 证据：add_region 区域条目 +0x42 为 u16，process_candidates 有序候选为 u16 下标。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub struct ClutterCandidateIndex(pub u16);

/// 自动杂物区域（AutoClutterRegion）。hashbrown 条目 stride 0x48。
/// 证据（add_region 反编译，条目基 = 哈希桶 + index*0x48）：
/// - +0x00 key（ClutterRegionId，8B；hash 键含 GlobalId）
/// - +0x08..+0x40：中心/基向量 float（等价比较逐字段：+0x20..+0x3c 两组 Vec3）
/// - +0x42：u16（ClutterCandidateIndex）
/// - +0x44：byte epoch（= AutoClutterState+0x90，remove_stale_regions 用）
#[derive(Clone, Debug)]
pub struct AutoClutterRegion {
    pub key: ClutterRegionId,
    /// 中心/基向量（unknown 精确语义；add_region 用 Rectangle2d::aabb 推 AABB）。
    pub center: Vec3,
    pub basis: [Vec2; 2],
    /// 候选索引（u16 @+0x42）。
    pub candidate_index: ClutterCandidateIndex,
    /// epoch（byte @+0x44，state epoch 副本）。
    pub epoch: u8,
}

/// 区域 id（hashbrown 键；hash_one<&GlobalId>，值内容 unknown）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ClutterRegionId(pub u64);

/// 自动杂物对象（实体组件；字段布局 unknown，manage 用其键建集合）。
pub struct AutoClutterObject {
    /// unknown 字段；仅确认键类型 u64。
    pub key: u64,
}

/// 自动杂物状态资源（AutoClutterState，build 里 insert_resource）。
/// 布局证据（add_region 反编译 + process_candidates/lantern/tree 写入点）：
/// - +0x20/+0x28：Vec<AutoClutterCandidate>（stride 0x98；add_region 更新时产出区域候选）
/// - +0x30/+0x38/+0x40：Vec<u16>（候选待处理索引；cap/data/len）
/// - +0x48/+0x50/+0x58：Vec<AutoClutterSpawn>（stride 0x68；birdnest/lantern/tree/
///   process_candidates 共同写；+0x48 容量、+0x50 数据、+0x58 写入计数）
/// - +0x60/+0x68/+0x70/+0x78：hashbrown<ClutterRegionId, AutoClutterRegion>
///   （control bytes / bucket mask / count / insert counter）
/// - +0x90：byte epoch
#[derive(Default)]
pub struct AutoClutterState {
    /// 区域候选（stride 0x98，字段见 AutoClutterCandidate）。
    pub region_candidates: Vec<AutoClutterCandidate>,
    /// 候选待处理 u16 索引表（add_region 更新区域时 push 区域候选索引）。
    pub pending_candidate_indices: Vec<u16>,
    /// 输出候选（stride 0x68，见 AutoClutterSpawn）。
    pub spawn_candidates: Vec<AutoClutterSpawn>,
    /// 区域哈希表。
    pub regions: HashMap<ClutterRegionId, AutoClutterRegion>,
    /// epoch（写进区域 +0x44；remove_stale_regions 比对）。
    pub epoch: u8,
    // 布局：hashbrown 域 @+0x60 前还有 +0x30 cap / +0x40 len 等，部分 unknown。
}

/// 区域候选（stride 0x98）。字段布局证据（process_candidates 复制 16 字段到输出）：
/// - +0x00：种类判别 u8（on_windows 跳表分派 / process 按 0x3/0x4/0x5 clone；与输出记录 type 一致）
/// - +0x08：共享引用（Arc 强计数等；process 0x4 分支强计数++、0x5 分支裸指针）
/// - +0x10/+0x18：键/索引（u32 或 8B）
/// - +0x18..+0x50：位置/基向量 float 与 id
/// - +0x30：byte（==6 强制接受路径）
/// - +0x34 u32、+0x4c u32、+0x5c u32 等（unknown 语义）
#[derive(Clone, Debug)]
pub struct AutoClutterCandidate {
    pub kind: ClutterType,
    pub shared: Arc<[Vec3]>,
    /// unknown 字段。
    pub unknown: [u64; 14],
}

/// 输出候选（stride 0x68）。字段布局证据（lantern/tree/birdnest/process_candidates 写入点）：
/// - +0x00：u64（type：birdnest=4、lantern/tree=5）
/// - +0x08：String（prefab 名：`lantern`/`tree`；birdnest 为 Arc 强计数域）
/// - +0x18：u32（=2 常量）
/// - +0x1c：u64（放置键，calculate_hash 结果）
/// - +0x2c：u32（unknown）
/// - +0x30：byte（=5；process 侧 ==6 强制标记）
/// - +0x31..0x34：unknown
/// - +0x38：位置 u64（Vec3 打包）
/// - +0x40：u64（unknown；is_started 门控输出）
/// - +0x48/+0x50/+0x54/+0x5c：unknown
/// - +0x60：float（缩放/朝向派生值）
#[derive(Clone, Debug)]
pub struct AutoClutterSpawn {
    /// +0x00 type 判别。
    pub kind: u8,
    /// +0x08 prefab 名。
    pub name: String,
    /// +0x1c 放置键。
    pub key: u64,
    /// +0x30 标记字节。
    pub marker: u8,
    /// +0x38 位置（unknown 精确字段；证据仅确认打包）。
    pub position: u64,
    /// 其余字段 unknown（+0x18、+0x2c、+0x40、+0x48..0x64）。
    pub unknown: [u64; 8],
}

impl AutoClutterState {
    /// 添加/更新一个区域候选（B 0x140ee3fd0）。
    /// 语义：hash key→查区域表；命中且等价→仅刷 lazy grid + push u16 索引；
    /// 命中不等价→remove 旧格 + add 新格 + 覆盖 + flush；未命中→插入新条目。
    pub fn add_region(&mut self, key: ClutterRegionId, region: &AutoClutterRegion) {
        // 证据：hash_one<&GlobalId>(key)；hashbrown 查询（stride 0x48）。
        if let Some(existing) = self.regions.get_mut(&key) {
            if existing.is_equivalent(region) {
                // 仅刷新 lazy grid + push 候选索引（partial 证据）。
                self.pending_candidate_indices.push(region.candidate_index.0);
                return;
            }
            // 覆盖字段 + flush_candidates_pending_removal。
            *existing = region.clone();
            existing.epoch = self.epoch;
            self.flush_candidates_pending_removal();
        } else {
            self.regions.insert(key, region.clone());
            self.flush_candidates_pending_removal();
        }
    }

    /// 冲刷待移除候选（B 0x140ee4bf0；语义 unknown，仅确认被 add_region/populate 调用）。
    fn flush_candidates_pending_removal(&mut self) {
        // evidence：被 add_region（2 处）、populate（2 处）调用；具体删除逻辑未精读。
        // TODO(evidence): 反编译 0x140ee4bf0 确认。
    }

    /// 标记脏格（B 0x140ee48c0）。
    fn mark_dirty(&mut self) {
        // evidence：populate 位集迭代按位调用。
        // TODO(evidence): 反编译 0x140ee48c0 确认。
    }

    /// 删除过期区域（B：HashMap::retain<remove_stale_regions::closure_env_0>）。
    fn remove_stale_regions(&mut self) {
        self.regions.retain(|_, r| r.epoch == self.epoch);
    }
}

impl AutoClutterRegion {
    /// 逐字段等价比较（add_region 内联：逐 float/byte 比较，unknown 精确字段集）。
    fn is_equivalent(&self, other: &AutoClutterRegion) -> bool {
        self.center == other.center && self.basis == other.basis
    }
}

/// 周期复位（mod.rs L531 reset_autoclutter_for_new_frame）。
pub fn reset_autoclutter_for_new_frame(_state: &mut AutoClutterState) {
    // evidence：mod.rs 顶层函数；语义 unknown。
}
