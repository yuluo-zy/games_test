// ==SKELETON== -> ==RECONSTRUCTED== (water-ice-a)
// SOURCE PATH : shaders/water_x_paths.raster.hlsl
// KIND        : raster_main
// COMPILED AS MAIN IN (2 bins):
//   _water_x_paths.raster.hlsl_b903e43ffb3da915.bin   (vs paths-in-water lift; ps blends rocky/theme/path)
//   _water_x_paths.raster.hlsl_f0adff76827e73ff.bin   (vs identical; ps empty)
// entries: vs:vs_main  ps:ps_main
// Status: O (bindings/entry; vs + ps from spirv-cross)
// Evidence: spirv-cross vs (water-mask bounds-checked lift of path/building masks,
//   NaN-dip outside water; C0=(0.13,0.13,0.10), C1=normal, C2=world, C3=prim_center)
//   and ps (b903e: rocky_terrain_texture triplanar with hash jitter, theme-colored
//   blend, explicit-arithmetic perlin dry/water mask, path_mask tint for theme 4,
//   RG8/RGB8 pack; f0adff: empty). index_pass W2.
// Note: two 16-hex variants are a compile-time feature variant — b903e = colored
//   x-paths, f0adff = no-output. Gated on WATER_X_PATHS_COLOR (I).

#ifndef WATER_X_PATHS_COLOR
#define WATER_X_PATHS_COLOR 1
#endif

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "water/water_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/math.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/texture_blend.hlsl"
#include "inc/perlin.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 water_mask_dims;
    uint2 path_mask_dims;
    uint2 buildings_mask_dims;
    uint2 rocky_terrain_texture_dims;
    int bool_render_shadows;
};

Texture2D<float4> water_mask : register(t14, space0);
Texture2D<float4> path_mask : register(t15, space0);
Texture2D<float4> buildings_mask : register(t16, space0);

struct VsIn
{
    float3 Vertex_Position : TEXCOORD0;
    float3 Vertex_Normal : TEXCOORD1;
    float3 prim_center : TEXCOORD2;
};

struct VsOut
{
    float3 C0 : TEXCOORD0;
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float4 position : SV_Position;
};

