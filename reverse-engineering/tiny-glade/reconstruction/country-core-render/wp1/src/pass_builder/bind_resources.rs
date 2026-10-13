//! bind_resources.rs — BindGameResources trait（bind_color_library / bind_ray_tracing / bind_shadowmap）。
//!
//! 机器码锚点：
//! - `bind_color_library<ComputePass>` 0x1415fc070（行 55）；`<DrawBuilder>` 0x1416286f0（行 55）。
//! - `bind_ray_tracing<ComputePass>` 0x1415fc230（行 59）；`<DrawBuilder>` 0x1416289d0（行 59）。

use crate::render::pass_builder::compute_pass_builder::AccessRange;
use crate::render::pass_builder::pass_resource_access;
use crate::render::pass_builder::pass_resource_access::SsboType;

/// 资源槽表项（`HashMap<&str, PassResourceSlot>`，键为资源名字符串）。
#[derive(Clone, Copy, Debug)]
pub struct PassResourceSlot {
    pub access_index: u8,
}

/// 游戏资源绑定 trait。
/// 实现者：`ComputePass`、`DrawBuilder`。
pub trait BindGameResources {
    /// 登记 `ColorLibrarySsbo` 读访问（行 55）。
    fn bind_color_library(&mut self) {
        bind_color_library_impl(self)
    }
    /// 登记软件 BVH 组（RtTriangles / RtTriangleMeta / RtBvh）+ 墙色回读（行 59）。
    fn bind_ray_tracing(&mut self) {
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
    ) -> rhapsody::rg::pass_builder::PassBuilder<'_, crate::render::pass_builder::pass_resources::PassResources>;
    /// 绑定 Vec 载体（接入 rhapsody 时由实现方给出真实 Vec）。
    fn bindings_placeholder(&mut self) -> alloc::vec::Vec<rhapsody::rg::pass_api::RenderPassBinding>;
}

/// 绑定颜色库读访问 + 记录访问范围。
/// 观测：槽为空 → `read_access_buffer<ColorLibrarySsbo>` + 插入；再按 hash
/// `0x52ec6a17beb8fef0` 解析 resource_index → 追加 8B access_range。
pub fn bind_color_library_impl<P: BindGameResources>(pass: &mut P) {
    let access_index = {
        let slot = pass.resource_slot("ColorLibrarySsbo");
        if let Some(slot) = slot.as_ref() {
            slot.access_index
        } else {
            let idx = pass_resource_access::read_access_buffer::<ColorLibrarySsbo>(
                &mut pass.as_pass_builder(),
                &mut pass.bindings_placeholder(),
            );
            *slot = Some(PassResourceSlot {
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
        let access_index = {
            let slot = pass.resource_slot(name);
            if let Some(slot) = slot.as_ref() {
                slot.access_index
            } else {
                let idx = match name {
                    "RtTrianglesSsbo" => pass_resource_access::read_access_buffer::<RtTrianglesSsbo>(
                        &mut pass.as_pass_builder(),
                        &mut pass.bindings_placeholder(),
                    ),
                    "RtTriangleMetaSsbo" => pass_resource_access::read_access_buffer::<RtTriangleMetaSsbo>(
                        &mut pass.as_pass_builder(),
                        &mut pass.bindings_placeholder(),
                    ),
                    _ => pass_resource_access::read_access_buffer::<RtBvhSsbo>(
                        &mut pass.as_pass_builder(),
                        &mut pass.bindings_placeholder(),
                    ),
                };
                *slot = Some(PassResourceSlot { access_index: idx });
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

// 以下 SSBO 类型由 WP2 缓冲 / WP3 持有，仅作泛型参数引用；此处占位结构体在各自包
// 实现 `SsboType` 后由跨包集成替换（WP1 不重定义 SSBO 结构体）。
pub struct ColorLibrarySsbo;
pub struct RtTrianglesSsbo;
pub struct RtTriangleMetaSsbo;
pub struct RtBvhSsbo;
pub struct WallColorIdSsbo;
