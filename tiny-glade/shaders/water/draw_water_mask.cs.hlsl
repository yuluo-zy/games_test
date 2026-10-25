// ==RECONSTRUCTED== shaders/water/draw_water_mask.cs.hlsl | evidence: spirv-cross + raw SPIR-V disasm of _water_draw_water_mask.cs.hlsl_cs_main_0.bin (input SV_DispatchThreadID is int2; $Globals{out_water_mask_dims,is_add,is_subtract,cursor_position_ws}; pixel->world via (float2(pixel)+0.5)/float2(int2(dims))*2-1 then *65.0f.xx; coverage = ((2-clamp(distance,0,2))*0.5)>0 ? 1:0; add=max, subtract=max(0,prev.x-coverage)) | status: O (algorithm + member names from SPIR-V); I (brush radius 2.0, 65.0 = TEXTURE_SIZE_WS/2)
//
// 绘制水面掩码(画笔工具):像素转世界坐标,按笔刷半径算覆盖度,
// 叠加(is_add=max)或擦除(is_subtract=max(0, prev.x - 覆盖度))。

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 out_water_mask_dims;
    int is_add;
    int is_subtract;
    float3 cursor_position_ws;
};

RWTexture2D<float4> out_water_mask : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(int2 pixel : SV_DispatchThreadID)
{
    // 像素 -> 世界 XZ(-1..1 再乘半世界尺寸 65)。
    float2 world_xz = ((float2(pixel) + 0.5f) / float2(int2(out_water_mask_dims)) * 2.0f - 1.0f) * 65.0f.xx;

    // 距笔刷中心的距离 -> 覆盖度(半径 2 内线性衰减)。
    float d = distance(float3(world_xz.x, 0.0f, world_xz.y).xz, cursor_position_ws.xz);
    float coverage = ((2.0f - clamp(d, 0.0f, 2.0f)) * 0.5f) > 0.0f ? 1.0f : 0.0f;

    float4 prev = out_water_mask[uint2(pixel)];
    float4 result = prev;
    if (is_add == 1)
        result = max(prev, float4(coverage, coverage, coverage, 1.0f));
    else if (is_subtract == 1)
    {
        float v = max(prev.x - coverage, 0.0f);
        result = float4(v, v, v, 1.0f);
    }
    out_water_mask[uint2(pixel)] = result;
}
