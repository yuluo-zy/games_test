//! water_mask.rs — `country_core::resources::water_mask` 水 mask 资源（WP7 恢复）。
//!
//! 源（B skeleton）：`crates/country-core/src/resources/water_mask.rs`（metadata-only，
//! 5 记录，0 procedure）。实际映射构造器 B 14161d570 明确 u32x8192/0x8000B。
//! 语义（I，`reconstruction/shader-source/shaders/water/read_water_mask.cs.hlsl` O）：
//! 512×512 水 mask 纹理 `.y` 通道打包回读为水/排除位（`>0.5` water bit / `>0.2` clutter bit）。
//! 生命周期（O，泛型胶水）：`AssetSsboLibrary::add_new/get_rg_buffer`、
//! `TypedStorageBuffer::new_storage_buffer`、`ComputePass::write_storage_buffer`、
//! `pass_resource_access::write_access_buffer<WaterMaskSsbo>`（0x140974a20）。

#![allow(dead_code, unused_imports)]

use crate::render::pass_builder::pass_resource_access::SsboType as PassSsboType;

/// B constructor 14161d570: one packed u32 word per 32 mask cells.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct WaterMaskSsbo(pub u32);

unsafe impl bytemuck::Zeroable for WaterMaskSsbo {}
unsafe impl bytemuck::Pod for WaterMaskSsbo {}

impl PassSsboType for WaterMaskSsbo {
    /// Same canonical short-name key as the actual SSBO library registration.
    fn rg_buffer_hash() -> u64 {
        crate::name_hash::NameHash::compute(<Self as crate::render::ssbo::StorageElement>::NAME).0
    }
}

/// `ReadbackSsbos`（卡7）要求的库内 `SsboType` 实现：`write_storage_buffer` 用
/// `MappedStorageBuffer`（0x2000 线程回读）。
impl crate::asset_libraries::ssbo_library::SsboType for WaterMaskSsbo {
    type Storage = crate::render::ssbo::MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        crate::render::ssbo::MappedStorageBuffer::new_storage_buffer(device, initial)
    }
}
impl crate::render::ssbo::StorageElement for WaterMaskSsbo {
    const NAME: &'static str = "WaterMask";
    const BUFFER_LEN: usize = 0x2000;
    const MAPPED_MEMORY: u8 = crate::render::ssbo::MAPPED_MEMORY_READBACK;
}

/// 水 mask 栅格尺寸（O：512×512）。
pub const WATER_MASK_GRID: usize = 512;
/// 水位阈值（O：`read_water_mask.cs.hlsl` `water_mask[coord].y > 0.5f`）。
pub const WATER_BIT_THRESHOLD: f32 = 0.5;
/// 排除 clutter 阈值（O：`water_mask[coord].y > 0.2f`）。
pub const CLUTTER_BIT_THRESHOLD: f32 = 0.2;
/// 世界→mask 采样缩放（O：`water_mask_sampler.inc.hlsl`
/// `world_xz * 0.0076923077 + 0.5`，即 1/130 与地形高度图同映射）。
pub const WATER_MASK_SAMPLE_SCALE: f32 = 1.0 / 130.0;
/// 水 mask 世界半宽（O：`draw_water_mask.cs.hlsl` `(uv*2-1)*65.0`）。
pub const WATER_WORLD_HALF_EXTENT: f32 = 65.0;
/// 水笔刷覆盖半径（O：`draw_water_mask.cs.hlsl` `2.0 - clamp(dist, 0, 2)`）。
pub const WATER_BRUSH_RADIUS: f32 = 2.0;

