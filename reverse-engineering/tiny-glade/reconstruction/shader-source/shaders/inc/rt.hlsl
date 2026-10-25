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

// Software-BVH node: 5 x uint4 (80 bytes). Bounds are quantized: min corner is
// stored as float3 in data[0].xyz; the node extent per axis is 2^(byte) using
// the three bytes of data[0].w; each of the up-to-4 child AABBs is encoded as
// two byte-triplets across data[2].x/.z, data[3].x/.z, data[4].x/.z (child A)
// and data[2].y/.w, data[3].y/.w, data[4].y/.w (child B).
struct PackedBlBvhNode {
    uint4 data[5];
};

// Software-BVH triangle (24 bytes): 3 vertices + 3 packed edge/alpha words.
struct PackedTriangle {
    float v[3];
    uint e[3];
};

// Load the triangle at the given index from the software BVH triangle buffer.
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

// Möller–Trumbore ray/triangle intersection; returns the hit distance.
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

// Unpack a half2 from the low and high halfs of a uint (packed triangle edges).
float2 rt_unpack_half2x16(uint value)
{
    return f16tof32(uint2(value & 0xFFFFu, value >> 16u));
}

// Morton-ordered software-BVH traversal (transcribed from the deferred_rt /
// ice_deferred_rt bins). `stack` is the per-lane 4-entry stack; when it overflows the
// traversal spills to the groupshared rt_lds_stack[256] at lane + (sp-4)*64 when the
// consumer defined TINYGLADE_RT_LDS_STACK_SPILL and declared that groupshared before
// including this file (the gi trace does; see gi/trace_gi_candidates.cs.hlsl).
// Returns true on hit, filling hit_triangle / hit_t / hit_bary.
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

    // Ray-direction octant permutation byte (0x01010101 per octant), hoisted out of
    // the traversal loop to mirror the bin (computed once per ray).
    uint octant = (7u - uint((((rd.x < 0.0f) ? 4u : 0u) | ((rd.y < 0.0f) ? 2u : 0u)) | (rd.z < 0.0f ? 1u : 0u))) * 16843009u;

    for (uint iter = 0u; iter < 1024u; ++iter)
    {
        uint2 node_entry;
        uint2 child;
        int push_depth;
        if ((trav.y & 0xFF000000u) != 0u)
        {
            // Internal node: pop the highest slot, keep the remainder on the stack.
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

            // Morton child-index decode of the popped slot.
            PackedBlBvhNode n;
            [unroll]
            for (uint i = 0u; i < 5u; ++i)
            {
                n.data[i] = rt_bl_bvh.Load<uint4>(i * 16u + (trav.x + countbits(trav.y & (~(0xFFFFFFFFu << (((b - 24u) ^ (octant & 255u)) & 31u))))) * 80u + 0u);
            }

            // Slab test with inv_dir/offset. Extent = asfloat(byte << 23) = 2^(byte-127).
            float3 inv_dir = float3(asfloat(((n.data[0].w >> 0u) & 255u) << 23u) / rd.x,
                                       asfloat(((n.data[0].w >> 8u) & 255u) << 23u) / rd.y,
                                       asfloat(((n.data[0].w >> 16u) & 255u) << 23u) / rd.z);
            float3 offset = (asfloat(n.data[0].xyz) - ro) / rd;

            // Child A (data[1].z): lane-mask formula + octant sign-selected corners.
            uint xor0 = (n.data[1].z ^ (octant & ((((n.data[1].z & (n.data[1].z << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count0 = (n.data[1].z >> 5u) & 117901063u;
            uint c0 = (rd.x < 0.0f) ? n.data[2].z : n.data[2].x;
            uint c1 = (rd.x < 0.0f) ? n.data[2].x : n.data[2].z;
            uint c2 = (rd.y < 0.0f) ? n.data[3].z : n.data[3].x;
            uint c3 = (rd.y < 0.0f) ? n.data[3].x : n.data[3].z;
            uint c4 = (rd.z < 0.0f) ? n.data[4].z : n.data[4].x;
            uint c5 = (rd.z < 0.0f) ? n.data[4].x : n.data[4].z;

            // Child B (data[1].w): same lane-mask formula + sign-selected corners.
            uint xor1 = (n.data[1].w ^ (octant & ((((n.data[1].w & (n.data[1].w << 1u)) & 269488144u) >> 4u) * 255u))) & 522133279u;
            uint count1 = (n.data[1].w >> 5u) & 117901063u;
            uint d0 = (rd.x < 0.0f) ? n.data[2].w : n.data[2].y;
            uint d1 = (rd.x < 0.0f) ? n.data[2].y : n.data[2].w;
            uint d2 = (rd.y < 0.0f) ? n.data[3].w : n.data[3].y;
            uint d3 = (rd.y < 0.0f) ? n.data[3].y : n.data[3].w;
            uint d4 = (rd.z < 0.0f) ? n.data[4].w : n.data[4].y;
            uint d5 = (rd.z < 0.0f) ? n.data[4].y : n.data[4].w;

            // Count-byte mask accumulation across both children (4 byte-slots each).
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

            // Node entry (data[1].x + accumulated slot count) and triangle leaf mask.
            node_entry = uint2(n.data[1].x, key);
            node_entry.y = (accum & 0xFF000000u) | ((n.data[0].w >> 24u) & 255u);
            child = uint2(n.data[1].y, accum & 0x00FFFFFFu);
            push_depth = nd;
        }
        else
        {
            // Leaf / stack entry: process the current node's triangle mask.
            node_entry = uint2(0u, 0u);
            child = trav;
            push_depth = int(stack_size);
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

        // Traversal control flow: break when the stack is exhausted, else pop.
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
