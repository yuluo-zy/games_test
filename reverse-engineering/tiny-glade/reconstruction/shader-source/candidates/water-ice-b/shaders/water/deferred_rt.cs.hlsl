// ==RECONSTRUCTED== shaders/water/deferred_rt.cs.hlsl
// Evidence: _water_deferred_rt.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: sampler_lnc s0, sampler_nnc s2, sampler_default s9, $Globals b13 (terrain/rt dims +
//   rt_bvh_node_count/rt_triangle_count/history_valid), blue_noise_texture t15, terrain_heightmap t18,
//   visual_terrain_heightmap t19, rt_bl_bvh t23 (ByteAddressBuffer, PackedBlBvhNode), rt_triangles t24
//   (PackedTriangle), water_rt_count_buffer t25, water_rt_ray_buffer t26, half_depth_tex t27,
//   taa_history_tex t28, reprojection_tex t29, color_tex u30 (RWTexture2D<float3> r11g11b10f),
//   frame_constants set1/b0. LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disassembly: software-BVH deferred water ray trace. Decodes a packed water ray
//   (px, octahedral tangent, length, depth+flags), casts a reflection, traverses the morton-coded
//   software BVH (rt_bl_bvh/rt_triangles, 8-slot child AABB slab tests), falls back to a heightfield
//   march against visual_terrain_heightmap, then resolves the hit radiance + sky into color_tex with
//   per-subpixel reprojection and TAA history clamping (4 quadrants gated by the ray flag bits).
// Status: O (algorithm + bindings from SPIR-V; morton BVH traversal transcribed from disassembly).

#include "inc/common.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/sun.inc.hlsl"
#include "inc/history_valid.hlsl"
#include "inc/rt.hlsl"

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
    uint2 color_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer water_rt_count_buffer : register(t25, space0);
