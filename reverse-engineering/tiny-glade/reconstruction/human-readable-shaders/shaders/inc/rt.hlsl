// ======================================================================
// inc/rt.hlsl —— 软件光线追踪(BVH)公共头文件
// ======================================================================
// 用途:
//   为 GI 追踪(_gi_trace_gi_candidates)、水面延迟光追(_water_deferred_rt、
//   _water_ice_deferred_rt)提供软件 BVH 的数据结构、三角形求交与莫顿编码
//   遍历。该头文件被上述着色器按需 include。
//
// 关键数据结构:
//   PackedBlBvhNode  —— 80 字节节点:data[0].xyz 是量化后的最小角 float3,
//                       data[0].w 三个字节表示每轴以 2^byte 编码的包围盒尺寸,
//                       data[1].z/w 为打包的子节点槽位,data[2..4] 存放 4 个
//                       子节点的量化角点对。
//   PackedTriangle   —— 24 字节三角形:3 个顶点 + 3 个打包的边/UV 数据。
//
// 关键函数:
//   load_packed_triangle      按索引加载三角形
//   ray_triangle_intersect    射线/三角形求交(Möller–Trumbore)
//   rt_unpack_half2x16        从 uint 拆出两个 half
//   trace_software_bvh        莫顿编码软件 BVH 遍历(与 oracle bin 逐指令一致)
//
// 绑定:rt_bl_bvh / rt_triangles 为 ByteAddressBuffer;遍历使用调用方声明的
//       groupshared uint2 rt_lds_stack[256](定义 TINYGLADE_RT_LDS_STACK_SPILL
//       时启用栈溢出落盘)。
// ======================================================================
// ==RECONSTRUCTED== shaders/inc/rt.hlsl | evidence: spirv-cross (_gi_trace_gi_candidates/_water_deferred_rt/_water_ice_deferred_rt) PackedBlBvhNode{uint4 data[5]} 80B (data[0].xyz=quantized min float, data[0].w=max-corner byte extents 2^b, data[1].z/w=packed child slots, data[2..4]=4 quantized child corner pairs), PackedTriangle{float v[3]; uint e[3]} 24B (v=vertices, e=packed edge/uv data); rt_bl_bvh/rt_triangles/rt_triangle_meta ByteAddressBuffers; include-graph edge rt.hlsl -> shadow.hlsl | status: O (structs/buffers); O/I (morton child-index decode + traversal Inference)
// MERGE (water-ice review): trace_software_bvh rewritten as the morton-coded uint2-stack
// traversal transcribed from the embedded _water_deferred_rt/_water_ice_deferred_rt bins
// (review water-ice bvh_traversal_finding). Hybrid of candidate A (count-byte mask
// accumulation `((count>>shift)&255)<<((mask>>shift)&31)`, node entry
// (data[1].x,(accum&0xFF000000)|(data[0].w>>24&255)) + leaf (data[1].y,accum&0xFFFFFF),
// break/pop control flow) and candidate B (octant sign-selected corner bytes
// rd.x<0?data[2].z:data[2].x etc.). Extent decode fixed to asfloat(byte<<23) (2^(byte-127);
// the old exp2(float(byte)) form was wrong). decode_bvh_corner/load_child_aabb removed
// (the bin traversal uses the direct slab form, not the quantized-corner decode).
// The old non-morton trace_software_bvh did NOT match the bins.
// MERGE (gi-deferred review, RECONCILED 2026-10-13): the _gi_trace_gi_candidates bin
// (SPIR-V offset 23, decompiled offsets ~15002-17700) confirms the hybrid morton decode
// already present here (compact-index node entry, 2-child x 4-byte-slot corner decode,
// extent asfloat(byte<<23), ray-slot XOR (7-sign)*0x01010101) AND shows the stack is a
// per-lane uint2[4] local stack spilling to groupshared rt_lds_stack[256] at
// lane + (sp-4)*64. trace_software_bvh now takes that 4-entry stack and a lane index;
// consumers that declare `groupshared uint2 rt_lds_stack[256];` and define
// TINYGLADE_RT_LDS_STACK_SPILL before this include get the groupshared spill.
// NOTE (review delta rejected): the review prose described the corner bytes as 3 nested
// box levels and the buffers as StructuredBuffer<PackedBlBvhNode>. The trace bin
// decompile uses FOUR byte positions (0/8/16/24) per child and ByteAddressBuffer.Load
// (both sides hard-gate PASS with ByteAddressBuffer), so those two review claims are
// NOT applied.

#ifndef TINYGLADE_INC_RT_HLSL
#define TINYGLADE_INC_RT_HLSL

#include "shadow.hlsl"

// 软件 BVH 节点:5 个 uint4(80 字节)。包围盒为量化存储:
// data[0].xyz 是最小角 float3,data[0].w 的三个字节按 2^byte 编码每轴尺寸;
// 最多 4 个孩子的 AABB 各编码为两个字节三元组:
//   child A = data[2].x/.z, data[3].x/.z, data[4].x/.z
//   child B = data[2].y/.w, data[3].y/.w, data[4].y/.w
struct PackedBlBvhNode {
    uint4 data[5];
};

