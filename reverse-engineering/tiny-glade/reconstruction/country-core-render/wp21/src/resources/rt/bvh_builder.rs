//! rt/bvh_builder.rs — BVH 后台构建器（WP16 逻辑重述）。
//!
//! O：`RtBvhBuilderGlue`（三角形列表 +0x48/+0x50、元 +0x60、BVH 节点 +0x30、busy/idle
//! +0x14/+0x15）；`bvh_builder_worker`（0x140b24d50）后台线程状态机收集脏网格三角形
//! （40B `InputTriangle`，stride 0x28）→ build_gpu_bvh → 写回清 busy。`InputTriangle`
//! 40B。`RtBvhBuilderOutput`。

use crate::typed_gap::Gap;

/// `InputTriangle`（O：40 字节：3 顶点 + 法向/元；stride 0x28）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InputTriangle(pub Gap);

/// `RtBvhBuilderOutput`（O：构建输出；布局 U）。
#[derive(Clone, Debug, Default)]
pub struct RtBvhBuilderOutput(pub Gap);

/// `RtBvhBuilderGlue`（O：+0x30 BVH 节点、+0x48/+0x50 三角形列表、+0x60 三角形元、
/// busy/idle 标志 +0x14/+0x15）。
#[derive(Clone, Debug, Default)]
pub struct RtBvhBuilderGlue {
    /// busy 标志（O：+0x14）。
    pub busy: bool,
    /// idle 标志（O：+0x15）。
    pub idle: bool,
    /// 三角形列表（O：+0x48/+0x50）。
    pub triangles: Vec<InputTriangle>,
    /// 三角形元（O：+0x60）。
    pub triangle_meta: Vec<Gap>,
    /// BVH 节点（O：+0x30）。
    pub bvh_nodes: Vec<Gap>,
    /// 未映射区（U）。
    pub _gap: Gap,
}

impl RtBvhBuilderGlue {
    /// `is_idle`（O：RtBvhBuilderGlueResource::is_idle 读锁调用）。
    pub fn is_idle(&self) -> bool {
        self.idle
    }
}

/// `bvh_builder_worker`（O 0x140b24d50）：后台线程主循环。状态==2 空闲 sleep，否则
/// 收集脏网格三角形 → build_gpu_bvh → 写回 glue，清 busy。
///
/// 复用边界：obvhs cwbvh（typed gap）；线程/锁为 std/Arc<RwLock>。
pub fn bvh_builder_worker(_glue: &std::sync::RwLock<RtBvhBuilderGlue>, _world: &crate::typed_gap::Gap) {
    // 系统级后台循环；骨架表面。
}
