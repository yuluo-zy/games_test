//! bind_resources.rs — BindGameResources trait（bind_color_library / bind_ray_tracing / bind_shadowmap）。
//!
//! 机器码锚点：
//! - `bind_color_library<ComputePass>` 0x1415fc070（行 55）；`<DrawBuilder>` 0x1416286f0（行 55）。
//! - `bind_ray_tracing<ComputePass>` 0x1415fc230（行 59）；`<DrawBuilder>` 0x1416289d0（行 59）。
//!
//! 接入说明（assembly）：`ColorLibrarySsbo` 解析到 WP4 `asset_libraries::color_library`；
//! 光线追踪 / 墙色 SSBO 载体保持为 typed-gap 占位并实现 `SsboType`（WP1 不重定义 SSBO 结构体；
//! 集成时若相应游戏类型落地则替换）。`rhapsody::rg::pass_api` 走 `crate::rhapsody_contract`。

use crate::render::pass_builder::compute_pass_builder::AccessRange;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::rhapsody_contract::{PassBuilder, RenderPassBinding};
use alloc::vec::Vec;

/// 资源槽表项（`HashMap<&str, PassResourceSlot>`，键为资源名字符串）。
#[derive(Clone, Copy, Debug)]
pub struct PassResourceSlot {
    pub access_index: u8,
}

/// 游戏资源绑定 trait。
/// 实现者：`ComputePass`、`DrawBuilder`。
pub trait BindGameResources {
    /// 登记 `ColorLibrarySsbo` 读访问（行 55）。
    fn bind_color_library(&mut self)
    where
        Self: Sized,
    {
        bind_color_library_impl(self)
    }
    /// 登记软件 BVH 组（RtTriangles / RtTriangleMeta / RtBvh）+ 墙色回读（行 59）。
    fn bind_ray_tracing(&mut self)
    where
        Self: Sized,
    {
        bind_ray_tracing_impl(self)
    }
    /// 登记 shadowmap（inline 12 宿主）。
    fn bind_shadowmap(&mut self) {}

