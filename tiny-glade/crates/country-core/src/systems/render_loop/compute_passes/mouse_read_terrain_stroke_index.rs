//! `mouse_read_terrain_stroke_index.rs` — 鼠标所指地形笔画下标回读（W6，0x1409768b0，1000 B）。
//!
//! 机器码（O）：
//! 1. 资源 `CursorTerrainRaycast`（TypeId `0x062dfe8f62b6fcc5 / 0xf65008e548632ba7`，缺失 → `Option::unwrap`）。
//!    `if (*tag & 1)`：`x = *(u32*)(r+4)`、`z = *(u32*)(r+0xc)`（即位置的 x / z 位模式），否则两者为 0。
//! 2. `ComputePass::new("Read terrain stroke", "/read_terrain_stroke.cs.hlsl")`（名长 0x13、URI 长 0x1c）。
//! 3. `write_to_cpu_mapped_storage_buffer<MouseTerrainRaycastSsbo>(out_buffer)` →
//!    `stroke_indices`（预设 0xb，读 0x19）→ push constants：8 字节 `[x, z]`（`malloc(8, align 1)`）→ `dispatch_1d(1)`
//!    （闭包状态 `{1,1,1,1}`，0x142958fe0）。
//!
//! `MouseTerrainRaycastSsbo`：`new_storage_buffer`（0x14161cd80）是 `MappedStorageBuffer`；`SsboId` 立即数
//! 0x50787cf1536c7500 = xxh3_64("MouseTerrainRaycast")（已复算）；元素布局 U。存储用 `TypedStorageBuffer`（理由同 bush_cpu）。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};
use crate::typed_gap::Gap;

pub const NAME: &str = "Read terrain stroke";
pub const URI: &str = "/read_terrain_stroke.cs.hlsl";
pub const OUT_BUFFER: u64 = binding_hash("out_buffer");
/// `CursorTerrainRaycast` 的 TypeId（O）。
pub const CURSOR_RAYCAST_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x062d_fe8f_62b6_fcc5,
    hi: 0xf650_08e5_4863_2ba7,
};
pub const STROKE_INDICES_PRESET: u8 = 0xb;

/// 鼠标地形射线回读缓冲（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct MouseTerrainRaycastSsbo(pub Gap);

impl StorageElement for MouseTerrainRaycastSsbo {
    const NAME: &'static str = "MouseTerrainRaycast";
    /// I：原版是固定 0x40000 B 的映射缓冲（`BUFFER_LEN` 概念不适用）；这里用 1 个元素让存储可建（偏差，U）。
    const BUFFER_LEN: usize = 1;
    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}

impl SsboType for MouseTerrainRaycastSsbo {
    type Storage = TypedStorageBuffer<MouseTerrainRaycastSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        match initial {
            None => TypedStorageBuffer::<MouseTerrainRaycastSsbo>::new(device, None),
            Some(b) => {
                TypedStorageBuffer::<MouseTerrainRaycastSsbo>::new_storage_buffer(device, Some(b))
            }
        }
    }
}

/// push constants：`[x, z]` 的 f32 位模式（O：tag 位 0 为 0 → 两个 0）。
pub fn push_constants_for(raycast: &CursorTerrainRaycast) -> [u8; 8] {
    let (x, z) = if raycast.tag & 1 != 0 {
        (raycast.pos.x, raycast.pos.z)
    } else {
        (0.0, 0.0)
    };
    let mut out = [0u8; 8];
    out[..4].copy_from_slice(&x.to_bits().to_le_bytes());
    out[4..].copy_from_slice(&z.to_bits().to_le_bytes());
    out
}

/// W6：把光标射线命中的地形笔画下标读回 CPU。
pub fn mouse_read_terrain_stroke_index(
    env: &mut PassEnv<'_>,
    raycast: &CursorTerrainRaycast,
) -> RecordedComputePass {
    let constants = push_constants_for(raycast);
    env.pass(NAME, URI)
        .write_to_cpu_mapped_storage_buffer::<MouseTerrainRaycastSsbo>(OUT_BUFFER)
        .bind_texture(
            binding::STROKE_INDICES,
            &TexturePreset(STROKE_INDICES_PRESET),
            0x19,
        )
        .push_constants(&constants)
        .dispatch_1d(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, threads,
    };
    use glam::Vec3;

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(OUT_BUFFER, 0xa868_b696_44bd_36ba);
        assert_eq!(binding::STROKE_INDICES, 0xe365_9c0f_bde7_bf03);
    }

    #[test]
    fn push_constants_follow_tag_bit0() {
        let hit = CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(1.5, 9.0, -2.25),
            ..Default::default()
        };
        let b = push_constants_for(&hit);
        assert_eq!(f32::from_le_bytes(b[..4].try_into().unwrap()), 1.5);
        assert_eq!(f32::from_le_bytes(b[4..].try_into().unwrap()), -2.25);
        // tag 偶数（位 0 = 0）→ 全 0，即使 pos 非零
        let miss = CursorTerrainRaycast {
            tag: 2,
            pos: Vec3::new(5.0, 5.0, 5.0),
            ..Default::default()
        };
        assert_eq!(push_constants_for(&miss), [0u8; 8]);
        assert_eq!(push_constants_for(&CursorTerrainRaycast::new()), [0u8; 8]);
    }

    #[test]
    fn declares_one_thread_dispatch_with_push_constants() {
        let mut e = env_with(&[OUT_BUFFER, binding::STROKE_INDICES], &[], |lib, dev| {
            lib.add_new::<MouseTerrainRaycastSsbo>(dev);
        });
        let ray = CursorTerrainRaycast {
            tag: 1,
            pos: Vec3::new(3.0, 0.0, 4.0),
            ..Default::default()
        };
        let pass = e.run(|env| mouse_read_terrain_stroke_index(env, &ray));
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (1, 1, 1));
        assert_eq!(pass.push_constants, push_constants_for(&ray));
        let codes: Vec<u8> = accesses(&pass).iter().map(|a| a.1).collect();
        assert_eq!(codes, [0x32, 0x19]);
    }
}
