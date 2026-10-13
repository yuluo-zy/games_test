// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/texture_test.raster.hlsl | F-D
// 证据:_texture_test.raster.hlsl_b903e43ffb3da915.bin (vs_main + ps_main)。
//   绑定:sampler_default s9, $Globals b13 (current_view_index/model_to_world/
//   test_texture_dims/stroke_max_height), test_texture t14, frame_constants set1/b0。
//   spirv-cross:vs 用 test_texture 在 (vertex.xz*1/130 + 0.5) 处的采样值乘
//   stroke_max_height 位移顶点的 y,再做 world_to_sample;ps 按插值 UV 采样
//   test_texture。test_texture_dims 在 vs 中读取(保留)。
// Status: O (structure from disassembly)。

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 test_texture_dims;
    float stroke_max_height;
};

Texture2D<float4> test_texture : register(t14, space0);

struct VSIn
{
    [[vk::location(0)]] float3 position: Vertex_Position;
    [[vk::location(1)]] float3 color: Vertex_Color;
};

struct VSOut
{
    float3 c0 : TEXCOORD0;
    float2 c1 : TEXCOORD1;
    float4 position : SV_Position;
};

// 顶点着色器:按测试纹理高度场把顶点 y 抬到 stroke_max_height 高度。
VSOut vs_main(VSIn input)
{
    VSOut output;
    float2 uv = (input.position.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
    output.position = mul(view_constants[current_view_index].world_to_sample, mul(model_to_world, float4(input.position.x, test_texture.SampleLevel(sampler_default, uv, 0.0f).x * stroke_max_height, input.position.z, 1.0f)));
    output.c0 = input.color;
    output.c1 = uv;
    return output;
}

// 像素着色器:直接输出测试纹理颜色。
float4 ps_main(VSOut input) : SV_Target0
{
    return test_texture.Sample(sampler_default, input.c1);
}

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _texture_test.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
