//! `resources::color_id_roof` — 屋顶颜色 SSBO 元素（B 原模块路径
//! `country_core::resources::color_id_roof`）。
//!
//! 证据（O，fresh 反编译 2026-10-17）：
//! * `init_color_ids_buffer` 0x142290750 以
//!   `AssetSsboLibrary::add_new<color_id_roof::RoofColorIdSsbo>(&device)` 注册；
//! * `write_roof_color_id_to_gpu` 0x142298520 经 `FrozenMap::get<SsboId,..>` 取
//!   `BufferState`，`assert`（memcmp 0x37 = 55B）核对 `state.name == type_name`
//!   （= `"country_core::resources::color_id_roof::RoofColorIdSsbo"`，55 字符 ✓），
//!   经 vtable +0x18 取具体缓冲后读 `+0x18` 字节容量、逐元素 `update_buffer_bytes
//!   (4B, offset=idx<<2)`——元素 = u32（ColorId 表）；
//! * 着色器槽名（SsboId 键）`"RoofColorIds"`（draw_callbacks_b/instanced_roof/
//!   draw_bricks `read_storage_buffer<RoofColorIdSsbo>`；`ssbo_tags::RoofColorIdTag`
//!   全名/短名/哈希已登记）。
//!
//! 缓冲形态 = `TypedStorageBuffer<RoofColorIdSsbo>`（写面读 `+0x18` 字节容量；
//! 无 CPU 影子）。
use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::resources::color_id::MAX_NUMBER_OF_WALLS;

/// 屋顶颜色 id SSBO 元素（O：4B u32；`RoofColorIdSsbo`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RoofColorIdSsbo(pub u32);

impl StorageElement for RoofColorIdSsbo {
    /// 槽名（O：SsboId 键 = `xxh3_64("RoofColorIds")`）。
    const NAME: &'static str = "RoofColorIds";
    /// 固定 `MAX_NUMBER_OF_WALLS` 个 u32（O：与 `RoofColorIds` Vec 等长，无初始数据
    /// 也创建）。
    const BUFFER_LEN: usize = MAX_NUMBER_OF_WALLS;
}

impl LibrarySsbo for RoofColorIdSsbo {
    type Storage = TypedStorageBuffer<RoofColorIdSsbo>;
    fn new_storage_buffer(
        device: &dyn GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        TypedStorageBuffer::<RoofColorIdSsbo>::new_storage_buffer(device, initial)
    }
}

impl SsboType for RoofColorIdSsbo {
    /// `SsboId = xxh3_64("RoofColorIds")`（与 `ssbo_tags::RoofColorIdTag` 短名一致）。
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(Self::NAME).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_layout_matches_the_machine_code() {
        assert_eq!(core::mem::size_of::<RoofColorIdSsbo>(), 4);
        assert_eq!(RoofColorIdSsbo::BUFFER_LEN, MAX_NUMBER_OF_WALLS);
        // 全名 55 字符 → write_roof_color_id_to_gpu 的 memcmp 0x37 断言（O）。
        let full = core::any::type_name::<RoofColorIdSsbo>();
        assert_eq!(full, "country_core::resources::color_id_roof::RoofColorIdSsbo");
        assert_eq!(full.len(), 0x37);
    }
}
