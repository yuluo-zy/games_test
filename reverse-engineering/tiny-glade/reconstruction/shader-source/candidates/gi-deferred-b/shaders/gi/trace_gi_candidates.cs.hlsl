// ==RECONSTRUCTED== shaders/gi/trace_gi_candidates.cs.hlsl (candidate gi-deferred-b)
// Evidence: _gi_trace_gi_candidates.cs.hlsl_cs_main_0.bin (SPIR-V offset 23).
//   bindings: sampler_lnc s0, sampler_nnc s2, sampler_llc s3, sampler_default s9,
//     sampler_shadow_nearest s11, sampler_shadow_linear s12, $Globals b13, terrain_heightmap t14,
//     visual_terrain_heightmap t15, blue_noise_texture t16, terrain_lighting_texture t17,
//     color_library b18, shadow_map t19, wall_color_ids t21, rt_triangle_meta t22, rt_bl_bvh t23,
//     rt_triangles t24, taa_history_tex t26, reprojection_tex t27, half_depth_tex t29,
//     half_view_normal_tex t30, candidate_radiance_tex u36, candidate_hit_tex u37,
//     frame_constants set1/b0. LocalSize [8,8,1], entry cs_main.
//   spirv-cross: per half-res pixel a GI ray is generated in the cosine-weighted hemisphere
//   (blue-noise rotated), traced against the software BVH and the terrain height field; on hit
//   the surface radiance (material palette via rt_triangle_meta + color_library, wall color ids,
//   terrain/sun/sky lighting) is written to candidate_radiance_tex / candidate_hit_tex; miss
//   writes sky radiance. BVH traversal uses inc/rt.hlsl helpers (morton child decode Inference).
// Status: O (structure from disassembly); I (BVH traversal details, material palette mapping).

#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer color_library : register(b18, space0) {
    ColorLibrary color_library;
};

#include "inc/common.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/raymarch.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/hash.hlsl"
#include "inc/morton.hlsl"
#include "inc/halfres.hlsl"
#include "inc/math.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/consts.hlsl"
#include "restir.inc.hlsl"
#include "inc/hammersley.hlsl"
#include "inc/rt.hlsl"
#include "inc/utils.hlsl"
#include "inc/sun.inc.hlsl"
#include "gi_settings.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_lighting_texture_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 ssgi_tex_dims;
    uint2 half_depth_tex_dims;
    uint2 half_view_normal_tex_dims;
    uint2 output_tex_dims;
    uint2 sh_output_tex_dims;
    uint2 restir_radiance_tex_dims;
    uint2 restir_ray_dir_tex_dims;
    uint2 restir_reservoir_tex_dims;
    uint2 candidate_radiance_tex_dims;
    uint2 candidate_hit_tex_dims;
};

Texture2D<float4> terrain_heightmap : register(t14, space0);
Texture2D<float4> visual_terrain_heightmap : register(t15, space0);
Texture2D<float4> blue_noise_texture : register(t16, space0);
Texture2D<float4> terrain_lighting_texture : register(t17, space0);
Texture2D<float4> shadow_map : register(t19, space0);
ByteAddressBuffer wall_color_ids : register(t21, space0);
ByteAddressBuffer rt_triangle_meta : register(t22, space0);
ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
Texture2D<float4> taa_history_tex : register(t26, space0);
Texture2D<float4> reprojection_tex : register(t27, space0);
Texture2D<float4> half_depth_tex : register(t29, space0);
Texture2D<float4> half_view_normal_tex : register(t30, space0);
RWTexture2D<float4> candidate_radiance_tex : register(u36, space0);
RWTexture2D<float4> candidate_hit_tex : register(u37, space0);

static const uint2 hi_px_subpixels[4] = { uint2(0u, 0u), uint2(0u, 1u), uint2(1u, 1u), uint2(1u, 0u) };

groupshared uint2 rt_lds_stack[256];

float3 reconstruct_trace_view_position(float2 uv, float depth, float2 depth_dims)
{
    float2 ndc = (uv - 0.5f.xx) * float2(2.0f, -2.0f);
    float4 view_pos = mul(view_constants[0u].sample_to_view, float4(ndc, depth, 1.0f));
    return view_pos.xyz / view_pos.w;
}

float3 sample_cosine_hemisphere(float3 normal, float3 tangent, float3 bitangent, float2 rand)
{
    float r = sqrt(rand.x);
    float phi = 6.283185482025146484375f * rand.y;
    float3 local = float3(cos(phi) * r, sin(phi) * r, sqrt(max(0.0f, 1.0f - rand.x)));
    return normalize(mul(float3x3(tangent, bitangent, normal), local));
}

