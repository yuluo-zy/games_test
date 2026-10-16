//! `stairs_walls.rs` — 楼梯-墙支柱提议（B `resources/stairs_walls.rs`，1 业务 + 4 胶水）。
//!
//! `StairsPillarProposals::add`（0x140901270，O 反编译 7KB）：哈希表 `HashMap<key u64,
//! Vec<44B 记录>>`——命中 key 时把 44 字节记录 append 到桶 Vec，未命中时分配 44 字节
//! 插入单元素桶。44 字节记录字段语义 U（墙交界支柱放置提议）。
//! 章节：`chapters/resources/stairs_walls.rs.md`。
use bevy_ecs::resource::Resource;
use hashbrown::HashMap;

/// 墙交界支柱放置提议记录（44 B，字段语义 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StairsPillarProposal([u8; 44]);

impl Default for StairsPillarProposal {
    fn default() -> Self {
        Self([0; 44])
    }
}

impl StairsPillarProposal {
    pub fn from_observed(bytes: [u8; 44]) -> Self {
        Self(bytes)
    }
    pub fn observed(&self) -> &[u8; 44] {
        &self.0
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
            .push(StairsPillarProposal(proposal));
    }

    /// 全量取出（pillars_process_proposals/pillars_visual 消费）。
    pub fn drain_all(&mut self, out: &mut Vec<[u8; 44]>) {
        for v in self.proposals.values_mut() {
            out.extend(v.iter().map(|p| p.0));
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
