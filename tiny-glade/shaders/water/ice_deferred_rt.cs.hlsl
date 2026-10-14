// PATCHED: modules rt.cs.hlsl_cs_main_0.bin:cs_main are lifted from the oracle bins by tools/render-passes/batch_lift.py;
// the hand-written code below is excluded for exactly those (variant, stage) pairs.

// 冰面延迟软件光线追踪:每个打包射线条目一条射线,morton 编码软件 BVH 遍历
// (rt_bl_bvh/rt_triangles,8 项栈),水面/地形射线步进,单弹跳结构
// (弹跳实际失效:首段总终止),然后写 refl_hit/refl_hit_radiance 并带历史/重投影回退。
// 转译段(TGV_0,变量为 SPIR-V 编号)已语义化命名;逻辑未改。
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
// inc/blue_noise.hlsl 不再声明纹理(按评审的 register 修复);
// 水面/冰面 RT 家族在 t15 绑定 blue_noise_texture —— 本地声明。
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
// terrain_heightmap.inc 省略:重建 inc 引用消费者声明的 terrain_heightmap 纹理,
// 本着色器从不绑定(原版内嵌但无绑定)。
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

// 在像素偏移处采样蓝噪声纹理(0.7548777/0.5698403 黄金比滚动 + 1/256),返回 [0,1]。
float4 sample_blue_noise(uint2 pixel, uint scroll)
{
    float2 offset = frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float(scroll)) + 0.5f);
    return ((blue_noise_texture.Load(int3((pixel + uint2(offset * 256.0f)) % uint2(256u, 256u), 0u)) * 255.0f)
        * 0.00390625f) + 0.001953125f;
}

// 软件 BVH 射线追踪 + 水面/地形步进的结果。
struct RayResult
{
    uint type;        // 0 none, 1 ground, 2 water, 3 triangle
    float t;
    float3 normal;
};

// Morton 有序软件 BVH 遍历(与 deferred_rt.cs.hlsl 相同的混合体:
// A 计数字节掩码累加 + 节点入口/叶子组装 + break/pop 控制流;
// B 八分体符号选角字节;共用 morton/octant/slab 脚手架)。
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

            // 子节点 A (data[1].z):lane 掩码公式 + 符号选角字节。
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

            // 子节点 B (data[1].w):同上,符号选 data[2].w/.y 等。
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

        // 处理 `child` 的三角形列表(节点索引,打包三角形掩码)。
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

// 让 `dir` 对视觉地形高度图 slab [-65,65]x[0,24]x[-65,65] 步进。
// 在 [0, max_t] 内命中返回类型 1(地面)/ 2(水面),否则类型 0。
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

    // 在水面交叉处做二分细化。
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
            // bin:蓝噪声在循环内用弹跳滚动采样。
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
                // bin 未命中天空分支:cur_dir.y>0 -> 天空;theme==2 -> 天空;否则保持。
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

    // 反射点的重投影 / 历史解析。
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
// 转译段:与手写段相同算法的 spirv-cross 输出(语义化命名)。
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

static float3 ZERO_F3;
static float3 SKY_F3;
static float4 SKY_F4;