// 软件 BVH 三角形(24 字节):3 个顶点 + 3 个打包的边/alpha 字。
struct PackedTriangle {
    float v[3];
    uint e[3];
};

// 从软件 BVH 三角形缓冲按索引加载三角形。
PackedTriangle load_packed_triangle(ByteAddressBuffer rt_triangles, uint index)
{
    PackedTriangle tri;
    [unroll]
    for (uint i = 0u; i < 3u; ++i) {
        tri.v[i] = rt_triangles.Load<float>(i * 4u + index * 24u + 0u);
        tri.e[i] = rt_triangles.Load<uint>(i * 4u + index * 24u + 12u);
    }
    return tri;
}

// Möller–Trumbore 射线/三角形求交;命中时返回距离 t。
bool ray_triangle_intersect(float3 ro, float3 rd, float3 v0, float3 v1, float3 v2,
                            float t_max, out float t, out float2 bary)
{
    t = 0.0;
    bary = float2(0.0, 0.0);
    float3 e1 = v1 - v0;
    float3 e2 = v2 - v0;
    float3 p = cross(rd, e2);
    float det = dot(e1, p);
    if (abs(det) < 1e-9) {
        return false;
    }
    float inv_det = 1.0 / det;
    float3 s = ro - v0;
    float u = dot(s, p) * inv_det;
    if (u < 0.0 || u > 1.0) {
        return false;
    }
    float3 q = cross(s, e1);
    float v = dot(rd, q) * inv_det;
    if (v < 0.0 || (u + v) > 1.0) {
        return false;
    }
    t = dot(e2, q) * inv_det;
    bary = float2(u, v);
    return t >= 0.0 && t <= t_max;
}

// 从 uint 的低/高 16 位拆出 half2(用于打包的三角形边数据)。
float2 rt_unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

