// ==RECONSTRUCTED== shaders/rasterize_buildings.cs.hlsl | evidence: spirv-cross of _rasterize_buildings.cs.hlsl_cs_main_0.bin (mask buildings: shape_id 0 = circle, 1 = oriented box, into out_mask.x + out_mask2) | status: O (bindings/entry/LocalSize/flow); I (names)

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"
#include "buildings/building_mask.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 out_mask_dims;
    uint2 out_mask2_dims;
    int buildings_count;
};

StructuredBuffer<BuildingShape> buildings_buffer : register(t16, space0);
RWTexture2D<float2> out_mask : register(u14, space0);
RWTexture2D<unorm float> out_mask2 : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f.xx) / float2(int2(out_mask_dims));
    float2 world_xz = ((uv * 2.0f) - 1.0f.xx) * 65.0f.xx;

    float result = 0.0f;
    for (int i = 0; i < buildings_count; i++)
    {
        if (buildings_buffer[i].shape_id == 0)
        {
            if (distance(world_xz, buildings_buffer[i].center) < (buildings_buffer[i].dims.x + 0.75f))
            {
                result = 1.0f;
                break;
            }
        }
        else
        {
            if (buildings_buffer[i].shape_id == 1)
            {
                float2 local = mul(world_xz - buildings_buffer[i].center,
                                   float2x2(float2(buildings_buffer[i].orientation.x, -buildings_buffer[i].orientation.y),
                                            float2(buildings_buffer[i].orientation.y, buildings_buffer[i].orientation.x)));
                float half_w = (buildings_buffer[i].dims.x + 1.0f) * 0.5f;
                float half_h = (buildings_buffer[i].dims.y + 1.0f) * 0.5f;
                if ((((int(-local.x <= half_w) & int(-local.y <= half_h)) & int(local.x <= half_w)) & int(local.y <= half_h)) != 0)
                {
                    result = 1.0f;
                    break;
                }
            }
        }
    }

    uint2 pixel = uint2(int3(dispatch_thread_id).xy);
    out_mask[pixel] = float2(result, out_mask[pixel].y);
    out_mask2[pixel] = result;
}
