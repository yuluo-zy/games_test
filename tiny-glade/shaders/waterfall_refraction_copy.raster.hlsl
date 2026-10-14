// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// 中文说明:
//   瀑布折射拷贝 pass(index_pass W44)。bin:
//   _waterfall_refraction_copy.raster.hlsl_{b903e43ffb3da915,f0adff76827e73ff}.bin
//   - b903e:ps 直接拷贝 refraction_source_tex.Load(fragCoord).xyz;
//   - f0adff:ps 为空(下方转译)。
//   vs 来自共享 water/waterfall_vert.inc.hlsl(每实例位移 + 3D value-noise;
//   C0=顶点法线,C1=世界)。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/waterfall_refraction_copy.raster.hlsl
// 源路径 shaders/waterfall_refraction_copy.raster.hlsl,入口 vs:vs_main ps:ps_main
// 两个 16 位哈希变体由 WATERFALL_REFRACTION_COPY 门控(b903e=拷贝,f0adff=空)。

#ifndef WATERFALL_REFRACTION_COPY
#define WATERFALL_REFRACTION_COPY 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 refraction_source_tex_dims;
};

// vs_main(使用 $Globals.current_view_index + instance_buffer)在此包含。
#include "water/waterfall_vert.inc.hlsl"

#if WATERFALL_REFRACTION_COPY
struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

Texture2D<float4> refraction_source_tex : register(t15, space0);

// 直接把折射源纹素拷到输出。
float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(position.xy), 0u)).xyz, 1.0f);
}
#else
struct PsInEmpty
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
}
#endif

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 转译自 _waterfall_refraction_copy.raster.hlsl_f0adff76827e73ff.bin
// (spirv-cross;行为精确,变量名为 SPIR-V 编号)——精简变体为空 PS 占位。
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
