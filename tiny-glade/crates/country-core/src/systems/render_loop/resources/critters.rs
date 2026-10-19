//! `render_loop/resources/critters.rs` — critter 的 GPU 实例缓冲元素类型（四个 SSBO）。
//!
//! PDB 里本模块没有独立函数（functions=0）；类型的真实内容由下面几组单态化实例给出（均为 O）：
//!
//! | 类型 | `new_storage_buffer<T>` | 槽名 / `SsboId` | 元素 | 缓冲 |
//! |---|---|---|---|---|
//! | `ButterflySsbo` | 0x14161b0f0（`impl$4`，映射缓冲） | "Butterfly Buffer" / 0x61619a2e1d85da94 | 32 B | 0x300 B = 24 个 |
//! | `FireflySsbo` | 0x14161f1e0（`impl$4`） | "Firefly Buffer" / 0x29d4a64d5ea39871 | 16 B | 0x600 B = 96 个 |
//! | `FireflyVisibilitySsbo` | 0x14161fea0（`impl$4`，`memory = 4`） | "Firefly Vis Buffer" / 0x9248897309ea4fea | 4 B | 0x180 B = 96 个 |
//! | `SheepAnimationSsbo` | 0x14160f920（`impl$0`，类型化缓冲） | "Sheep Animation" / 0xc70858c12c64d9ba | 0x3e0 B | 随初始数据 |
//!
//! 元素布局：
//! - Butterfly / Firefly 来自 `map<…>::next` 闭包（0x141dbcb20 / 0x141dbd070；`Query<(&Transform, &Butterfly|&Firefly)>`
//!   → SSBO 元素）：位置 = `Transform.translation`（`Transform` 0x30 B：旋转 @0、平移 @0x10、缩放 @0x1c），
//!   `Butterfly` 的 `f32 @+0x14` 写到元素 @0xc，朝向 = `rotation * Vec3::NEG_Z`（闭包里内联的四元数乘向量，常量
//!   `[-0,-0,-1]`），末尾填充 0。着色器侧同形（`shaders/critters/{butterfly,firefly}.raster.hlsl` 的
//!   `struct Butterfly { float3 position; float anim_t; float3 forward; uint pad; }`）。
//! - `FireflyVisibilitySsbo` 是 `f32`（`firefly_vis_query.cs.hlsl` 的 `RWStructuredBuffer<float>`；缓冲 0x180 = 96 × 4）。
//! - `SheepAnimationSsbo` 元素 0x3e0 B（`grow_one` / `add_new_with_data` 里的 `imul 0x3e0`），布局见着色器
//!   `sheep.raster.hlsl`：30 个位置 + 30 个法线（float4）+ 去光照姿态对。
//!
//! 缺省值（无初始数据时，O）：蝴蝶 24 个元素全部写成 `position=0, anim_t=0, forward=(-0,-0,-1), pad=0`
//! （0x14161b172 起的展开）；萤火虫 / 可见性为全 0（`memset`）。
//!
//! 库侧 `SsboType`（B 阶段 i_infra）：三个映射缓冲 `ButterflySsbo` / `FireflySsbo` / `FireflyVisibilitySsbo` 的存储是
//! `MappedStorageBuffer<'static, T>`（i_api 已修为 `BUFFER_LEN × size_of::<T>()` 字节、`None` 时写缺省元素、`memory = T::MAPPED_MEMORY`），
//! 与 `impl$4::new_storage_buffer` 逐实例吻合；`FireflyVisibilitySsbo` 的 memory = 4（GPU→CPU 回读，I）。
//! `SheepAnimationSsbo` 是 `TypedStorageBuffer`，同样实现库侧 `SsboType`。

use glam::{Quat, Vec3};

use crate::asset_libraries::ssbo_library::SsboType as LibrarySsbo;
use crate::name_hash::NameHash;
use crate::render::pass_builder::pass_resource_access::SsboType;
use crate::render::ssbo::{
    MAPPED_MEMORY_READBACK, MappedStorageBuffer, StorageElement, TypedStorageBuffer,
};
use crate::systems::render_loop::draw_passes::draw_shaders::ssbo_tags::DrawSsbo;

/// 蝴蝶实例（32 B）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ButterflySsbo {
    pub position: Vec3,
    /// `Butterfly` 组件 `+0x14` 处的 f32（着色器里叫 `anim_t`）。
    pub anim_t: f32,
    pub forward: Vec3,
    pub pad: u32,
}

impl ButterflySsbo {
    /// 闭包 0x141dbcb20：`position = translation`、`forward = rotation * -Z`、`pad = 0`。
    pub fn from_components(translation: Vec3, rotation: Quat, anim_t: f32) -> Self {
        ButterflySsbo {
            position: translation,
            anim_t,
            forward: rotation * Vec3::NEG_Z,
            pad: 0,
        }
    }
}