// 莫顿编码软件 BVH 遍历(由 deferred_rt / ice_deferred_rt bin 逐指令转写)。
// `stack` 是每 lane 的 4 槽栈;溢出时,若调用方定义了 TINYGLADE_RT_LDS_STACK_SPILL
// 并在 include 前声明了 groupshared uint2 rt_lds_stack[256](GI 追踪即如此),
// 则落盘到 rt_lds_stack[lane + (sp-4)*64]。命中返回 true,并回填
// hit_triangle / hit_t / hit_bary。
bool trace_software_bvh(ByteAddressBuffer rt_bl_bvh,
                        ByteAddressBuffer rt_triangles,
                        uint node_count,
                        uint lane,
                        float3 ro, float3 rd, float t_min, float t_max,
                        inout uint2 stack[4], inout uint stack_size,
                        out uint hit_triangle, out float hit_t, out float2 hit_bary)
{
    hit_triangle = 0xFFFFFFFFu;
    hit_t = t_max;
    hit_bary = float2(0.0f, 0.0f);

    uint2 trav = uint2(0u, 0x80000000u);
    float3 best_normal = 0.0f;
    stack_size = 0u;
    int best_triangle = -1;
    float best_t = 3.4028234663852885981170418348452e+38f;

    // 射线方向的八分位置换字节(每个八分位 0x01010101),提到遍历循环外,
    // 与 bin 一致(每条射线只计算一次)。
    uint octant = (7u - uint((((rd.x < 0.0f) ? 4u : 0u) | ((rd.y < 0.0f) ? 2u : 0u)) | (rd.z < 0.0f ? 1u : 0u))) * 16843009u;

    for (uint iter = 0u; iter < 1024u; ++iter)
    {
        uint2 node_entry;
        uint2 child;
        int push_depth;
        if ((trav.y & 0xFF000000u) != 0u)
        {
            // 内部节点:弹出最高槽位,其余部分留在栈上。
            uint b = firstbithigh(trav.y);
            uint key = trav.y & ~(1u << (b & 31u));
            uint2 kept = trav;
            kept.y = key;
            int nd;
            if ((key & 0xFF000000u) != 0u)
            {
#ifdef TINYGLADE_RT_LDS_STACK_SPILL
                if (stack_size >= 4u)
                {
                    rt_lds_stack[lane + (stack_size - 4u) * 64u] = kept;
                }
                else
#endif
                {
                    stack[stack_size] = kept;
                }
                nd = int(stack_size) + 1;
            }
            else
            {
                nd = int(stack_size);
            }

            // 对被弹出的槽位做莫顿子索引解码。
            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh.Load<uint4>(i * 16u + (trav.x + countbits(trav.y & (~(0xFFFFFFFFu << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80u + 0u);
            }

            // slab 求交,使用 inv_dir/offset。尺寸 = asfloat(byte << 23) = 2^(byte-127)。
            float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / rd.x,
                                       asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / rd.y,
                                       asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / rd.z);
            float3 offset = (asfloat(n.data[0].xyz) - ro) / rd;

            // 子节点 A(data[1].z):lane 掩码公式 + 八分位符号选择的角点。
            uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count0 = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = (rd.x < 0.0f) ? n.data[2].z : n.data[2].x;
            uint c1 = (rd.x < 0.0f) ? n.data[2].x : n.data[2].z;
            uint c2 = (rd.y < 0.0f) ? n.data[3].z : n.data[3].x;
            uint c3 = (rd.y < 0.0f) ? n.data[3].x : n.data[3].z;
            uint c4 = (rd.z < 0.0f) ? n.data[4].z : n.data[4].x;
            uint c5 = (rd.z < 0.0f) ? n.data[4].x : n.data[4].z;

            // 子节点 B(data[1].w):同样的 lane 掩码公式 + 符号选择的角点。
            uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count1 = (n.data[1].w >> 5u) & 117901063u;
            uint d0 = (rd.x < 0.0f) ? n.data[2].w : n.data[2].y;
            uint d1 = (rd.x < 0.0f) ? n.data[2].y : n.data[2].w;
            uint d2 = (rd.y < 0.0f) ? n.data[3].w : n.data[3].y;
            uint d3 = (rd.y < 0.0f) ? n.data[3].y : n.data[3].w;
            uint d4 = (rd.z < 0.0f) ? n.data[4].w : n.data[4].y;
            uint d5 = (rd.z < 0.0f) ? n.data[4].y : n.data[4].w;

            // 两个子节点各 4 个字节槽的 count-byte 掩码累加。
            uint accum = 0u;
            [unroll]
            for (uint s = 0u; s < 32u; s += 8u)
            {
                float3 lo = (float3(float((c0 >> s) & 255u), float((c2 >> s) & 255u), float((c4 >> s) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((c1 >> s) & 255u), float((c3 >> s) & 255u), float((c5 >> s) & 255u)) * inv_dir) + offset;
                bool hit = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit)
                {
                    accum |= (((count0 >> s) & 255u) << ((xor0 >> s) & 31u));
                }
            }
            [unroll]
            for (uint s = 0u; s < 32u; s += 8u)
            {
                float3 lo = (float3(float((d0 >> s) & 255u), float((d2 >> s) & 255u), float((d4 >> s) & 255u)) * inv_dir) + offset;
                float3 hi = (float3(float((d1 >> s) & 255u), float((d3 >> s) & 255u), float((d5 >> s) & 255u)) * inv_dir) + offset;
                bool hit = max(lo.x, max(lo.y, max(lo.z, 9.9999997473787516355514526367188e-05f))) <= min(hi.x, min(hi.y, min(hi.z, best_t)));
                if (hit)
                {
                    accum |= (((count1 >> s) & 255u) << ((xor1 >> s) & 31u));
                }
            }

            // 节点入口(data[1].x + 累加槽位数)与三角形叶子掩码。
            node_entry = uint2(n.data[1].x, key);
            node_entry.y = (accum & 0xFF000000u) | ((n.data[0].w >> 24u) & 255u);
            child = uint2(n.data[1].y, accum & 0x00FFFFFFu);
            push_depth = nd;
        }
        else
        {
            // 叶子 / 栈入口:处理当前节点的三角形掩码。
            node_entry = uint2(0u, 0u);
            child = trav;
            push_depth = int(stack_size);
        }

        // 处理 `child` 的三角形列表(节点索引 + 打包的三角形掩码)。
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
                tri.v[j] = rt_triangles.Load<float>(j * 4u + tindex * 24u + 0u);
            }
            [unroll]
            for (uint j = 0u; j < 3u; ++j)
            {
                tri.e[j] = rt_triangles.Load<uint>(j * 4u + tindex * 24u + 12u);
            }
            float3 edge_a = float3(rt_unpack_half2x16((tri.e[0] >> 16u) & 65535u).x,
                                   rt_unpack_half2x16((tri.e[1] >> 16u) & 65535u).x,
                                   rt_unpack_half2x16((tri.e[2] >> 16u) & 65535u).x);
            float3 edge_b = float3(rt_unpack_half2x16(tri.e[0] & 65535u).x,
                                   rt_unpack_half2x16(tri.e[1] & 65535u).x,
                                   rt_unpack_half2x16(tri.e[2] & 65535u).x);
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

        // 遍历控制流:栈耗尽则跳出,否则继续弹栈。
        if ((node_entry.y & 0xFF000000u) == 0u)
        {
            if (push_depth == 0)
            {
                break;
            }
            int pd = push_depth - 1;
#ifdef TINYGLADE_RT_LDS_STACK_SPILL
            if (pd >= 4)
            {
                trav = rt_lds_stack[lane + uint((push_depth - 5) * 64)];
            }
            else
#endif
            {
                trav = stack[pd];
            }
            stack_size = uint(pd);
        }
        else
        {
            trav = node_entry;
            stack_size = uint(push_depth);
        }
    }

    if (best_triangle != -1 && best_t < t_max)
    {
        hit_triangle = uint(best_triangle);
        hit_t = best_t;
        hit_bary = float2(0.0f, 0.0f);
        return true;
    }
    return false;
}

#endif // TINYGLADE_INC_RT_HLSL
