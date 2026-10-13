// PATCHED: modules rt.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.
#if !((defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)))
// ==RECONSTRUCTED== shaders/water/ice_deferred_rt.cs.hlsl
// SOURCE PATH : shaders/water/ice_deferred_rt.cs.hlsl
// KIND        : compute_main
// COMPILED AS MAIN IN (1 bins): _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin
//   entry cs:cs_main | local_size [64,1,1] | index_pass W45 (water/ice_surface_pass)
// Status: O (bindings/entry/LocalSize/caps; body transcribed from spirv-cross)
// Evidence: spirv-cross — ice deferred software ray tracing: one ray per packed
//   ray-buffer entry, morton-coded software-BVH traversal over rt_bl_bvh/
//   rt_triangles (8-entry stack), water-surface/terrain ray march, single-bounce
//   structure (bounce effectively dead: first segment always terminates), then
//   refl_hit/refl_hit_radiance writes with history/reprojection fallback.
// MERGE (water-ice review water/ice_deferred_rt.cs.hlsl): A base adopted wholesale;
//   same hybrid traversal fix as deferred_rt (sign-selected corner bytes + combined
//   count-byte accumulation), the blue-noise scroll `(frame_index+1234567u)+
//   (bounce*137u)`, and the post-bounce miss sky branch (cur_dir.y>0 / theme==2).
// NOTE: inc/rt.hlsl provides PackedBlBvhNode/PackedTriangle and now the same morton
//   traversal (review bvh_traversal_finding); it is still inlined here as in the bins.

