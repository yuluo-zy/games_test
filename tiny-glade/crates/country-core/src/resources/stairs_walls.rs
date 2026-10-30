//! `stairs_walls.rs` — 楼梯-墙支柱提议（B `resources/stairs_walls.rs`，1 业务 + 4 胶水）。
//!
//! `StairsPillarProposals::add`（0x140901270，O 反编译 7KB）：哈希表 `HashMap<key u64,
//! Vec<44B 记录>>`——命中 key 时把 44 字节记录 append 到桶 Vec，未命中时分配 44 字节
//! 插入单元素桶。44B记录语义已按140498d66..dad原producer及pillar缓存consumer核实。
//! 章节：`chapters/resources/stairs_walls.rs.md`。
use bevy_ecs::resource::Resource;
use hashbrown::HashMap;

/// 支柱提议的语义值。Rust不强制复刻原44B私有对齐；legacy wire只在边界转换。
#[derive(Clone, Copy, Debug)]
pub struct StairsPillarProposal {
    pub position: glam::Vec2,
    pub top_y: f32,
    pub bottom_y: f32,
    pub tangent: glam::Vec2,
    pub width: f32,
    pub source_id: u32,
    /// 原两u32节点顺序保留；wire边界不做StairSegment::new的重新排序。
    pub nodes: [crate::resources::walls::decorator::DecoratorId; 2],
    pub kind: u8,
    /// 原writer只显写kind；保留已有wire末三字节用于无损兼容，不赋予业务含义。
    pub wire_tail: [u8; 3],
}

impl PartialEq for StairsPillarProposal {
    fn eq(&self, other: &Self) -> bool { self.observed() == other.observed() }
}
impl Eq for StairsPillarProposal {}
impl std::hash::Hash for StairsPillarProposal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.observed().hash(state); }
}

impl Default for StairsPillarProposal {
    fn default() -> Self {
        Self::from_observed([0; 44])
    }
}

impl StairsPillarProposal {
    pub fn from_observed(bytes: [u8; 44]) -> Self {
        let float = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        let word = |offset| u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        Self {position: glam::Vec2::new(float(0), float(4)), top_y: float(8), bottom_y: float(12),
            tangent: glam::Vec2::new(float(16), float(20)), width: float(24), source_id: word(28),
            nodes: [crate::resources::walls::decorator::DecoratorId(word(32)),
                crate::resources::walls::decorator::DecoratorId(word(36))],
            kind: bytes[40], wire_tail: bytes[41..44].try_into().unwrap()}
    }
    pub fn observed(&self) -> [u8; 44] {
        let mut bytes = [0; 44];
        let words = [self.position.x.to_bits(), self.position.y.to_bits(), self.top_y.to_bits(),
            self.bottom_y.to_bits(), self.tangent.x.to_bits(), self.tangent.y.to_bits(),
            self.width.to_bits(), self.source_id, self.nodes[0].0, self.nodes[1].0];
        for (slot, word) in bytes[..40].chunks_exact_mut(4).zip(words) { slot.copy_from_slice(&word.to_le_bytes()); }
        bytes[40] = self.kind; bytes[41..].copy_from_slice(&self.wire_tail); bytes
    }
}

/// `StairsPillarProposals`：楼梯与墙交界处的支柱放置提议集合。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsPillarProposals {
    /// key（墙 id u64）→ 44B 提议记录桶。
    proposals: HashMap<u64, Vec<StairsPillarProposal>>,
}

impl StairsPillarProposals {
    /// `StairsPillarProposals::add`（0x140901270，O）：按 key 追加 44B 提议记录。
    pub fn add(&mut self, key: u64, proposal: [u8; 44]) {
        self.proposals
            .entry(key)
            .or_default()
            .push(StairsPillarProposal::from_observed(proposal));
    }

    /// 全量取出（pillars_process_proposals/pillars_visual 消费）。
    pub fn drain_all(&mut self, out: &mut Vec<[u8; 44]>) {
        for v in self.proposals.values_mut() {
            out.extend(v.iter().map(StairsPillarProposal::observed));
        }
        self.proposals.clear();
    }
    pub fn get(&self, key: u64) -> Option<&[StairsPillarProposal]> {
        self.proposals.get(&key).map(|bucket| bucket.as_slice())
    }
    pub fn is_empty(&self) -> bool {
        self.proposals.is_empty()
    }
    pub fn clear(&mut self) {
        self.proposals.clear();
    }
}

/// 拱支撑请求（O：`StairsArchSupportRequests`；construct_arches/
/// arch_walker 族消费；按墙 id 的支柱/支撑请求桶）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct StairsArchSupportRequests {
    /// 墙 id → 支撑请求记录（观测 8B 键 + 16B 值；布局 I）。
    pub requests: hashbrown::HashMap<u64, [u8; 16]>,
}

/// 楼梯支架请求（O：`StairsBracketsRequests`；construct_playermade_stairs
/// brackets 消费）。
#[derive(bevy_ecs::resource::Resource, Clone, Debug, Default)]
pub struct StairsBracketsRequests {
    /// 墙 id → 支架请求记录（布局 I）。
    pub requests: hashbrown::HashMap<u64, [u8; 16]>,
}
