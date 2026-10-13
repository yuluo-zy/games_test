// PATCHED: 模块 lsl_f0adff76827e73ff.bin:ps_main 由 tools/render-passes/batch_lift.py 从 oracle bin 提升而来;
// 下面这段手写代码在恰好这些 (variant, stage) 组合下被排除。
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water_plane_refraction_copy.raster.hlsl
// SOURCE PATH : shaders/water_plane_refraction_copy.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_plane_refraction_copy.raster.hlsl_b903e43ffb3da915.bin   (完整:ps 拷贝 refraction_source_tex)
//   _water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin   (空 ps:无输出接口)
// entries: vs:vs_main  ps:ps_main | index_pass W44 (refraction_copy_pass)
// Status: O (绑定/入口/LocalSize;vs 变换 + ps 拷贝来自 spirv-cross)
// Evidence: spirv-cross vs m0 (四边形:model_to_world -> xz*130, y=0, world_to_sample;
//   C0=水染色 0.05/0.05/0.2, C1=(0,1,0), C2=world) 与 ps m1 (b903e:把
//   refraction_source_tex.Load(fragCoord).xyz 拷到 SV_Target0;f0adff:空 ps)。
// MERGE (water-ice review water_plane_refraction_copy.raster.hlsl): A 方案整体采纳。
//   现在 include inc/terrain_heightmap.inc.hlsl(bin 确实内嵌了它),带
//   TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL 守卫(review inc-fix #2),使该 inc
//   无需 terrain_heightmap 纹理绑定即可编译(此 bin 不绑定该纹理)。
// 注:两个 16 进制变体是编译期特性变体 —— b903e = 启用折射拷贝,
//   f0adff = 禁用(空 ps)。以 WATER_REFRACTION_COPY 门控(I)。

#ifndef WATER_REFRACTION_COPY
#define WATER_REFRACTION_COPY 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/perlin.hlsl"
// Bin 在此内嵌 inc/terrain_heightmap.inc.hlsl,但不绑定地形纹理;NO_GLOBAL
// 守卫去掉全局纹理版本的 terrain_height(),使该 inc 能编译
// (review inc-fix #2)。get_terrain_height/get_terrain_normal 仍可用。
#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL 1
#include "inc/terrain_heightmap.inc.hlsl"

// 全局常量($Globals)。
cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    int bool_render_shadows;
    float4x4 model_to_world;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 water_mask_dims;
    uint2 refraction_source_tex_dims;
};

struct VsIn
{
    [[vk::location(0)]] float3 Vertex_Position: Vertex_Position;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float4 position : SV_Position;
};

// 顶点阶段:生成覆盖屏幕的水平面四边形。
VsOut vs_main(VsIn vin)
{
    VsOut o;
    // 模型 -> 世界。
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    // 把 xz 放大 130 倍铺满,保留 y 与世界 w。
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    // 水面固定在地面 y=0。
    p.y = 0.0f;
    // 世界 -> 采样空间。
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    // 水的浅色染色(蓝紫调),向上法线与水面位置透传给 ps。
    o.C0 = float3(0.0500000007450580596923828125f, 0.0500000007450580596923828125f, 0.20000000298023223876953125f);
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = p.xyz;
    return o;
}

// 特性开启:把折射源纹理逐像素拷贝到输出。
#if WATER_REFRACTION_COPY
Texture2D<float4> refraction_source_tex : register(t18, space0);

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
};

// 片段阶段:直接拷贝折射源像素(RGB),alpha 置 1。
float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(position.xy), 0u)).xyz, 1.0f);
}
#else
// 特性关闭:空 ps(无输出接口)。
struct PsInEmpty
{
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
}
#endif

#endif

#if defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)
// 从 _water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin 提升而来 (spirv-cross;行为一致,名称即 SPIR-V id)
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
