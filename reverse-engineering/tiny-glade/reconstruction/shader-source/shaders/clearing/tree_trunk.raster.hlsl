// ==RECONSTRUCTED== shaders/clearing/tree_trunk.raster.hlsl
// Evidence: _clearing_tree_trunk.raster.hlsl_b903e43ffb3da915.bin / _f0adff76827e73ff.bin.
//   bindings (vs): $Globals b13 (current_view_index/rocky_terrain_dims/wind_strength/
//   is_firstperson_mode/max_cone_cull_dist_range/bark_*_dims), nani_instance_data t16,
//   instance_indirection_buffer t17, frame_constants. (ps): sampler_llr s1, sampler_default s9,
//   rocky_terrain t15, bark_color t18, bark_color_naked t19, bark_lines t20, frame_constants.
//   vs_main: placement transform with trunk sway (two stacked sine bends + vertical wind),
//   growth-age blend from anim_t_packed. ps_main: first-person cone-cull, bark color/naked
//   variant blend, bark-lines detail, packed normal+albedo with a bark flags byte.
//   Status: O (structure verbatim).

#include "inc/common.hlsl"
#include "inc/current_view.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"

// color_library.hlsl guard-skipped (not bound by this shader).
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/color_library.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/pack_unpack.hlsl"

Texture2D<float4> rocky_terrain : register(t15, space0);
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/math.hlsl"
#include "inc/perlin.hlsl"
#include "clearing/inc/instanced_placement.hlsl"
#include "clearing/inc/canopy_common.inc.hlsl"
#include "nani/inc/user.hlsl"
#include "inc/material/solid_material.inc.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
// NOTE: inc/lighting.hlsl omitted (nested color_library reference; see billboard_forest note).
#include "inc/sun.inc.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/sh.hlsl"
#include "inc/texture_blend.hlsl"

// $Globals members (implicit cbuffer at set0/b13).
uint2 rocky_terrain_dims;
float wind_strength;
int is_firstperson_mode;
float max_cone_cull_dist_range;
uint2 bark_color_dims;
uint2 bark_color_naked_dims;
uint2 bark_lines_dims;

ByteAddressBuffer nani_instance_data : register(t16, space0);
ByteAddressBuffer instance_indirection_buffer : register(t17, space0);
Texture2D<float4> bark_color : register(t18, space0);
Texture2D<float4> bark_color_naked : register(t19, space0);
Texture2D<float4> bark_lines : register(t20, space0);

struct VS_IN
{
    float3 vertex_position : TEXCOORD0;
    float3 vertex_color : TEXCOORD1;
    float3 vertex_normal : TEXCOORD2;
    float3 appear_pos : TEXCOORD3;
    float age : TEXCOORD4;
};

struct VS_OUT
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    nointerpolation int C6 : TEXCOORD6;
    nointerpolation uint C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
    float4 position : SV_Position;
};

