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
//! 0x50787cf1536c7500 = xxh3_64("MouseTerrainRaycast")；32-byte mapped element, index@16, native miss=-1.

use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::pass_builder::pass_resources::TypeId128;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{MAPPED_MEMORY_UPLOAD, MappedStorageBuffer, StorageElement};
use crate::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use crate::resources::texture_presets::TexturePreset;
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};

pub const NAME: &str = "Read terrain stroke";
pub const URI: &str = "/read_terrain_stroke.cs.hlsl";
pub const OUT_BUFFER: u64 = binding_hash("out_buffer");
/// `CursorTerrainRaycast` 的 TypeId（O）。
pub const CURSOR_RAYCAST_TYPE_ID: TypeId128 = TypeId128 {
    lo: 0x062d_fe8f_62b6_fcc5,
    hi: 0xf650_08e5_4863_2ba7,
};
pub const STROKE_INDICES_PRESET: u8 = 0xb;

/// B 0x14161cd80: one 32-byte, 16-aligned mapped element. The producer
/// initializes the first16 bytes to zero and index@16 to-1; trailing bytes are
/// padding, explicitly represented here so typed uploads never read Rust padding.
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MouseTerrainRaycastSsbo {
    pub leading_words: [u32; 4],
    pub terrain_stroke_index: i32,
    pub reserved_words: [u32; 3],
}
unsafe impl bytemuck::Zeroable for MouseTerrainRaycastSsbo {}
unsafe impl bytemuck::Pod for MouseTerrainRaycastSsbo {}
impl Default for MouseTerrainRaycastSsbo {
    fn default() -> Self {
        Self {
            leading_words: [0; 4],
            terrain_stroke_index: -1,
            reserved_words: [0; 3],
        }
    }
}
impl StorageElement for MouseTerrainRaycastSsbo {
    const NAME: &'static str = "MouseTerrainRaycast";
    const BUFFER_LEN: usize = 1;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_UPLOAD;
    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}
impl SsboType for MouseTerrainRaycastSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        MappedStorageBuffer::new_storage_buffer(device, initial)
    }
}

/// The original optional lookup tolerates an unregistered mouse buffer, but a
/// registered empty mapped slice is an invariant failure. AssetSsboLibrary owns
/// type/name checks and the real mapped storage; no separate success flag exists.
#[cfg(test)]
fn read_terrain_stroke_index(library: &AssetSsboLibrary) -> Option<i32> {
    try_read_terrain_stroke_index(library).flatten()
}

/// Outer None means GPU work is still in flight; Some(None) means the optional
/// buffer is absent. Live consumers must preserve their previous result on busy.
pub fn try_read_terrain_stroke_index(library: &AssetSsboLibrary) -> Option<Option<i32>> {
    let Some(buffer) = library.get::<MouseTerrainRaycastSsbo>() else {
        return Some(None);
    };
    buffer.try_cpu_read(|samples| Some(samples.first()
        .expect("original mouse terrain mapped slice is empty")
        .terrain_stroke_index))
        .expect("mouse terrain readback failed")
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

#[cfg(test)]
mod mapped_readback_tests {
    use super::*;

    /// The generic TestDevice intentionally ignores initial upload bytes.
    /// This provider test needs a mapped page backed by the constructor's
    /// actual upload, so reuse the recording device's captured contents.
    #[derive(Default)]
    struct InitializedMappedDevice(crate::render::ssbo::tests::MockDevice);
    impl GraphicsDevice for InitializedMappedDevice {
        fn create_buffer(
            &self,
            desc: &crate::render::ssbo::rhapsody_boundary::BufferDesc,
            name: &str,
            initial: Option<&[u8]>,
        ) -> Result<
            crate::render::ssbo::rhapsody_boundary::Buffer,
            crate::render::ssbo::rhapsody_boundary::BackendError,
        > {
            self.0.create_buffer(desc, name, initial)
        }
        fn update_buffer_bytes(
            &self,
            buffer: &crate::render::ssbo::rhapsody_boundary::Buffer,
            data: &[u8],
            ranges: &[core::ops::Range<usize>],
            offset: usize,
        ) -> Result<(), crate::render::ssbo::rhapsody_boundary::BackendError> {
            self.0.update_buffer_bytes(buffer, data, ranges, offset)
        }
        fn mapped_slice_mut(
            &self,
            buffer: &crate::render::ssbo::rhapsody_boundary::Buffer,
        ) -> Option<&mut [u8]> {
            let size = buffer.byte_size as usize;
            let page = Box::leak(vec![0_u128; size.div_ceil(16)].into_boxed_slice());
            // The recording backend owns a suitably aligned, leaked test page.
            let bytes =
                unsafe { core::slice::from_raw_parts_mut(page.as_mut_ptr().cast::<u8>(), size) };
            bytes.copy_from_slice(&self.0.mem.borrow());
            Some(bytes)
        }
    }
    #[test]
    fn mapped_provider_preserves_native_default_and_reads_live_index() {
        assert_eq!(core::mem::size_of::<MouseTerrainRaycastSsbo>(), 32);
        assert_eq!(core::mem::align_of::<MouseTerrainRaycastSsbo>(), 16);
        assert_eq!(
            core::mem::offset_of!(MouseTerrainRaycastSsbo, terrain_stroke_index),
            16
        );
        let mut library = AssetSsboLibrary::new();
        assert_eq!(read_terrain_stroke_index(&library), None);
        library.add_new::<MouseTerrainRaycastSsbo>(&InitializedMappedDevice::default());
        assert_eq!(read_terrain_stroke_index(&library), Some(-1));
        library.get_mut::<MouseTerrainRaycastSsbo>().unwrap().mapped[0].terrain_stroke_index = 17;
        assert_eq!(read_terrain_stroke_index(&library), Some(17));
    }
}
