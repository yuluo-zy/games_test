// ==RECONSTRUCTED== shaders/water/calc_deferred_rt_args.cs.hlsl
// SOURCE PATH : shaders/water/calc_deferred_rt_args.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins):
//   _water_calc_deferred_rt_args.cs.hlsl_cs_main_0.bin
//     pass/stage   : _water_calc_deferred_rt (water) / cs
//     entry        : cs:cs_main
//     variant      : 0
//     local_size   : [1, 1, 1]
//     index_pass   : W2 (Inference)
//     names        : water_rt_count_buffer
// Status: O (binding/entry/LocalSize verbatim; body transcribed from SPIR-V decompile)
// Evidence: spirv-cross (_water_calc_deferred_rt_args.cs.hlsl_cs_main_0_m0) — one-shot
//   dispatch-size write: stores (count+63)/64 (group count), then 1,1 and the raw count
//   into the first 16 bytes of the water RT count buffer.
// MERGE (water-ice review water/calc_deferred_rt_args.cs.hlsl): A adopted wholesale.
//   The bin decompiles to RWByteAddressBuffer (B used RWStructuredBuffer<uint4> — wrong
//   access type vs the bin's byte-address access).

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"

RWByteAddressBuffer water_rt_count_buffer : register(u13, space0);

[numthreads(1, 1, 1)]
void cs_main()
{
    water_rt_count_buffer.Store4(0,
        uint4((water_rt_count_buffer.Load(0) + 63u) / 64u, 1u, 1u, water_rt_count_buffer.Load(0)));
}
