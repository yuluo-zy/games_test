// ==RECONSTRUCTED== shaders/terrain_editing/cache_stroke.cs.hlsl | evidence: spirv-cross of _terrain_editing_cache_stroke.cs.hlsl_cs_main_0.bin (accumulate max stroke into cache, clear stroke) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/smooth_max.hlsl"

RWTexture2D<float4> stroke : register(u14, space0);
RWTexture2D<float4> cache : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 pixel = uint2(int3(dispatch_thread_id).xy);
    float m = max(stroke[pixel].x, cache[pixel].x);
    cache[pixel] = float4(m, m, m, 1.0f);
    stroke[pixel] = float4(0.0f, 0.0f, 0.0f, 1.0f);
}