#include "inc/common.hlsl"
#include "inc/samplers.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/pack_unpack.hlsl"
// inc/blue_noise.hlsl no longer declares the texture (per-review register fix);
// the water/ice RT family binds blue_noise_texture at t15 — declared locally.
#include "inc/hammersley.hlsl"
#include "inc/math_const.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/rt.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/color_library.hlsl"
#include "inc/color/srgb.hlsl"
#include "gi/gi_settings.inc.hlsl"
#include "inc/shadow.hlsl"
#include "inc/cascades.hlsl"
// terrain_heightmap.inc omitted: reconstructed inc references a consumer-declared
// terrain_heightmap texture this shader never binds (original embedded it, no binding).
#include "inc/material_eval.inc.hlsl"
#include "inc/brdf.hlsl"
#include "inc/lighting.hlsl"
#include "inc/sh.hlsl"
#include "inc/lighting_indirect.hlsl"
#include "inc/material/eval/solid_material.inc.hlsl"
#include "inc/brdf_lut.hlsl"
#include "inc/highlight_color.hlsl"
#include "inc/wall_color_ids.hlsl"
#include "inc/history_valid.hlsl"
#include "ice_shared.inc.hlsl"

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims;
    uint2 blue_noise_texture_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 brdf_fg_lut_dims;
    uint rt_bvh_node_count;
    uint rt_triangle_count;
    uint history_valid;
    uint2 half_depth_tex_dims;
    uint2 taa_history_tex_dims;
    uint2 reprojection_tex_dims;
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
};

ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer ice_rt_count_buffer : register(t25, space0);
ByteAddressBuffer ice_rt_ray_buffer : register(t26, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> refl_hit_tex : register(u30, space0);
[[vk::image_format("r32ui")]] RWTexture2D<uint> refl_hit_radiance_tex : register(u31, space0);

// Sample the blue-noise texture at a pixel offset (0.7548777/0.5698403 golden-ratio
// scroll + 1/256) returning half-incremented [0,1] values.
float4 sample_blue_noise(uint2 pixel, uint scroll)
{
    float2 offset = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(scroll)) + 0.5f);
    return ((blue_noise_texture.Load(int3((pixel + uint2(offset * 256.0f)) % uint2(256u, 256u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
}

// Result of the software-BVH ray trace + water/terrain march.
struct RayResult
{
    uint type;        // 0 none, 1 ground, 2 water, 3 triangle
    float t;
    float3 normal;
};

// Morton-ordered software-BVH traversal (identical hybrid to deferred_rt.cs.hlsl:
// A count-byte mask accumulation + node-entry/leaf assembly + break/pop control
// flow; B octant sign-selected corner bytes; shared morton/octant/slab scaffold).
RayResult trace_bl_bvh(ByteAddressBuffer rt_bl_bvh_, ByteAddressBuffer rt_triangles_,
                       float3 ro, float3 rd)
{
    RayResult res;
    res.type = 0u;
    res.t = 100000.0f;
    res.normal = float3(0.0f, 1.0f, 0.0f);

    uint2 stack[8];
    uint2 trav = uint2(0u, 0x80000000u);
    float3 best_normal = 0.0f;
    uint stack_depth = 0u;
    int best_triangle = -1;
    float best_t = 3.4028234663852885981170418348452e+38f;

    for (uint iter = 0u; iter < 1024u; ++iter)
    {
        uint2 node_entry;
        uint2 child;
        int push_depth;
        if ((trav.y & 0xFF000000u) != 0u)
        {
            uint b = firstbithigh(trav.y);
            uint key = trav.y & ~(1u << (b & 31u));
            uint2 kept = trav;
            kept.y = key;
            int nd;
            if ((key & 0xFF000000u) != 0u)
            {
                stack[stack_depth] = kept;
                nd = int(stack_depth) + 1;
            }
            else
            {
                nd = int(stack_depth);
            }

            uint octant = (7u - uint((((rd.x < 0.0f) ? 4u : 0u) | ((rd.y < 0.0f) ? 2u : 0u)) | (rd.z < 0.0f ? 1u : 0u))) * 16843009u;

            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh_.Load<uint4>(i * 16u + (trav.x + countbits(trav.y & (~(0xFFFFFFFFu << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80u + 0u);
            }

            float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / rd.x,
                                       asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / rd.y,
                                       asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / rd.z);
            float3 offset = (asfloat(n.data[0].xyz) - ro) / rd;

            uint accum = 0u;

            // Child A (data[1].z): lane-mask formula + sign-selected corner bytes.
            uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count0 = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = (rd.x < 0.0f) ? n.data[2].z : n.data[2].x;
            uint c1 = (rd.x < 0.0f) ? n.data[2].x : n.data[2].z;
            uint c2 = (rd.y < 0.0f) ? n.data[3].z : n.data[3].x;
            uint c3 = (rd.y < 0.0f) ? n.data[3].x : n.data[3].z;
            uint c4 = (rd.z < 0.0f) ? n.data[4].z : n.data[4].x;
            uint c5 = (rd.z < 0.0f) ? n.data[4].x : n.data[4].z;
            [unroll]
            for (uint s0 = 0u; s0 < 32u; s0 += 8u)
            {
                float3 lo = (float3(float((c0 >> s0) & 255u), float((c2 >> s0) & 255u), float((c4 >> s0) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((c1 >> s0) & 255u), float((c3 >> s0) & 255u), float((c5 >> s0) & 255u)) * inv_dir) + offset;
                bool hit0 = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit0)
                {
                    accum |= (((count0 >> s0) & 255u) << ((xor0 >> s0) & 31u));
                }
            }

            // Child B (data[1].w): same, with sign-selected data[2].w/.y etc.
            uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count1 = (n.data[1].w >> 5u) & 117901063u;
            uint d0 = (rd.x < 0.0f) ? n.data[2].w : n.data[2].y;
            uint d1 = (rd.x < 0.0f) ? n.data[2].y : n.data[2].w;
            uint d2 = (rd.y < 0.0f) ? n.data[3].w : n.data[3].y;
            uint d3 = (rd.y < 0.0f) ? n.data[3].y : n.data[3].w;
            uint d4 = (rd.z < 0.0f) ? n.data[4].w : n.data[4].y;
            uint d5 = (rd.z < 0.0f) ? n.data[4].y : n.data[4].w;
            [unroll]
            for (uint s1 = 0u; s1 < 32u; s1 += 8u)
            {
                float3 lo = (float3(float((d0 >> s1) & 255u), float((d2 >> s1) & 255u), float((d4 >> s1) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((d1 >> s1) & 255u), float((d3 >> s1) & 255u), float((d5 >> s1) & 255u)) * inv_dir) + offset;
                bool hit1 = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit1)
                {
                    accum |= (((count1 >> s1) & 255u) << ((xor1 >> s1) & 31u));
                }
            }

            // Node entry (data[1].x, accumulated slot count) + triangle leaf mask.
            node_entry = uint2(n.data[1].x, key);
            node_entry.y = (accum & 0xFF000000u) | ((n.data[0].w >> 24u) & 255u);
            child = uint2(n.data[1].y, accum & 0x00FFFFFFu);
            push_depth = nd;
        }
        else
        {
            node_entry = uint2(0u, 0u);
            child = trav;
            push_depth = int(stack_depth);
        }

        // Process the triangle list of `child` (node index, packed triangle mask).
        uint2 tri_state;
        tri_state = child;
        float3 tri_normal = best_normal;
        float tri_best = best_t;
        int tri_hit = best_triangle;
        while (tri_state.y != 0u)
        {
            uint tb = firstbithigh(tri_state.y);
            tri_state.y = tri_state.y & ~(1u << (tb & 31u));
            uint tindex = tri_state.x + tb;
            PackedTriangle tri;
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.v[j] = rt_triangles_.Load<float>(j * 4u + tindex * 24u + 0u);
            }
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.e[j] = rt_triangles_.Load<uint>(j * 4u + tindex * 24u + 12u);
            }
            float3 edge_a = float3(unpack_half2x16((tri.e[0] >> 16u) & 65535u).x,
                                   unpack_half2x16((tri.e[1] >> 16u) & 65535u).x,
                                   unpack_half2x16((tri.e[2] >> 16u) & 65535u).x);
            float3 edge_b = float3(unpack_half2x16(tri.e[0] & 65535u).x,
                                   unpack_half2x16(tri.e[1] & 65535u).x,
                                   unpack_half2x16(tri.e[2] & 65535u).x);
            float3 p = cross(rd, edge_b);
            float det = dot(edge_a, p);
            float3 s = ro - float3(tri.v[0], tri.v[1], tri.v[2]);
            float3 q = cross(s, edge_a);
            float t = dot(edge_b, q) / det;
            if (t >= 0.0f && t < tri_best)
            {
                float2 bary = float2(dot(s, p), dot(rd, q)) / det;
                float3 w = float3(bary.x, bary.y, (1.0f - bary.x) - bary.y);
                if (all(w >= -9.9999997473787516355514526367188e-05f))
                {
                    tri_best = t;
                    tri_normal = cross(edge_a, edge_b);
                    tri_hit = int(tindex);
                }
            }
        }
        best_normal = tri_normal;
        best_t = tri_best;
        best_triangle = tri_hit;

        if ((node_entry.y & 0xFF000000u) == 0u)
        {
            if (push_depth == 0)
            {
                break;
            }
            int pd = push_depth - 1;
            trav = stack[pd];
            stack_depth = uint(pd);
        }
        else
        {
            trav = node_entry;
            stack_depth = uint(push_depth);
        }
    }

    if (best_triangle != -1 && best_t < 100000.0f)
    {
        res.type = 3u;
        res.t = best_t;
        res.normal = best_normal;
    }
    return res;
}

// March `dir` against the visual-terrain heightmap slab [-65,65]x[0,24]x[-65,65].
// Returns type 1 (ground) / 2 (water) hit inside [0, max_t], else type 0.
RayResult water_terrain_march(float3 origin, float3 dir, float max_t)
{
    RayResult res;
    res.type = 0u;
    res.t = max_t;
    res.normal = float3(0.0f, 1.0f, 0.0f);

    float3 bb_lo = (float3(-65.0f, 0.0f, -65.0f) - origin) / dir;
    float3 bb_hi = (float3(65.0f, 24.0f, 65.0f) - origin) / dir;
    float3 tmin = min(bb_lo, bb_hi);
    float3 tmax = max(bb_lo, bb_hi);
    float t_near = max(max(tmin.x, tmin.y), tmin.z);
    float t_far = min(min(tmax.x, tmax.y), tmax.z);
    if (!(t_near <= t_far && t_far >= 0.0f))
    {
        return res;
    }

    float t_lo = max(t_near, 0.0f);
    float step = (t_far - t_lo) * 0.03125f;
    float t = t_lo;
    uint step_idx = 0u;
    bool found = false;
    for (;;)
    {
        if (step_idx < 32u)
        {
            float t_next = t + step;
            float3 p = origin + (dir * t_next);
            if (p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc,
                    (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x))
            {
                found = true;
                break;
            }
            ++step_idx;
            t = t_next;
            continue;
        }
        else
        {
            break;
        }
    }
    if (!(found && t < res.t && t < t_far))
    {
        return res;
    }

    // Bisection refine on the water-surface crossing.
    float lo = t;
    float hi = t_far;
    float last_inside = lo;
    for (uint bi = 0u; bi < 8u; ++bi)
    {
        float mid = (last_inside + hi) * 0.5f;
        float3 p = origin + (dir * mid);
        bool under = p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc,
            (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x);
        hi = under ? mid : hi;
        last_inside = under ? last_inside : mid;
    }
    if (!(last_inside < res.t))
    {
        return res;
    }

    float2 uv = ((origin + (dir * last_inside)).xz * 0.0076923076994717121124267578125f) + 0.5f;
    res.type = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x) + 0.100000001490116119384765625f)
        > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, uv, 0.0f).x)) ? 1u : 2u;
    res.t = last_inside;
    return res;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gtid : SV_DispatchThreadID)
{
    if (gtid.x >= ice_rt_count_buffer.Load<uint>(12))
    {
        return;
    }

    uint4 ray = ice_rt_ray_buffer.Load<uint4>(gtid.x * 16u + 0u);
    uint2 pixel = uint2(ray.x & 65535u, (ray.x >> 16u) & 65535u);
    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 ray_origin = (cam4.xyz / cam4.w) + float3(unpack_half2x16(ray.y & 65535u).x,
                                                     unpack_half2x16(ray.z & 65535u).x,
                                                     unpack_half2x16(ray.w & 65535u).x);
    float3 ray_dir = float3(unpack_half2x16((ray.y >> 16u) & 65535u).x,
                            unpack_half2x16((ray.z >> 16u) & 65535u).x,
                            unpack_half2x16((ray.w >> 16u) & 65535u).x);

    // Single-segment trace with the original's (effectively dead) bounce structure:
    // the first segment always terminates, but the bounce branches keep the
    // blue-noise sample live so the binding matches (as in the original compile).
    float4 hit = float4(0.0f, 0.0f, 0.0f, 100000.0f);
    float3 cur_origin = ray_origin;
    float3 cur_dir = ray_dir;
    uint bounce = 0u;
    for (;;)
    {
        if (bounce < 1u)
        {
            // Bin: blue-noise is sampled inside the loop with the bounce scroll.
            float4 bn = sample_blue_noise(pixel, (frame_index + 1234567u) + (bounce * 137u));

            RayResult tri = trace_bl_bvh(rt_bl_bvh, rt_triangles, cur_origin, cur_dir);
            RayResult water = water_terrain_march(cur_origin, cur_dir, tri.t);

            uint type;
            float t;
            float3 n;
            if (water.type != 0u)
            {
                type = water.type;
                t = water.t;
                n = water.normal;
            }
            else if (tri.type == 3u)
            {
                type = 3u;
                t = tri.t;
                n = tri.normal;
            }
            else
            {
                type = 0u;
                t = tri.t;
                n = tri.normal;
            }

            if (type == 0u)
            {
                // Bin miss sky branch: cur_dir.y>0 -> sky; theme==2 -> sky; else previous.
                float3 sky = lerp(sky_low_color.xyz, sky_high_color.xyz,
                    pow(clamp(1.0f - pow(1.0f - clamp(cur_dir.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f),
                        0.64999997615814208984375f));
                if (cur_dir.y > 0.0f)
                {
                    hit = float4(sky, 100000.0f);
                }
                else if (glade_theme_id == 2u)
                {
                    hit = float4(sky, 100000.0f);
                }
                else
                {
                    hit = float4(0.0f, 0.0f, 0.0f, 100000.0f);
                }
                break;
            }

            float4 hit_t4 = (bounce == 0u) ? float4(0.0f, 0.0f, 0.0f, t) : hit;

            if (type == 1u || type == 2u)
            {
                if ((bounce + 1u) == 1u)
                {
                    hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                    break;
                }
                // Dead bounce path (never taken: bounce stays 0) — kept so the
                // blue-noise Load survives to match the original binding.
                float c = 1.0f - (2.0f * bn.x);
                float s = sqrt(max(0.0f, 1.0f - (c * c)));
                float a = 6.283185482025146484375f * bn.y;
                hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                cur_dir = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(a) * s, sin(a) * s, c));
                cur_origin = hit_t4.xyz + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
            }
            else
            {
                if (dot(n, cur_dir) > 0.0f)
                {
                    hit = hit_t4;
                    break;
                }
                float c = 1.0f - (2.0f * bn.x);
                float s = sqrt(max(0.0f, 1.0f - (c * c)));
                float a = 6.283185482025146484375f * bn.y;
                float3 nn = normalize(n);
                hit = float4(0.0f, 0.0f, 0.0f, hit_t4.w);
                cur_dir = normalize(nn + float3(cos(a) * s, sin(a) * s, c));
                cur_origin = hit_t4.xyz + (nn * 0.001000000047497451305389404296875f);
            }
            ++bounce;
            continue;
        }
        else
        {
            break;
        }
    }

    // Reprojection / history resolve of the reflected point.
    float4 sp4 = mul(view_constants[0].view_to_sample,
        mul(view_constants[0].world_to_view, float4(ray_origin + (ray_dir * hit.w), 1.0f)));
    float3 sp = sp4.xyz / sp4.w;
    float2 sp_uv = (sp.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, sp_uv, 0.0f);
    float4 current_hit = refl_hit_tex[pixel].xyyy;

    float3 sky = 0.0f;
    float mix_factor = 0.0f;
    bool occluded = all(abs(sp) < 1.0f) && sp.z > 0.0f;
    if (occluded && half_depth_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).x < (sp.z * 1.10000002384185791015625f))
    {
        sky = (history_valid != 0u && frame_index != 0u)
            ? taa_history_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).xyz
            : 0.0f;
        mix_factor = 0.0f;
    }
    else
    {
        mix_factor = float(hit.w > 10000.0f);
    }
    if (mix_factor > 0.0f)
    {
        sky += lerp(sky_low_color.xyz, sky_high_color.xyz,
            pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f),
                0.64999997615814208984375f)) * mix_factor;
    }

    refl_hit_tex[pixel] = float2(current_hit.x + hit.w, current_hit.y);
    float3 clamped = clamp(sky, 0.0f, 65408.0f);
    float scale = asfloat((asuint(max(max(1.52587890625e-05f, clamped.x), max(clamped.y, clamped.z))) + 125845504u) & 2139095040u);
    uint3 mantissa = asuint(clamped + scale);
    refl_hit_radiance_tex[pixel] = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
}

