//! `read_bush_positions_for_cpu.rs` — 灌木位置回读 CPU（W15，0x140866540，696 B）。
//!
//! 机器码（O）：`ComputePass::new("Read bush position for cpu", "/garden/read_bush_positions_for_cpu.cs.hlsl")`
//! （名长 0x1a、URI 长 0x2b）。顺序：`write_to_cpu_mapped_storage_buffer<BushCpuSsbo>(bush_cpu)` →
//! `read_storage_buffer<BushSsbo>(bush_buffer)` → `read_storage_buffer<DrawIndexedIndirectBuffer>(draw_commands)` →
//! uniform i32 `draw_indirect_buffer_offset = 10` → `dispatch_1d(0x2581)`（闭包状态 `{1, 0x2581, 1, 1}`，0x142a95de0）。
//! 无 world 资源读取、无分支。
//!
//! `BushCpuSsbo` 原构造 B 0x14161bd30：8 字节行 / 4 字节对齐，9601 行，
//! 第一行承载 count，后续行为两个 f32 的原始 u32 位。名称是 "Bush positions for CPU"。
//! None 不上传初始数据；按类型的构造策略复用现 device / MappedStorageBuffer，
//! 不改变其他 mapped 类型的初始化策略。原字节和读者合同见本任务 core-bush-cpu-20261018。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice, usage};
use crate::render::ssbo::{MAPPED_MEMORY_READBACK, MappedStorageBuffer, StorageElement};
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;

pub const NAME: &str = "Read bush position for cpu";
pub const URI: &str = "/garden/read_bush_positions_for_cpu.cs.hlsl";
pub const BUSH_CPU: u64 = binding_hash("bush_cpu");
pub const BUSH_BUFFER: u64 = binding_hash("bush_buffer");
/// uniform `draw_indirect_buffer_offset`（O：`mov r9d, 10`）。
pub const DRAW_INDIRECT_OFFSET: i32 = 10;
/// `dispatch_1d` 线程总数（O：0x2581 = 9601）。
pub const DISPATCH_THREADS: u32 = 0x2581;
/// 原 `get_rg_buffer<BushCpuSsbo>` 的键，与实际 NAME 的 xxh3 一致。
pub const BUSH_CPU_SSBO_ID: u64 = 0x0def_f5ab_6a35_972a;

/// 原 `[u32; 2]` 行。第 0 行是头，其余行按原始 f32 bits 解释为 XZ。
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct BushCpuSsbo(pub [u32; 2]);

impl StorageElement for BushCpuSsbo {
    const NAME: &'static str = "Bush positions for CPU";
    const BUFFER_LEN: usize = 9601;
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
}

impl SsboType for BushCpuSsbo {
    type Storage = MappedStorageBuffer<'static, Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        const ELEMENT_SIZE: usize = core::mem::size_of::<BushCpuSsbo>();
        const ALIGNMENT: usize = core::mem::align_of::<BushCpuSsbo>();
        const SIZE: usize = <BushCpuSsbo as StorageElement>::BUFFER_LEN * ELEMENT_SIZE;
        if let Some(data) = initial {
            // Original validation order: byte length, pointer alignment, capacity.
            assert_eq!(data.len() % ELEMENT_SIZE, 0);
            assert_eq!(data.as_ptr() as usize % ALIGNMENT, 0);
            if data.len() >= SIZE + ELEMENT_SIZE {
                panic!(
                    "Excessive data size supplied for buffer {}",
                    core::any::type_name::<Self>()
                );
            }
        }
        let desc = BufferDesc::new(
            SIZE,
            usage::STORAGE_BUFFER | usage::TRANSFER_DST,
            Self::MAPPED_MEMORY,
        )
        .alignment(ALIGNMENT);
        let buffer = device
            .create_buffer(&desc, Self::NAME, initial)
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        let mapped = device
            .mapped_slice_mut(&buffer)
            .expect("called `Option::unwrap()` on a `None` value");
        assert!(
            mapped.len() as u64 >= buffer.byte_size,
            "assertion failed: data.len() >= buffer.buffer.desc.size"
        );
        assert_eq!(mapped.len() % ELEMENT_SIZE, 0);
        assert_eq!(mapped.as_ptr() as usize % ALIGNMENT, 0);
        let ptr = mapped.as_mut_ptr().cast::<Self>();
        let len = mapped.len() / ELEMENT_SIZE;
        // SAFETY: size/alignment are checked; Self is Pod. The current device/library
        // owns this persistent mapping for the same lifetime as its other mapped SSBOs.
        // Keep that established owner rather than allocating a CPU mirror.
        let mapped = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        MappedStorageBuffer::from_mapping_with_access(mapped, buffer, device.mapped_upload_writer(&buffer))
    }
}

/// W15：灌木位置读回 pass。`BushSsbo` = `compute_garden::BushSsbo`。
pub fn read_bush_positions_for_cpu<BushSsbo: SsboType>(
    env: &mut PassEnv<'_>,
) -> RecordedComputePass {
    read_bush_positions_for_cpu_with_building_prefix::<BushSsbo>(env, true)
}

