//! `leaf_particles.rs` — W18 leaf particles（O：`leaf_particles_compute` 0x1408f4d00，1154 B）。
//!
//! 流程（逐条对照机器码）：
//! 1. `get_resource_id(0x3dce36237fb50dff, 0x93a27cf4512d885c)` → [`LoadedLeafParticleCount`]（缺失 → `Option::unwrap` panic）；
//!    读其首个 `usize` 为已加载叶片粒子总数；**为 0 时整函数直接返回**（不建 pass）。
//! 2. `n = min(count, 0x340)`（`cmp rax, 0x340; cmovb`），即同时最多 832 个粒子；`n` 同时是 `dispatch_1d` 的线程数。
//! 3. `ComputePass::new("LeafParticles", "/leaf_particles.cs.hlsl")`，依次：
//!    `write_storage_buffer<LeafParticleSsbo>`（`leaf_particle_buffer`）→
//!    `read_storage_buffer<ParticleEmitterSsbo>`（`particle_emitter_buffer`）→ `bind_water_object_push` →
//!    `write_uniform<i32>`（`total_particle_count` = `n`）。
//! 4. `get_resource_id(0x4259a350202a056c, 0x1e9e3f559133d465)`（[`LoadedParticleEmitterCount`]，缺失 → unwrap panic，
//!    **发生在上一步之后**）→ 首个 qword 低 32 位写 `write_uniform<i32>`（`particle_emitter_count`）。
//! 5. `dispatch_1d(n)`（模式 1：x=n, y=z=1）。
//!
//! 绑定名（O：FNV-1a-64 与 `movabs r8` 立即数一致，HLSL `leaf_particles.cs.hlsl` 里的名字）。
//! SSBO 名：`LeafParticleSsbo` → "LeafParticles"、`ParticleEmitterSsbo` → "ParticleEmitter"（O：SsboId 立即数
//! 0xda2386d830255aac / 0x059e5b18ab7bc931 与 xxh3_64 比对一致）。

use crate::render::pass_builder::pass_resources::TypeId128;
use crate::systems::render_loop::compute_passes::pass_kit::{
    PassEnv, ResourceView, RippleSourceBuf, binding_hash,
};
use crate::systems::render_loop::draw_passes::draw_shaders::common_includes_compute::bind_water_object_push_compute;

/// B ctor@141614110:32-byte record. The raw words retain exact default
/// bytes; CPU names for these shader-owned fields are not inferred.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LeafParticleSsbo {
    pub words: [u32; 8],
}
impl Default for LeafParticleSsbo {
    fn default() -> Self {
        // @1416141b0..1dc, constant bytes @142aa7908: (0,1,0).
        Self {
            words: [0, 0, 0, 0, 0x3f800000, 0, 0x3f800000, 0],
        }
    }
}
impl crate::render::ssbo::StorageElement for LeafParticleSsbo {
    const NAME: &'static str = "LeafParticles";
    const BUFFER_LEN: usize = 0x340;
}
impl crate::asset_libraries::ssbo_library::SsboType for LeafParticleSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::rhapsody_boundary::BufferDesc;
        let defaults;
        let data = match initial {
            Some(bytes) => {
                assert_eq!(bytes.len() % 32, 0);
                assert_eq!(bytes.as_ptr() as usize % 4, 0);
                assert!(
                    bytes.len() < 0x6820,
                    "Excessive data size supplied for LeafParticleSsbo"
                );
                bytes
            }
            None => {
                defaults = vec![Self::default(); 0x340];
                bytemuck::cast_slice(&defaults)
            }
        };
        let buffer = device
            .create_buffer(
                &BufferDesc::new(0x6800, 0x22, 1).alignment(4),
                "LeafParticles",
                Some(data),
            )
            .expect("creating original LeafParticles buffer");
        crate::render::ssbo::TypedStorageBuffer::from_created_buffer(buffer)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for LeafParticleSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