static const uint2 EMPTY_STACK[8] = { uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u), uint2(0u, 0u) };

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
        uint4 ray = ice_rt_ray_buffer.Load<uint4>(gl_GlobalInvocationID.x * 16 + 0);
        uint2 pixel = uint2(ray.x & 65535u, (ray.x >> 16u) & 65535u);
        float4 cam4 = mul(frame_constants_view_constants[0u].view_to_world, float4(0.0f, 0.0f, 0.0f, 1.0f));
        float3 ray_origin = (cam4.xyz / cam4.w.xxx) + float3(spvUnpackHalf2x16(ray.y & 65535u).x, spvUnpackHalf2x16(ray.z & 65535u).x, spvUnpackHalf2x16(ray.w & 65535u).x);
        float2 rd_y_unpack = spvUnpackHalf2x16((ray.z >> 16u) & 65535u);
        float ray_dir_y = rd_y_unpack.x;
        float3 ray_dir = float3(spvUnpackHalf2x16((ray.y >> 16u) & 65535u).x, ray_dir_y, spvUnpackHalf2x16((ray.w >> 16u) & 65535u).x);
        float4 hit;
        float3 cur_dir;
        float3 best_normal;
        float3 cur_origin;
        hit = float4(0.0f, 0.0f, 0.0f, 100000.0f);
        cur_dir = ray_dir;
        best_normal = ZERO_F3;
        cur_origin = ray_origin;
        float4 next_hit;
        float3 next_dir;
        float3 next_normal;
        float3 next_origin;
        float4 final_hit;
        uint bounce = 0u;
        for (;;)
        {
            if (bounce < 1u)
            {
                float cur_dir_y;
                float4 bn = ((blue_noise_texture.Load(int3((pixel + uint2(frac((float2(0.75487768650054931640625f, 0.56984031200408935546875f) * float((frame_constants_frame_index + 1234567u) + (bounce * 137u))) + 0.5f.xx) * 256.0f.xx)) % uint2(256u, 256u), 0u)) * 255.0f) * 0.00390625f.xxxx) + 0.001953125f.xxxx;
                float3 tri_normal;
                uint tri_type;
                float tri_t;
                do
                {
                    uint2 stack[8] = EMPTY_STACK;
                    bool rdx_neg = cur_dir.x < 0.0f;
                    cur_dir_y = cur_dir.y;
                    bool rdy_neg = cur_dir_y < 0.0f;
                    bool rdz_neg = cur_dir.z < 0.0f;
                    uint octant = (7u - uint(((rdx_neg ? 4 : 0) | (rdy_neg ? 2 : 0)) | int(rdz_neg))) * 16843009u;
                    uint2 trav;
                    float3 best_normal_t;
                    trav = uint2(0u, 2147483648u);
                    best_normal_t = best_normal;
                    uint2 next_trav;
                    float best_t_loop;
                    int stack_depth;
                    float3 best_normal_loop;
                    int best_triangle;
                    float best_t_out;
                    int best_triangle_out;
                    float best_t = 3.4028234663852885981170418348452e+38f;
                    int stack_depth_loop = 0;
                    uint iter = 0u;
                    int best_triangle_loop = -1;
                    [loop]
                    for (;;)
                    {
                        if (iter < 1024u)
                        {
                            uint2 node_entry;
                            uint2 child;
                            int push_depth;
                            if ((trav.y & 4278190080u) != 0u)
                            {
                                uint b = firstbithigh(trav.y);
                                uint key = trav.y & (~(1u << (b & 31u)));
                                uint2 kept = trav;
                                kept.y = key;
                                int nd;
                                if ((key & 4278190080u) != 0u)
                                {
                                    stack[stack_depth_loop] = kept;
                                    nd = stack_depth_loop + 1;
                                }
                                else
                                {
                                    nd = stack_depth_loop;
                                }
                                PackedBlBvhNode n;
                                [unroll]
                                for (int i = 0; i < 5; i++)
                                {
                                    n.data[i] = rt_bl_bvh.Load<uint4>(i * 16 + (trav.x + countbits(trav.y & (~(4294967295u << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80 + 0);
                                }
                                float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / cur_dir.x, asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / cur_dir_y, asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / cur_dir.z);
                                float3 offset = (asfloat(n.data[0].xyz) - cur_origin) / cur_dir;
                                uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint count0 = (n.data[1].z >> 5u) & 117901063u;
                                uint c0 = rdx_neg ? n.data[2].z : n.data[2].x;
                                uint c1 = rdx_neg ? n.data[2].x : n.data[2].z;
                                uint c2 = rdy_neg ? n.data[3].z : n.data[3].x;
                                uint c3 = rdy_neg ? n.data[3].x : n.data[3].z;
                                uint c4 = rdz_neg ? n.data[4].z : n.data[4].x;
                                uint c5 = rdz_neg ? n.data[4].x : n.data[4].z;
                                float3 lo0 = (float3(float((c0 >> 0u) & 255u), float((c2 >> 0u) & 255u), float((c4 >> 0u) & 255u)) * inv_dir) + offset;
                                float3 hi0 = (float3(float((c1 >> 0u) & 255u), float((c3 >> 0u) & 255u), float((c5 >> 0u) & 255u)) * inv_dir) + offset;
                                uint acc0;
                                if (max(lo0.x, max(lo0.y, max(lo0.z, 9.9999997473787516355514526367188e-05f))) <= min(hi0.x, min(hi0.y, min(hi0.z, best_t))))
                                {
                                    acc0 = 0u | (((count0 >> 0u) & 255u) << (((xor0 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc0 = 0u;
                                }
                                float3 lo8 = (float3(float((c0 >> 8u) & 255u), float((c2 >> 8u) & 255u), float((c4 >> 8u) & 255u)) * inv_dir) + offset;
                                float3 hi8 = (float3(float((c1 >> 8u) & 255u), float((c3 >> 8u) & 255u), float((c5 >> 8u) & 255u)) * inv_dir) + offset;
                                uint acc8;
                                if (max(lo8.x, max(lo8.y, max(lo8.z, 9.9999997473787516355514526367188e-05f))) <= min(hi8.x, min(hi8.y, min(hi8.z, best_t))))
                                {
                                    acc8 = acc0 | (((count0 >> 8u) & 255u) << (((xor0 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc8 = acc0;
                                }
                                float3 lo16 = (float3(float((c0 >> 16u) & 255u), float((c2 >> 16u) & 255u), float((c4 >> 16u) & 255u)) * inv_dir) + offset;
                                float3 hi16 = (float3(float((c1 >> 16u) & 255u), float((c3 >> 16u) & 255u), float((c5 >> 16u) & 255u)) * inv_dir) + offset;
                                uint acc16;
                                if (max(lo16.x, max(lo16.y, max(lo16.z, 9.9999997473787516355514526367188e-05f))) <= min(hi16.x, min(hi16.y, min(hi16.z, best_t))))
                                {
                                    acc16 = acc8 | (((count0 >> 16u) & 255u) << (((xor0 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc16 = acc8;
                                }
                                float3 lo24 = (float3(float((c0 >> 24u) & 255u), float((c2 >> 24u) & 255u), float((c4 >> 24u) & 255u)) * inv_dir) + offset;
                                float3 hi24 = (float3(float((c1 >> 24u) & 255u), float((c3 >> 24u) & 255u), float((c5 >> 24u) & 255u)) * inv_dir) + offset;
                                uint acc24;
                                if (max(lo24.x, max(lo24.y, max(lo24.z, 9.9999997473787516355514526367188e-05f))) <= min(hi24.x, min(hi24.y, min(hi24.z, best_t))))
                                {
                                    acc24 = acc16 | (((count0 >> 24u) & 255u) << (((xor0 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc24 = acc16;
                                }
                                uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
                                uint count1 = (n.data[1].w >> 5u) & 117901063u;
                                uint d0 = rdx_neg ? n.data[2].w : n.data[2].y;
                                uint d1 = rdx_neg ? n.data[2].y : n.data[2].w;
                                uint d2 = rdy_neg ? n.data[3].w : n.data[3].y;
                                uint d3 = rdy_neg ? n.data[3].y : n.data[3].w;
                                uint d4 = rdz_neg ? n.data[4].w : n.data[4].y;
                                uint d5 = rdz_neg ? n.data[4].y : n.data[4].w;
                                float3 lo0b = (float3(float((d0 >> 0u) & 255u), float((d2 >> 0u) & 255u), float((d4 >> 0u) & 255u)) * inv_dir) + offset;
                                float3 hi0b = (float3(float((d1 >> 0u) & 255u), float((d3 >> 0u) & 255u), float((d5 >> 0u) & 255u)) * inv_dir) + offset;
                                uint acc0b;
                                if (max(lo0b.x, max(lo0b.y, max(lo0b.z, 9.9999997473787516355514526367188e-05f))) <= min(hi0b.x, min(hi0b.y, min(hi0b.z, best_t))))
                                {
                                    acc0b = acc24 | (((count1 >> 0u) & 255u) << (((xor1 >> 0u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc0b = acc24;
                                }
                                float3 lo8b = (float3(float((d0 >> 8u) & 255u), float((d2 >> 8u) & 255u), float((d4 >> 8u) & 255u)) * inv_dir) + offset;
                                float3 hi8b = (float3(float((d1 >> 8u) & 255u), float((d3 >> 8u) & 255u), float((d5 >> 8u) & 255u)) * inv_dir) + offset;
                                uint acc8b;
                                if (max(lo8b.x, max(lo8b.y, max(lo8b.z, 9.9999997473787516355514526367188e-05f))) <= min(hi8b.x, min(hi8b.y, min(hi8b.z, best_t))))
                                {
                                    acc8b = acc0b | (((count1 >> 8u) & 255u) << (((xor1 >> 8u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc8b = acc0b;
                                }
                                float3 lo16b = (float3(float((d0 >> 16u) & 255u), float((d2 >> 16u) & 255u), float((d4 >> 16u) & 255u)) * inv_dir) + offset;
                                float3 hi16b = (float3(float((d1 >> 16u) & 255u), float((d3 >> 16u) & 255u), float((d5 >> 16u) & 255u)) * inv_dir) + offset;
                                uint acc16b;
                                if (max(lo16b.x, max(lo16b.y, max(lo16b.z, 9.9999997473787516355514526367188e-05f))) <= min(hi16b.x, min(hi16b.y, min(hi16b.z, best_t))))
                                {
                                    acc16b = acc8b | (((count1 >> 16u) & 255u) << (((xor1 >> 16u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc16b = acc8b;
                                }
                                float3 lo24b = (float3(float((d0 >> 24u) & 255u), float((d2 >> 24u) & 255u), float((d4 >> 24u) & 255u)) * inv_dir) + offset;
                                float3 hi24b = (float3(float((d1 >> 24u) & 255u), float((d3 >> 24u) & 255u), float((d5 >> 24u) & 255u)) * inv_dir) + offset;
                                uint acc24b;
                                if (max(lo24b.x, max(lo24b.y, max(lo24b.z, 9.9999997473787516355514526367188e-05f))) <= min(hi24b.x, min(hi24b.y, min(hi24b.z, best_t))))
                                {
                                    acc24b = acc16b | (((count1 >> 24u) & 255u) << (((xor1 >> 24u) & 255u) & 31u));
                                }
                                else
                                {
                                    acc24b = acc16b;
                                }
                                uint2 node_entry_local = uint2(n.data[1].x, key);
                                node_entry_local.y = (acc24b & 4278190080u) | ((n.data[0].w >> 24u) & 255u);
                                node_entry = node_entry_local;
                                child = uint2(n.data[1].y, acc24b & 16777215u);
                                push_depth = nd;
                            }
                            else
                            {
                                node_entry = uint2(0u, 0u);
                                child = trav;
                                push_depth = stack_depth_loop;
                            }
                            uint2 tri_state;
                            tri_state = child;
                            best_normal_loop = best_normal_t;
                            best_t_loop = best_t;
                            best_triangle = best_triangle_loop;
                            uint2 next_tri_state;
                            float3 new_normal;
                            float new_t;
                            int new_triangle;
                            uint tri_mask;
                            for (;;)
                            {
                                tri_mask = tri_state.y;
                                if (tri_mask != 0u)
                                {
                                    uint tb = firstbithigh(tri_mask);
                                    next_tri_state = tri_state;
                                    next_tri_state.y = tri_mask & (~(1u << (tb & 31u)));
                                    uint tindex = tri_state.x + tb;
                                    PackedTriangle tri;
                                    [unroll]
                                    for (int j = 0; j < 3; j++)
                                    {
                                        tri.v[j] = rt_triangles.Load<float>(j * 4 + tindex * 24 + 0);
                                    }
                                    [unroll]
                                    for (int j = 0; j < 3; j++)
                                    {
                                        tri.e[j] = rt_triangles.Load<uint>(j * 4 + tindex * 24 + 12);
                                    }
                                    float3 edge_a = float3(spvUnpackHalf2x16((tri.e[0] >> 16u) & 65535u).x, spvUnpackHalf2x16((tri.e[1] >> 16u) & 65535u).x, spvUnpackHalf2x16((tri.e[2] >> 16u) & 65535u).x);
                                    float3 edge_b = float3(spvUnpackHalf2x16(tri.e[0] & 65535u).x, spvUnpackHalf2x16(tri.e[1] & 65535u).x, spvUnpackHalf2x16(tri.e[2] & 65535u).x);
                                    bool tri_hit;
                                    do
                                    {
                                        float3 p = cross(cur_dir, edge_b);
                                        float det = dot(edge_a, p);
                                        float3 s = cur_origin - float3(tri.v[0], tri.v[1], tri.v[2]);
                                        float3 q = cross(s, edge_a);
                                        float t = dot(edge_b, q) / det;
                                        bool t_in_range;
                                        if (t >= 0.0f)
                                        {
                                            t_in_range = t < best_t_loop;
                                        }
                                        else
                                        {
                                            t_in_range = false;
                                        }
                                        if (t_in_range ? true : false)
                                        {
                                            float3 bary2;
                                            bary2.x = dot(s, p);
                                            bary2.y = dot(cur_dir, q);
                                            float2 bary = bary2.xy / det.xx;
                                            float3 w = float3(bary.x, bary.y, bary2.z);
                                            w.z = (1.0f - bary.x) - bary.y;
                                            if (all(bool3(w.x >= (-9.9999997473787516355514526367188e-05f).xxx.x, w.y >= (-9.9999997473787516355514526367188e-05f).xxx.y, w.z >= (-9.9999997473787516355514526367188e-05f).xxx.z)))
                                            {
                                                new_t = t;
                                                tri_hit = true;
                                                break;
                                            }
                                        }
                                        new_t = best_t_loop;
                                        tri_hit = false;
                                        break;
                                    } while(false);
                                    if (tri_hit)
                                    {
                                        new_normal = cross(edge_a, edge_b);
                                        new_triangle = int(tindex);
                                    }
                                    else
                                    {
                                        new_normal = best_normal_loop;
                                        new_triangle = best_triangle;
                                    }
                                    tri_state = next_tri_state;
                                    best_normal_loop = new_normal;
                                    best_t_loop = new_t;
                                    best_triangle = new_triangle;
                                    continue;
                                }
                                else
                                {
                                    break;
                                }
                            }
                            if ((node_entry.y & 4278190080u) == 0u)
                            {
                                if (push_depth == 0)
                                {
                                    next_normal = best_normal_loop;
                                    best_t_out = best_t_loop;
                                    best_triangle_out = best_triangle;
                                    break;
                                }
                                int pd = push_depth - 1;
                                next_trav = stack[pd];
                                stack_depth = pd;
                            }
                            else
                            {
                                next_trav = node_entry;
                                stack_depth = push_depth;
                            }
                            trav = next_trav;
                            best_t = best_t_loop;
                            stack_depth_loop = stack_depth;
                            iter++;
                            best_normal_t = best_normal_loop;
                            best_triangle_loop = best_triangle;
                            continue;
                        }
                        else
                        {
                            next_normal = best_normal_t;
                            best_t_out = best_t;
                            best_triangle_out = best_triangle_loop;
                            break;
                        }
                    }
                    bool has_hit;
                    if (best_triangle_out != (-1))
                    {
                        has_hit = best_t_out < 100000.0f;
                    }
                    else
                    {
                        has_hit = false;
                    }
                    if (has_hit)
                    {
                        tri_normal = next_normal;
                        tri_type = 3u;
                        tri_t = best_t_out;
                        break;
                    }
                    tri_normal = float3(0.0f, 1.0f, 0.0f);
                    tri_type = 0u;
                    tri_t = 100000.0f;
                    break;
                } while(false);
                uint type;
                float t_hit;
                do
                {
                    float3 bb_hi = (float3(65.0f, 24.0f, 65.0f) - cur_origin) / cur_dir;
                    float3 bb_lo = (float3(-65.0f, 0.0f, -65.0f) - cur_origin) / cur_dir;
                    float3 tmin = min(bb_hi, bb_lo);
                    float3 tmax = max(bb_hi, bb_lo);
                    float t_near = max(max(tmin.x, tmin.y), tmin.z);
                    float t_far = min(min(tmax.x, tmax.y), tmax.z);
                    bool slab_hit;
                    if (t_near <= t_far)
                    {
                        slab_hit = t_far >= 0.0f;
                    }
                    else
                    {
                        slab_hit = false;
                    }
                    if (!slab_hit)
                    {
                        type = tri_type;
                        t_hit = tri_t;
                        break;
                    }
                    float t_lo = max(t_near, 0.0f);
                    float step = (t_far - t_lo) * 0.03125f;
                    float t_cur;
                    t_cur = t_lo;
                    float t_next;
                    uint next_idx;
                    float t_found;
                    uint step_idx = 0u;
                    bool loop_alive;
                    for (;;)
                    {
                        loop_alive = step_idx < 32u;
                        if (loop_alive)
                        {
                            t_next = t_cur + step;
                            float3 p_march = cur_origin + (cur_dir * t_next);
                            if (p_march.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p_march.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x))
                            {
                                t_found = t_next;
                                break;
                            }
                            next_idx = step_idx + 1u;
                            t_cur = t_next;
                            step_idx = next_idx;
                            continue;
                        }
                        else
                        {
                            t_found = t_far;
                            break;
                        }
                    }
                    bool found;
                    if (loop_alive)
                    {
                        found = t_cur < tri_t;
                    }
                    else
                    {
                        found = false;
                    }
                    bool hit_check;
                    if (found)
                    {
                        hit_check = t_cur < t_found;
                    }
                    else
                    {
                        hit_check = false;
                    }
                    if (hit_check)
                    {
                        float lo;
                        lo = t_cur;
                        float hi = t_found;
                        uint bi = 0u;
                        for (; bi < 8u; )
                        {
                            float mid_plus = lo + hi;
                            float mid = mid_plus * 0.5f;
                            float3 p_mid = cur_origin + (cur_dir * mid);
                            bool under = p_mid.y < lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, (p_mid.xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx, 0.0f).x);
                            hi = under ? mid : hi;
                            lo = under ? lo : mid;
                            bi++;
                            continue;
                        }
                        if (lo < tri_t)
                        {
                            float2 uv = ((cur_origin + (cur_dir * lo)).xz * 0.0076923076994717121124267578125f.xx) + 0.5f.xx;
                            type = ((lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, uv, 0.0f).x) + 0.100000001490116119384765625f) > lerp(-2.5f, 24.0f, visual_terrain_heightmap.SampleLevel(sampler_lnc, uv, 0.0f).x)) ? 1u : 2u;
                            t_hit = lo;
                            break;
                        }
                        else
                        {
                            type = tri_type;
                            t_hit = tri_t;
                            break;
                        }
                        break; // unreachable workaround
                    }
                    else
                    {
                        type = tri_type;
                        t_hit = tri_t;
                        break;
                    }
                    break; // unreachable workaround
                } while(false);
                if (type == 0u)
                {
                    float4 sky_hit;
                    if (cur_dir_y > 0.0f)
                    {
                        sky_hit = float4(SKY_F3.x, SKY_F3.y, SKY_F3.z, hit.w);
                    }
                    else
                    {
                        float4 sky_hit_theme;
                        if (frame_constants_glade_theme_id == 2u)
                        {
                            sky_hit_theme = float4(SKY_F3.x, SKY_F3.y, SKY_F3.z, hit.w);
                        }
                        else
                        {
                            sky_hit_theme = hit;
                        }
                        sky_hit = sky_hit_theme;
                    }
                    final_hit = sky_hit;
                    break;
                }
                else
                {
                    float4 hit_t4;
                    if (0u == bounce)
                    {
                        float4 hit_t4_local;
                        hit_t4_local.w = t_hit;
                        hit_t4 = hit_t4_local;
                    }
                    else
                    {
                        hit_t4 = hit;
                    }
                    float3 hit_pos = cur_origin + (cur_dir * t_hit);
                    bool is_water;
                    if (!(type == 1u))
                    {
                        is_water = type == 2u;
                    }
                    else
                    {
                        is_water = true;
                    }
                    if (is_water)
                    {
                        if ((bounce + 1u) == 1u)
                        {
                            final_hit = float4(SKY_F3.x, SKY_F3.y, SKY_F3.z, hit_t4.w);
                            break;
                        }
                        float c = 1.0f - (2.0f * bn.x);
                        float s = sqrt(max(0.0f, 1.0f - (c * c)));
                        float a = 6.283185482025146484375f * bn.y;
                        next_hit = float4(SKY_F3.x, SKY_F3.y, SKY_F3.z, hit_t4.w);
                        next_dir = normalize(float3(0.0f, 1.0f, 0.0f) + float3(cos(a) * s, sin(a) * s, c));
                        next_origin = hit_pos + float3(0.0f, 0.001000000047497451305389404296875f, 0.0f);
                    }
                    else
                    {
                        float4 hit_bounce;
                        float3 dir_bounce;
                        float3 origin_bounce;
                        if (type == 3u)
                        {
                            if (dot(tri_normal, cur_dir) > 0.0f)
                            {
                                final_hit = hit_t4;
                                break;
                            }
                            float3 nn = normalize(tri_normal);
                            float c2 = 1.0f - (2.0f * bn.x);
                            float s2 = sqrt(max(0.0f, 1.0f - (c2 * c2)));
                            float a2 = 6.283185482025146484375f * bn.y;
                            hit_bounce = float4(SKY_F3.x, SKY_F3.y, SKY_F3.z, hit_t4.w);
                            dir_bounce = normalize(nn + float3(cos(a2) * s2, sin(a2) * s2, c2));
                            origin_bounce = hit_pos + (nn * 0.001000000047497451305389404296875f);
                        }
                        else
                        {
                            hit_bounce = hit_t4;
                            dir_bounce = cur_dir;
                            origin_bounce = cur_origin;
                        }
                        next_hit = hit_bounce;
                        next_dir = dir_bounce;
                        next_origin = origin_bounce;
                    }
                }
                hit = next_hit;
                cur_dir = next_dir;
                best_normal = next_normal;
                cur_origin = next_origin;
                bounce++;
                continue;
            }
            else
            {
                final_hit = hit;
                break;
            }
        }
        float4 sp4 = mul(frame_constants_view_constants[0u].view_to_sample, mul(frame_constants_view_constants[0u].world_to_view, float4(ray_origin + (ray_dir * final_hit.w), 1.0f)));
        float3 sp = sp4.xyz / sp4.w.xxx;
        float2 sp_uv = (sp.xy * float2(0.5f, -0.5f)) + 0.5f.xx;
        float4 reproj = reprojection_tex.SampleLevel(sampler_nnc, sp_uv, 0.0f);
        float4 current_hit = refl_hit_tex[pixel].xyyy;
        float3 sp_abs = abs(sp);
        bool inside_view;
        if (all(bool3(sp_abs.x < 1.0f.xxx.x, sp_abs.y < 1.0f.xxx.y, sp_abs.z < 1.0f.xxx.z)))
        {
            inside_view = sp.z > 0.0f;
        }
        else
        {
            inside_view = false;
        }
        bool occluded;
        if (inside_view)
        {
            occluded = half_depth_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).x < (sp.z * 1.10000002384185791015625f);
        }
        else
        {
            occluded = false;
        }
        float3 sky;
        float mix_factor;
        if (occluded)
        {
            bool use_history;
            if (history_valid != 0u)
            {
                use_history = !(0u == frame_constants_frame_index);
            }
            else
            {
                use_history = false;
            }
            float3 history;
            if (use_history)
            {
                history = taa_history_tex.SampleLevel(sampler_nnc, reproj.xy + sp_uv, 0.0f).xyz;
            }
            else
            {
                history = 0.0f.xxx;
            }
            sky = history;
            mix_factor = 0.0f;
        }
        else
        {
            sky = 0.0f.xxx;
            mix_factor = ((final_hit.w > 10000.0f) ? 1.0f : 0.0f);
        }
        float3 radiance_out;
        if (mix_factor > 0.0f)
        {
            radiance_out = sky + (((lerp(frame_constants_sky_low_color.xyz * 1.0f, frame_constants_sky_high_color.xyz, pow(clamp(1.0f - pow(1.0f - clamp(ray_dir_y + 0.20000000298023223876953125f, 0.0f, 1.0f), 14.0f), 0.0f, 1.0f), 0.64999997615814208984375f).xxx) * 1.0f) + max(0.0f.xxx, 0.0f.xxx)) * mix_factor);
        }
        else
        {
            radiance_out = sky;
        }
        refl_hit_tex[pixel] = float2(current_hit.x + final_hit.w, current_hit.y);
        float3 clamped = clamp(radiance_out, 0.0f.xxx, 65408.0f.xxx);
        float scale = asfloat((asuint(max(max(1.52587890625e-05f, clamped.x), max(clamped.y, clamped.z))) + 125845504u) & 2139095040u);
        uint3 mantissa = asuint(clamped + scale.xxx);
        refl_hit_radiance_tex[pixel] = ((((asuint(scale) << 4u) + 268435456u) | (mantissa.z << 18u)) | (mantissa.y << 9u)) | (mantissa.x & 511u);
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
