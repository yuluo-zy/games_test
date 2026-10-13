// ==RECONSTRUCTED== shaders/water/ice_deferred_rt.cs.hlsl
// Evidence: _water_ice_deferred_rt.cs.hlsl_cs_main_0.bin (SPIR-V, offset 23).
//   bindings: sampler_lnc s0, sampler_nnc s2, sampler_default s9, $Globals b13 (13 members),
//   blue_noise_texture t15, terrain_heightmap t18, visual_terrain_heightmap t19, rt_bl_bvh t23,
//   rt_triangles t24, ice_rt_count_buffer t25, ice_rt_ray_buffer t26, half_depth_tex t27,
//   taa_history_tex t28, reprojection_tex t29, refl_hit_tex u30 (RWTexture2D<float2> rg16f),
//   refl_hit_radiance_tex u31 (RWTexture2D<uint> r32ui), frame_constants set1/b0.
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross + disassembly: software-BVH deferred ice ray trace (sibling of water deferred_rt).
//   Decodes a packed ice ray (origin = camera + low-half world dir, dir = high-half reflected dir
//   from ice_refl_march), traverses the morton software BVH + heightfield march, then accumulates
//   the ray distance into refl_hit_tex.x and packs the reprojected/occlusion-blended radiance
//   (history or sky) into refl_hit_radiance_tex.
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
    uint2 refl_hit_tex_dims;
    uint2 refl_hit_radiance_tex_dims;
};

Texture2D<float4> blue_noise_texture : register(t15, space0);
Texture2D<float4> terrain_heightmap : register(t18, space0);
Texture2D<float4> visual_terrain_heightmap : register(t19, space0);
ByteAddressBuffer rt_bl_bvh : register(t23, space0);
ByteAddressBuffer rt_triangles : register(t24, space0);
ByteAddressBuffer ice_rt_count_buffer : register(t25, space0);
ByteAddressBuffer ice_rt_ray_buffer : register(t26, space0);
Texture2D<float4> half_depth_tex : register(t27, space0);
Texture2D<float4> taa_history_tex : register(t28, space0);
Texture2D<float4> reprojection_tex : register(t29, space0);
[[vk::image_format("rg16f")]] RWTexture2D<float2> refl_hit_tex : register(u30, space0);
[[vk::image_format("r32ui")]] RWTexture2D<uint> refl_hit_radiance_tex : register(u31, space0);

#include "inc/terrain_heightmap.inc.hlsl"

uint pack_ray_slots(float3 rd)
{
    uint oct = 7u - uint((rd.x < 0.0f ? 4u : 0u) | (rd.y < 0.0f ? 2u : 0u) | (rd.z < 0.0f ? 1u : 0u));
    return oct * 16843009u;
}

