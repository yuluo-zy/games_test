//! terrain_rock_mask.rs — 岩体掩码 512×512 位栅格（WP8 恢复）。
//!
//! 源：B skeleton `resources/terrain_rock_mask.rs`。
//! 反编译：`impl$1::sample_at_world_xz`@0x1409ea800（行 43）：
//!   `(x/130 + 0.5)` 截断 [0,1] → ×512 → 索引；`(y*512 | x) >> 5 < len` 边界检查。
//! 布局（O）：岩体掩码为按位打包 u32 数组（512×512 = 8192 word = 32KB）。世界范围 ±130。
//! `read_terrain_data`@0x14098c680 `write_storage_buffer<TerrainRockMaskSsbo>` 从 GPU 回拷。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 岩体掩码 SSBO 元素类型（O：`read_terrain_data` `write_storage_buffer<TerrainRockMaskSsbo>`；
/// `nani_update_preplaced_flower` 断言其完整类型名 `country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo`）。
/// 一个元素 = 一个 u32 位字（I：512×512 位 = 0x2000 字，`dispatch_1d(0x2000)` 吻合）。
#[derive(Resource, Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct TerrainRockMaskSsbo(pub u32);

/// 位字数（O：`dispatch_1d(0x2000)`；512*512/32）。
pub const TERRAIN_ROCK_MASK_WORDS: usize = 0x2000;

impl crate::render::ssbo::StorageElement for TerrainRockMaskSsbo {
    const NAME: &'static str = "TerrainRockMaskSsbo";
    const BUFFER_LEN: usize = TERRAIN_ROCK_MASK_WORDS;
}

/// 存储为带 CPU 影子的缓冲（I：`ShadowedBuffer`；O：vtable `+0x18` 给出 u32 数组）。
impl crate::asset_libraries::ssbo_library::SsboType for TerrainRockMaskSsbo {
    type Storage = crate::render::shadowed_buffer::ShadowedBuffer<TerrainRockMaskSsbo>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        let words: Vec<TerrainRockMaskSsbo> = match initial {
            Some(bytes) => bytes
                .chunks_exact(4)
                .map(|c| TerrainRockMaskSsbo(u32::from_le_bytes([c[0], c[1], c[2], c[3]])))
                .collect(),
            None => vec![TerrainRockMaskSsbo(0); TERRAIN_ROCK_MASK_WORDS],
        };
        crate::render::shadowed_buffer::ShadowedBuffer::new(device, words)
    }
}

/// 岩体掩码数据（O 语义：512×512 位，按位打包；世界范围 ±130，f32→像素 `x/130+0.5`）。
/// 布局 U → typed gap。
#[derive(Resource, Clone, Debug, Default)]
pub struct TerrainRockMask {
    /// typed gap：{u32 位图数组, 世界范围} 字段 U。
    pub _gap: Gap,
}

impl TerrainRockMask {
    /// 世界 XZ → 岩体存在位（O：`sample_at_world_xz`@0x1409ea800；位索引 `(z*512|x)`，word `idx>>5`）。
    pub fn sample_at_world_xz(&self, x: f32, z: f32) -> bool {
        let fx = (x / 130.0 + 0.5).clamp(0.0, 1.0) * 512.0;
        let fz = (z / 130.0 + 0.5).clamp(0.0, 1.0) * 512.0;
        let (ix, iz) = (fx as usize, fz as usize);
        let idx = iz * 512 + ix;
        let _ = idx >> 5; // 边界检查依赖 word 长度（O）；布局 U
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
    use crate::render::ssbo::tests::MockDevice;

    #[test]
    fn registered_mask_exposes_cpu_shadow_words() {
        let dev = MockDevice::default();
        let lib = AssetSsboLibrary::new();
        lib.add_new_with_data::<TerrainRockMaskSsbo>(
            &dev,
            &[TerrainRockMaskSsbo(0), TerrainRockMaskSsbo(0x8000_0001)],
        );
        let words = lib
            .get_cpu_shadow_words::<TerrainRockMaskSsbo>()
            .expect("registered");
        assert_eq!(words, &[0, 0x8000_0001]);
    }

    #[test]
    fn unregistered_mask_is_none_like_the_original_unwrap_path() {
        let lib = AssetSsboLibrary::new();
        assert!(lib.get_cpu_shadow_words::<TerrainRockMaskSsbo>().is_none());
    }

    #[test]
    fn full_type_name_matches_the_asserted_literal() {
        // 机器码里断言的完整名以 `country_core::` 开头（crate 名在隔离台里不同，只比对模块路径部分）。
        assert!(
            core::any::type_name::<TerrainRockMaskSsbo>()
                .ends_with("::resources::terrain_rock_mask::TerrainRockMaskSsbo")
        );
    }
}