VS_OUT vs_main(VS_IN i, uint instance_id : SV_InstanceID)
{
    VS_OUT o;
    uint indir = instance_indirection_buffer.Load<uint>(instance_id * 4u + 0);
    float growth = sqrt(clamp((1.0f - i.age) + lerp(-1.0f, 1.0f, clamp(f16tof32(nani_instance_data.Load<uint>(indir * 48u + 32) & 0xFFFFu), 0.0f, 1.0f)), 0.0f, 1.0f));
    float3 pos = lerp(i.appear_pos, i.vertex_position, growth);

    float wind_amp = pos.y * 0.02f;
    float t_sec = float(game_time_sec);
    float t_frac = game_time_frac;
    float x_amp = 0.2f * pos.z;
    float sway_x = pos.x + ((wind_amp * sin(fmod(t_sec * 0.823529f, 6.2831855f) + (t_frac * 0.823529f) + (0.2f * pos.x)))
        * sin(fmod(t_sec * 0.893617f, 6.2831855f) + (t_frac * 0.893617f) + x_amp));
    float sway_z = pos.z + ((wind_amp * sin(fmod(t_sec * 0.792453f, 6.2831855f) + (t_frac * 0.792453f) + (0.2f * sway_x)))
        * sin(fmod(t_sec * 0.688525f, 6.2831855f) + (t_frac * 0.688525f) + x_amp));

    uint idx = uint(int(indir));
    float rot = radians(nani_instance_data.Load<float>(idx * 48u + 16));
    float3 fwd = normalize(float3(cos(rot), 0.0f, sin(rot)));
    float4x4 basis = transpose(float4x4(float4(fwd, 0.0f),
                                        float4(0.0f, 1.0f, 0.0f, 0.0f),
                                        float4(normalize(cross(fwd, float3(0.0f, 1.0f, 0.0f))), 0.0f),
                                        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float px = nani_instance_data.Load<float3>(idx * 48u + 0).x;
    float py = nani_instance_data.Load<float3>(idx * 48u + 0).y;
    float pz = nani_instance_data.Load<float3>(idx * 48u + 0).z;
    float scale = nani_instance_data.Load<float>(idx * 48u + 12);
    float4 placed = mul(float4x4(float4(1.0f, 0.0f, 0.0f, px),
                                 float4(0.0f, 1.0f, 0.0f, py),
                                 float4(0.0f, 0.0f, 1.0f, pz),
                                 float4(0.0f, 0.0f, 0.0f, 1.0f)),
                        mul(basis, mul(float4x4(float4(scale, 0.0f, 0.0f, 0.0f),
                                               float4(0.0f, scale, 0.0f, 0.0f),
                                               float4(0.0f, 0.0f, scale, 0.0f),
                                               float4(0.0f, 0.0f, 0.0f, 1.0f)), float4(sway_x, pos.y, sway_z, 1.0f))));

    float2 bend = nani_instance_data.Load<float2>(indir * 48u + 36);
    float2 bent = placed.xz + (bend * ((clamp(i.vertex_position.y * 0.1f, 0.0f, 1.0f) * 0.75f) * lerp(0.5f, 1.0f, i.age)));
    o.position = mul(view_constants[current_view_index].world_to_sample, float4(bent.x, placed.y, bent.y, 1.0f));
    o.C1 = i.vertex_color;
    o.C2 = mul(basis, float4(i.vertex_normal, 0.0f)).xyz;
    o.C3 = float4(bent.x, placed.y, bent.y, placed.w).xyz;
    o.C4 = growth;
    o.C5 = f16tof32((nani_instance_data.Load<uint>(indir * 48u + 32) >> 16u) & 0xFFFFu);
    o.C6 = int(uint(nani_instance_data.Load<int>(idx * 48u + 20)));
    o.C7 = uint(nani_instance_data.Load<int>(indir * 48u + 44));
    o.C8 = i.vertex_position.y;
    return o;
}

struct PS_IN
{
    float3 C1 : TEXCOORD1;
    float3 C2 : TEXCOORD2;
    float3 C3 : TEXCOORD3;
    float C4 : TEXCOORD4;
    float C5 : TEXCOORD5;
    nointerpolation uint C7 : TEXCOORD7;
    float C8 : TEXCOORD8;
};

struct PS_OUT
{
    uint2 target : SV_Target0;
};

PS_OUT ps_main(PS_IN i)
{
    PS_OUT o;
    if (i.C4 < 0.3f)
    {
        discard;
    }
    float4 view4 = mul(view_constants[current_view_index].world_to_view, float4(i.C3, 1.0f));
    float3 view_pos = view4.xyz;

    float wind_cone;
    if ((render_overrides & 2u) != 0u)
    {
        wind_cone = i.C5 * smoothstep(0.0f, 6.0f, i.C8);
    }
    else
    {
        wind_cone = 0.0f;
    }
    bool naked = (i.C7 == 3u);
    bool cone_active = (int(wind_cone > 0.0f) == 1) && (is_firstperson_mode == 0);
    if (cone_active && !naked)
    {
        view_pos.x = view4.x * 0.769230782985687255859375f;
        view_pos.y = view4.y * 1.1f;
        float2 cone_range = float2(6.66f, max_cone_cull_dist_range) * view_constants[0].view_to_clip[1].y;
        float dist = -view4.z;
        if (dist < cone_range.y)
        {
            float2 cull_cell1 = i.C3.zy * 3.0f;
            int2 cell1 = int2(floor(cull_cell1));
            float2 frac1 = frac(cull_cell1);
            float2 smooth1 = frac1 * frac1 * (3.0f - (2.0f * frac1));
            float n1 = ((1.0f - smooth1.y) * ((incb_perlin_hash01(hash(uint2(cell1 + int2(0, 0)))) * (1.0f - smooth1.x)) + (incb_perlin_hash01(hash(uint2(cell1 + int2(1, 0)))) * smooth1.x)))
                + ((incb_perlin_hash01(hash(uint2(cell1 + int2(0, 1)))) * (1.0f - smooth1.x)) * smooth1.y)
                + ((incb_perlin_hash01(hash(uint2(cell1 + int2(1, 1)))) * smooth1.x) * smooth1.y);
            float2 cull_cell2 = i.C3.xy * 3.0f;
            int2 cell2 = int2(floor(cull_cell2));
            float2 frac2 = frac(cull_cell2);
            float2 smooth2 = frac2 * frac2 * (3.0f - (2.0f * frac2));
            float n2 = ((1.0f - smooth2.y) * ((incb_perlin_hash01(hash(uint2(cell2 + int2(0, 0)))) * (1.0f - smooth2.x)) + (incb_perlin_hash01(hash(uint2(cell2 + int2(1, 0)))) * smooth2.x)))
                + ((incb_perlin_hash01(hash(uint2(cell2 + int2(0, 1)))) * (1.0f - smooth2.x)) * smooth2.y)
                + ((incb_perlin_hash01(hash(uint2(cell2 + int2(1, 1)))) * smooth2.x) * smooth2.y);
            float3 offset = float3(((n1 * 2.0f) - 1.0f), 0.0f, ((n2 * 2.0f) - 1.0f) * 2.0f);
            float2 jitter = (frac(float2(4.27438831329345703125f, 3.349201679229736328125f)) - 0.5f) * 2.0f;
            float3 jittered = view_pos + float3(offset.x + jitter.x, offset.y, offset.z + jitter.y);
            jittered.z = min(jittered.z, -10.0f);
            float f = abs(view_constants[0].clip_to_view[1].y);
            bool small_f = f < 1.0f;
            float inv_f = small_f ? f : (1.0f / f);
            float inv_f2 = inv_f * inv_f;
            float atan_f = (1.0f + (((-0.301895f) + (0.087293f * inv_f2)) * inv_f2)) * inv_f;
            float half_angle = small_f ? atan_f : (1.570796f - atan_f);
            float angle = (view_constants[0].clip_to_view[1].y < 0.0f) ? (-half_angle) : half_angle;
            if (dot(float3(0.0f, 0.0f, -1.0f), normalize(jittered)) > cos(((max(0.07f, angle) * 1.2f) * smoothstep(cone_range.y, cone_range.x, dist)) * wind_cone))
            {
                discard;
            }
        }
    }

    float3 n = normalize(i.C2);
    float3 trunk_uv = i.C3 * (1.0f / 30.0f);
    float3 bark_a;
    float3 bark_b;
    if (naked)
    {
        bark_a = bark_color_naked.SampleBias(sampler_llr, trunk_uv.xy, global_mip_bias).xyz;
        bark_b = bark_color_naked.SampleBias(sampler_llr, trunk_uv.zy, global_mip_bias).xyz;
    }
    else
    {
        bark_a = bark_color.SampleBias(sampler_llr, trunk_uv.xy, global_mip_bias).xyz;
        bark_b = bark_color.SampleBias(sampler_llr, trunk_uv.zy, global_mip_bias).xyz;
    }
    float3 n_abs = pow(abs(n), 1.0f);
    float lum_a = dot(bark_a, float3(0.2126f, 0.7152f, 0.0722f)) + n_abs.z;
    float lum_b = dot(bark_b, float3(0.2126f, 0.7152f, 0.0722f)) + n_abs.x;
    float min_lum = max(lum_a, lum_b) - 0.2f;
    float wa = max(lum_a - min_lum, 0.0f);
    float wb = max(lum_b - min_lum, 0.0f);
    float3 bark = ((bark_a * wa) + (bark_b * wb)) / (wa + wb);

    float3 tint;
    if (glade_theme_id == 2u)
    {
        tint = float3(0.15f, 0.09f, 0.04f);
    }
    else if (glade_theme_id == 4u)
    {
        tint = lerp(bark, 1.0f, ((1.0f - smoothstep(-0.5f, 4.0f, i.C8)) * 0.3f) * smoothstep(-0.1f, 0.0f, i.C3.y));
    }
    else
    {
        tint = float3(0.06f, 0.1f, 0.03f);
    }

    float2 trunk_uv2 = (i.C3.xz * (1.0f / 130.0f)) + 0.5f;
    bool in_bounds = !((trunk_uv2.x < 0.0f) || (trunk_uv2.x > 1.0f) || (trunk_uv2.y < 0.0f) || (trunk_uv2.y > 1.0f));
    float rocky = in_bounds ? rocky_terrain.SampleLevel(sampler_default, trunk_uv2, 0.0f).x : 0.0f;
    float3 bark_blend = lerp(bark, tint, ((1.0f - smoothstep(-0.1f, 0.4f, i.C1.y)) * clamp(1.0f - rocky, 0.7f, 1.0f)));
    float4 lines = bark_lines.SampleBias(sampler_llr, i.C1.xy * 1.2f, global_mip_bias);
    float3 bark_detail = lerp(bark_blend, lerp(bark_blend, tint, 0.5f) * 0.6f, 1.0f - lines.xyz);
    bool naked_color = naked && (i.C1.x <= (-99.0f));
    float3 bark_final = naked_color ? float3(0.25f, 0.15f, 0.0375f) : bark_detail;
    float3 albedo;
    if (glade_theme_id == 4u)
    {
        albedo = lerp(bark_final, 0.7f, (smoothstep(-0.1f, 0.0f, i.C3.y) * smoothstep(0.2f, 1.0f, clamp(n.y, 0.0f, 1.0f))));
    }
    else
    {
        albedo = bark_final;
    }

    float3 packed_n = (n * 0.5f) + 0.5f;
    float3 color = sqrt(clamp(albedo, 0.0f, 1.0f));
    uint normal_packed = (uint(clamp(packed_n.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(packed_n.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(packed_n.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u)) + 0xE6000000u;
    uint albedo_packed = (uint(clamp(color.x, 0.0f, 1.0f) * 255.0f + 0.5f)
        + (uint(clamp(color.y, 0.0f, 1.0f) * 255.0f + 0.5f) << 8u)
        + (uint(clamp(color.z, 0.0f, 1.0f) * 255.0f + 0.5f) << 16u));
    albedo_packed |= ((7u | (uint(naked_color ? false : true) << 6u)) << 24u);
    o.target = uint2(normal_packed, albedo_packed);
    return o;
}