impl Default for ButterflySsbo {
    /// O 0x14161b172 起：位置 0、`anim_t` 0、朝向 `(-0.0, -0.0, -1.0)`（常量 0x1429cd330 = `(-0.0, -0.0)`）、`pad` 0。
    fn default() -> Self {
        ButterflySsbo {
            position: Vec3::ZERO,
            anim_t: 0.0,
            forward: Vec3::new(-0.0, -0.0, -1.0),
            pad: 0,
        }
    }
}

impl StorageElement for ButterflySsbo {
    const NAME: &'static str = "Butterfly Buffer";
    /// 0x300 / 32。
    const BUFFER_LEN: usize = 24;
    /// O 0x14161b172 起：无初始数据时 24 个元素全部写成常量元素（见 `Default`）。
    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}

/// 萤火虫实例（16 B）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FireflySsbo {
    pub position: Vec3,
    pub pad: u32,
}

impl FireflySsbo {
    /// 闭包 0x141dbd070：`position = translation`，`pad = 0`。
    pub fn from_translation(translation: Vec3) -> Self {
        FireflySsbo {
            position: translation,
            pad: 0,
        }
    }
}

impl StorageElement for FireflySsbo {
    const NAME: &'static str = "Firefly Buffer";
    /// 0x600 / 16。
    const BUFFER_LEN: usize = 96;
}

/// 萤火虫可见性（`f32`，由 `_critters_firefly_vis_query.cs` 写）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FireflyVisibilitySsbo(pub f32);

impl StorageElement for FireflyVisibilitySsbo {
    const NAME: &'static str = "Firefly Vis Buffer";
    /// 0x180 / 4。
    const BUFFER_LEN: usize = 96;
    /// O 0x14161fea0：`BufferDesc::new(0x180, 0x22, 4)`（回读缓冲；名称 I）。
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

/// 三个映射缓冲的库侧 `SsboType`（O：`impl$4::new_storage_buffer<T>` 0x14161b0f0 / 0x14161f1e0 / 0x14161fea0）。
macro_rules! mapped_library_ssbo {
    ($($t:ty),*) => {$(
        impl LibrarySsbo for $t {
            type Storage = MappedStorageBuffer<'static, $t>;
            fn new_storage_buffer(
                device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
                initial: Option<&[u8]>,
            ) -> Self::Storage {
                MappedStorageBuffer::<$t>::new_storage_buffer(device, initial)
            }
        }
    )*};
}
mapped_library_ssbo!(ButterflySsbo, FireflySsbo, FireflyVisibilitySsbo);

/// 绵羊动画（0x3e0 B = 62 × float4）。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheepAnimationSsbo {
    pub positions: [[f32; 4]; 30],
    pub normals: [[f32; 4]; 30],
    pub delighted_pos: [f32; 4],
    pub delighted_normal: [f32; 4],
}

// SAFETY: repr(C) contains only62 consecutive [f32;4] arrays. Alignment4
// and992-byte size leave no implicit padding or invalid initialized byte.
unsafe impl bytemuck::NoUninit for SheepAnimationSsbo {}
const _: () = {
    assert!(core::mem::size_of::<SheepAnimationSsbo>() == 992);
    assert!(core::mem::align_of::<SheepAnimationSsbo>() == 4);
};

impl StorageElement for SheepAnimationSsbo {
    const NAME: &'static str = "Sheep Animation";
    // BUFFER_LEN = 0：缓冲大小随初始数据（`new_storage_buffer<SheepAnimationSsbo>` 无 BUFFER_LEN 常量）。
}

impl LibrarySsbo for SheepAnimationSsbo {
    type Storage = TypedStorageBuffer<SheepAnimationSsbo>;

    /// O 0x14160f920（`impl$0::new_storage_buffer`）：必须带数据（`BufferDesc::new(len, 0x22, 2).alignment(4)`，名字
    /// "Sheep Animation"）。
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        TypedStorageBuffer::<SheepAnimationSsbo>::new_storage_buffer(device, initial)
    }
}