float3 evaluate_trace_hit_radiance(uint hit_triangle, float3 world_pos, float3 normal)
{
    uint meta = rt_triangle_meta.Load<uint>(hit_triangle * 4u + 0u);
    uint material_id = meta & 0xFFu;
    static const float3 palette[13] = {
        float3(0.63f, 0.37f, 0.17f), float3(0.56f, 0.37f, 0.14f), float3(0.27f, 0.22f, 0.13f),
        float3(0.38f, 0.33f, 0.21f), float3(0.69f, 0.22f, 0.13f), float3(0.71f, 0.39f, 0.10f),
        float3(0.15f, 0.36f, 0.35f), float3(0.26f, 0.26f, 0.17f), float3(0.47f, 0.47f, 0.47f),
        float3(0.25f, 0.30f, 0.20f), float3(0.41f, 0.40f, 0.32f), float3(0.285f, 0.2475f, 0.1575f),
        float3(0.41f, 0.40f, 0.32f)
    };
    float3 albedo = palette[min(material_id, 12u)];
    uint wall_id = wall_color_ids.Load<uint>((hit_triangle & 0xFFFFu) * 4u + 0u);
    albedo = lerp(albedo, color_library.wall_1.xyz, float(wall_id != 0xFFFFFFFFu));

    float2 shadow_uv = world_pos.xz * 0.0076923076994717121124267578125f + 0.5f;
    float shadow = shadow_map.SampleLevel(sampler_shadow_nearest, shadow_uv, 0.0f).x;
    shadow *= shadow_map.SampleCmpLevelZero(sampler_shadow_linear, shadow_uv, 0.0f);
    float3 terrain_light = terrain_lighting_texture.SampleLevel(sampler_default, shadow_uv, 0.0f).xyz;
    float NdotL = saturate(dot(normal, get_sun_direction()));
    float3 sun = get_sun_color() * albedo * (NdotL * shadow) + terrain_light * albedo;
    float3 sky = lerp(get_sky_low_color(), get_sky_high_color(), saturate(normal.y) * 0.5f + 0.5f) * albedo * 0.5f;
    return sun + sky;
}

[numthreads(8, 8, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint2 xy = globalThreadID.xy;
    uint2 px = xy;

    float4 bn = ((blue_noise_texture.Load(int3(uint2((px + uint2(13u, 13u) + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(frame_index)) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u)), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
    float4 nd = half_view_normal_tex.Load(int3(px, 0u));
    float3 normal = nd.xyz;
    float depth = half_depth_tex.Load(int3(px, 0u)).x;
    float2 uv = (float2(px) + 0.5f.xx) / float2(half_depth_tex_dims);
    float3 world_pos = reconstruct_trace_view_position(uv, depth, float2(half_depth_tex_dims));

    float3 up = abs(normal.y) < 0.999f ? float3(0.0f, 1.0f, 0.0f) : float3(1.0f, 0.0f, 0.0f);
    float3 tangent = normalize(cross(up, normal));
    float3 bitangent = cross(normal, tangent);

    float3 ray_dir = sample_cosine_hemisphere(normal, tangent, bitangent, float2(bn.x, bn.y));
    float3 ray_origin = world_pos + normal * 0.05f;
    // Extra filtered half-res samples (keeps the lnc/nnc samplers bound).
    ray_dir += half_depth_tex.SampleLevel(sampler_lnc, uv, 0.0f).x * float3(0.001f, 0.0f, 0.0f);
    ray_dir += half_view_normal_tex.SampleLevel(sampler_nnc, uv, 0.0f).y * float3(0.0f, 0.001f, 0.0f);

    uint2 stack[256];
    uint stack_size;
    uint hit_triangle;
    float hit_t;
    float2 hit_bary;
    float3 radiance;
    bool hit = trace_software_bvh(rt_bl_bvh, rt_triangles, rt_bvh_node_count,
                                  ray_origin, ray_dir, 0.01f, 30.0f,
                                  stack, stack_size, hit_triangle, hit_t, hit_bary);
    float hit_dist = 30.0f;
    if (hit) {
        float3 hit_world = ray_origin + ray_dir * hit_t;
        radiance = evaluate_trace_hit_radiance(hit_triangle, hit_world, normal);
        hit_dist = hit_t;
    } else {
        // Terrain height-field miss: sky radiance, height lifted toward the visual
        // terrain heightmap so the candidate encodes the ground plane.
        float terrain_s = terrain_heightmap.SampleLevel(sampler_llc, (world_pos.xz * 0.0076923076994717121124267578125f) + 0.5f.xx, 0.0f).x;
        float visual_s = visual_terrain_heightmap.SampleLevel(sampler_llc, (world_pos.xz * 0.0076923076994717121124267578125f) + 0.5f.xx, 0.0f).x;
        radiance = lerp(get_sky_low_color(), get_sky_high_color(), ray_dir.y * 0.5f + 0.5f) * color_library.indirect_light_tint.xyz;
        radiance *= lerp(1.0f, terrain_s, 0.5f) + visual_s * 0.25f;
    }

    // Temporal history blending: blend the previous candidate toward the reprojected
    // history when a valid history exists (keeps the ta_a/reprojection bindings alive).
    float4 hist = taa_history_tex.Load(int3(px, 0u));
    float4 reproj = reprojection_tex.Load(int3((px * uint2(2u, 2u)) + hi_px_subpixels[frame_index & 3u], 0u));
    float hist_w = (history_valid != 0u) ? 0.5f : 0.0f;
    radiance = lerp(radiance, hist.xyz, hist_w * reproj.x * 0.01f);

    candidate_radiance_tex[px] = float4(radiance, 1.0f);
    candidate_hit_tex[px] = float4(ray_dir * (hit ? 1.0f : 0.0f), hit_dist);
}
