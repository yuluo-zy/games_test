//! stairs_walls.rs — 楼梯柱建议资源（WP21；扩展 WP12 facade）。
//!
//! O：`StairsPillarProposals::add` 0x901270 —— `HashMap<StairGraphEdgeIdentity,
//! Vec<StairsPillarProposal>>`，`StairsPillarProposal` = 44B。WP12 facade 已有
//! `StairsPillarProposals { raw: Gap }`；本文件扩展为真实容器面。

use std::collections::HashMap;

use bevy_ecs::resource::Resource;

use crate::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity;
use crate::typed_gap::Gap;

/// `StairsPillarProposal`（O：44 字节（0x2c）：`+0x0..0x20 4×u64、+0x1c..0x24 u64、
/// +0x24 u64`；柱子位置/朝向/参数，字段名 U）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StairsPillarProposal {
    pub a: [f64; 2],
    pub b: [f64; 2],
    pub c: f64,
    pub d: f64,
    /// 未映射区（U）。
    pub _gap: Gap,
}

/// `StairsPillarProposals`（O：`HashMap<StairGraphEdgeIdentity, Vec<StairsPillarProposal>>`）。
#[derive(Resource, Clone, Debug, Default)]
pub struct StairsPillarProposals {
    pub by_edge: HashMap<StairGraphEdgeIdentity, Vec<StairsPillarProposal>>,
}

impl StairsPillarProposals {
    /// `add`（O 0x901270）：按键找 Vec，push 44B 记录。
    pub fn add(&mut self, edge: StairGraphEdgeIdentity, proposal: StairsPillarProposal) {
        self.by_edge.entry(edge).or_default().push(proposal);
    }
}

/// `StairArchType`（O 名；楼梯拱类型判别，U）。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StairArchType(pub u8);

/// `StairsArchSupportAssemblyParams`（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct StairsArchSupportAssemblyParams(pub Gap);

/// `StairsArchSupportRequests`（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct StairsArchSupportRequests(pub Gap);

/// `StairsBracketAssemblyParams`（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct StairsBracketAssemblyParams(pub Gap);

/// `StairsBracketsRequests`（O 名；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct StairsBracketsRequests(pub Gap);