VsOut vs_main(VsIn vin)
{
    VsOut o;
    float4 pos = float4(vin.Vertex_Position, 1.0f);
    float2 prim_uv = (vin.prim_center.xz * 0.0076923076994717121124267578125f) + 0.5f;
    float2 pos_uv = (vin.Vertex_Position.xz * 0.0076923076994717121124267578125f) + 0.5f;

    float water;
    bool out_x = (prim_uv.x < 0.0f) || (prim_uv.x > 1.0f);
    bool out_xy = out_x || (prim_uv.y < 0.0f);
    bool out_xyx = out_xy || (prim_uv.y > 1.0f);
    if (out_xyx)
    {
        water = 0.0f;
    }
    else
    {
        water = water_mask.SampleLevel(sampler_default, prim_uv, 0.0f).y;
    }

    if (water > 0.20000000298023223876953125f)
    {
        uint h = uint((vin.prim_center.x + vin.prim_center.z) + 100.0f);
        h += (h << 10u);
        h ^= (h >> 6u);
        h += (h << 3u);
        h ^= (h >> 11u);
        float jitter = asfloat(((h + (h << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
        pos.y = (vin.Vertex_Position.y + (-3.0f))
            + (pow(path_mask.SampleLevel(sampler_llb, prim_uv, 0.0f).x, 0.20000000298023223876953125f) * 1.5f)
            + ((1.5f * pow(path_mask.SampleLevel(sampler_llb, pos_uv, 0.0f).x, 0.20000000298023223876953125f))
                * (1.2000000476837158203125f + lerp(-0.100000001490116119384765625f, 0.300000011920928955078125f, jitter)))
            - smoothstep(0.3499999940395355224609375f, 0.60000002384185791015625f,
                buildings_mask.SampleLevel(sampler_default, pos_uv, 0.0f).x);
    }
    else
    {
        pos = float4(asfloat(0x7fbfffffu), asfloat(0x7fbfffffu), asfloat(0x7fbfffffu), pos.w);
    }

    o.position = mul(view_constants[current_view_index].world_to_sample, pos);
    o.C0 = float3(0.12999999523162841796875f, 0.12999999523162841796875f, 0.100000001490116119384765625f);
    o.C1 = vin.Vertex_Normal;
    o.C2 = pos.xyz;
    o.C3 = vin.prim_center;
    return o;
}

#if WATER_X_PATHS_COLOR
struct PsIn
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
};

Texture2D<float4> rocky_terrain_texture : register(t17, space0);

uint2 ps_main(PsIn vin) : SV_Target0
{
    float3 n = normalize(vin.C1);
    float3 world_frac = vin.C2 * 0.0333333350718021392822265625f;

    uint h = uint(1000.0f * (abs(vin.C3.x) + abs(vin.C3.z)));
    h += (h << 10u);
    h ^= (h >> 6u);
    h += (h << 3u);
    h ^= (h >> 11u);
    float jitter = asfloat(((h + (h << 15u)) & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
    float2 j2 = (jitter * 0.20000000298023223876953125f).xx;

    float3 rocky_zy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.zy * 2.0f) + j2).xyz;
    float3 rocky_xy = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xy * 2.0f) + j2).xyz;
    float3 rocky_xz = rocky_terrain_texture.Sample(sampler_mipmap, (world_frac.xz * 2.0f) + j2).xyz;

    float3 pa = pow(abs(n), 1.0f);
    float l_xy = dot(rocky_xy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
    float l_zy = dot(rocky_zy, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f)) + pa.x;
    float l_a = l_xy + pa.z;
    float l_b = l_xy + pa.y;
    float lm = max(max(l_zy, l_a), l_b) - 0.20000000298023223876953125f;
    float w_zy = max(l_zy - lm, 0.0f);
    float w_a = max(l_a - lm, 0.0f);
    float w_b = max(l_b - lm, 0.0f);

    bool theme_dark = (glade_theme_id == 2u) || (glade_theme_id == 4u);
    float3 theme_col = theme_dark
        ? float3(0.0625f, 0.0399999953806400299072265625f, 0.005624999292194843292236328125f)
        : float3(0.0419999994337558746337890625f, 0.070000000298023223876953125f, 0.02099999971687793731689453125f);

    // 2D value-noise (explicit-arithmetic form; see water_snow_plane note).
    float2 p = (((vin.C2.xz + vin.C2.xy) + (vin.C2.yz * 0.5f)) + (jitter * 200.0f)) * 0.85000002384185791015625f;
    int2 cell = int2(floor(p));
    float2 f = frac(p);
    float2 u = f * f * (3.0f - 2.0f * f);
    float n00 = incb_perlin_hash01(hash(uint2(cell + int2(0, 0))));
    float n10 = incb_perlin_hash01(hash(uint2(cell + int2(1, 0))));
    float n01 = incb_perlin_hash01(hash(uint2(cell + int2(0, 1))));
    float n11 = incb_perlin_hash01(hash(uint2(cell + int2(1, 1))));
    float noise = (1.0f - u.y) * ((n00 * (1.0f - u.x)) + (n10 * u.x))
                + (n01 * (1.0f - u.x)) * u.y + (n11 * u.x) * u.y;
    float dry = smoothstep(0.5f, 1.0f, noise);

    float3 rock_blend = ((((rocky_zy * w_zy) + (rocky_xy * w_a)) + (rocky_xz * w_b))
        / (w_zy + w_a + w_b)) * 1.10000002384185791015625f;
    float3 col = lerp(lerp(rock_blend, theme_col,
                ((1.0f - smoothstep(-0.20000000298023223876953125f, 0.60000002384185791015625f - dry, vin.C2.y)) * 0.75f)),
            theme_col, clamp(pa.x + pa.z, 0.0f, 1.0f) * dry);

    if (glade_theme_id == 4u && vin.C2.y > 0.0f)
    {
        float pm = path_mask.SampleLevel(sampler_llb, (vin.C2.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x;
        float pm2 = pm * pm;
        col = lerp(col, 0.699999988079071044921875f,
            (smoothstep(0.20000000298023223876953125f, 1.0f,
                clamp((n.y * (pm2 * pm2)) * lerp(0.5f, 1.0f, dry), 0.0f, 1.0f)) + 0.100000001490116119384765625f));
    }

    float3 n_pack = (n * 0.5f) + 0.5f;
    float3 c_pack = sqrt(clamp(col, 0.0f, 1.0f));
    uint albedo = (uint((clamp(c_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
                + (uint((clamp(c_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
                + (uint((clamp(c_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u);
    uint2 out_px = uint2(
        ((uint((clamp(n_pack.x, 0.0f, 1.0f) * 255.0f) + 0.5f)
        + (uint((clamp(n_pack.y, 0.0f, 1.0f) * 255.0f) + 0.5f) << 8u))
        + (uint((clamp(n_pack.z, 0.0f, 1.0f) * 255.0f) + 0.5f) << 16u)) + 3422552064u,
        albedo);
    out_px.y = albedo | 117440512u;
    return out_px;
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
