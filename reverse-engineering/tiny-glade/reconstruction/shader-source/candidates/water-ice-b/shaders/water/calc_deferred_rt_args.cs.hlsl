// ==RECONSTRUCTED== shaders/water/calc_deferred_rt_args.cs.hlsl
// Evidence: _water_calc_deferred_rt_args.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: water_rt_count_buffer u13 (RWByteAddressBuffer; debug name in the bin is
//   type.RWStructuredBuffer.v4uint but access is a byte-address uint load / uint4 store),
//   no frame_constants, LocalSize [1,1,1], entry cs_main, caps [Shader].
//   spirv-cross + disassembly: resets the water RT indirect-dispatch args:
//   uint4(cell_count=(count+63)/64, 1, 1, original_count). Written before deferred_rt
//   to bound the [64,1,1] dispatch to the number of emitted water rays.
// Status: O (algorithm + binding from SPIR-V).

#include "inc/common.hlsl"

RWStructuredBuffer<uint4> water_rt_count_buffer : register(u13, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    water_rt_count_buffer[0] = uint4((water_rt_count_buffer[0].x + 63u) / 64u, 1u, 1u, water_rt_count_buffer[0].x);
}
