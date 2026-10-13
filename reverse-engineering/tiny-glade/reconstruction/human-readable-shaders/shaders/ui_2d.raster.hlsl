// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/ui_2d.raster.hlsl | F-E | bins: _ui_2d.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin | entries vs:vs_main ps:ps_main
// 简单 2D UI 四边形(不做每相机变换):位置/尺寸直接用窗口像素。
// vs 生成 6 顶点四边形;ps 以 -1 mip 偏移采样 ui_texture,并应用可选的
// 压暗(半透明遮罩)与高亮颜色效果。
//
// 中文说明:最基础的 UI 贴图渲染。vs 把窗口像素坐标映射到裁剪空间;
// ps 采样贴图,按 darken_semitransparent/highlight 两个标志调整输出颜色。

#include "inc/common.hlsl"            // 采样器 (sampler_llc s3)
#include "inc/quad_mesh.hlsl"

cbuffer $Globals : register(b13, space0)
{
    float2 ui_position;              // 窗口像素坐标中的 UI 位置
    float2 ui_size;                  // 窗口像素尺寸
    float2 window_size;              // 目标窗口尺寸
    int darken_semitransparent;      // 半透明遮罩标志
    int highlight;                   // 高亮着色标志
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

    // 窗口像素坐标 -> 归一化坐标 -> 裁剪空间坐标(y 上下翻转)。
    float2 pos = (ui_position + (corner.xy * ui_size)) / window_size;
    pos.y = 1.0f - pos.y;
    o.position = float4(pos * 2.0f - 1.0f, 0.0f, 1.0f);
    o.uv = corner.zw;
    return o;
}

float4 ps_main(float2 uv : TEXCOORD0) : SV_Target0
{
    // 采样 UI 贴图,-1 mip 偏移获得更锐利的字体/图标。
    float4 c = ui_texture.SampleBias(sampler_llc, uv, -1.0f);

    float4 col;
    if (darken_semitransparent == 1)
        // 半透明遮罩:在贴图颜色与深色(0.4)之间按 0.9 混合,再整体乘 0.6。
        col = float4(c.rgb * lerp(c.rgb, 0.4f, 0.9f) * c.a, c.a) * 0.6f;
    else
        col = float4(c.rgb * c.a, c.a);

    if (highlight == 1)
        // 高亮:按颜色亮度在暗紫(0.08,0.048,0.064)与琥珀(0.7225,0.4675,0.17)之间混合。
        col = float4(lerp(float3(0.08f, 0.048f, 0.064f), float3(0.7225f, 0.4675f, 0.17f),
                          (1.0f - smoothstep(0.0f, 0.6f, length(col.rgb))).xxx) * col.w, col.w);

    return col;
}

#endif

// f0adff76827e73ff 变体的像素阶段:原版 bin 为空壳(无指令),按行为逐字提升。
#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _ui_2d.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
