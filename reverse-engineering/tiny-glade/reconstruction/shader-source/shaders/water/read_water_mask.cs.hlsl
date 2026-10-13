// ==RECONSTRUCTED== shaders/water/read_water_mask.cs.hlsl | evidence: spirv-cross + raw disasm of _water_read_water_mask.cs.hlsl_cs_main_0.bin (water_mask.y > 0.5 -> water bits; > 0.2 -> clutter bits; 32 texels/thread in a 512-wide row; RWStructuredBuffer<uint> aliases) | status: O (algorithm + thresholds + member names from SPIR-V); I (threshold values are literals; buffer access uses the embedded __alias_ names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"

Texture2D<float4> water_mask : register(t14, space0);
RWStructuredBuffer<uint> __alias_mask_out : register(u15, space0);
RWStructuredBuffer<uint> __alias_mask_out_clutter : register(u16, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint row_start = id.x * 32u;
    uint2 start = uint2(row_start % 512u, row_start / 512u);

    uint water_bits = 0u;
    uint clutter_bits = 0u;
    for (uint i = 0u; i < 32u; i++)
    {
        uint2 coord = start + uint2(i, 0u);
        water_bits |= (uint(water_mask[coord].y > 0.5f) << (i & 31u));
        clutter_bits |= (uint(water_mask[coord].y > 0.2f) << (i & 31u));
    }
    __alias_mask_out[id.x] = water_bits;
    __alias_mask_out_clutter[id.x] = clutter_bits;
}
