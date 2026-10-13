// ==RECONSTRUCTED== shaders/rasterize_buildings.cs.hlsl | 证据: spirv-cross 还原
//   _rasterize_buildings.cs.hlsl_cs_main_0.bin(把建筑掩码栅格化:shape_id 0 = 圆形,
//   1 = 朝向盒,写入 out_mask.x 与 out_mask2)| 状态: O(绑定/入口/工作组尺寸/流程);
//   I(命名)
//   用途: 把建筑轮廓绘制到两张掩码纹理。对每个像素遍历建筑列表,圆形建筑按
//   到中心距离(半径 dims.x + 0.75)判断,盒形建筑按旋转到局部坐标系后的半宽/半高
//   判断;命中的像素在 out_mask.x 记 1,out_mask2 记 1。
//   绑定: $Globals b13 (out_mask_dims/out_mask2_dims/buildings_count),
//   buildings_buffer t16, out_mask u14, out_mask2 u15, 工作组尺寸 [8,8,1]。

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"
#include "buildings/building_mask.inc.hlsl"

// 掩码尺寸与建筑数量。
cbuffer $Globals : register(b13, space0)
{
    uint2 out_mask_dims;
    uint2 out_mask2_dims;
    int buildings_count;
};

// 建筑形状列表(中心/朝向/尺寸)。
StructuredBuffer<BuildingShape> buildings_buffer : register(t16, space0);
// 建筑掩码(.x 记录命中,.y 保留)。
RWTexture2D<float2> out_mask : register(u14, space0);
// 简化掩码(只记命中标记)。
RWTexture2D<unorm float> out_mask2 : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 像素中心 -> [0,1] UV -> 世界 xz 坐标(半径 65 的世界范围)。
    float2 uv = (float2(int3(dispatch_thread_id).xy) + 0.5f.xx) / float2(int2(out_mask_dims));
    float2 world_xz = ((uv * 2.0f) - 1.0f.xx) * 65.0f.xx;

    float result = 0.0f;
    // 遍历所有建筑,任一命中即停止。
    for (int i = 0; i < buildings_count; i++)
    {
        if (buildings_buffer[i].shape_id == 0)
        {
            // 圆形建筑:到中心距离小于半径(dims.x + 0.75)即命中。
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
                // 盒形建筑:旋转到建筑局部坐标系,检查是否落入半宽/半高范围。
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
    // 写命中标记到两张掩码(x 通道 + 简化掩码)。
    out_mask[pixel] = float2(result, out_mask[pixel].y);
    out_mask2[pixel] = result;
}
