//! bind_resources.rs — ComputePass 资源绑定（bind_color_library / bind_ray_tracing）。
//!
//! 机器码锚点：
//! - `bind_color_library<ComputePass>` 0x1405896e0（448 B）/ 0x1415fc070（ICF 同体）；`<DrawBuilder>` 0x1416286f0（736 B）。
//! - `bind_ray_tracing<ComputePass>` 0x1415fc230（3287 B）；`<DrawBuilder>` 0x1416289d0。
//!
//! `bind_color_library<ComputePass>`（O）：
//!   1. `read_access_buffer::<ColorLibrarySsbo>(&PassResources, &mut PassBuilder, &mut bindings, access 0x1a)` → 下标；
//!   2. 注册表非空且命中名字哈希 `0x52ec6a17beb8fef0`（`assert_eq!(set, 0)`）→ 追加 `AccessRange{binding, 下标}`。
//!   **没有槽表**：DrawBuilder 版才有 `HashMap<&str, PassResourceSlot>::rustc_entry`（access 码 0x1f）。
//!
//! `bind_ray_tracing<ComputePass>`（O）：
//!   1. 从 world 取 `AssetSsboLibrary`（TypeId D；缺失 `Option::unwrap`）；
//!   2. 按 `SsboId` 取 `RtTrianglesSsbo` 与 `RtBvhSsbo` 的状态，`assert_eq!(state.name, type_name::<T>())`，
//!      取实际 `InstanceBuffer`（缺失 `Option::unwrap`）；
//!   3. 依次 `read_access_buffer<T>`（0x1a）+ 注册表追加：`RtTrianglesSsbo`/`0x168ddb2c3b5f38a9`、
//!      `RtTriangleMetaSsbo`/`0x47ddebfb264383d0`、`RtBvhSsbo`/`0x339e76846042a271`、
//!      `WallColorIdSsbo`/`0xe9701a257aa8828c`（ComputePass 版**不**写 CPU 缓冲）；
//!   4. 两次 `write_uniform<u32>`：键 `0x1c9f2eef0ac739ac` ← `RtTriangles` 的 instance_count 低 32 位，
//!      键 `0xc2b7f2fdf5e21521` ← `RtBvh` 的 instance_count 低 32 位。
//!
//! Fresh B recovery: RT entries use canonical InstanceBuffer storage: +0 is
//! instance_count, +8 is the actual Buffer. The two uniforms receive count,
//! never Buffer.id. Native add_new/upload/bind are one producer-consumer chain.

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(clippy::doc_lazy_continuation)]
use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::pass_builder::compute_pass_builder::ComputePass;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::{InstanceCounted, StorageElement};

/// `bind_color_library` 在注册表里查的名字哈希（O 立即数）。
pub const COLOR_LIBRARY_BINDING_HASH: u64 = 0x52ec6a17beb8fef0;
/// `bind_ray_tracing` 的绑定名哈希（O 立即数，按调用顺序）。
pub const RT_TRIANGLES_BINDING_HASH: u64 = 0x168ddb2c3b5f38a9;
pub const RT_TRIANGLE_META_BINDING_HASH: u64 = 0x47ddebfb264383d0;
pub const RT_BVH_BINDING_HASH: u64 = 0x339e76846042a271;
pub const WALL_COLOR_ID_BINDING_HASH: u64 = 0xe9701a257aa8828c;
/// `bind_ray_tracing` 两次 `write_uniform<u32>` 的键（O 立即数）。
pub const RT_TRIANGLES_UNIFORM_HASH: u64 = 0x1c9f2eef0ac739ac;
pub const RT_BVH_UNIFORM_HASH: u64 = 0xc2b7f2fdf5e21521;

impl ComputePass {
    /// Register the real typed color-library access and reflected range.
    pub fn bind_color_library(&mut self) {
        self.read_ssbo_in_place::<ColorLibrarySsbo>(COLOR_LIBRARY_BINDING_HASH);
    }
    /// Register real triangle/meta/BVH/wall-color storage and instance counts.
    pub fn bind_ray_tracing(&mut self) {
        bind_ray_tracing_compute::<RtTrianglesSsbo, RtTriangleMetaSsbo, RtBvhSsbo, WallColorIdSsbo>(self);
    }
}

/// `bind_ray_tracing<ComputePass>`（0x1415fc230），对四类 SSBO 泛型。
fn bind_ray_tracing_compute<Tri, Meta, Bvh, Wall>(pass: &mut ComputePass)
where
    Tri: LibrarySsbo,
    Tri::Storage: InstanceCounted,
    Meta: LibrarySsbo,
    Bvh: LibrarySsbo,
    Bvh::Storage: InstanceCounted,
    Wall: LibrarySsbo,
{
    let library = pass.resources().ssbo_library();
    // Original downcast returns InstanceBuffer: count at +0; Buffer begins +8.
    let triangles_word = library.get::<Tri>().unwrap().instance_count() as u32;
    let bvh_word = library.get::<Bvh>().unwrap().instance_count() as u32;

    pass.read_ssbo_in_place::<Tri>(RT_TRIANGLES_BINDING_HASH);
    pass.read_ssbo_in_place::<Meta>(RT_TRIANGLE_META_BINDING_HASH);
    pass.read_ssbo_in_place::<Bvh>(RT_BVH_BINDING_HASH);
    pass.read_ssbo_in_place::<Wall>(WALL_COLOR_ID_BINDING_HASH);

    pass.write_uniform_in_place(RT_TRIANGLES_UNIFORM_HASH, triangles_word);
    pass.write_uniform_in_place(RT_BVH_UNIFORM_HASH, bvh_word);
}