// Morton software-BVH traversal (shared with water/ice deferred RT). See deferred_rt.cs.hlsl.
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
            }
            next_node = uint2(n.data[1].x, rest);
            leaf = uint2(n.data[1].y, child_mask & 0x00FFFFFFu);
        }
        else
        {
            next_node = uint2(0u, 0u);
            leaf = node;
        }

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
            float3 e0 = float3(f16tof32(tri.e[0] & 0xFFFFu), f16tof32(tri.e[1] & 0xFFFFu), f16tof32(tri.e[2] & 0xFFFFu));
            float3 e1 = float3(f16tof32(tri.e[0] >> 16u), f16tof32(tri.e[1] >> 16u), f16tof32(tri.e[2] >> 16u));

            float3 pvec = cross(rd, e0);
            float det = dot(e1, pvec);
            float3 tvec = ro - tri.v[0];
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
        float lo = t;
        float hi = hit_scan ? t + step : t_leave;
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
    if (gl_GlobalInvocationID.x >= ice_rt_count_buffer.Load<uint>(12))
    {
        return;
    }
    uint4 ray = ice_rt_ray_buffer.Load<uint4>(gl_GlobalInvocationID.x * 16u);
    uint2 px = uint2(ray.x & 65535u, (ray.x >> 16u) & 65535u);

    float4 cam4 = mul(view_constants[0].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
    float3 cam_pos = cam4.xyz / cam4.w;
    // Ray origin = camera + (low-half world dir); ray dir = (high-half reflected dir).
    float3 ray_origin = cam_pos + float3(f16tof32(ray.y & 0xFFFFu), f16tof32(ray.z & 0xFFFFu), f16tof32(ray.w & 0xFFFFu));
    float3 ray_dir = float3(f16tof32((ray.y >> 16u) & 0xFFFFu), f16tof32((ray.z >> 16u) & 0xFFFFu), f16tof32((ray.w >> 16u) & 0xFFFFu));

    // One-bounce trace (BVH, then terrain fallback) — shared with deferred_rt.
    float4 result = float4(0.0f, 0.0f, 0.0f, 100000.0f);
    float3 dir = ray_dir;
    float3 ro = ray_origin;
    float3 normal = float3(0.0f, 1.0f, 0.0f);
    uint hit_type = 0u;
    float hit_t = 100000.0f;
    uint bounce = 0u;
    for (; bounce < 1u; ++bounce)
    {
        float4 bn = ((blue_noise_texture.Load(int3((px + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_index + 1234567u) + (bounce * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f) + 0.001953125f;

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
            break;
        }
    }

    // Sky color for unoccluded rays (uses the ray's y component).
    if (hit_type == 0u)
    {
        float3 sky = (ray_dir.y > 0.0f)
            ? lerp(sky_low_color.xyz, sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx)
            : ((glade_theme_id == 2u) ? lerp(sky_low_color.xyz, sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx) : result.xyz);
        result = float4(sky, result.w);
    }

    // Resolve: reproject the ray endpoint, blend history / sky, accumulate distance + radiance.
    float4 sp = mul(view_constants[0].view_to_sample,
                    mul(view_constants[0].world_to_view, float4(ray_origin + (ray_dir * result.w), 1.0f)));
    float3 sp_n = sp.xyz / sp.w;
    float2 sp_uv = (sp_n.xy * float2(0.5f, -0.5f)) + 0.5f;
    float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, sp_uv, 0.0f);
    bool in_view = all(bool3(abs(sp_n.x) < 1.0f, abs(sp_n.y) < 1.0f, abs(sp_n.z) < 1.0f)) && sp_n.z > 0.0f;
    bool occluded = in_view && half_depth_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).x < (sp_n.z * 1.1f);

    float3 radiance;
    if (occluded)
    {
        radiance = (history_valid != 0u && frame_index != 0u)
            ? taa_history_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).xyz : 0.0f;
    }
    else
    {
        radiance = 0.0f;
    }
    float sky_amt = occluded ? 0.0f : float(result.w > 10000.0f);
    if (sky_amt > 0.0f)
    {
        radiance = radiance + (lerp(sky_low_color.xyz, sky_high_color.xyz,
            pow(clamp(1.0f - pow(1.0f - clamp(ray_dir.y + 0.2f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.65f).xxx) * sky_amt);
    }

    float2 prev_hit = refl_hit_tex[px];
    refl_hit_tex[px] = float2(prev_hit.x + result.w, prev_hit.y);

    // Pack radiance into a 3x9-bit mantissa + 5-bit shared exponent word.
    float3 clamp_rad = clamp(radiance, 0.0f, 65408.0f);
    float exp_f = asfloat((asuint(max(max(1.52587890625e-05f, clamp_rad.x), max(clamp_rad.y, clamp_rad.z)))
        + 125845504u) & 2139095040u);
    uint3 mant = asuint(clamp_rad + exp_f);
    refl_hit_radiance_tex[px] = (((asuint(exp_f) << 4u) + 268435456u) | (mant.z << 18u))
        | (mant.y << 9u) | (mant.x & 511u);
}