#endif

#if defined(TGV_0) && (__SHADER_TARGET_STAGE == __SHADER_STAGE_COMPUTE)
// lifted from _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin (spirv-cross; behaviour-exact, names are SPIR-V ids)
#pragma dxc diagnostic push
#pragma dxc diagnostic ignored "-Wconversion"
#pragma dxc diagnostic ignored "-Wunused-value"
#pragma dxc diagnostic ignored "-Wambig-lit-shift"
struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct PackedBlBvhNode
{
    uint4 data[5];
};

struct PackedTriangle
{
    float v[3];
    uint e[3];
};

static float3 _190;
static float3 _210;
static float4 _211;

static const uint2 _192[8] = { uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u) };

cbuffer $Globals : register(b13, space0)
{
    uint2 terrain_lighting_texture_dims : packoffset(c0);
    uint2 blue_noise_texture_dims : packoffset(c0.z);
    uint2 terrain_heightmap_dims : packoffset(c1);
    uint2 visual_terrain_heightmap_dims : packoffset(c1.z);
    uint2 brdf_fg_lut_dims : packoffset(c2);
    uint rt_bvh_node_count : packoffset(c2.z);
    uint rt_triangle_count : packoffset(c2.w);
    uint history_valid : packoffset(c3);
    uint2 half_depth_tex_dims : packoffset(c3.y);
    uint2 taa_history_tex_dims : packoffset(c4);
    uint2 reprojection_tex_dims : packoffset(c4.z);
    uint2 refl_hit_tex_dims : packoffset(c5);
    uint2 refl_hit_radiance_tex_dims : packoffset(c5.z);
};

