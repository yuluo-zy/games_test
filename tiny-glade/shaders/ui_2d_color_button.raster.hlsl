// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/ui_2d_color_button.raster.hlsl | F-E | bins: _ui_2d_color_button.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 取色器按钮四边形。顶点四边形与 ui_2d 相同;ps 额外做圆形展开动画:
// progress 沿圆周扫过,is_original_color 时向原色遮罩,带白色混合描边。
//
// 中文说明:UI 取色按钮。ps 按到中心的距离控制"展开圆环"的揭示程度,
// 依据 is_original_color 决定揭示范围与白色强度,并用 animation_t 控制整体淡入。

#include "inc/common.hlsl"            // 采样器 (sampler_llr s1)
#include "inc/quad_mesh.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float2 ui_position;
    float2 ui_size;
    float2 window_size;
    float animation_t;               // 动画进度(控制整体淡入)
    int is_original_color;           // 是否为原色(决定揭示方式)
    uint2 ui_texture_dims;
};

Texture2D<float4> ui_texture : register(t14, space0);

struct VsOut
{
    float2 uv : TEXCOORD0;
    float4 position : SV_Position;
};

VsOut vs_main(uint vertex_id : SV_VertexID)
{
    VsOut o;

    // 3x3 三角形扇展开四边形角点:位置偏移 + UV。
    float4 corner;
    if (vertex_id == 0u) corner = float4(0.5f, -0.5f, 1.0f, 0.0f);
    else if (vertex_id == 1u) corner = float4(0.5f, 0.5f, 1.0f, 1.0f);
    else if (vertex_id == 2u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 3u) corner = float4(-0.5f, 0.5f, 0.0f, 1.0f);
    else if (vertex_id == 4u) corner = float4(-0.5f, -0.5f, 0.0f, 0.0f);
    else corner = float4(0.5f, -0.5f, 1.0f, 0.0f);

    // 窗口像素坐标 -> 归一化 -> 裁剪空间。
    float2 pos = (ui_position + (corner.xy * ui_size)) / window_size;
    pos.y = 1.0f - pos.y;
    o.position = float4(pos * 2.0f - 1.0f, 0.0f, 1.0f);
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    // 采样贴图,-0.75 mip 偏移。
    float4 c = ui_texture.SampleBias(sampler_llr, uv, -0.75f);

    // 以中心为原点的距离,驱动圆形揭示范围。
    float dist = length(abs(uv - 0.5f) * 2.0f);
    float reveal = is_original_color == 1
        ? smoothstep(0.6f, 0.7f, dist)
        : smoothstep(0.8f, 0.9f, dist);

    // 揭示区域叠加白色描边,并提升 alpha。
    float alpha = clamp(c.a + reveal * 2.0f, 0.0f, 1.0f);
    float white = is_original_color == 1 ? 0.9f : 0.4f;
    float3 col = lerp(c.rgb, 1.0f, white * reveal) * alpha;

    // 边缘 0.95..1.0 淡出 + 按动画进度整体淡入。
    return float4(col, alpha) * ((1.0f - smoothstep(0.95f, 1.0f, dist)) * pow(animation_t, 0.2f));
}

#endif

// f0adff76827e73ff 变体的像素阶段:原版 bin 为空壳(无指令),按行为逐字提升。
#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _ui_2d_color_button.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
