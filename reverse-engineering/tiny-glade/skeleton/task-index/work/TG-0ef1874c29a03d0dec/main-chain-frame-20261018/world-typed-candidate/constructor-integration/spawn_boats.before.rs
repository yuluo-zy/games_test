//! `spawn_boats.rs` — 船 / 水车 spawn 的 SSBO 载体（O：该骨架文件在 PDB 里**没有任何过程**，只有 `BoatSsbo` 一个命名实体）。
//!
//! 事实（O）：
//! - `/water/spawn_boats.cs.hlsl` 存在于重建 HLSL（`reconstruction/shader-source/shaders/water/spawn_boats.cs.hlsl`），
//!   其 ComputePass 的 URI 在原 EXE 里是**动态拼接**的（静态 URI 表里没有它）；声明该 pass 的函数属于 `compute_water`
//!   （`water_spawn_vegetation` 族，由 compute_water 的拥有者恢复），不在本文件。
//! - `spawn_waterwheels.cs.hlsl` 的 GPU 着色器是空体（O：SPIR-V）→ 水车 spawn 在 CPU 侧（I）。
//! - `BoatSsbo` 的库键：`SsboId = xxh3_64("Boats")` = 0x4f69c51e83e1d670（O：`get_rg_buffer<BoatSsbo>` 的立即数与
//!   `NameHash::compute("Boats")` 逐位一致），故 `StorageElement::NAME = "Boats"`。
//! - 生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new<BoatSsbo>`（0x1412c23d0）、`get_rg_buffer<BoatSsbo>`（0x1412d3af0）、
//!   `read_access_buffer<BoatSsbo>`（0x14096fe20）、`DrawBuilder::read_storage_buffer<BoatSsbo>`（0x141631690）。
//!
//! 本文件因此只提供：带真实 `NAME` 的库 SSBO 类型（`LibrarySsbo`），使上述泛型胶水可以直接实例化。

use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::systems::render_loop::compute_passes::pass_kit::{binding_hash, ssbo_carrier};
use crate::typed_gap::Gap;

/// 船实例 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct BoatSsbo(pub Gap);
ssbo_carrier!(BoatSsbo, "Boats");

/// 绑定名（O：`spawn_boats.cs.hlsl` 里的 SSBO 名；推断为 `boat_buffer`，立即数未在已恢复函数中出现 → U）。
pub const BOAT_BINDING: u64 = binding_hash("boat_buffer");

impl SsboType for BoatSsbo {
    /// 旧 trait（`rg_buffer_hash`）：原值 = 注册键 `xxh3_64("Boats")`（O）。
    fn rg_buffer_hash() -> u64 {
        0x4f69c51e83e1d670
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::ssbo::StorageElement;
    use crate::render::test_device::TestDevice;
    use crate::rhapsody_types::RenderGraph;

    #[test]
    fn library_key_name_is_boats() {
        assert_eq!(BoatSsbo::NAME, "Boats");
        assert_eq!(BoatSsbo::rg_buffer_hash(), 0x4f69c51e83e1d670);
    }

    #[test]
    fn register_then_import_into_the_render_graph() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<BoatSsbo>(&TestDevice(1));
        let mut rg = RenderGraph::default();
        let a = lib.get_rg_buffer::<BoatSsbo>(&mut rg);
        // 再取一次返回同一个导入句柄（O：已导入则复用）。
        assert_eq!(lib.get_rg_buffer::<BoatSsbo>(&mut rg), a);
    }

    #[test]
    #[should_panic(expected = "Tried to register the same storage buffer twice")]
    fn double_registration_panics() {
        let lib = AssetSsboLibrary::new();
        lib.add_new::<BoatSsbo>(&TestDevice(1));
        lib.add_new::<BoatSsbo>(&TestDevice(1));
    }
}