cbuffer frame_constants : register(b0, space1)
{
    float4 frame_constants_sun_color : packoffset(c0);
    float4 frame_constants_sky_low_color : packoffset(c1);
    float4 frame_constants_sky_high_color : packoffset(c2);
    float4 frame_constants_fog_tint_thickness : packoffset(c3);
    float4 frame_constants_fake_light_color : packoffset(c4);
    float4 frame_constants_fake_light_dir : packoffset(c5);
    float4 frame_constants_fake_light2_color : packoffset(c6);
    float4 frame_constants_fake_light2_dir : packoffset(c7);
    uint frame_constants_frame_index : packoffset(c8);
    float frame_constants_delta_time_seconds : packoffset(c8.y);
    float frame_constants_ev_mult : packoffset(c8.z);
    uint frame_constants_glade_theme_id : packoffset(c8.w);
    uint2 frame_constants_inner_rendering_resolution : packoffset(c9);
    uint frame_constants_game_time_sec : packoffset(c9.z);
    float frame_constants_game_time_frac : packoffset(c9.w);
    float3 frame_constants_editing_highlight_color : packoffset(c10);
    float frame_constants_time_of_day : packoffset(c10.w);
    float3 frame_constants_dir_to_sun : packoffset(c11);
    uint frame_constants_camera_idx : packoffset(c11.w);
    float3 frame_constants_dir_to_moon : packoffset(c12);
    uint frame_constants_render_overrides : packoffset(c12.w);
    float frame_constants_global_mip_bias : packoffset(c13);
    uint frame_constants_highlighted_transient_stair_edge_id : packoffset(c13.y);
    uint frame_constants_pad1 : packoffset(c13.z);
    uint frame_constants_pad2 : packoffset(c13.w);
    float4 frame_constants_debug_tweakers : packoffset(c14);
    ViewConstants frame_constants_view_constants[5] : packoffset(c15);
};

ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer ice_rt_count_buffer : register(t25, space0);
ByteAddressBuffer ice_rt_ray_buffer : register(t26, space0);
SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_default : register(s9, space0);
Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
RWTexture2D<float2> refl_hit_tex : register(u30, space0);
RWTexture2D<uint> refl_hit_radiance_tex : register(u31, space0);

