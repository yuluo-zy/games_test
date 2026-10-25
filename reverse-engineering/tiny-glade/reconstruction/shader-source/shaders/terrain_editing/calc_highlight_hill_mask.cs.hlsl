// ==RECONSTRUCTED== shaders/terrain_editing/calc_highlight_hill_mask.cs.hlsl | evidence: spirv-cross of _terrain_editing_calc_highlight_hill_mask.cs.hlsl_cs_main_0.bin (fill mask where the highlight stroke passes) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 stroke_indices_dims;
    int highlight_stroke_index;
    uint2 img_out_dims;
};

Texture2D<int4> stroke_indices : register(t14, space0);
RWTexture2D<half> img_out : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    if (highlight_stroke_index == -1)
    {
        img_out[uint2(int3(dispatch_thread_id).xy)] = 0.0f;
    }
    else
    {
        int2 dims = int2(stroke_indices_dims);
        float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f) / float2(int2(img_out_dims)) - 0.5f;
        float3 query = ((float2(130.0f, 130.0f) * uv).xyy * 0.0076923076994717121124267578125f.xxx) + 0.5f.xxx;
        int2 pixel = int2(int(query.x * float(dims.x)), int(query.z * float(dims.y)));
        if (stroke_indices.Load(int3(uint2(pixel), 0)).x == highlight_stroke_index)
        {
            img_out[uint2(int3(dispatch_thread_id).xy)] = 1.0f;
        }
        else
        {
            img_out[uint2(int3(dispatch_thread_id).xy)] = 0.0f;
        }
    }
}