    // —— 内部辅助（实现方提供）——
    /// 资源槽表访问。
    fn resource_slot(&mut self, name: &'static str) -> &mut Option<PassResourceSlot>;
    /// 追加资源访问范围。
    fn push_access_range(&mut self, range: AccessRange);
    /// 墙色 ID 回读（ComputePass 变体省略 CPU 写）。
    fn read_wall_color_ids(&mut self);
    /// rhapsody PassBuilder 边界接入。
    fn as_pass_builder(
        &mut self,
    ) -> PassBuilder<'_, crate::render::pass_builder::pass_resources::PassResources>;
    /// 绑定 Vec 载体（接入 rhapsody 时由实现方给出真实 Vec）。
    fn bindings_placeholder(&mut self) -> Vec<RenderPassBinding>;
}

/// 绑定颜色库读访问 + 记录访问范围。
/// 观测：槽为空 → `read_access_buffer<ColorLibrarySsbo>` + 插入；再按 hash
/// `0x52ec6a17beb8fef0` 解析 resource_index → 追加 8B access_range。
pub fn bind_color_library_impl<P: BindGameResources>(pass: &mut P) {
    let existing = pass.resource_slot("ColorLibrarySsbo").as_ref().map(|s| s.access_index);
    let access_index = match existing {
        Some(idx) => idx,
        None => {
            // read_access_buffer::<ColorLibrarySsbo>(&mut pass.as_pass_builder(),
            //   &mut pass.bindings_placeholder()) — 双 `&mut pass` 借用待 rhapsody
            //   PassBuilder 落地（as_pass_builder 为 todo 边界）；typed-gap 形态。
            let _ = pass.as_pass_builder();
            let idx = pass.bindings_placeholder().len() as u8;
            *pass.resource_slot("ColorLibrarySsbo") = Some(PassResourceSlot {
                access_index: idx,
            });
            idx
        }
    };
    if let Some((_set, resource_index)) = resolve_resource_hash(0x52ec6a17beb8fef0) {
        pass.push_access_range(AccessRange {
            resource_index,
            access_index,
        });
    }
}

/// 绑定软件 BVH 组读访问 + 墙色回读。
/// 观测：按 SsboId frozen map 取 BufferState（名称字面量断言），逐资源登记读访问，
/// 最后 `read_storage_buffer<WallColorIdSsbo>` 并写 CPU 缓冲。
pub fn bind_ray_tracing_impl<P: BindGameResources>(pass: &mut P) {
    for (name, hash) in [
        ("RtTrianglesSsbo", 0x168ddb2c3b5f38a9u64),
        ("RtTriangleMetaSsbo", 0x47ddebfb264383d0u64),
        ("RtBvhSsbo", 0x339e76846042a271u64),
    ] {
        let existing = pass.resource_slot(name).as_ref().map(|s| s.access_index);
        let access_index = match existing {
            Some(idx) => idx,
            None => {
                // 观测：read_access_buffer::<RtTrianglesSsbo/RtTriangleMetaSsbo/RtBvhSsbo>
                //   (&mut pass.as_pass_builder(), &mut pass.bindings_placeholder()) —
                //   双 `&mut pass` 借用待 rhapsody PassBuilder 落地（typed-gap 形态）。
                let _ = pass.as_pass_builder();
                let idx = pass.bindings_placeholder().len() as u8;
                *pass.resource_slot(name) = Some(PassResourceSlot { access_index: idx });
                idx
            }
        };
        if let Some((_set, resource_index)) = resolve_resource_hash(hash) {
            pass.push_access_range(AccessRange {
                resource_index,
                access_index,
            });
        }
    }
    // 墙色 ID 回读（DrawBuilder 变体）；ComputePass 变体省略 CPU 写。
    pass.read_wall_color_ids();
}

/// 注册表 hash 解析（未命中 None；观测 assert set_index==0）。
/// typed gap：经 *(pass+0xe8) 注册表解析；接入 rhapsody 时落实。
fn resolve_resource_hash(_hash: u64) -> Option<(u32, u32)> {
    None
}

// ---- SSBO 类型载体与 `SsboType` 实现 ----
// `ColorLibrarySsbo` 由 WP4 `asset_libraries::color_library` 拥有；光线追踪 / 墙色
// SSBO 载体为 typed-gap 占位（WP1 不重定义游戏 SSBO 结构体）。hash 为观测注册表键。

/// WP4 拥有的颜色库 SSBO。
pub use crate::asset_libraries::color_library::ColorLibrarySsbo;

/// 光线追踪三角形缓冲（typed-gap 载体；hash O）。
pub struct RtTrianglesSsbo;
/// 光线追踪三角形元数据缓冲（typed-gap 载体；hash O）。
pub struct RtTriangleMetaSsbo;
/// 软件 BVH 节点缓冲（typed-gap 载体；hash O）。
pub struct RtBvhSsbo;
/// 墙色 ID 回读缓冲（typed-gap 载体）。
pub struct WallColorIdSsbo;

impl SsboType for ColorLibrarySsbo {
    fn rg_buffer_hash() -> u64 {
        0x52ec6a17beb8fef0
    }
}
impl SsboType for RtTrianglesSsbo {
    fn rg_buffer_hash() -> u64 {
        0x168ddb2c3b5f38a9
    }
}
impl SsboType for RtTriangleMetaSsbo {
    fn rg_buffer_hash() -> u64 {
        0x47ddebfb264383d0
    }
}
impl SsboType for RtBvhSsbo {
    fn rg_buffer_hash() -> u64 {
        0x339e76846042a271
    }
}
impl SsboType for WallColorIdSsbo {
    fn rg_buffer_hash() -> u64 {
        0
    }
}
