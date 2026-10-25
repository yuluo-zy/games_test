// ==RECONSTRUCTED== shaders/terrain_editing/read_terrain_height.cs.hlsl | evidence: spirv-cross of _terrain_editing_read_terrain_height.cs.hlsl_cs_main_0.bin (pairs of 16-bit packed heights into two alias buffers) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/pack_unpack.hlsl"
Texture2D<float4> terrain_heightmap : register(t14, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"

RWByteAddressBuffer __alias_height_out : register(u16, space0);
RWByteAddressBuffer __alias_visual_height_out : register(u17, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);

// Pack a [0,1] height sample into a 16-bit unsigned integer.
uint pack_height(float h)
{
    return uint((clamp(h, 0.0f, 1.0f) * 65535.0f) + 0.5f);
}

[numthreads(64, 1, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint index = dispatch_thread_id.x * 2u;
    uint2 pixel = uint2(index % 512u, index / 512u);
    uint2 pixel_next = pixel + uint2(1u, 0u);

    uint packed = pack_height(terrain_heightmap.Load(int3(pixel, 0)).x)
                + (pack_height(terrain_heightmap.Load(int3(pixel_next, 0)).x) << 16u);
    uint packed_visual = pack_height(visual_terrain_heightmap.Load(int3(pixel, 0)).x)
                       + (pack_height(visual_terrain_heightmap.Load(int3(pixel_next, 0)).x) << 16u);

    __alias_height_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, packed);
    __alias_visual_height_out.Store<uint>(dispatch_thread_id.x * 4u + 0u, packed_visual);
}