/// With W14 disabled, the real preceding draw-command reset leaves slot10=0;
/// original SPV writes its count header but does not fetch any Bush record.
/// Preserve the binding/synchronization while declaring that actual no-read.
pub fn read_bush_positions_for_cpu_with_building_prefix<BushSsbo: SsboType>(
    env: &mut PassEnv<'_>,
    has_building_prefix: bool,
) -> RecordedComputePass {
    let pass = env.pass(NAME, URI)
        .write_to_cpu_mapped_storage_buffer::<BushCpuSsbo>(BUSH_CPU)
        .read_storage_buffer::<BushSsbo>(BUSH_BUFFER);
    let pass = if has_building_prefix {
        pass
    } else {
        pass.inactive_storage_buffer_contents(BUSH_BUFFER)
    };
    pass.read_storage_buffer::<DrawIndexedIndirectBuffer>(binding::DRAW_COMMANDS)
        .uniform(binding::DRAW_INDIRECT_BUFFER_OFFSET, DRAW_INDIRECT_OFFSET)
        .dispatch_1d(DISPATCH_THREADS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{
        accesses, env_with, test_carrier, threads,
    };

    test_carrier!(Bush, "Bush");

    #[test]
    fn canonical_library_uses_the_original_mapping_without_default_upload() {
        use crate::asset_libraries::ssbo_library::{AssetSsboLibrary, SsboId};
        use crate::render::ssbo::tests::MockDevice;
        let device = MockDevice::default();
        let size = 9601 * 8;
        device.map.borrow_mut().resize(size + 16, 0xa5);
        let address = device.map.borrow().as_ptr() as usize;
        let mut library = AssetSsboLibrary::default();
        library.add_new::<BushCpuSsbo>(&device);
        let mapped = library.get::<BushCpuSsbo>().unwrap();
        assert_eq!(core::mem::size_of::<BushCpuSsbo>(), 8);
        assert_eq!(core::mem::align_of::<BushCpuSsbo>(), 4);
        assert_eq!(SsboId::of::<BushCpuSsbo>().0.0, BUSH_CPU_SSBO_ID);
        assert_eq!(mapped.mapped.as_ptr() as usize, address);
        assert_eq!(mapped.mapped.len(), 9603);
        assert_eq!(mapped.mapped[1].0, [0xa5a5_a5a5; 2]);
        let calls = device.created.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, BufferDesc::new(size, 0x22, 4).alignment(4));
        assert_eq!(calls[0].1, "Bush positions for CPU");
        assert!(calls[0].2.is_none());
    }

    #[test]
    fn explicit_initial_rows_are_forwarded_exactly_and_bad_input_fails_before_create() {
        use crate::render::ssbo::tests::MockDevice;
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let device = MockDevice::default();
        let rows = [[2u32, 0x1234], [1f32.to_bits(), (-2f32).to_bits()]];
        let bytes = bytemuck::cast_slice(&rows);
        let mapped = BushCpuSsbo::new_storage_buffer(&device, Some(bytes));
        assert_eq!(mapped.mapped.len(), 9601);
        assert_eq!(device.created.borrow()[0].2.as_deref(), Some(bytes));
        drop(mapped);
        let invalid_length = [0u32; 3];
        assert!(
            catch_unwind(AssertUnwindSafe(|| BushCpuSsbo::new_storage_buffer(
                &device,
                Some(bytemuck::cast_slice(&invalid_length))
            )))
            .is_err()
        );
        let unaligned = &bytes[1..9];
        assert!(
            catch_unwind(AssertUnwindSafe(|| BushCpuSsbo::new_storage_buffer(
                &device,
                Some(unaligned)
            )))
            .is_err()
        );
        let excessive = vec![[0u32; 2]; 9602];
        assert!(
            catch_unwind(AssertUnwindSafe(|| BushCpuSsbo::new_storage_buffer(
                &device,
                Some(bytemuck::cast_slice(&excessive))
            )))
            .is_err()
        );
        assert_eq!(device.created.borrow().len(), 1);
    }

    #[test]
    fn names_match_exe_immediates() {
        assert_eq!(BUSH_CPU, 0xfc7c_5f72_86f4_34fe);
        assert_eq!(BUSH_BUFFER, 0x5bae_fac4_a34b_3dee);
        assert_eq!(binding::DRAW_INDIRECT_BUFFER_OFFSET, 0x0e95_11a6_16b7_5789);
    }

    #[test]
    fn writes_cpu_buffer_first_then_reads_and_dispatches_9601() {
        let mut e = env_with(
            &[BUSH_CPU, BUSH_BUFFER, binding::DRAW_COMMANDS],
            &[binding::DRAW_INDIRECT_BUFFER_OFFSET],
            |lib, dev| {
                lib.add_new::<BushCpuSsbo>(dev);
                lib.add_new::<Bush>(dev);
                lib.add_new::<DrawIndexedIndirectBuffer>(dev);
            },
        );
        let pass = e.run(read_bush_positions_for_cpu::<Bush>);
        assert_eq!(pass.name, NAME);
        assert_eq!(
            *e.world.backend.paths.borrow(),
            vec![format!("{URI}:cs_main")]
        );
        assert_eq!(threads(&pass), (9601, 1, 1));
        assert_eq!(
            e.uniform_u32(&pass, binding::DRAW_INDIRECT_BUFFER_OFFSET),
            10
        );
        let a = accesses(&pass);
        let codes: Vec<u8> = a.iter().map(|x| x.1).collect();
        assert_eq!(
            codes,
            [0x32, 0x1a, 0x1a],
            "cpu-mapped write, then two storage reads"
        );
    }
}
