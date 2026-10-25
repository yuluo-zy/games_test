//! rt/bvh.rs — BVH 构建（WP16 逻辑重述）。
//!
//! O：`build_gpu_bvh`（0x140b0da50）：输入三角形（3×16B 顶点行 [x,y,z,z]）→
//! `obvhs::cwbvh::build_cwbvh_from_tris`（复用库，typed gap）→ 叶序三角形 + `Vec<u32>`
//! 索引 → `GpuBlBvh` + 三角形重排。`GpuBlBvh` 节点 SSBO 记录 80B/项。

use glam::Vec3;

use crate::typed_gap::Gap;

/// `GpuBlBvh`（O：BVH 节点记录 80B/项；布局 U）。
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GpuBlBvh(pub Gap);

/// `build_gpu_bvh`（O 0x140b0da50）：输入三角形（含 AABB）→ 输出重排三角形 + 索引 +
/// BVH 节点。
///
/// 复用边界：`obvhs::cwbvh::builder::build_cwbvh_from_tris`（本地 crate，typed gap）。
pub fn build_gpu_bvh(
    triangles: &[[Vec3; 3]],
    _aabbs: &[Gap],
) -> (Vec<Gap>, Vec<u32>, GpuBlBvh) {
    let _ = triangles;
    (Vec::new(), Vec::new(), GpuBlBvh::default())
}