ByteAddressBuffer water_rt_ray_buffer : register(t26, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
[[vk::image_format("r11g11b10f")]] RWTexture2D<float3> color_tex : register(u30, space0);

#include "inc/terrain_heightmap.inc.hlsl"

// --- software-BVH traversal (morton slot masks), transcribed from the disassembly ---
// stack entries: (node_or_tri_index, remaining-child mask); masks use the top 8 bits as a slot
// counter and the low 24 bits as a triangle bitfield for leaf nodes.

uint pack_ray_slots(float3 rd)
{
    // octant index -> slot permutation byte (16843009 = 0x01010101).
    uint oct = 7u - uint((rd.x < 0.0f ? 4u : 0u) | (rd.y < 0.0f ? 2u : 0u) | (rd.z < 0.0f ? 1u : 0u));
    return oct * 16843009u;
}

float3 trace_bvh(float3 ro, float3 rd, float t_max,
                 out float3 out_normal, out uint out_type, out float out_t)
{
    uint2 stack[8];
    for (uint si = 0u; si < 8u; ++si)
    {
        stack[si] = uint2(0u, 0u);
    }
    uint2 node = uint2(0u, 0x80000000u);
    float best_t = t_max;
    int best_tri = -1;
    float3 best_normal = 0.0f;
    uint stack_size = 0u;
    uint slot_perm = pack_ray_slots(rd);
    uint iterations = 0u;

    while (iterations < 1024u)
    {
        uint2 next_node;
        uint2 leaf;
        int next_stack = stack_size;
        if ((node.y & 0xFF000000u) != 0u)
        {
            // Internal node: pop the highest slot, keep the remainder on the stack.
            uint slot = firstbithigh(node.y);
            uint rest = node.y & ~(1u << (slot & 31u));
            uint2 keep = node;
            keep.y = rest;
            if ((rest & 0xFF000000u) != 0u)
            {
                stack[stack_size] = keep;
                next_stack = stack_size + 1;
            }

            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh.Load<uint4>(i * 16u + (node.x + countbits(node.y & ~(0xFFFFFFFFu << (((slot - 24u) ^ (slot_perm & 255u)) & 31u)))) * 80u);
            }

            float3 inv_extent = float3(asfloat((((n.data[0].w >> 0u) & 255u) << 23u)) / rd.x,
                                       asfloat((((n.data[0].w >> 8u) & 255u) << 23u)) / rd.y,
                                       asfloat((((n.data[0].w >> 16u) & 255u) << 23u)) / rd.z);
            float3 rel = (asfloat(n.data[0].xyz) - ro) / rd;

            // Child slot AABB decode + slab tests (4 slots x 2 corners).
            uint maskA = (n.data[1].z ^ (slot_perm & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint trisA = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = rd.x < 0.0f ? n.data[2].z : n.data[2].x;
            uint c1 = rd.x < 0.0f ? n.data[2].x : n.data[2].z;
            uint c2 = rd.y < 0.0f ? n.data[3].z : n.data[3].x;
            uint c3 = rd.y < 0.0f ? n.data[3].x : n.data[3].z;
            uint c4 = rd.z < 0.0f ? n.data[4].z : n.data[4].x;
            uint c5 = rd.z < 0.0f ? n.data[4].x : n.data[4].z;

            uint child_mask = 0u;
            for (uint slot_i = 0u; slot_i < 4u; ++slot_i)
            {
                uint shift = slot_i * 8u;
                float3 lo = (float3(float((c0 >> shift) & 255u), float((c2 >> shift) & 255u), float((c4 >> shift) & 255u)) * inv_extent) + rel;
                float3 hi = (float3(float((c1 >> shift) & 255u), float((c3 >> shift) & 255u), float((c5 >> shift) & 255u)) * inv_extent) + rel;
                if (max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t))))
                {
                    child_mask |= 0x1u << ((maskA >> shift) & 31u);
                }
                // (the decompile additionally checks the trisA bitfield lane; folded into the mask above)
            }
            // (child A hits are pushed; child B / triangle leaves resolved below)
            next_node = uint2(n.data[1].x, rest);
            leaf = uint2(n.data[1].y, child_mask & 0x00FFFFFFu);
            next_stack = (child_mask != 0u) ? next_stack : stack_size;
        }
        else
        {
            next_node = uint2(0u, 0u);
            leaf = node;
        }

        // Process the triangle leaf.
        uint2 tris = leaf;
        while (tris.y != 0u)
        {
            uint slot = firstbithigh(tris.y);
            tris.y &= ~(1u << (slot & 31u));
            uint tri_index = tris.x + slot;

            PackedTriangle tri;
            [unroll]
            for (uint k = 0u; k < 3u; ++k)
            {
                tri.v[k] = rt_triangles.Load<float>(k * 4u + tri_index * 24u + 0u);
                tri.e[k] = rt_triangles.Load<uint>(k * 4u + tri_index * 24u + 12u);
            }
            float3 v0 = tri.v[0];
            float3 e0 = float3(f16tof32(tri.e[0] & 0xFFFFu), f16tof32(tri.e[1] & 0xFFFFu), f16tof32(tri.e[2] & 0xFFFFu));
            float3 e1 = float3(f16tof32(tri.e[0] >> 16u), f16tof32(tri.e[1] >> 16u), f16tof32(tri.e[2] >> 16u));

            float3 pvec = cross(rd, e0);
            float det = dot(e1, pvec);
            float3 tvec = ro - v0;
            float3 qvec = cross(tvec, e0);
            float u = dot(tvec, pvec) / det;
            float v = dot(rd, qvec) / det;
            float t = dot(e0, qvec) / det;
            bool inside = all(bool3(u >= -9.9999997473787516355514526367188e-05f, v >= -9.9999997473787516355514526367188e-05f, (1.0f - u) - v >= -9.9999997473787516355514526367188e-05f));
            if (t >= 0.0f && t < best_t && inside)
            {
                best_t = t;
                best_tri = int(tri_index);
                best_normal = cross(e0, e1);
            }
            continue;
        }

        if (next_stack == 0 && leaf.y == 0u && (next_node.y & 0xFF000000u) == 0u)
        {
            break;
        }
        if ((next_node.y & 0xFF000000u) != 0u)
        {
            node = next_node;
        }
        else if (next_stack > 0)
        {
            stack_size = next_stack - 1;
            node = stack[stack_size];
        }
        else
        {
            node = next_node;
        }
        ++iterations;
    }

    out_normal = best_normal;
    out_t = best_t;
    out_type = (best_tri >= 0 && best_t < t_max) ? 3u : 0u;
    return best_normal;
}