/// B ctor@141610600: 16-byte record,1100 slots, no None upload.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleEmitterSsbo {
    pub words: [u32; 4],
}
impl crate::render::ssbo::StorageElement for ParticleEmitterSsbo {
    const NAME: &'static str = "ParticleEmitter";
    const BUFFER_LEN: usize = 1100;
}
impl crate::asset_libraries::ssbo_library::SsboType for ParticleEmitterSsbo {
    type Storage = crate::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(
        device: &dyn crate::render::ssbo::rhapsody_boundary::GraphicsDevice,
        initial: Option<&[u8]>,
    ) -> Self::Storage {
        use crate::render::ssbo::rhapsody_boundary::BufferDesc;
        if let Some(bytes) = initial {
            assert_eq!(bytes.len() % 16, 0);
            assert_eq!(bytes.as_ptr() as usize % 4, 0);
            assert!(
                bytes.len() < 0x44d0,
                "Excessive data size supplied for ParticleEmitterSsbo"
            );
        }
        let buffer = device
            .create_buffer(
                &BufferDesc::new(0x44c0, 0x22, 1).alignment(4),
                "ParticleEmitter",
                initial,
            )
            .expect("creating original ParticleEmitter buffer");
        crate::render::ssbo::TypedStorageBuffer::from_created_buffer(buffer)
    }
}
impl crate::render::pass_builder::pass_resource_access::SsboType for ParticleEmitterSsbo {
    fn rg_buffer_hash() -> u64 {
        crate::asset_libraries::ssbo_library::SsboId::of::<Self>()
            .0
            .0
    }
}

/// 已加载叶片粒子总数（O：首个 `usize`，其余布局 U；语义由 `total_particle_count` 的写入点确定）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadedLeafParticleCount(pub usize);

// SAFETY: 首个 qword = 数量（O：`mov rax,[res]`）。
unsafe impl ResourceView for LoadedLeafParticleCount {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0x3dce36237fb50dff,
        hi: 0x93a27cf4512d885c,
    };
}

/// 已加载粒子发射器数（O 名 `LoadedParticleEmitterCount`；首个 qword 的低 32 位写 `particle_emitter_count`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LoadedParticleEmitterCount(pub u64);

// SAFETY: 首个 qword 被读取（O）。
unsafe impl ResourceView for LoadedParticleEmitterCount {
    const TYPE_ID: TypeId128 = TypeId128 {
        lo: 0x4259a350202a056c,
        hi: 0x1e9e3f559133d465,
    };
}

/// pass 名（O：`ComputePass::new` 第一个 `&str`，13 B）。
pub const PASS_NAME: &str = "LeafParticles";
/// shader URI（O，23 B）。
pub const SHADER_URI: &str = "/leaf_particles.cs.hlsl";
/// 同时最多处理的发射器数（O：`cmp rax, 0x340`）。
pub const MAX_EMITTERS: usize = 0x340;

pub const HASH_LEAF_PARTICLES: u64 = binding_hash("leaf_particle_buffer"); // 0x0316dd88f93e7113
pub const HASH_EMITTERS: u64 = binding_hash("particle_emitter_buffer"); // 0x1ad3d8a882713639
/// 粒子总数 uniform 键（O：`movabs r8, -0x51985049a1ae734c`）。
pub const KEY_TOTAL_PARTICLES: u64 = binding_hash("total_particle_count");
/// 发射器数 uniform 键（O：`movabs r8, 0x6ae860ef74719a42`）。
pub const KEY_EMITTER_COUNT: u64 = binding_hash("particle_emitter_count");

/// `min(count, 0x340)`（O：`cmp rax, 0x340; mov esi, 0x340; cmovb rsi, rax`）。
pub fn clamp_emitter_count(count: usize) -> u32 {
    count.min(MAX_EMITTERS) as u32
}

