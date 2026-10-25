// PATCHED: modules lsl_f0adff76827e73ff.bin:ps_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_f0adff76827e73ff) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_PIXEL)))
// ==RECONSTRUCTED== shaders/water_plane_refraction_copy.raster.hlsl
// SOURCE PATH : shaders/water_plane_refraction_copy.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_plane_refraction_copy.raster.hlsl_b903e43ffb3da915.bin   (full: ps copies refraction_source_tex)
//   _water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin   (empty ps: no output interface)
// entries: vs:vs_main  ps:ps_main | index_pass W44 (refraction_copy_pass)
// Status: O (bindings/entry/LocalSize; vs transform + ps copy from spirv-cross)
// Evidence: spirv-cross vs m0 (quad: model_to_world -> xz*130, y=0, world_to_sample;
//   C0=water tint 0.05/0.05/0.2, C1=(0,1,0), C2=world) and ps m1 (b903e: copy
//   refraction_source_tex.Load(fragCoord).xyz -> SV_Target0; f0adff: empty ps).
// MERGE (water-ice review water_plane_refraction_copy.raster.hlsl): A adopted wholesale.
//   inc/terrain_heightmap.inc.hlsl is now included (the bin DOES embed it here) with the
//   TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL guard (review inc-fix #2) so the inc compiles
//   without a terrain_heightmap texture binding (this bin binds none).
// Note: two 16-hex variants are compile-time feature variants — b903e = refraction
//   copy enabled, f0adff = disabled (empty ps). Gated on WATER_REFRACTION_COPY (I).

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
// Bin embeds inc/terrain_heightmap.inc.hlsl here but binds no terrain texture; the
// NO_GLOBAL guard drops the global-texture `terrain_height()` so the include compiles
// (review inc-fix #2). get_terrain_height/get_terrain_normal remain available.
#define TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL 1
#include "inc/terrain_heightmap.inc.hlsl"

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

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = 0.0f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C0 = float3(0.0500000007450580596923828125f, 0.0500000007450580596923828125f, 0.20000000298023223876953125f);
    o.C1 = float3(0.0f, 1.0f, 0.0f);
    o.C2 = p.xyz;
    return o;
}

#if WATER_REFRACTION_COPY
Texture2D<float4> refraction_source_tex : register(t18, space0);

struct PsIn
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
};

float4 ps_main(PsIn vin, float4 position : SV_Position) : SV_Target0
{
    return float4(refraction_source_tex.Load(int3(uint2(position.xy), 0u)).xyz, 1.0f);
}
#else
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
// lifted from _water_plane_refraction_copy.raster.hlsl_f0adff76827e73ff.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
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
