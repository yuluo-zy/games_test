// ==RECONSTRUCTED== shaders/water_snow_plane.raster.hlsl
// SOURCE PATH : shaders/water_snow_plane.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_snow_plane.raster.hlsl_b903e43ffb3da915.bin  (ps writes uint2(0xFF80..) snow tint)
//   _water_snow_plane.raster.hlsl_f0adff76827e73ff.bin  (ps: same samples/discards, no output interface)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs transform + ps snow-mask/perlin from spirv-cross)
// Evidence: spirv-cross vs (water plane at y=-0.01, xz*130 heightmap-space, world_to_sample;
//   C1 = world.xyz) and ps (water_mask.SampleLevel(sampler_llc) green channel vs 0.2;
//   perlin(C1.xz*1.5) variance discard; b903e writes const uint2 snow tint, f0adff no target).
// MERGE (water-ice review water_snow_plane.raster.hlsl): A adopted wholesale — exact
//   transcription (thresholds/tint uint2(4286644096,114677205)).
// Note: the two 16-hex variants are a compile-time feature variant — b903e = colored
//   snow plane, f0adff = no-output (depth/mask-only). Gated on WATER_SNOW_PLANE_COLOR (I).

#ifndef WATER_SNOW_PLANE_COLOR
#define WATER_SNOW_PLANE_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/consts.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/utils.hlsl"
#include "inc/hash.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    float4x4 model_to_world;
    uint2 water_mask_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
};

struct VsOut
{
    float3 C1 : TEXCOORD1;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 world = mul(model_to_world, float4(vin.Vertex_Position, 1.0f));
    float2 scaled = world.xz * 130.0f.xx;
    float4 p = float4(scaled.x, world.y, scaled.y, world.w);
    p.y = -0.00999999977648258209228515625f;
    o.position = mul(view_constants[current_view_index].world_to_sample, p);
    o.C1 = p.xyz;
    return o;
}

#if WATER_SNOW_PLANE_COLOR
struct PsIn
{
    float3 C1 : TEXCOORD1;
};

uint2 ps_main(PsIn vin) : SV_Target0
{
    float4 wm = water_mask.SampleLevel(sampler_llc, (vin.C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }

    // 2D value-noise (perlin.hlsl) inlined with explicit bilinear arithmetic.
    float2 p = vin.C1.xz * 1.5f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;

    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f,
                wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
    return uint2(4286644096u, 114677205u);
}
#else
struct PsInEmpty
{
    float3 C1 : TEXCOORD1;
};

void ps_main(PsInEmpty vin)
{
    float4 wm = water_mask.SampleLevel(sampler_llc, (vin.C1.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f);
    float wm_g = wm.y;
    if (wm_g < 0.20000000298023223876953125f)
    {
        discard;
    }

    float2 p = vin.C1.xz * 1.5f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;

    if (clamp(1.0f - smoothstep(0.60000002384185791015625f, 0.64999997615814208984375f,
                wm_g + ((noise * noise) * 0.300000011920928955078125f)), 0.0f, 1.0f) < 0.800000011920928955078125f)
    {
        discard;
    }
}
#endif
