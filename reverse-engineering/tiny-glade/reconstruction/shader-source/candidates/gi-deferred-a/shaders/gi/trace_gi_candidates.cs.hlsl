// ==RECONSTRUCTED== shaders/gi/trace_gi_candidates.cs.hlsl | status: O (bindings/entry/LocalSize,
// ray generation + output structure from disassembly); I/U (screen-space march + material eval
// bodies are Inference — see decisions.md)
// Evidence: _gi_trace_gi_candidates.cs.hlsl_cs_main_0.bin (LocalSize 8x8, entry cs_main).
//   bindings: samplers s0/s2/s3/s9/s11/s12, $Globals b13 (21 members), terrain_heightmap t14,
//   visual_terrain_heightmap t15, blue_noise_texture t16, terrain_lighting_texture t17,
//   color_library b18, shadow_map t19, wall_color_ids u21 (ByteAddressBuffer),
//   rt_triangle_meta t22, rt_bl_bvh t23, rt_triangles t24 (ByteAddressBuffers),
//   taa_history_tex t26, reprojection_tex t27, half_depth_tex t29, half_view_normal_tex t30,
//   candidate_radiance_tex u36, candidate_hit_tex u37, frame_constants set1/b0;
//   caps {Shader, RuntimeDescriptorArray}.
//   For each full-res texel: build a cosine-weighted hemisphere ray from the view-space
//   normal via the morton-scrambled pixel + frame golden-angle sample, screen-space march
//   the ray against half_depth, and on a screen hit read radiance from taa_history;
//   otherwise trace the software BVH (rt_bl_bvh/rt_triangles) and evaluate the hit
//   material (terrain heightmap + lighting texture + shadow, or wall color ids from the
//   triangle meta buffer); sky samples write zero radiance. Writes the clamped candidate
//   radiance and candidate hit (ray dir) to the 2x2 full-res texels.
//   NOTE: groupshared rt_lds_stack[256] (uint2) is the BVH traversal stack.
//   Residual: screen march + material eval are simplified (histogram gap); OpSource
//   closure differs (~22 target vs ~19 candidate).

// color_library is redeclared below at b18 (this pass's binding); guard-skip the inc.
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/math_const.hlsl"
#include "inc/hash.hlsl"
#include "inc/halfres.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/reservoir.hlsl"
#include "inc/rt.hlsl"
#include "inc/color/srgb.hlsl"

struct ColorLibrary
{
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
cbuffer color_library : register(b18, space0)
{
    ColorLibrary color_library;
};

#include "inc/sun.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/lighting.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "gi/restir.inc.hlsl"

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
Texture2DArray<float4> shadow_map : register(t19, space0);
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

groupshared uint2 rt_lds_stack[256];

// Tangent frame from a normal (Frisvad-style, matching the original TBN).
void tangent_frame(float3 n, out float3 t, out float3 b)
{
    if (n.z < 0.0f) {
        float r = 1.0f / (1.0f - n.z);
        t = float3((n.x * n.y) * r, ((n.y * n.y) * r) - 1.0f, -n.y);
        b = float3(1.0f - ((n.x * n.x) * r), -(n.x * n.y) * r, n.x);
    } else {
        float r = 1.0f / (1.0f + n.z);
        t = float3(-(n.x * n.y) * r, 1.0f - ((n.y * n.y) * r), -n.y);
        b = float3(1.0f - ((n.x * n.x) * r), (n.x * n.y) * r, -n.x);
    }
}

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint2 px = gl_GlobalInvocationID.xy;
    uint2 full_px = px * 2u;

    float depth = half_depth_tex.Load(int3(full_px, 0)).x;
    // Far / sky samples produce no radiance candidate.
    if (depth > 2000.0f) {
        [unroll]
        for (uint j0 = 0u; j0 < 2u; ++j0) {
            [unroll]
            for (uint i0 = 0u; i0 < 2u; ++i0) {
                uint2 out_px = full_px + uint2(i0, j0);
                candidate_radiance_tex[out_px] = 0.0f;
                candidate_hit_tex[out_px] = float4(0.0f, 1.0f, 0.0f, 0.0f);
            }
        }
        return;
    }

    // World normal of the full-res texel.
    float4 world_normal = mul(view_constants[0].view_to_world,
                              float4(half_view_normal_tex.Load(int3(full_px, 0)).xyz, 0.0f));

    // Morton-scrambled pixel id seeds the golden-angle sample per texel.
    uint scrambled;
    {
        uint a = gl_GlobalInvocationID.y;
        uint b = gl_GlobalInvocationID.x;
        uint acc = 0u;
        for (uint m = 32u; m > 0u; m >>= 1u) {
            uint bx = uint((b & m) > 0u);
            uint by = uint((a & m) > 0u);
            acc += (m * m) * ((3u * bx) ^ by);
            if (by == 0u) {
                if (bx == 1u) {
                    a = 63u - a;
                    b = 63u - b;
                }
            }
        }
        scrambled = acc;
    }

    // Rotating blue-noise dither (used in the material / march path).
    float2 noise_dither = blue_noise_texture.Load(int3((full_px + uint2(37u, 17u)) % uint2(blue_noise_texture_dims), 0)).xy;

