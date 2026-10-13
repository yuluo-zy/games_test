// ==RECONSTRUCTED== shaders/rasterize_buildings_animate.cs.hlsl | evidence: spirv-cross of _rasterize_buildings_animate.cs.hlsl_cs_main_0.bin (fade building mask highlight: x = highlight, y lerps toward x) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "buildings/building_mask.inc.hlsl"

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWTexture2D<float2> out_mask : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 pixel = uint2(int3(dispatch_thread_id).xy);
    float4 value = out_mask[pixel].xyyy;
    float highlight = value.x;
    float fade = (highlight < 0.5f) ? 3.0f : 0.5f;
    out_mask[pixel] = float2(highlight, lerp(value.y, highlight, fade * delta_time_seconds));
}
