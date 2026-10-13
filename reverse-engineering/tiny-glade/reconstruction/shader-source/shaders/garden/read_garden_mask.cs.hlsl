// ==RECONSTRUCTED== shaders/garden/read_garden_mask.cs.hlsl | evidence: spirv-cross of _garden_read_garden_mask.cs.hlsl_cs_main_0.bin (garden_mask.y > 0.3 -> bits, 32 texels/thread in 512-wide row) | status: O (algorithm + threshold from SPIR-V); I (buffer uses embedded __alias_ name)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"

Texture2D<float4> garden_mask : register(t14, space0);
RWStructuredBuffer<uint> __alias_mask_out : register(u15, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    uint row_start = id.x * 32u;
    uint2 start = uint2(row_start % 512u, row_start / 512u);

    uint bits = 0u;
    for (uint i = 0u; i < 32u; i++)
    {
        uint2 coord = start + uint2(i, 0u);
        bits |= (uint(garden_mask[coord].y > 0.3f) << (i & 31u));
    }
    __alias_mask_out[id.x] = bits;
}
