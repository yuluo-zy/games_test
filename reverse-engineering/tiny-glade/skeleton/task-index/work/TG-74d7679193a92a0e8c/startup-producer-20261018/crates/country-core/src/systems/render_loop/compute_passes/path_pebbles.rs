//! `path_pebbles.rs` — 路面鹅卵石摆放（W17，`path_pebbles_compute` 0x140958f70，1496 B）。
//!
//! 机器码（O）：`ComputePass::new("Pebble Path", "/path_pebbles.cs.hlsl")`（名长 0xb、URI 长 0x15）。顺序：
//! `write_storage_buffer<PebblePlacementSsbo>(pebble_placement_buffer)` →
//! `write_storage_buffer_concurrent<DrawIndexedIndirectBuffer>(draw_commands, 1)` →
//! uniform i32：`draw_indirect_buffer_offset = 3`、`pebble_buffer_size = 0x240`、`scatter_grid_dim = 0x18` →
//! 纹理读 0x19：预设 0xe `rocky_terrain`、9 `terrain_heightmap`、10 `visual_terrain_heightmap`、6 `path_mask`、0x13 `water_mask` →
//! `dispatch_1d(0x240)`（闭包状态 `{1, 0x240, 1, 1}`，0x142ab4a30）。无 world 资源读取、无分支。
//!
//! `PebblePlacementSsbo`：`new_storage_buffer`（0x141610db0）里缓冲 0x6c00 B、用途 0x22、对齐 4，调试名
//! `"PebblePlacement"`（SsboId 立即数 0x392675f5c9079eea = xxh3_64("PebblePlacement") 已离线复算）；
//! 0x6c00 / 0x240 = 48 B/元素（U：字段布局）。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

pub const NAME: &str = "Pebble Path";
pub const URI: &str = "/path_pebbles.cs.hlsl";
pub const PEBBLE_PLACEMENT_BUFFER: u64 = binding_hash("pebble_placement_buffer");
pub const PEBBLE_BUFFER_SIZE: u64 = binding_hash("pebble_buffer_size");
pub const SCATTER_GRID_DIM: u64 = binding_hash("scatter_grid_dim");
pub const ROCKY_TERRAIN: u64 = binding_hash("rocky_terrain");
pub const PATH_MASK: u64 = binding_hash("path_mask");
pub const WATER_MASK: u64 = binding_hash("water_mask");

/// uniform `draw_indirect_buffer_offset`（O：`mov r9d, 3`）。
pub const DRAW_INDIRECT_OFFSET: i32 = 3;
/// uniform `pebble_buffer_size`（O：0x240）= 缓冲元素个数。
pub const PEBBLE_COUNT: i32 = 0x240;
/// uniform `scatter_grid_dim`（O：0x18）。
pub const SCATTER_DIM: i32 = 0x18;

/// 鹅卵石摆放元素（O：48 B；字段 U）。`PebblePlacement` = `T::NAME`，`BUFFER_LEN` = 0x240。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PebblePlacementSsbo(pub [u32; 12]);

impl StorageElement for PebblePlacementSsbo {
    const NAME: &'static str = "PebblePlacement";
    const BUFFER_LEN: usize = PEBBLE_COUNT as usize;
}

impl SsboType for PebblePlacementSsbo {
    type Storage = TypedStorageBuffer<PebblePlacementSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        // Original None branches directly to141610e39 with no allocation
        // or default upload. Keep the fixed-size byte constructor's None.
        TypedStorageBuffer::<PebblePlacementSsbo>::new_storage_buffer(device, initial)
    }
}