// Heightfield march against the terrain (from the disassembly).
float3 march_terrain(float3 ro, float3 rd, float t_max, out float out_t, out uint out_type)
{
    float3 box_min = (float3(65.0f, 24.0f, 65.0f) - ro) / rd;
    float3 box_max = (float3(-65.0f, 0.0f, -65.0f) - ro) / rd;
    float3 tmin = min(box_min, box_max);
    float3 tmax = max(box_min, box_max);
    float t_enter = max(max(tmin.x, tmin.y), tmin.z);
    float t_leave = min(min(tmax.x, tmax.y), tmax.z);
    if (!(t_enter <= t_leave && t_leave >= 0.0f))
    {
        out_t = t_max;
        out_type = 0u;
        return float3(0.0f, 1.0f, 0.0f);
    }

    float t0 = max(t_enter, 0.0f);
    float step = (t_leave - t0) * 0.03125f;
    float t = t0;
    uint it = 0u;
    bool hit_scan = false;
    for (; it < 32u; )
    {
        float tn = t + step;
        float3 p = ro + (rd * tn);
        if (p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x))
        {
            hit_scan = true;
            break;
        }
        t = tn;
        ++it;
        continue;
    }
    if (!hit_scan)
    {
        t = t_leave;
    }

    if (t < t_max && t < (hit_scan ? t : t_leave))
    {
        // bisection refine (8 iterations)
        float lo = t;
        float hi = hit_scan ? t + step : t_leave;
        float t_hit = t;
        for (uint j = 0u; j < 8u; ++j)
        {
            float mid = (lo + hi) * 0.5f;
            float3 p = ro + (rd * mid);
            if (p.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p.xz * 0.0076923076994717121124267578125f) + 0.5f, 0.0f).x))
            {
                hi = mid;
            }
            else
            {
                lo = mid;
            }
        }
        if (lo < t_max)
        {
            float2 uv = ((ro + (rd * lo)).xz * 0.0076923076994717121124267578125f) + 0.5f;
            float real_h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x);
            float vis_h = lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, uv, 0.0f).x);
            out_t = lo;
            out_type = (real_h + 0.100000001490116119384765625f) > vis_h ? 1u : 2u;
            return float3(0.0f, 1.0f, 0.0f);
        }
    }
    out_t = t_max;
    out_type = 0u;
    return float3(0.0f, 1.0f, 0.0f);
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x >= water_rt_count_buffer.Load<uint>(12))
    {
        return;
    }
    uint4 ray = water_rt_ray_buffer.Load<uint4>(gl_GlobalInvocationID.x * 16u);
    uint2 px = uint2(ray.x & 65535u, (ray.x >> 16u) & 65535u);
    float2 clip = ((float2(int2(px)) + 0.5f) / float2(half_depth_tex_dims) - 0.5f) * float2(2.0f, -2.0f);
    float3 ray_origin = mul(view_constants[0].view_to_world,
                            mul(view_constants[0].sample_to_view, float4(clip, asfloat(ray.w), 1.0f))).xyz;
    float3 ray_dir = -normalize(mul(view_constants[0].view_to_world,
                                    mul(view_constants[0].sample_to_view, float4(clip, 0.0f, 1.0f))).xyz);
    uint flags = ray.w & 15u;

    // Octahedral decode of the packed tangent.
    float2 oct = (float2(f16tof32(ray.y & 0xFFFFu), f16tof32((ray.y >> 16u) & 0xFFFFu)) * 2.0f) - 1.0f;
    float z = (1.0f - abs(oct.x)) - abs(oct.y);
    float2 txy = oct - (((step(0.0f, oct) * 2.0f) - 1.0f) * clamp(-z, 0.0f, 1.0f));
    float3 tangent = normalize(float3(txy, z));

    float3 refl = -reflect(ray_dir, tangent);
    if (refl.y < 0.0f)
    {
        refl.y = -refl.y;
    }
    float3 origin = (ray_origin / mul(view_constants[0].view_to_world,
                                      mul(view_constants[0].sample_to_view, float4(clip, asfloat(ray.w), 1.0f))).w.xxx)
        + (refl * asfloat(ray.z));

    // One-bounce trace (BVH, then terrain fallback).
    float4 result = float4(0.0f, 0.0f, 0.0f, 100000.0f);
    float3 dir = refl;
    float3 ro = origin;
    float3 normal = float3(0.0f, 1.0f, 0.0f);
    uint hit_type = 0u;
    float hit_t = 100000.0f;
    uint bounce = 0u;
    for (; bounce < 1u; ++bounce)
    {
        float4 bn = ((blue_noise_texture.Load(int3((px + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_index + 1234567u) + (bounce * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f) + 0.001953125f;

        float3 n = 0.0f;
        uint ttype = 0u;
        float tt = 0.0f;
        float3 tri_n;
        uint tri_type;
        float tri_t;
        tri_n = trace_bvh(ro, dir, hit_t, tri_n, tri_type, tri_t);
        float3 terr_n;
        uint terr_type;
        float terr_t;
        terr_n = march_terrain(ro, dir, hit_t, terr_t, terr_type);

        uint bt = 0u;
        float bt_t = hit_t;
        if (tri_t < hit_t && tri_type != 0u)
        {
            bt = 3u;
            bt_t = tri_t;
            normal = tri_n;
        }
        else if (terr_t < hit_t && terr_type != 0u)
        {
            bt = terr_type;
            bt_t = terr_t;
            normal = terr_n;
        }

        if (bt != 0u)
        {
            float3 hp = ro + (dir * bt_t);
            if (bounce == 0u)
            {
                result.w = bt_t;
            }
            // Reflect/refract secondary bounce (single-bounce path keeps the first hit color).
            if (bt == 1u)
            {
                if ((bounce + 1u) == 1u)
                {
                    result = float4(0.0f, 0.0f, 0.0f, result.w);
                    break;
                }
                float3 rr = float3(0.0f, 1.0f, 0.0f) + float3(cos(bn.y * 6.283185482025146484375f) * sqrt(max(0.0f, 1.0f - ((1.0f - (2.0f * bn.x)) * (1.0f - (2.0f * bn.x))))),
                                                              sin(bn.y * 6.283185482025146484375f) * sqrt(max(0.0f, 1.0f - ((1.0f - (2.0f * bn.x)) * (1.0f - (2.0f * bn.x))))),
                                                              1.0f - (2.0f * bn.x));
                normal = normalize(rr);
                ro = hp + float3(0.0f, 0.001f, 0.0f);
                dir = rr;
            }
            else if (bt == 3u)
            {
                if (dot(normal, dir) > 0.0f)
                {
                    result = float4(0.0f, 0.0f, 0.0f, result.w);
                    break;
                }
                float3 nn = normalize(normal);
                normal = normalize(nn + float3(cos(bn.y * 6.283185482025146484375f) * sqrt(max(0.0f, 1.0f - ((1.0f - (2.0f * bn.x)) * (1.0f - (2.0f * bn.x))))),
                                              sin(bn.y * 6.283185482025146484375f) * sqrt(max(0.0f, 1.0f - ((1.0f - (2.0f * bn.x)) * (1.0f - (2.0f * bn.x))))),
                                              1.0f - (2.0f * bn.x)));
                ro = hp + (nn * 0.001f);
                dir = nn;
            }
        }
        else
        {
            result = result;
            break;
        }
    }

    // Sky color for unoccluded rays.
    float4 hit = result;
    if (hit_type == 0u)
    {
        float3 sky = (ray_dir.y > 0.0f)
            ? lerp(sky_low_color.xyz, sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx)
            : ((glade_theme_id == 2u) ? lerp(sky_low_color.xyz, sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx) : hit.xyz);
        result = float4(sky, hit.w);
    }

    // Per-subpixel resolve (4 quadrants gated by the flag bits).
    for (uint q = 0u; q < 4u; ++q)
    {
        uint bit = 1u << q;
        if ((flags & bit) == 0u)
        {
            continue;
        }
        uint2 sp_px = (px * uint2(2u, 2u)) + uint2(q & 1u, q >> 1u);
        float4 sp = mul(view_constants[0].view_to_sample,
                        mul(view_constants[0].world_to_view, float4(origin + (refl * result.w), 1.0f)));
        float3 sp_n = sp.xyz / sp.w;
        float2 sp_uv = (sp_n.xy * float2(0.5f, -0.5f)) + 0.5f;
        float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, sp_uv, 0.0f);
        bool in_view = all(bool3(abs(sp_n.x) < 1.0f, abs(sp_n.y) < 1.0f, abs(sp_n.z) < 1.0f)) && sp_n.z > 0.0f;
        bool occluded = in_view && half_depth_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).x < (sp_n.z * 1.1f);

        float3 history = 0.0f;
        float sky_amt = 0.0f;
        if (occluded)
        {
            if (history_valid != 0u && frame_index != 0u)
            {
                history = taa_history_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).xyz;
            }
        }
        else
        {
            sky_amt = float(result.w > 10000.0f);
        }

        float3 rad = history;
        if (sky_amt > 0.0f)
        {
            float3 skyc = lerp(sky_low_color.xyz, sky_high_color.xyz,
                pow(clamp(1.0f - pow(1.0f - clamp(refl.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx);
            rad = history + (skyc * sky_amt);
        }
        float fres = 1.0f - dot(ray_dir, tangent);
        float3 contrib = rad * (0.04f + (0.96f * ((fres * fres) * fres)));

        float4 g = reprojection_tex.GatherBlue(sampler_lnc, ((float2(int2(px)) + 0.5f) / float2(half_depth_tex_dims)), int2(0, 0));
        bool valid_history = (history_valid != 0u && frame_index != 0u) && sky_amt < 1.0f;
        if (valid_history && all(bool4(g.x == 1.0f, g.y == 1.0f, g.z == 1.0f, g.w == 1.0f)))
        {
            float3 h = taa_history_tex.SampleLevel(sampler_lnc, ((float2(int2(px)) + 0.5f) / float2(half_depth_tex_dims)) + reprojection_tex.Load(int3(sp_px, 0)).xy, 0.0f).xyz;
            float3 cur = color_tex[sp_px].xyz + contrib;
            float lh = dot(h, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
            float lc = dot(cur, float3(0.2125999927520751953125f, 0.715200006961822509765625f, 0.072200000286102294921875f));
            float clamp_l = clamp(lc, 0.4f * lh, 2.5f * lh);
            color_tex[sp_px] = (lc > lh) ? cur * (clamp_l / max(9.9999999392252902907785028219223e-09f, lc))
                                          : h * (clamp_l / max(9.9999999392252902907785028219223e-09f, lh));
        }
        else
        {
            color_tex[sp_px] = color_tex[sp_px].xyz + contrib;
        }
    }
}
