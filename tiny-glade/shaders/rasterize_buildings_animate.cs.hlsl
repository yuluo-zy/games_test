// ==RECONSTRUCTED== shaders/rasterize_buildings_animate.cs.hlsl | 证据: spirv-cross
//   还原 _rasterize_buildings_animate.cs.hlsl_cs_main_0.bin(建筑掩码高亮淡入淡出:
//   x = 高亮值,y 向 x 线性逼近)| 状态: O(绑定/入口/工作组尺寸/流程); I(命名)
//   用途: 让建筑掩码的高亮值随时间平滑过渡。读取当前高亮 x,用速度 3.0 或 0.5
//   按帧增量把 y 向 x 插值。
//   绑定: out_mask u14 (RWTexture2D<float2>), 工作组尺寸 [8,8,1], 入口 cs_main。

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "buildings/building_mask.inc.hlsl"

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
// 建筑掩码(.x = 目标高亮,.y = 当前过渡值)。
RWTexture2D<float2> out_mask : register(u14, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    uint2 pixel = uint2(int3(dispatch_thread_id).xy);
    float4 value = out_mask[pixel].xyyy;
    float highlight = value.x;
    // 淡化速度:高亮较暗(<0.5)时加快(3.0),否则放缓(0.5)。
    float fade = (highlight < 0.5f) ? 3.0f : 0.5f;
    // 按帧时长把 y 向目标高亮 x 线性插值。
    out_mask[pixel] = float2(highlight, lerp(value.y, highlight, fade * delta_time_seconds));
}
