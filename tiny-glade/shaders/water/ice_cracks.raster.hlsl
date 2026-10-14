// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water/ice_cracks.raster.hlsl
// SOURCE PATH : shaders/water/ice_cracks.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_ice_cracks.raster.hlsl_b903e43ffb3da915.bin   (ps packs crack tint -> uint2)
//   _water_ice_cracks.raster.hlsl_f0adff76827e73ff.bin   (empty ps, no output interface)
// entries: vs:vs_main  ps:ps_main | index_pass W2 (Inference)
// Status: O (bindings/entry; vs + ps from spirv-cross)
// Evidence: spirv-cross vs (model_to_world transform, world_to_sample; C0=vertex color,
//   C1=model*normal, C2=world) and ps (b903e: view_constants[2] view_to_world up-vector
//   color, sqrt-clamp of C0*0.4, RGB8 pack into uint2 albedo+material-id; f0adff: empty).
// MERGE (water-ice review water_ice_cracks.raster.hlsl): A adopted wholesale. A's file
//   path water/ice_cracks.raster.hlsl matches the bin's embedded source path (NOT
//   water_ice_cracks.raster.hlsl); water_mask_sampler.inc added (the bin closure
//   includes shaders/water/water_mask_sampler.inc.hlsl + texture_utils).
// Note: two 16-hex variants are a compile-time feature variant — b903e = colored
//   cracks, f0adff = no-output. Gated on WATER_ICE_CRACKS_COLOR (I).
//
// 冰面裂纹:顶点把网格变换到采样空间并传递顶点色/法线/世界坐标;
// 片元把裂纹色调打包成 uint2(albedo + 材质 id)。f0adff 变体无输出。

#ifndef WATER_ICE_CRACKS_COLOR
#define WATER_ICE_CRACKS_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    float4x4 model_to_world;
};

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
    [[vk::location(1)]] float3 Vertex_Color: Vertex_Color;
    [[vk::location(2)]] float3 Vertex_Normal: Vertex_Normal;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

// 顶点着色器:模型 -> 世界 -> 采样空间变换,传递顶点色/法线/世界坐标。
VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    o.position = mul(view_constants[current_view_index].world_to_sample, world);
    o.C0 = vin.Vertex_Color;
    o.C1 = mul(model_to_world, float4(vin.Vertex_Normal, 0.0f)).xyz;
    o.C2 = world.xyz;
    return o;
}

#if WATER_ICE_CRACKS_COLOR
struct PsIn
{
    float3 C0 : TEXCOORD0;
};

// 片元着色器(彩色裂纹变体):把裂纹色调与向上的视图颜色打包成 uint2。
uint2 ps_main(PsIn vin) : SV_Target0
{
    // 视图空间 up 向量颜色(半偏置)。
    float3 up_color = (mul(view_constants[2].view_to_world, float4(0.0f, 0.0f, 1.0f, 0.0f)).xyz * 0.5f) + 0.5f;
    // 顶点色开平方 + 0.4 增益,再 clamp。
    float3 c = sqrt(clamp(vin.C0 * 0.4000000059604644775390625f, 0.0f, 1.0f));
    // RGB8 打包进 albedo。
    uint albedo = (uint((clamp(c.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                + (uint((clamp(c.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                + (uint((clamp(c.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    // 材质 id(1291845632u = 高 4 位材质标记)+ albedo。
    uint2 out_px = uint2(
        ((uint((clamp(up_color.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
        + (uint((clamp(up_color.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
        + (uint((clamp(up_color.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 1291845632u,
        albedo);
    // albedo 再叠上另一组材质位(100663296u)。
    out_px.y = albedo | 100663296u;
    return out_px;
}
#else
struct PsInEmpty
{
    float3 C0 : TEXCOORD0;
};

void ps_main(PsInEmpty vin)
{
}
#endif

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// lifted from _water_ice_cracks.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
// 该变体 ps 无指令(空函数)。
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
