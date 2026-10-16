//! 墙材质统计（walls/material_stats.rs）。
//!
//! 原 B `resources/walls/material_stats.rs`（PDB 实体 `MaterialStats`；
//! construct_arches/construct_lintels 写回：set_arches_segments_count/
//! add_arches_length/add_gate_length/set_gates_segments_count）。
use crate::Gap;

/// 墙材质统计（O 名；布局 U → Gap）。
#[derive(Clone, Debug, Default)]
pub struct MaterialStats {
    /// 原 64B 统计块观测（I：字段布局待独立证据）。
    pub raw: Gap,
}

impl MaterialStats {
    /// 拱段计数写回（O：construct_arches 调用）。
    pub fn set_arches_segments_count(&mut self, _count: u32) {}
    /// 拱长度累加（O）。
    pub fn add_arches_length(&mut self, _length: f32) {}
    /// 门长度累加（O）。
    pub fn add_gate_length(&mut self, _length: f32) {}
    /// 门段计数写回（O）。
    pub fn set_gates_segments_count(&mut self, _count: u32) {}
}
