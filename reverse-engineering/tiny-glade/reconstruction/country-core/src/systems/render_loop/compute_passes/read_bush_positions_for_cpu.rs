//! `read_bush_positions_for_cpu.rs` — W15 bush readback for CPU (0x140866540).
//!
//! URI `/garden/read_bush_positions_for_cpu.cs.hlsl`. `ComputePass::read_draw_indirect_cmd_buffer`
//! + dispatch_1d (O). `BushCpuSsbo` (O name; layout U).

use crate::typed_gap::Gap;

pub struct BushCpuSsbo(pub Gap);

pub fn read_bush_positions_for_cpu(world: &mut Gap, _bushes: &Gap) {
    let _ = world;
    unimplemented!()
}