#[cfg(test)]
mod native_mask_storage_tests {
    use super::*;
    use crate::{
        asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId, SsboType},
        render::{
            pass_builder::pass_resource_access::SsboType as PassSsboType,
            ssbo::{MappedStorageBuffer, StorageElement, tests::MockDevice},
        },
        resources::{
            garden_mask::GardenMaskSsbo, water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo,
        },
    };

    fn check_storage<T>(name: &str, original_key: u64)
    where
        T: SsboType<Storage = MappedStorageBuffer<'static, T>> + bytemuck::Pod + PassSsboType,
    {
        assert_eq!(
            (core::mem::size_of::<T>(), core::mem::align_of::<T>()),
            (4, 4)
        );
        let device = MockDevice::default();
        device.map.borrow_mut().resize(0x8000, 0);
        let mut library = AssetSsboLibrary::new();
        library.add_new::<T>(&device);
        let storage = library.get::<T>().unwrap();
        assert_eq!(storage.mapped.len(), 0x2000);
        assert_eq!(
            storage.mapped.as_ptr() as usize,
            device.map.borrow().as_ptr() as usize
        );
        assert_eq!(storage.buffer.byte_size, 0x8000);
        {
            let calls = device.created.borrow();
            let (desc, actual_name, initial) = &calls[0];
            assert_eq!(calls.len(), 1);
            assert_eq!(actual_name, name);
            assert_eq!(
                (desc.size, desc.usage, desc.memory, desc.alignment),
                (0x8000, 0x22, 4, 4)
            );
            let initial = initial.as_ref().unwrap();
            assert_eq!(initial.len(), 0x8000);
            assert!(initial.iter().all(|byte| *byte == 0));
        }
        let words: &mut [u32] =
            bytemuck::cast_slice_mut(&mut library.get_mut::<T>().unwrap().mapped[..]);
        words[0x1fff] = 0x8000_0001;
        let words: &[u32] = bytemuck::cast_slice(&library.get::<T>().unwrap().mapped[..]);
        assert_eq!((words[0], words[0x1fff]), (0, 0x8000_0001));
        assert_eq!(<T as PassSsboType>::rg_buffer_hash(), original_key);
        assert_eq!(SsboId::of::<T>().0.0, original_key);

        let device = MockDevice::default();
        let library = AssetSsboLibrary::new();
        let words = [0x1122_3344u32, 0xaabb_ccdd];
        library.add_new_with_data::<T>(&device, bytemuck::cast_slice(&words));
        let calls = device.created.borrow();
        assert_eq!(
            calls[0].2.as_ref().unwrap().as_slice(),
            bytemuck::cast_slice::<u32, u8>(&words)
        );
        assert_eq!(library.get::<T>().unwrap().mapped.len(), 0x2000);
    }

    #[test]
    fn original_three_mapped_mask_library_constructors() {
        check_storage::<WaterMaskSsbo>("WaterMask", 0x4652_4552_97da_b420);
        check_storage::<WaterExcludeClutterMaskSsbo>(
            "WaterExcludeClutterMask",
            0x8dbc_70d3_68e4_3c84,
        );
        check_storage::<GardenMaskSsbo>("GardenMask", 0x6fd8_113d_2359_5a99);
    }

    #[test]
    fn actual_readback_passes_use_the_same_registered_mask_types() {
        use crate::systems::render_loop::compute_passes::{
            pass_kit::test_env::{env, threads},
            read_terrain_data::{self as read, TerrainReadbackSsbos},
        };
        let mut environment = env(
            &[
                read::H_WATER_MASK,
                read::H_MASK_OUT,
                read::H_MASK_OUT_CLUTTER,
                read::H_GARDEN_MASK,
            ],
            &[],
        );
        let device = MockDevice::default();
        environment.world.ssbos.add_new::<WaterMaskSsbo>(&device);
        environment
            .world
            .ssbos
            .add_new::<WaterExcludeClutterMaskSsbo>(&device);
        environment.world.ssbos.add_new::<GardenMaskSsbo>(&device);
        // The shared recording mock returns one handle for every allocation;
        // assign distinct observed handles here so graph bindings stay distinct.
        environment
            .world
            .ssbos
            .get_mut::<WaterMaskSsbo>()
            .unwrap()
            .buffer
            .id = 101;
        environment
            .world
            .ssbos
            .get_mut::<WaterExcludeClutterMaskSsbo>()
            .unwrap()
            .buffer
            .id = 102;
        environment
            .world
            .ssbos
            .get_mut::<GardenMaskSsbo>()
            .unwrap()
            .buffer
            .id = 103;
        let mut water = environment.run(|env| read::read_water_mask::<TerrainReadbackSsbos>(env));
        let mut garden = environment.run(|env| read::read_garden_mask::<TerrainReadbackSsbos>(env));
        assert_eq!(threads(&water), (0x2000, 1, 1));
        assert_eq!(threads(&garden), (0x2000, 1, 1));
        let a = environment
            .world
            .ssbos
            .get_rg_buffer::<WaterMaskSsbo>(&mut water.builder.render_graph)
            .resource as u32;
        let b = environment
            .world
            .ssbos
            .get_rg_buffer::<WaterExcludeClutterMaskSsbo>(&mut water.builder.render_graph)
            .resource as u32;
        let c = environment
            .world
            .ssbos
            .get_rg_buffer::<GardenMaskSsbo>(&mut garden.builder.render_graph)
            .resource as u32;
        assert_ne!(a, b);
        for (pass, resource) in [(&water, a), (&water, b), (&garden, c)] {
            assert!(
                pass.builder
                    .accesses
                    .iter()
                    .any(|v| v.resource == resource && v.access == 0x32)
            );
        }
    }
}