// ---- SSBO 类型载体与 `SsboType` 实现 ----
// `ColorLibrarySsbo` 由 WP4 `asset_libraries::color_library` 拥有；光线追踪 / 墙色 SSBO 载体为
// All four types now come from their real producer modules.

/// WP4 拥有的颜色库 SSBO。
pub use crate::asset_libraries::color_library::ColorLibrarySsbo;

/// 载体的 `SsboType::rg_buffer_hash`：库键 `SsboId = xxh3_64(NAME)`（O：`get_rg_buffer<T>` 的立即数；B 阶段 i_api 核对，
/// 旧版误返回 binding 名哈希——`PassHost::rg_buffer_resource` / `AssetSsboLibrary` 按 `SsboId` 工作）。
fn ssbo_id_of<T: StorageElement>() -> u64 {
    crate::asset_libraries::ssbo_library::SsboId::of::<T>().0.0
}

// These aliases preserve one canonical producer/consumer TypeId. No zero-size
// carrier or second same-NAME library entry is admitted.
pub use crate::resources::color_id::WallColorIdSsbo;
pub use crate::resources::rt::{RtBvhSsbo, RtTriangleMetaSsbo, RtTrianglesSsbo};

impl SsboType for ColorLibrarySsbo {
    fn rg_buffer_hash() -> u64 {
        // `SsboId = xxh3_64("ColorLibrary")`（旧版误返回 binding 名哈希 `COLOR_LIBRARY_BINDING_HASH`）。
        ssbo_id_of::<ColorLibrarySsbo>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binding_name_hashes_are_the_exe_immediates() {
        assert_eq!(COLOR_LIBRARY_BINDING_HASH, 0x52ec6a17beb8fef0);
        assert_eq!(RT_TRIANGLES_BINDING_HASH, 0x168ddb2c3b5f38a9);
        assert_eq!(RT_TRIANGLE_META_BINDING_HASH, 0x47ddebfb264383d0);
        assert_eq!(RT_BVH_BINDING_HASH, 0x339e76846042a271);
        // movabs rdx, -0x168fe5da85577d74 / -0x3d480d020a1deadf（有符号立即数取补）。
        assert_eq!(WALL_COLOR_ID_BINDING_HASH, (-0x168fe5da85577d74_i64) as u64);
        assert_eq!(RT_BVH_UNIFORM_HASH, (-0x3d480d020a1deadf_i64) as u64);
        assert_eq!(RT_TRIANGLES_UNIFORM_HASH, 0x1c9f2eef0ac739ac);
    }

    /// 载体与库侧 `SsboId` 一致（O：`get_rg_buffer<T>` 立即数 `xxh3_64(NAME)`；槽名即 NAME）。
    /// 立即数取自 `get_rg_buffer<RtTrianglesSsbo/RtTriangleMetaSsbo/RtBvhSsbo/WallColorIdSsbo>`
    /// （0x1412c7bb0 / 0x1412d1e40 / 0x1412d4150 / 0x1412cc500）。
    #[test]
    fn ssbo_hash_trait_is_the_library_ssbo_id() {
        assert_eq!(
            <RtTrianglesSsbo as SsboType>::rg_buffer_hash(),
            (-0x415cd24c40ce8837_i64) as u64
        );
        assert_eq!(
            <RtTriangleMetaSsbo as SsboType>::rg_buffer_hash(),
            0x0aa4d4a026938e06
        );
        assert_eq!(
            <RtBvhSsbo as SsboType>::rg_buffer_hash(),
            0x282b72fc752587c0
        );
        assert_eq!(
            <WallColorIdSsbo as SsboType>::rg_buffer_hash(),
            0xaf42315d263aa22a
        );
        assert_eq!(
            <ColorLibrarySsbo as SsboType>::rg_buffer_hash(),
            xxhash_rust::xxh3::xxh3_64(b"ColorLibrary")
        );
        // 绑定名哈希是另一回事（`read_storage_buffer` 的第二参数）。
        assert_ne!(
            <RtBvhSsbo as SsboType>::rg_buffer_hash(),
            RT_BVH_BINDING_HASH
        );
        assert_eq!(<RtBvhSsbo as StorageElement>::NAME, "RtBvh");
        assert_eq!(<WallColorIdSsbo as StorageElement>::NAME, "WallColorIds");
    }
}