static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= ice_rt_count_buffer.Load<uint>(12))
        {
            break;
        }
        uint4 _224 = ice_rt_ray_buffer.Load<uint4>(gl_GlobalInvocationID.x * 16 + 0);
        uint2 _229 = uint2(_224.x & 65535u, (_224.x >> 16u) & 65535u);
        float4 _232 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 _250 = (_232.xyz / _232.w.xxx) + float3(spvUnpackHalf2x16(_224.y & 65535u).x, spvUnpackHalf2x16(_224.z & 65535u).x, spvUnpackHalf2x16(_224.w & 65535u).x);
        float2 _257 = spvUnpackHalf2x16((_224.z >> 16u) & 65535u);
        float _258 = _257.x;
        float3 _263 = float3(spvUnpackHalf2x16((_224.y >> 16u) & 65535u).x, _258, spvUnpackHalf2x16((_224.w >> 16u) & 65535u).x);
        float4 _265;
        float3 _268;
        float3 _270;
        float3 _272;
        _265 = float4(0.0f, 0.0f, 0.0f, 100000.0f);
        _268 = _263;
        _270 = _190;
        _272 = _250;
        float4 _266;
        float3 _269;
        float3 _271;
        float3 _273;
        float4 _1138;
        uint _274 = 0u;
        for (;;)
        {
            if (_274 < 1u)
            {
                float _302;
                float4 _296 = ((blue_noise_texture.Load(int3((_229 + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_constants_frame_index + 1234567u) + (_274 * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
                float3 _933;
                uint _934;
                float _935;
                do
                {
                    uint2 _213[8] = _192;
                    bool _300 = _268.x < 0.0f;
                    _302 = _268.y;
                    bool _303 = _302 < 0.0f;
                    bool _307 = _268.z < 0.0f;
                    uint _312 = (7u - uint(((_300 ? 4 : 0) | (_303 ? 2 : 0)) | int(_307))) * 16843009u;
                    uint2 _314;
                    float3 _323;
                    _314 = uint2(0u, 2147483648u);
                    _323 = _270;
                    uint2 _315;
                    float _318;
                    int _320;
                    float3 _324;
                    int _326;
                    float _924;
                    int _925;
                    float _317 = 3.4028234663852885981170418348452e+38f;
                    int _319 = 0;
                    uint _321 = 0u;
                    int _325 = -1;
                    [loop]
                    for (;;)
                    {
                        if (_321 < 1024u)
                        {
                            uint2 _820;
                            uint2 _821;
                            int _822;
                            if ((_314.y & 4278190080u) != 0u)
                            {
                                uint _336 = firstbithigh(_314.y);
                                uint _341 = _314.y & (~(1u << (_336 & 31u)));
                                uint2 _342 = _314;
                                _342.y = _341;
                                int _349;
                                if ((_341 & 4278190080u) != 0u)
                                {
                                    _213[_319] = _342;
                                    _349 = _319 + 1;
                                }
                                else
                                {
                                    _349 = _319;
                                }
                                PackedBlBvhNode _360;
                                [unroll]
                                for (int _6ident = 0; _6ident < 5; _6ident++)
                                {
                                    _360.data[_6ident] = rt_bl_bvh.Load<uint4>(_6ident * 16 + (_314.x + countbits(_314.y & (~(4294967295u << (((_336 - 24u) ^ (_312 & 255u)) & 31u))))) * 80 + 0);
                                }
                                float3 _385 = float3(asfloat(((_360.data[0].w >> 0u) & 255u) << 23u) / _268.x, asfloat(((_360.data[0].w >> 8u) & 255u) << 23u) / _302, asfloat(((_360.data[0].w >> 16u) & 255u) << 23u) / _268.z);
                                float3 _387 = (asfloat(_360.data[0].xyz) - _272) / _268;
                                uint _396 = (_360.data[1].z ^ (_312 & ((((_360.data[1].z & (_360.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _398 = (_360.data[1].z >> 5u) & 117901063u;
                                uint _405 = _300 ? _360.data[2].z : _360.data[2].x;
                                uint _406 = _300 ? _360.data[2].x : _360.data[2].z;
                                uint _407 = _303 ? _360.data[3].z : _360.data[3].x;
                                uint _408 = _303 ? _360.data[3].x : _360.data[3].z;
                                uint _409 = _307 ? _360.data[4].z : _360.data[4].x;
                                uint _410 = _307 ? _360.data[4].x : _360.data[4].z;
                                float3 _432 = (float3(float((_405 >> 0u) & 255u), float((_407 >> 0u) & 255u), float((_409 >> 0u) & 255u)) * _385) + _387;
                                float3 _434 = (float3(float((_406 >> 0u) & 255u), float((_408 >> 0u) & 255u), float((_410 >> 0u) & 255u)) * _385) + _387;
                                uint _457;
                                if (max(_432.x, max(_432.y, max(_432.z, 9.9999997473787516355514526367188e-05f))) <= min(_434.x, min(_434.y, min(_434.z, _317))))
                                {
                                    _457 = 0u | (((_398 >> 0u) & 255u) << (((_396 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _457 = 0u;
                                }
                                float3 _479 = (float3(float((_405 >> 8u) & 255u), float((_407 >> 8u) & 255u), float((_409 >> 8u) & 255u)) * _385) + _387;
                                float3 _481 = (float3(float((_406 >> 8u) & 255u), float((_408 >> 8u) & 255u), float((_410 >> 8u) & 255u)) * _385) + _387;
                                uint _504;
                                if (max(_479.x, max(_479.y, max(_479.z, 9.9999997473787516355514526367188e-05f))) <= min(_481.x, min(_481.y, min(_481.z, _317))))
                                {
                                    _504 = _457 | (((_398 >> 8u) & 255u) << (((_396 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _504 = _457;
                                }
                                float3 _526 = (float3(float((_405 >> 16u) & 255u), float((_407 >> 16u) & 255u), float((_409 >> 16u) & 255u)) * _385) + _387;
                                float3 _528 = (float3(float((_406 >> 16u) & 255u), float((_408 >> 16u) & 255u), float((_410 >> 16u) & 255u)) * _385) + _387;
                                uint _551;
                                if (max(_526.x, max(_526.y, max(_526.z, 9.9999997473787516355514526367188e-05f))) <= min(_528.x, min(_528.y, min(_528.z, _317))))
                                {
                                    _551 = _504 | (((_398 >> 16u) & 255u) << (((_396 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _551 = _504;
                                }
                                float3 _573 = (float3(float((_405 >> 24u) & 255u), float((_407 >> 24u) & 255u), float((_409 >> 24u) & 255u)) * _385) + _387;
                                float3 _575 = (float3(float((_406 >> 24u) & 255u), float((_408 >> 24u) & 255u), float((_410 >> 24u) & 255u)) * _385) + _387;
                                uint _598;
                                if (max(_573.x, max(_573.y, max(_573.z, 9.9999997473787516355514526367188e-05f))) <= min(_575.x, min(_575.y, min(_575.z, _317))))
                                {
                                    _598 = _551 | (((_398 >> 24u) & 255u) << (((_396 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _598 = _551;
                                }
                                uint _607 = (_360.data[1].w ^ (_312 & ((((_360.data[1].w & (_360.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint _609 = (_360.data[1].w >> 5u) & 117901063u;
                                uint _616 = _300 ? _360.data[2].w : _360.data[2].y;
                                uint _617 = _300 ? _360.data[2].y : _360.data[2].w;
                                uint _618 = _303 ? _360.data[3].w : _360.data[3].y;
                                uint _619 = _303 ? _360.data[3].y : _360.data[3].w;
                                uint _620 = _307 ? _360.data[4].w : _360.data[4].y;
                                uint _621 = _307 ? _360.data[4].y : _360.data[4].w;
                                float3 _643 = (float3(float((_616 >> 0u) & 255u), float((_618 >> 0u) & 255u), float((_620 >> 0u) & 255u)) * _385) + _387;
                                float3 _645 = (float3(float((_617 >> 0u) & 255u), float((_619 >> 0u) & 255u), float((_621 >> 0u) & 255u)) * _385) + _387;
                                uint _668;
                                if (max(_643.x, max(_643.y, max(_643.z, 9.9999997473787516355514526367188e-05f))) <= min(_645.x, min(_645.y, min(_645.z, _317))))
                                {
                                    _668 = _598 | (((_609 >> 0u) & 255u) << (((_607 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    _668 = _598;
                                }
                                float3 _690 = (float3(float((_616 >> 8u) & 255u), float((_618 >> 8u) & 255u), float((_620 >> 8u) & 255u)) * _385) + _387;
                                float3 _692 = (float3(float((_617 >> 8u) & 255u), float((_619 >> 8u) & 255u), float((_621 >> 8u) & 255u)) * _385) + _387;
                                uint _715;
                                if (max(_690.x, max(_690.y, max(_690.z, 9.9999997473787516355514526367188e-05f))) <= min(_692.x, min(_692.y, min(_692.z, _317))))
                                {
                                    _715 = _668 | (((_609 >> 8u) & 255u) << (((_607 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    _715 = _668;
                                }
                                float3 _737 = (float3(float((_616 >> 16u) & 255u), float((_618 >> 16u) & 255u), float((_620 >> 16u) & 255u)) * _385) + _387;
                                float3 _739 = (float3(float((_617 >> 16u) & 255u), float((_619 >> 16u) & 255u), float((_621 >> 16u) & 255u)) * _385) + _387;
                                uint _762;
                                if (max(_737.x, max(_737.y, max(_737.z, 9.9999997473787516355514526367188e-05f))) <= min(_739.x, min(_739.y, min(_739.z, _317))))
                                {
                                    _762 = _715 | (((_609 >> 16u) & 255u) << (((_607 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    _762 = _715;
                                }
                                float3 _784 = (float3(float((_616 >> 24u) & 255u), float((_618 >> 24u) & 255u), float((_620 >> 24u) & 255u)) * _385) + _387;
                                float3 _786 = (float3(float((_617 >> 24u) & 255u), float((_619 >> 24u) & 255u), float((_621 >> 24u) & 255u)) * _385) + _387;
                                uint _809;
                                if (max(_784.x, max(_784.y, max(_784.z, 9.9999997473787516355514526367188e-05f))) <= min(_786.x, min(_786.y, min(_786.z, _317))))
                                {
                                    _809 = _762 | (((_609 >> 24u) & 255u) << (((_607 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    _809 = _762;
                                }
                                uint2 _813 = uint2(_360.data[1].x, _341);
                                _813.y = (_809 & 4278190080u) | ((_360.data[0].w >> 24u) & 255u);
                                _820 = _813;
                                _821 = uint2(_360.data[1].y, _809 & 16777215u);
                                _822 = _349;
                            }
                            else
                            {
                                _820 = uint2(0u, 0u);
                                _821 = _314;
                                _822 = _319;
                            }
                            uint2 _824;
                            _824 = _821;
                            _324 = _323;
                            _318 = _317;
                            _326 = _325;
                            uint2 _825;
                            float3 _827;
                            float _828;
                            int _829;
                            uint _830;
                            for (;;)
                            {
                                _830 = _824.y;
                                if (_830 != 0u)
                                {
                                    uint _834 = firstbithigh(_830);
                                    _825 = _824;
                                    _825.y = _830 & (~(1u << (_834 & 31u)));
                                    uint _840 = _824.x + _834;
                                    PackedTriangle _842;
                                    [unroll]
                                    for (int _7ident = 0; _7ident < 3; _7ident++)
                                    {
                                        _842.v[_7ident] = rt_triangles.Load<float>(_7ident * 4 + _840 * 24 + 0);
                                    }
                                    [unroll]
                                    for (int _8ident = 0; _8ident < 3; _8ident++)
                                    {
                                        _842.e[_8ident] = rt_triangles.Load<uint>(_8ident * 4 + _840 * 24 + 12);
                                    }
                                    float3 _873 = float3(spvUnpackHalf2x16((_842.e[0] >> 16u) & 65535u).x, spvUnpackHalf2x16((_842.e[1] >> 16u) & 65535u).x, spvUnpackHalf2x16((_842.e[2] >> 16u) & 65535u).x);
                                    float3 _874 = float3(spvUnpackHalf2x16(_842.e[0] & 65535u).x, spvUnpackHalf2x16(_842.e[1] & 65535u).x, spvUnpackHalf2x16(_842.e[2] & 65535u).x);
                                    bool _908;
                                    do
                                    {
                                        float3 _877 = cross(_268, _874);
                                        float _878 = dot(_873, _877);
                                        float3 _879 = _272 - float3(_842.v[0], _842.v[1], _842.v[2]);
                                        float3 _880 = cross(_879, _873);
                                        float _882 = dot(_874, _880) / _878;
                                        bool _887;
                                        if (_882 >= 0.0f)
                                        {
                                            _887 = _882 < _318;
                                        }
                                        else
                                        {
                                            _887 = false;
                                        }
                                        if (_887 ? true : false)
                                        {
                                            float3 _892;
                                            _892.x = dot(_879, _877);
                                            _892.y = dot(_268, _880);
                                            float2 _897 = _892.xy / _878.xx;
                                            float3 _898 = float3(_897.x, _897.y, _892.z);
                                            _898.z = (1.0f - _897.x) - _897.y;
                                            if (all(bool3(_898.x >= (-9.9999997473787516355514526367188e-05f).xxx.x, _898.y >= (-9.9999997473787516355514526367188e-05f).xxx.y, _898.z >= (-9.9999997473787516355514526367188e-05f).xxx.z)))
                                            {
                                                _828 = _882;
                                                _908 = true;
                                                break;
                                            }
                                        }
                                        _828 = _318;
                                        _908 = false;
                                        break;
                                    } while(false);
                                    if (_908)
                                    {
                                        _827 = cross(_873, _874);
                                        _829 = int(_840);
                                    }
                                    else
                                    {
                                        _827 = _324;
                                        _829 = _326;
                                    }
                                    _824 = _825;
                                    _324 = _827;
                                    _318 = _828;
                                    _326 = _829;
                                    continue;
                                }
                                else
                                {
                                    break;
                                }
                            }
                            if ((_820.y & 4278190080u) == 0u)
                            {
                                if (_822 == 0)
                                {
                                    _271 = _324;
                                    _924 = _318;
                                    _925 = _326;
                                    break;
                                }
                                int _921 = _822 - 1;
                                _315 = _213[_921];
                                _320 = _921;
                            }
                            else
                            {
                                _315 = _820;
                                _320 = _822;
                            }
                            _314 = _315;
                            _317 = _318;
                            _319 = _320;
                            _321++;
                            _323 = _324;
                            _325 = _326;
                            continue;
                        }
                        else
                        {
                            _271 = _323;
                            _924 = _317;
                            _925 = _325;
                            break;
                        }
                    }
                    bool _930;
                    if (_925 != (-1))
                    {
                        _930 = _924 < 100000.0f;
                    }
                    else
                    {
                        _930 = false;
                    }
                    if (_930)
                    {
                        _933 = _271;
                        _934 = 3u;
                        _935 = _924;
                        break;
                    }
                    _933 = float3(0.0f, 1.0f, 0.0f);
                    _934 = 0u;
                    _935 = 100000.0f;
                    break;
                } while(false);
                uint _1050;
                float _1051;
                do
                {
                    float3 _939 = (float3(65.0f, 24.0f, 65.0f) - _272) / _268;
                    float3 _941 = (float3(-65.0f, 0.0f, -65.0f) - _272) / _268;
                    float3 _942 = min(_939, _941);
                    float3 _943 = max(_939, _941);
                    float _948 = max(max(_942.x, _942.y), _942.z);
                    float _953 = min(min(_943.x, _943.y), _943.z);
                    bool _958;
                    if (_948 <= _953)
                    {
                        _958 = _953 >= 0.0f;
                    }
                    else
                    {
                        _958 = false;
                    }
                    if (!_958)
                    {
                        _1050 = _934;
                        _1051 = _935;
                        break;
                    }
                    float _962 = max(_948, 0.0f);
                    float _964 = (_953 - _962) * 0.03125f;
                    float _966;
                    _966 = _962;
                    float _967;
                    uint _970;
                    float _989;
                    uint _969 = 0u;
                    bool _971;
                    for (;;)
                    {
                        _971 = _969 < 32u;
                        if (_971)
                        {
                            _967 = _966 + _964;
                            float3 _975 = _272 + (_268 * _967);
                            if (_975.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (_975.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x))
                            {
                                _989 = _967;
                                break;
                            }
                            _970 = _969 + 1u;
                            _966 = _967;
                            _969 = _970;
                            continue;
                        }
                        else
                        {
                            _989 = _953;
                            break;
                        }
                    }
                    bool _993;
                    if (_971)
                    {
                        _993 = _966 < _935;
                    }
                    else
                    {
                        _993 = false;
                    }
                    bool _997;
                    if (_993)
                    {
                        _997 = _966 < _989;
                    }
                    else
                    {
                        _997 = false;
                    }
                    if (_997)
                    {
                        float _1005;
                        _1005 = _966;
                        float _1002 = _989;
                        uint _1007 = 0u;
                        for (; _1007 < 8u; )
                        {
                            float _1011 = _1005 + _1002;
                            float _1012 = _1011 * 0.5f;
                            float3 _1014 = _272 + (_268 * _1012);
                            bool _1025 = _1014.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (_1014.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            _1002 = _1025 ? _1012 : _1002;
                            _1005 = _1025 ? _1005 : _1012;
                            _1007++;
                            continue;
                        }
                        if (_1005 < _935)
                        {
                            float2 _1034 = ((_272 + (_268 * _1005)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            _1050 = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, _1034, 0.0f).x) + 0.100000001490116119384765625f) > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, _1034, 0.0f).x)) ? 1u : 2u;
                            _1051 = _1005;
                            break;
                        }
                        else
                        {
                            _1050 = _934;
                            _1051 = _935;
                            break;
                        }
                        break; // unreachable workaround
                    }
                    else
                    {
                        _1050 = _934;
                        _1051 = _935;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                if (_1050 == 0u)
                {
                    float4 _1137;
                    if (_302 > 0.0f)
                    {
                        _1137 = float4(_210.x, _210.y, _210.z, _265.w);
                    }
                    else
                    {
                        float4 _1135;
                        if (frame_constants_glade_theme_id == 2u)
                        {
                            _1135 = float4(_210.x, _210.y, _210.z, _265.w);
                        }
                        else
                        {
                            _1135 = _265;
                        }
                        _1137 = _1135;
                    }
                    _1138 = _1137;
                    break;
                }
                else
                {
                    float4 _1060;
                    if (0u == _274)
                    {
                        float4 _1059;
                        _1059.w = _1051;
                        _1060 = _1059;
                    }
                    else
                    {
                        _1060 = _265;
                    }
                    float3 _1062 = _272 + (_268 * _1051);
                    bool _1068;
                    if (!(_1050 == 1u))
                    {
                        _1068 = _1050 == 2u;
                    }
                    else
                    {
                        _1068 = true;
                    }
                    if (_1068)
                    {
                        if ((_274 + 1u) == 1u)
                        {
                            _1138 = float4(_210.x, _210.y, _210.z, _1060.w);
                            break;
                        }
                        float _1110 = 1.0f - (2.0f * _296.x);
                        float _1114 = sqrt(max(0.0f, 1.0f - (_1110 * _1110)));
                        float _1116 = 6.283185482025146484375f * _296.y;
                        _266 = float4(_210.x, _210.y, _210.z, _1060.w);
                        _269 = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(_1116) * _1114, sin(_1116) * _1114, _1110));
                        _273 = _1062 + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
                    }
                    else
                    {
                        float4 _1099;
                        float3 _1100;
                        float3 _1101;
                        if (_1050 == 3u)
                        {
                            if (dot(_933, _268) > 0.0f)
                            {
                                _1138 = _1060;
                                break;
                            }
                            float3 _1079 = normalize(_933);
                            float _1085 = 1.0f - (2.0f * _296.x);
                            float _1089 = sqrt(max(0.0f, 1.0f - (_1085 * _1085)));
                            float _1091 = 6.283185482025146484375f * _296.y;
                            _1099 = float4(_210.x, _210.y, _210.z, _1060.w);
                            _1100 = normalize(_1079 + float3(cos(_1091) * _1089, sin(_1091) * _1089, _1085));
                            _1101 = _1062 + (_1079 * 0.001000000047497451305389404296875f);
                        }
                        else
                        {
                            _1099 = _1060;
                            _1100 = _268;
                            _1101 = _272;
                        }
                        _266 = _1099;
                        _269 = _1100;
                        _273 = _1101;
                    }
                }
                _265 = _266;
                _268 = _269;
                _270 = _271;
                _272 = _273;
                _274++;
                continue;
            }
            else
            {
                _1138 = _265;
                break;
            }
        }
        float4 _1151 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(_250 + (_263 * _1138.w), 1.0f)));
        float3 _1155 = _1151.xyz / _1151.w.xxx;
        float2 _1158 = (_1155.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float4 _1162 = reprojection_tex.SampleLevel(sampler_nnc, _1158, 0.0f);
        float4 _1164 = refl_hit_tex[_229].xyyy;
        float3 _1168 = abs(_1155);
        bool _1175;
        if (all(bool3(_1168.x < 1.0f.xxx.x, _1168.y < 1.0f.xxx.y, _1168.z < 1.0f.xxx.z)))
        {
            _1175 = _1155.z > 0.0f;
        }
        else
        {
            _1175 = false;
        }
        bool _1187;
        if (_1175)
        {
            _1187 = half_depth_tex.SampleLevel(sampler_nnc, _1162.xy + _1158, 0.0f).x < (_1155.z * 1.10000002384185791015625f);
        }
        else
        {
            _1187 = false;
        }
        float3 _1212;
        float _1213;
        if (_1187)
        {
            bool _1202;
            if (history_valid != 0u)
            {
                _1202 = !(0u == frame_constants_frame_index);
            }
            else
            {
                _1202 = false;
            }
            float3 _1211;
            if (_1202)
            {
                _1211 = taa_history_tex.SampleLevel(sampler_nnc, _1162.xy + _1158, 0.0f).xyz;
            }
            else
            {
                _1211 = 0.0f.xxx;
            }
            _1212 = _1211;
            _1213 = 0.0f;
        }
        else
        {
            _1212 = 0.0f.xxx;
            _1213 = ((_1138.w > 10000.0f) ? 1.0f : 0.0f);
        }
        float3 _1238;
        if (_1213 > 0.0f)
        {
            _1238 = _1212 + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(_258 + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * _1213);
        }
        else
        {
            _1238 = _1212;
        }
        refl_hit_tex[_229] = float2(_1164.x + _1138.w, _1164.y);
        float3 _1241 = clamp(_1238, 0.0f.xxx, 65408.0f.xxx);
        float _1251 = asfloat((asuint(max(max(1.52587890625e-05f, _1241.x), max(_1241.y, _1241.z))) + 125845504u) & 2139095040u);
        uint3 _1254 = asuint(_1241 + _1251.xxx);
        refl_hit_radiance_tex[_229] = ((((asuint(_1251) << 4u) + 268435456u) | (_1254.z << 18u)) | (_1254.y << 9u)) | (_1254.x & 511u);
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void cs_main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
#pragma dxc diagnostic pop
#endif
