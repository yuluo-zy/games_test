// Vermeer 缩略图链:把多采样帧颜色解析为单采样目标("acquire"步骤)。
// 把每个目标纹素映射到对应的 MS 颜色纹素并取采样 0。
// 绑定:$Globals b14{color_tex_dims,target_tex_dims},color_tex(Texture2DMS t13),
// target_tex(RWTexture2D<unorm float4> u15)。LocalSize 8,8,1。

#include "inc/common.hlsl"            // samplers
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/coc.hlsl"
#include "inc/color/srgb.hlsl"

cbuffer $Globals : register(b14, space0)
{
    uint2 color_tex_dims;
    uint2 target_tex_dims;
};

Texture2DMS<float4> color_tex : register(t13, space0);
RWTexture2D<unorm float4> target_tex : register(u15, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 dispatch_thread_id : SV_DispatchThreadID)
{
    // 目标纹素中心 UV 反向映射回颜色纹理坐标,采样 0 号样本。
    target_tex[dispatch_thread_id.xy] = float4(color_tex.Load(int2(((float2(int2(dispatch_thread_id.xy)) + 0.5f.xx) / float2(target_tex_dims)) * float2(color_tex_dims)), 0).xyz, 1.0f);
}
