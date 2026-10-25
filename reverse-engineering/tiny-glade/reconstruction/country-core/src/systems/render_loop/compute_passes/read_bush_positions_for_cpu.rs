//! `read_bush_positions_for_cpu.rs` — 灌木位置回读 CPU（W15，0x140866540，696 B）。
//!
//! 机器码（O）：`ComputePass::new("Read bush position for cpu", "/garden/read_bush_positions_for_cpu.cs.hlsl")`
//! （名长 0x1a、URI 长 0x2b）。顺序：`write_to_cpu_mapped_storage_buffer<BushCpuSsbo>(bush_cpu)` →
//! `read_storage_buffer<BushSsbo>(bush_buffer)` → `read_storage_buffer<DrawIndexedIndirectBuffer>(draw_commands)` →
//! uniform i32 `draw_indirect_buffer_offset = 10` → `dispatch_1d(0x2581)`（闭包状态 `{1, 0x2581, 1, 1}`，0x142a95de0）。
//! 无 world 资源读取、无分支。
//!
//! `BushSsbo` 属 `compute_garden`（别的模块；NAME U，SsboId 0x87df9c07cbe7e324），故作泛型参数。
//! `BushCpuSsbo`（SsboId 0x0deff5ab6a35972a）：`new_storage_buffer`（0x14161bd30）是 `MappedStorageBuffer<T>`（固定 0x40000 B）；
//! 元素布局 U、`T::NAME` U（试过 BushCpu/BushCpuSsbo/BushPositions…均不吻合 xxh3），这里 NAME 取类型名。
//! 存储用 `TypedStorageBuffer`（`MappedStorageBuffer<'static,_>` 需要 `MappedBuffers` 设备，`dyn GraphicsDevice` 给不出，U）。

use crate::asset_libraries::ssbo_library::SsboType;
use crate::render::pass_builder::compute_pass_builder::RecordedComputePass;
use crate::render::ssbo::rhapsody_boundary::GraphicsDevice;
use crate::render::ssbo::{StorageElement, TypedStorageBuffer};
use crate::systems::render_loop::compute_passes::pass_kit::{PassEnv, binding, binding_hash};
use crate::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer;
use crate::typed_gap::Gap;

pub const NAME: &str = "Read bush position for cpu";
pub const URI: &str = "/garden/read_bush_positions_for_cpu.cs.hlsl";
pub const BUSH_CPU: u64 = binding_hash("bush_cpu");
pub const BUSH_BUFFER: u64 = binding_hash("bush_buffer");
/// uniform `draw_indirect_buffer_offset`（O：`mov r9d, 10`）。
pub const DRAW_INDIRECT_OFFSET: i32 = 10;
/// `dispatch_1d` 线程总数（O：0x2581 = 9601）。
pub const DISPATCH_THREADS: u32 = 0x2581;
/// `get_rg_buffer<BushCpuSsbo>` 的 SsboId 立即数（O；与本类型 NAME 的 xxh3 不吻合，见模块文档）。
pub const BUSH_CPU_SSBO_ID: u64 = 0x0def_f5ab_6a35_972a;

/// 回读给 CPU 的灌木位置缓冲（O 名；元素布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct BushCpuSsbo(pub Gap);

impl StorageElement for BushCpuSsbo {
    const NAME: &'static str = "BushCpuSsbo";
    /// I：原版是固定 0x40000 B 的映射缓冲（`BUFFER_LEN` 概念不适用）；这里用 1 个元素让存储可建（偏差，U）。
    const BUFFER_LEN: usize = 1;
    fn default_value() -> Option<Self> {
        Some(Self::default())
    }
}

impl SsboType for BushCpuSsbo {
    type Storage = TypedStorageBuffer<BushCpuSsbo>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        match initial {
            None => TypedStorageBuffer::<BushCpuSsbo>::new(device, None),
            Some(b) => TypedStorageBuffer::<BushCpuSsbo>::new_storage_buffer(device, Some(b)),
        }
    }
}

/// W15：灌木位置读回 pass。`BushSsbo` = `compute_garden::BushSsbo`。
pub fn read_bush_positions_for_cpu<BushSsbo: SsboType>(env: &mut PassEnv<'_>) -> RecordedComputePass {
    env.pass(NAME, URI)
        .write_to_cpu_mapped_storage_buffer::<BushCpuSsbo>(BUSH_CPU)
        .read_storage_buffer::<BushSsbo>(BUSH_BUFFER)
        .read_storage_buffer::<DrawIndexedIndirectBuffer>(binding::DRAW_COMMANDS)
        .uniform(binding::DRAW_INDIRECT_BUFFER_OFFSET, DRAW_INDIRECT_OFFSET)
        .dispatch_1d(DISPATCH_THREADS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systems::render_loop::compute_passes::pass_kit::test_env::{accesses, env_with, test_carrier, threads};

    test_carrier!(Bush, "Bush");

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
        assert_eq!(*e.world.backend.paths.borrow(), vec![format!("{URI}:cs_main")]);
        assert_eq!(threads(&pass), (9601, 1, 1));
        assert_eq!(e.uniform_u32(&pass, binding::DRAW_INDIRECT_BUFFER_OFFSET), 10);
        let a = accesses(&pass);
        let codes: Vec<u8> = a.iter().map(|x| x.1).collect();
        assert_eq!(codes, [0x32, 0x1a, 0x1a], "cpu-mapped write, then two storage reads");
    }
}