/// W18 入口（O 0x1408f4d00）。粒子数为 0 → 不登记 pass；否则 `PassBuilder::render` 登记一个 pass。
pub fn leaf_particles_compute(env: &mut PassEnv<'_>) {
    let count = env.resource::<LoadedLeafParticleCount>().0;
    if count == 0 {
        return;
    }
    let n = clamp_emitter_count(count);
    let pass = env
        .pass(PASS_NAME, SHADER_URI)
        .write_storage_buffer::<LeafParticleSsbo>(HASH_LEAF_PARTICLES)
        .read_storage_buffer::<ParticleEmitterSsbo>(HASH_EMITTERS);
    let pass = bind_water_object_push_compute::<RippleSourceBuf>(pass)
        .uniform(KEY_TOTAL_PARTICLES, n as i32);
    // 第二个资源在第一个 uniform 之后取（缺失 → unwrap panic）。
    let emitters = env.resource::<LoadedParticleEmitterCount>().0;
    let recorded = pass
        .uniform(KEY_EMITTER_COUNT, emitters as u32 as i32)
        .dispatch_1d(n);
    env.record(recorded);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::pass_builder::compute_pass_builder::DispatchMode;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::*;

    fn world(count: usize, word: Option<u64>) -> TestWorld {
        let refl = Reflection::new()
            .bind(&[
                HASH_LEAF_PARTICLES,
                HASH_EMITTERS,
                binding_hash("ripple_source_buffer"),
                binding_hash("grass_object_push"),
                binding_hash("water_noise_texture"),
            ])
            .uniforms(&[KEY_TOTAL_PARTICLES, KEY_EMITTER_COUNT]);
        let w = TestWorld::new(refl)
            .with_ssbo::<LeafParticleSsbo>()
            .with_ssbo::<ParticleEmitterSsbo>()
            .with_ssbo::<RippleSourceBuf>()
            .with_resource(LoadedLeafParticleCount(count));
        match word {
            Some(v) => w.with_resource(LoadedParticleEmitterCount(v)),
            None => w,
        }
    }

    #[test]
    fn zero_emitters_registers_no_pass() {
        let w = world(0, None); // 第二个资源缺失也不 panic：count==0 在它之前返回
        let mut f = TestFrame::default();
        with_env(&w, &mut f, leaf_particles_compute);
        assert!(f.passes.is_empty());
        assert_eq!(
            w.backend.loads.get(),
            0,
            "no shader compiled when returning early"
        );
    }

    #[test]
    fn clamps_to_0x340_and_dispatches_1d() {
        assert_eq!(clamp_emitter_count(5), 5);
        assert_eq!(clamp_emitter_count(0x340), 0x340);
        assert_eq!(clamp_emitter_count(100_000), 0x340);
        let w = world(100_000, Some(0x1_0000_002a));
        let mut f = TestFrame::default();
        with_env(&w, &mut f, leaf_particles_compute);
        assert_eq!(f.passes.len(), 1);
        let p = &f.passes[0];
        assert_eq!(p.name, "LeafParticles");
        assert!(matches!(
            p.dispatch,
            DispatchMode::Threads {
                x: 0x340,
                y: 1,
                z: 1
            }
        ));
        assert_eq!(
            w.backend.paths.borrow()[0],
            "/leaf_particles.cs.hlsl:cs_main"
        );
        // uniform：粒子总数 @0；发射器数 = 第二个资源首 qword 的低 32 位 @16（高 32 位被截断）。
        assert_eq!(uniform_u32(p, 0), 0x340);
        assert_eq!(uniform_u32(p, 16), 0x2a);
    }

    #[test]
    fn small_count_passes_through_and_declares_buffers_in_order() {
        let w = world(7, Some(1));
        let mut f = TestFrame::default();
        with_env(&w, &mut f, leaf_particles_compute);
        let p = &f.passes[0];
        assert!(matches!(p.dispatch, DispatchMode::Threads { x: 7, .. }));
        // 访问登记顺序：color library(0x1a)、LeafParticles 写(0x32)、ParticleEmitter 读(0x1a)、
        // water_object_push：preset 0xf 读、Ripple 读、preset 0x18 读。
        let acc: Vec<u8> = p.builder.accesses.iter().map(|a| a.access).collect();
        assert_eq!(&acc[..3], &[0x1a, 0x32, 0x1a]);
        assert_eq!(&acc[3..], &[0x19, 0x1a, 0x19]);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_count_resource_unwraps() {
        let refl = Reflection::new();
        let w = TestWorld::new(refl);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, leaf_particles_compute);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn missing_second_resource_unwraps_after_first_uniform() {
        let w = world(3, None);
        let mut f = TestFrame::default();
        with_env(&w, &mut f, leaf_particles_compute);
    }

    #[test]
    fn constants_match_machine_code() {
        assert_eq!(KEY_TOTAL_PARTICLES, (-0x51985049a1ae734c_i64) as u64);
        assert_eq!(KEY_EMITTER_COUNT, 0x6ae860ef74719a42);
        assert_eq!(HASH_LEAF_PARTICLES, 0x0316dd88f93e7113);
        assert_eq!(HASH_EMITTERS, 0x1ad3d8a882713639);
        assert_eq!(
            <LoadedLeafParticleCount as ResourceView>::TYPE_ID.hi,
            (-0x6c5d830baed277a4_i64) as u64
        );
        assert_eq!(
            <LoadedLeafParticleCount as ResourceView>::TYPE_ID.lo,
            0x3dce36237fb50dff
        );
        assert_eq!(MAX_EMITTERS, 832);
    }
}

#[cfg(test)]
mod original_storage_tests {
    use super::*;
    use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboType};
    use crate::render::ssbo::rhapsody_boundary::{
        BackendError, Buffer, BufferDesc, GraphicsDevice,
    };
    use std::sync::Mutex;

    #[derive(Default)]
    struct Capture(Mutex<Vec<(String, BufferDesc, Option<Vec<u8>>)>>);
    impl GraphicsDevice for Capture {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            initial: Option<&[u8]>,
        ) -> Result<Buffer, BackendError> {
            let mut calls = self.0.lock().unwrap();
            calls.push((name.to_owned(), *desc, initial.map(<[u8]>::to_vec)));
            Ok(Buffer {
                id: calls.len() as u64,
                byte_size: desc.size as u64,
                ..Default::default()
            })
        }
    }
    #[test]
    fn original_particle_and_grass_startup_keep_actual_capacity_and_distinct_contents() {
        use crate::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo;
        let device = Capture::default();
        let mut library = AssetSsboLibrary::new();
        library.add_new::<GrassBladeDataSsbo>(&device);
        library.add_new::<LeafParticleSsbo>(&device);
        library.add_new::<ParticleEmitterSsbo>(&device);
        assert_eq!(
            library
                .get_buffer::<GrassBladeDataSsbo>()
                .unwrap()
                .byte_size,
            0x1200000
        );
        assert_eq!(
            library.get_buffer::<LeafParticleSsbo>().unwrap().byte_size,
            0x6800
        );
        assert_eq!(
            library
                .get_buffer::<ParticleEmitterSsbo>()
                .unwrap()
                .byte_size,
            0x44c0
        );
        let calls = device.0.lock().unwrap();
        for (_, desc, _) in calls.iter() {
            assert_eq!((desc.usage, desc.memory, desc.alignment), (0x22, 1, 4));
        }
        assert!(calls[0].2.is_none());
        assert!(calls[2].2.is_none());
        let bytes = calls[1].2.as_ref().unwrap();
        assert_eq!(bytes.len(), 0x6800);
        let original_record: [u8; 32] = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 63, 0, 0, 0, 0, 0, 0, 128,
            63, 0, 0, 0, 0,
        ];
        assert!(
            bytes
                .chunks_exact(32)
                .all(|record| record == original_record)
        );
        assert_eq!(std::mem::size_of::<GrassBladeDataSsbo>(), 32);
        assert_eq!(std::mem::size_of::<LeafParticleSsbo>(), 32);
        assert_eq!(std::mem::size_of::<ParticleEmitterSsbo>(), 16);
    }
    #[test]
    fn original_particle_constructor_honours_supplied_data_instead_of_default_payload() {
        let device = Capture::default();
        let record = LeafParticleSsbo { words: [7; 8] };
        <LeafParticleSsbo as SsboType>::new_storage_buffer(
            &device,
            Some(bytemuck::bytes_of(&record)),
        );
        let calls = device.0.lock().unwrap();
        assert_eq!(calls[0].2.as_ref().unwrap(), bytemuck::bytes_of(&record));
    }
}