macro_rules! rg_hash_from_name {
    ($($t:ty),*) => {$(
        impl SsboType for $t {
            /// `SsboId = NameHash(xxh3_64(NAME))`（A3 已用 EXE 立即数核对过该规则；各类型的值见测试）。
            fn rg_buffer_hash() -> u64 {
                NameHash::compute(<$t as StorageElement>::NAME).0
            }
        }
        impl DrawSsbo for $t {
            const SLOT_NAME: &'static str = <$t as StorageElement>::NAME;
            const TYPE_NAME: &'static str = concat!(
                "country_core::systems::render_loop::resources::critters::",
                stringify!($t)
            );
        }
    )*};
}
rg_hash_from_name!(
    ButterflySsbo,
    FireflySsbo,
    FireflyVisibilitySsbo,
    SheepAnimationSsbo
);

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::size_of;

    #[test]
    fn element_sizes_match_the_original_buffers() {
        assert_eq!(size_of::<ButterflySsbo>(), 32);
        assert_eq!(size_of::<FireflySsbo>(), 16);
        assert_eq!(size_of::<FireflyVisibilitySsbo>(), 4);
        assert_eq!(size_of::<SheepAnimationSsbo>(), 0x3e0);
        // 缓冲字节数 = BUFFER_LEN × 元素大小（0x14161b0f0 / 0x14161f1e0 / 0x14161fea0 里的 create_buffer 大小）。
        assert_eq!(
            ButterflySsbo::BUFFER_LEN * size_of::<ButterflySsbo>(),
            0x300
        );
        assert_eq!(FireflySsbo::BUFFER_LEN * size_of::<FireflySsbo>(), 0x600);
        assert_eq!(
            FireflyVisibilitySsbo::BUFFER_LEN * size_of::<FireflyVisibilitySsbo>(),
            0x180
        );
    }

    #[test]
    fn rg_hashes_equal_the_ssbo_ids_in_the_binary() {
        assert_eq!(ButterflySsbo::rg_buffer_hash(), 0x61619a2e1d85da94);
        assert_eq!(FireflySsbo::rg_buffer_hash(), 0x29d4a64d5ea39871);
        assert_eq!(FireflyVisibilitySsbo::rg_buffer_hash(), 0x9248897309ea4fea);
        assert_eq!(SheepAnimationSsbo::rg_buffer_hash(), 0xc70858c12c64d9ba);
        assert_eq!(<ButterflySsbo as DrawSsbo>::SLOT_NAME, "Butterfly Buffer");
        assert_eq!(
            <FireflyVisibilitySsbo as DrawSsbo>::SLOT_NAME,
            "Firefly Vis Buffer"
        );
    }

    #[test]
    fn butterfly_default_is_the_unrolled_constant_element() {
        let d = ButterflySsbo::default();
        assert_eq!(d.position, Vec3::ZERO);
        assert_eq!(d.anim_t, 0.0);
        assert_eq!(d.forward.z, -1.0);
        assert!(
            d.forward.x.is_sign_negative() && d.forward.x == 0.0,
            "-0.0（0x1429cd330 = 0x80000000 ×2）"
        );
        assert!(d.forward.y.is_sign_negative() && d.forward.y == 0.0);
        assert_eq!(d.pad, 0);
    }

    #[test]
    fn butterfly_from_components_rotates_negative_z() {
        // 恒等旋转 → 朝向 -Z。
        let e = ButterflySsbo::from_components(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, 0.25);
        assert_eq!(e.position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(e.anim_t, 0.25);
        assert!((e.forward - Vec3::NEG_Z).length() < 1e-6);
        // 绕 Y 轴转 90°：-Z → -X。
        let q = Quat::from_rotation_y(core::f32::consts::FRAC_PI_2);
        let e = ButterflySsbo::from_components(Vec3::ZERO, q, 0.0);
        assert!((e.forward - Vec3::NEG_X).length() < 1e-6, "{:?}", e.forward);
        assert_eq!(e.pad, 0);
    }

    #[test]
    fn firefly_from_translation_zeroes_the_pad() {
        let e = FireflySsbo::from_translation(Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(e.position, Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(e.pad, 0);
        assert_eq!(
            FireflySsbo::default(),
            FireflySsbo {
                position: Vec3::ZERO,
                pad: 0
            }
        );
    }

    #[test]
    fn mapped_critters_are_library_ssbos_with_the_original_buffer_sizes() {
        use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
        use crate::render::test_device::TestDevice;
        let dev = TestDevice(1);
        let lib = AssetSsboLibrary::new();
        lib.add_new::<ButterflySsbo>(&dev);
        lib.add_new::<FireflySsbo>(&dev);
        lib.add_new::<FireflyVisibilitySsbo>(&dev);
        // 缓冲大小 = BUFFER_LEN × 元素大小（0x14161b0f0 / 0x14161f1e0 / 0x14161fea0 的 create_buffer 大小）。
        assert_eq!(lib.get::<ButterflySsbo>().unwrap().buffer.byte_size, 0x300);
        assert_eq!(lib.get::<FireflySsbo>().unwrap().buffer.byte_size, 0x600);
        assert_eq!(
            lib.get::<FireflyVisibilitySsbo>().unwrap().buffer.byte_size,
            0x180
        );
        // 映射区按元素类型化：元素数 = 缓冲元素数。
        assert_eq!(lib.get::<FireflySsbo>().unwrap().mapped.len(), 96);
        // 无初始数据时蝴蝶写常量元素（forward = (-0,-0,-1)），萤火虫全零。
        assert_eq!(
            <ButterflySsbo as StorageElement>::default_value(),
            Some(ButterflySsbo::default())
        );
        assert_eq!(<FireflySsbo as StorageElement>::default_value(), None);
        assert_eq!(FireflyVisibilitySsbo::MAPPED_MEMORY, MAPPED_MEMORY_READBACK);
        assert_eq!(FireflySsbo::MAPPED_MEMORY, 2);
        assert_eq!(ButterflySsbo::MAPPED_MEMORY, 2);
    }
}
