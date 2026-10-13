// ==RECONSTRUCTED== shaders/terrain_editing/read_terrain_rock_mask.cs.hlsl | evidence: spirv-cross of _terrain_editing_read_terrain_rock_mask.cs.hlsl_cs_main_0.bin (pack 32 rock-presence bits per row) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"

RWByteAddressBuffer __alias_mask_out : register(u15, space0);
Texture2D<float4> rocky_terrain : register(t14, space0);

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint index = dispatch_thread_id.x * 32u;
    uint2 pixel = uint2(index % 512u, index / 512u);

    uint mask = 0u;
    for (uint i = 0u; i < 32u; i++)
    {
        float4 rocky = rocky_terrain.Load(int3(pixel + uint2(i, 0u), 0u));
        float rock_strength = (rocky.x + max(0.0f, rocky.y - 0.1f)) + max(0.0f, rocky.z - 0.2f);
        mask |= uint(rock_strength > 0.1f) << (i & 31u);
    }

    __alias_mask_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, mask);
}