/// W17：路面鹅卵石 compute pass。
pub fn path_pebbles_compute(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.pass(NAME, URI)
        .write_storage_buffer::<PebblePlacementSsbo>(PEBBLE_PLACEMENT_BUFFER)
        .write_storage_buffer_concurrent::<DrawIndexedIndirectBuffer>(binding::DRAW_COMMANDS, 1)
        .uniform(binding::DRAW_INDIRECT_BUFFER_OFFSET, DRAW_INDIRECT_OFFSET)
        .uniform(PEBBLE_BUFFER_SIZE, PEBBLE_COUNT)
        .uniform(SCATTER_GRID_DIM, SCATTER_DIM)
        .bind_texture(ROCKY_TERRAIN, &TexturePreset(0xe), 0x19)
        .bind_texture(binding::TERRAIN_HEIGHTMAP, &TexturePreset(9), 0x19)
        .bind_texture(binding::VISUAL_TERRAIN_HEIGHTMAP, &TexturePreset(10), 0x19)
        .bind_texture(PATH_MASK, &TexturePreset(6), 0x19)
        .bind_texture(WATER_MASK, &TexturePreset(0x13), 0x19)
        .dispatch_1d(PEBBLE_COUNT as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::pass_resource_access::access_is_write;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, threads,
    };

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(PEBBLE_PLACEMENT_BUFFER, 0xe853_ba5b_3b69_5a54);
        assert_eq!(PEBBLE_BUFFER_SIZE, 0x94de_63d0_ee6b_5af4);
        assert_eq!(SCATTER_GRID_DIM, 0x1714_e971_d34a_7947);
        assert_eq!(ROCKY_TERRAIN, 0x4ee0_98b3_8612_d2b9);
        assert_eq!(PATH_MASK, 0xed50_2fff_6dc4_021f);
        assert_eq!(WATER_MASK, 0x6ad6_3ccd_18c8_f041);
        assert_eq!(binding::DRAW_COMMANDS, 0xf1c0_7a2b_8012_765a);
    }

    #[test]
    fn native_pebble_none_stays_uninitialized_and_some_preserves_input() {
        use crate::render::ssbo::tests::MockDevice;
        let device = MockDevice::default();
        let _ = <PebblePlacementSsbo as SsboType>::new_storage_buffer(&device, None);
        let provided = [0x11223344u32; 12];
        let bytes = bytemuck::cast_slice(&provided);
        let _ = <PebblePlacementSsbo as SsboType>::new_storage_buffer(&device, Some(bytes));
        let created = device.created.borrow();
        assert_eq!(created.len(), 2);
        for (desc, name, _) in created.iter() {
            assert_eq!(
                (desc.size, desc.usage, desc.memory, desc.alignment),
                (0x6c00, 0x22, 1, 4)
            );
            assert_eq!(name, "PebblePlacement");
        }
        assert!(created[0].2.is_none(), "native None does not upload zeroes");
        assert_eq!(created[1].2.as_deref(), Some(bytes));
    }

    #[test]
    fn element_is_48_bytes_and_buffer_is_0x6c00() {
        assert_eq!(core::mem::size_of::<PebblePlacementSsbo>(), 48);
        assert_eq!(48 * PebblePlacementSsbo::BUFFER_LEN, 0x6c00);
        assert_eq!(PebblePlacementSsbo::NAME, "PebblePlacement");
    }

    #[test]
    fn declares_buffers_uniforms_and_five_textures() {
        let names = [
            PEBBLE_PLACEMENT_BUFFER,
            binding::DRAW_COMMANDS,
            ROCKY_TERRAIN,
            binding::TERRAIN_HEIGHTMAP,
            binding::VISUAL_TERRAIN_HEIGHTMAP,
            PATH_MASK,
            WATER_MASK,
        ];
        let mut e = env_with(
            &names,
            &[
                binding::DRAW_INDIRECT_BUFFER_OFFSET,
                PEBBLE_BUFFER_SIZE,
                SCATTER_GRID_DIM,
            ],
            |lib, dev| {
                lib.add_new::<PebblePlacementSsbo>(dev);
                lib.add_new::<DrawIndexedIndirectBuffer>(dev);
            },
        );
        let pass = e.run(path_pebbles_compute);
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (0x240, 1, 1));
        assert_eq!(
            e.uniform_u32(&pass, binding::DRAW_INDIRECT_BUFFER_OFFSET),
            3
        );
        assert_eq!(e.uniform_u32(&pass, PEBBLE_BUFFER_SIZE), 0x240);
        assert_eq!(e.uniform_u32(&pass, SCATTER_GRID_DIM), 0x18);
        let a = accesses(&pass);
        assert_eq!(a.len(), 7);
        assert_eq!(a[0].1, 0x32, "write_storage_buffer");
        assert!(access_is_write(a[1].1), "concurrent write");
        assert_eq!(a[1].2, 1, "concurrent group argument");
        assert!(
            a[2..].iter().all(|x| x.1 == 0x19 && x.2 == 1),
            "five texture reads"
        );
    }
}