    float2 noise = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) *
                         float(scrambled + (288u * (frame_index % 64u)))) + 0.5f);

    // Cosine-weighted hemisphere ray direction on the tangent frame.
    float3 t, b;
    tangent_frame(world_normal.xyz, t, b);
    float cos_theta = 1.0f - noise.x;
    float sin_theta = sqrt(1.0f - (cos_theta * cos_theta));
    float angle = noise.y * 6.283185482025146484375f;
    float3 ray_dir = mul(float3x3(b, t, world_normal.xyz), float3(cos(angle) * sin_theta, sin(angle) * sin_theta, cos_theta));

    // World-space ray origin (reconstructed from the depth at this texel).
    float2 ndc = ((float2(int2(full_px)) + 0.5f) / float2(half_depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float4 view_pos = mul(view_constants[0].sample_to_view, float4(ndc, depth, 1.0f));
    float3 world_pos = mul(view_constants[0].view_to_world, view_pos).xyz;

    // Screen-space march: sample the half-res depth along the projected ray and
    // stop when the ray depth crosses the buffer depth. (Body Inference.)
    float3 ray_origin = world_pos;
    float ray_len = 6.0f;
    float3 hit_pos = ray_origin + ray_dir * ray_len;
    float4 radiance = 0.0f;
    bool screen_hit = false;

    float4 seg_end_clip = mul(view_constants[0].view_to_clip,
                              mul(view_constants[0].world_to_view, float4(hit_pos, 1.0f)));
    if (seg_end_clip.w > 0.0f) {
        float2 seg_end_uv = ((seg_end_clip.xy / max(1e-10f, seg_end_clip.w)) * float2(0.5f, -0.5f)) + 0.5f;
        float2 ray_uv = (ndc * float2(0.5f, -0.5f)) + 0.5f;
        float steps = clamp(length((seg_end_uv - ray_uv) * float2(half_depth_tex_dims)) * 0.5f, 2.0f, 8.0f);
        for (uint s = 1u; s < uint(steps); ++s) {
            float tfrac = float(s) / steps;
            float2 march_uv = lerp(ray_uv, seg_end_uv, tfrac);
            float march_depth = half_depth_tex.SampleLevel(sampler_llc, march_uv, 0.0f).x;
            if ((tfrac * ray_len) < march_depth) {
                screen_hit = true;
                hit_pos = lerp(ray_origin, hit_pos, tfrac);
                break;
            }
        }
    }

    if (screen_hit) {
        // Screen-space hit: use the TAA history radiance at the reprojected location
        // (reprojection_tex carries the previous-frame offset).
        float2 hist_uv = saturate(((hit_pos.xy / max(1e-10f, hit_pos.z)) * float2(0.5f, -0.5f)) + 0.5f);
        float2 reproj_off = reprojection_tex.Load(int3(full_px, 0)).xy;
        float4 history = taa_history_tex.SampleLevel(sampler_nnc, hist_uv + reproj_off, 0.0f);
        radiance = history;
    } else {
        // Software-BVH fallback: trace the ray against the static scene BVH.
        uint2 stack[256];
        uint stack_size;
        uint hit_tri;
        float hit_t;
        float2 hit_bary;
        bool hit = trace_software_bvh(rt_bl_bvh, rt_triangles, rt_bvh_node_count,
                                      ray_origin, ray_dir, 0.001f, 2000.0f,
                                      stack, stack_size, hit_tri, hit_t, hit_bary);
        if (hit) {
            float3 hit_world = ray_origin + ray_dir * hit_t;
            uint tri_meta = rt_triangle_meta.Load<uint>(hit_tri * 4u + 0u);
            float2 terrain_uv = (hit_world.xz * 0.0076923076994717121124267578125f) + 0.5f;
            float terrain_h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, terrain_uv, 0.0f).x);
            float visual_h = lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, terrain_uv, 0.0f).x);
            if ((terrain_h + 0.1f) > visual_h) {
                // Terrain surface: baked terrain lighting + shadow.
                float3 lit = terrain_lighting_texture.SampleLevel(sampler_lnc, terrain_uv, 0.0f).xyz;
                float shadow = shadow_map.SampleCmpLevelZero(sampler_shadow_linear,
                    float3(frac(terrain_uv), 0u), 0.5f);
                shadow = min(shadow, shadow_map.SampleLevel(sampler_shadow_nearest, float3(frac(terrain_uv), 0u), 0.0f).x);
                radiance = float4(lit * max(shadow, 0.001f) * (0.9f + 0.2f * noise_dither.x), hit_t);
            } else {
                // Geometry: wall-color-id driven base color (palette tint via color_library).
                uint color_id = (tri_meta & 16777215u) * 4u;
                uint wall_id = wall_color_ids.Load<uint>(color_id);
                float3 base = color_library.indirect_light_tint.xyz * 0.5f;
                radiance = float4(base * (0.2f + 0.8f * frac(float(wall_id) * 0.61803f)), hit_t);
            }
        } else {
            // Sky / no hit.
            radiance = float4(0.0f, 0.0f, 0.0f, 0.0f);
        }
    }

    // Clamp the radiance magnitude and write the candidate (2x2 full-res texels).
    float rad_sum = max(radiance.x, max(radiance.y, radiance.z));
    float4 clamped = float4(radiance.xyz * (min(rad_sum, 10.0f) / max(1e-20f, rad_sum)), radiance.w);

    [unroll]
    for (uint j = 0u; j < 2u; ++j) {
        [unroll]
        for (uint i = 0u; i < 2u; ++i) {
            uint2 out_px = full_px + uint2(i, j);
            candidate_radiance_tex[out_px] = clamped;
            candidate_hit_tex[out_px] = float4(ray_dir, 0.0f);
        }
    }
}
