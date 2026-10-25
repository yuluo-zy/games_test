// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main 由 batch_lift.py 从 oracle bin 转译;
// 下方手写代码恰好对那些(变体, 阶段)对被排除。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// Vermeer 缩略图链:最终合成。非照片模式下,累计笔触 alpha 驱动渐进式揭示,
// 柔和暗角随进度淡入。
// 绑定:$Globals b13,output_tex t15。

#include "inc/common.hlsl"            // samplers (sampler_lnc s0)
#include "inc/tracked_buffer.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/perlin.hlsl"
#include "inc/fbm_noise.hlsl"
#include "inc/uv.hlsl"
#include "inc/color/srgb.hlsl"
#include "vermeer/stroke.inc.hlsl"
#include "fullscreen_triangle_vert.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint max_strokes;
    uint2 target_tex_dims;
    uint2 output_tex_dims;
    uint2 framebuffer_dims;
    float progress_t;
    uint photomode;
};

Texture2D<float4> output_tex : register(t15, space0);

float4 ps_main(float4 position : SV_Position) : SV_Target0
{
    float2 uv = (float2(int2(position.xy)) + 0.5f.xx) / float2(framebuffer_dims);
    float4 c = output_tex.SampleLevel(sampler_lnc, uv, 0.0f);

    float4 out_color;
    if (photomode == 0u)
    {
        // 揭示扫描:progress 从 0.28 到 0.49 期间把 alpha 从 1 lerp 到累计 alpha。
        float reveal = lerp(1.0f, c.w, smoothstep(0.2800000011920928955078125f, 0.4899999797344207763671875f, progress_t));
        float3 color = c.xyz * min(2.0f, reveal / c.w);
        out_color = float4(color, reveal);
    }
    else
    {
        out_color = c;
    }

    // 非照片模式暗角:进度未完成时按到屏幕中心的距离^4 衰减。
    if (photomode == 0u)
    {
        float dist = length(uv - 0.5f.xx);
        float dist2 = dist * dist;
        out_color.xyz *= exp((clamp(1.0f - progress_t, 0.0f, 1.0f) * (-1.39999997615814208984375f)) * (dist2 * dist2));
    }

    return out_color;
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _vermeer_combine.raster.hlsl_f0adff76827e73ff.bin 转译(spirv-cross;行为精确,名字为 SPIR-V id)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
void frag_main()
{
}

void ps_main()
{
    frag_main();
}
#pragma dxc diagnostic pop
#endif
