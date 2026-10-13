//! pass_resource_access.rs — SSBO 访问登记与纹理访问 trait。
//!
//! 机器码锚点：
//! - `read_access_buffer<T>` 0x140970620（53 实例）→ `write_access_buffer<T>` 0x140973720（37）
//!   → `write_access_buffer_concurrent<T>` 0x140975c20（9）。
//! - 纹理访问 trait impl$0..impl$6 的 as_texture/dims（行 37/43/66/76/83/91/133/141）。
//!
//! 接入说明（assembly）：`rhapsody::rg::pass_builder::PassBuilder` /
//! `rhapsody::rg::pass_api::{RenderPassBinding, bind}` /
//! `rhapsody::raw::texture::{Texture, Extent2D}` 均走 `crate::rhapsody_contract` 边界。

use crate::render::pass_builder::pass_resources::PassResources;
use crate::rhapsody_contract::{
    self, Extent2D, PassBuilder, ReadOnlyHandle, RenderPassBinding, Texture,
};
use alloc::vec::Vec;

// ---- 访问类型常量（观测值；Rust 枚举名 U）----
/// 读访问（add_resource_access 的 access_type，观测到 0x02/0x17/0x1a/0x1d/0x1e/0x07 家族）。
pub const ACCESS_READ: u8 = 0x02;
/// 写访问类型（观测值族，精确值 U）。
pub const ACCESS_WRITE: u8 = 0x17;
/// 并发写访问类型（观测值族，精确值 U）。
pub const ACCESS_WRITE_CONCURRENT: u8 = 0x1a;
/// 默认同步组（原 add_resource_access 观测同步组 0）。
pub const SYNC_GROUP_DEFAULT: u32 = 0;

/// 游戏 SSBO 类型 → RG buffer 句柄解析器。
/// 具体 `*Ssbo` 结构体由 WP2 缓冲 / WP3 nani / WP4 实现本 trait（跨包契约，本层不定义）。
pub trait SsboType {
    /// `AssetSsboLibrary::get_rg_buffer<T>()` 的句柄 hash（供注册表查找）。
    fn rg_buffer_hash() -> u64;
}

/// 把已解析的 RG 句柄登记一次资源访问并追加绑定。
/// 观测 ABI：先 `PassBuilder::add_resource_access`，再 `pass_api::bind()` 追加 0x44B
/// `RenderPassBinding`（上限 0x100，超出 `unwrap_failed`）。
/// `bindings` = 调用方 pass 的绑定 Vec（ComputePass +0x00 / DrawBuilder 经上下文）。
pub(crate) fn append_resource_binding<UD>(
    builder: &mut PassBuilder<'_, UD>,
    resource_hash: u64,
    access_type: u8,
    sync_group: u32,
    bindings: &mut Vec<RenderPassBinding>,
) -> u8 {
    // 1. 注册表解析（ComputePass::read_rg_storage_buffer 0x14096dac0 的 hashbrown 查找，
    //    `debug_assert!(set_index == 0)`）。
    let (set_index, resource_index) = PassBuilder::<'_, UD>::resolve_resource(resource_hash);
    debug_assert!(
        set_index == 0,
        "resource set index must be 0 (assert_failed<SetIndex>)"
    );
    // 2. 登记访问。
    builder.add_resource_access(resource_index, access_type, sync_group);
    // 3. 绑定计数上限。
    let access_index = bindings.len() as u8;
    assert!(
        (bindings.len() as u64) < 0x100,
        "too many pass bindings (unwrap_failed)"
    );
    // 4. 追加 0x44B RenderPassBinding。
    let binding = rhapsody_contract::bind(resource_index, access_type);
    bindings.push(binding);
    access_index
}

/// `read_access_buffer<T>`（行 146）：只读访问。
pub fn read_access_buffer<T: SsboType>(
    builder: &mut PassBuilder<'_, PassResources>,
    bindings: &mut Vec<RenderPassBinding>,
) -> u8 {
    append_resource_binding(builder, T::rg_buffer_hash(), ACCESS_READ, SYNC_GROUP_DEFAULT, bindings)
}

/// `write_access_buffer<T>`（行 171）：写访问。
pub fn write_access_buffer<T: SsboType>(
    builder: &mut PassBuilder<'_, PassResources>,
    bindings: &mut Vec<RenderPassBinding>,
) -> u8 {
    append_resource_binding(
        builder,
        T::rg_buffer_hash(),
        ACCESS_WRITE,
        SYNC_GROUP_DEFAULT,
        bindings,
    )
}

/// `write_access_buffer_concurrent<T>`（行 186）：并发写访问（同步组不同）。
pub fn write_access_buffer_concurrent<T: SsboType>(
    builder: &mut PassBuilder<'_, PassResources>,
    bindings: &mut Vec<RenderPassBinding>,
) -> u8 {
    append_resource_binding(
        builder,
        T::rg_buffer_hash(),
        ACCESS_WRITE_CONCURRENT,
        SYNC_GROUP_DEFAULT,
        bindings,
    )
}

/// `ssbo_to_rg_handle`（inline 56 宿主）：把 `&T` 转成 RG 句柄 hash。
/// 原为内联辅助；宿主为 `read_access_buffer<T>` 与各 `write_storage_buffer<T>`。
#[inline]
pub fn ssbo_to_rg_handle<T: SsboType>() -> u64 {
    T::rg_buffer_hash()
}

// ---- 纹理访问 trait 族（as_texture / dims）----
// 各 impl（impl$0..impl$6）对应不同“可当纹理访问”的类型：
//   impl$0（module 549）行 37/43；impl$2（module 500）行 66/76；impl$3（module 589）行 83/91；
//   impl$6（module 507）行 133/141；impl$1/impl$4/impl$5 无独立编译实例（内联）。

/// 可作只读纹理访问的资源。
pub trait ReadTexture {
    fn as_texture(&self) -> &Texture;
    fn dims(&self) -> Extent2D;
}

/// 可作写/读写纹理访问的资源（含 `WriteTextureUnchecked` 等）。
pub trait WriteTexture {
    fn as_texture_mut(&mut self) -> &mut Texture;
    fn dims(&self) -> Extent2D;
}

impl ReadTexture for &Texture {
    fn as_texture(&self) -> &Texture {
        self
    }
    fn dims(&self) -> Extent2D {
        Extent2D {
            width: self.width,
            height: self.height,
        }
    }
}

impl ReadTexture for &rhapsody_contract::Handle<Texture> {
    fn as_texture(&self) -> &Texture {
        todo!("managed Handle<Texture>::as_texture — wired to rhapsody managed handle")
    }
    fn dims(&self) -> Extent2D {
        todo!("managed Handle<Texture>::dims — wired to rhapsody managed handle")
    }
}

impl ReadTexture for &ReadOnlyHandle<Texture> {
    fn as_texture(&self) -> &Texture {
        todo!("ReadOnlyHandle<Texture>::as_texture — wired to rhapsody temporal pair")
    }
    fn dims(&self) -> Extent2D {
        todo!("ReadOnlyHandle<Texture>::dims — wired to rhapsody temporal pair")
    }
}
