// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/place_stairs.inc.hlsl
// KIND: include_only | embedded only in _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   places one Stair{Affine3Packed} per iso-height contour segment of a contouring-grid cell.
//   The two segment endpoints come from inc/find_root.hlsl (2-iteration sign-change bisection)
//   on the two crossing edges, each lifted to float3(x, height, z). Rejection tests (in order):
//   rocky_terrain.x at the midpoint < 0.01; both segment ends not near a path
//   (smoothstep(0.075,0.125,path_mask.x) < 0.01); segment length < 0.2. Then the stair transform is
//   translation(pos - (0,0.5,0)) * basis(forward = normalize(p1-p0), up = +Y) * scale(len,1,0.8),
//   with pos = midpoint - slope offset (cross of normalized gradient-frame vectors scaled by
//   lerp(0,0.3,clamp(rocky.x+rocky.z,0,1))). Records are appended with
//   __alias_draw_commands[draw_indirect_buffer_offset].instanceCount as the slot. A pebble is
//   additionally placed with probability rand01(hash(seed ^ 23784)) > 0.85 (seed = uint((p0.x+p0.y+p0.z)*100),
//   scale lerp(0.2,0.4,rand01(hash(seed ^ 90455))), lerp(p0,p1,rand01(hash(seed ^ 3845))) - offset),
//   appended at pebble_buffer_offset + slot under the pebble_indirect_draw_idx command.
// status: O (algorithm verbatim from inlined use-sites); I (function name/signature — inlined)

#pragma once
#ifndef TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL
#define TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL

#include "inc/find_root.hlsl"

// Place a stair step (and a pebble) along the iso-height segment of a
// contouring-grid cell: the segment crosses edge (e0a,e0b) and edge (e1a,e1b)
// at height `h`, and the endpoints are located with find_root.
void place_stairs(float2 e0a, float2 e0b, float2 e1a, float2 e1b, float h)
{
    float2 pa = find_root(e0a, e0b, h);
    float2 pb = find_root(e1a, e1b, h);
    float3 p0 = float3(pa.x, h, pa.y);
    float3 p1 = float3(pb.x, h, pb.y);

    float3 mid = lerp(p0, p1, 0.5f);
    if (sample_rocky(mid.xz).x < 0.01f)
        return;

    bool not_near_path0 = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, p0.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) < 0.01f;
    bool not_near_path1 = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, p1.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) < 0.01f;
    if (not_near_path0 && not_near_path1)
        return;

    float dist = distance(p0, p1);
    if (dist < 0.2f)
        return;

    float4 rock = sample_rocky(mid.xz);
    float4 g = terrain_heightmap.GatherRed(sampler_default, mid.xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, int2(0, 0));
    float2 grad = ((float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5f) * 26.5f) / (130.0f / float2(terrain_heightmap_dims));
    float3 offset = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f))) * lerp(0.0f, 0.3f, clamp(rock.x + rock.z, 0.0f, 1.0f));
    float3 pos = mid - offset;
    float3 forward = normalize(p1 - p0);
    uint seed = uint((p0.x + p0.y + p0.z) * 100.0f);

    // Rotation-only basis from the (already normalized) forward direction
    // (built inline — affine_basis would re-normalize and add a normalize).
    float4x4 basis = transpose(float4x4(
        float4(forward, 0.0f),
        float4(0.0f, 1.0f, 0.0f, 0.0f),
        float4(cross(forward, float3(0.0f, 1.0f, 0.0f)), 0.0f),
        float4(0.0f, 0.0f, 0.0f, 1.0f)));
    float4x4 m = mul(affine_translation(pos - float3(0.0f, 0.5f, 0.0f)), mul(basis, affine_scale(float3(dist, 1.0f, 0.8f))));

    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);
    __alias_stairs_buffer[slot].transform = pack_affine(m);

    if (rand01(hash(seed ^ 23784u)) > 0.85f)
    {
        float pebble_scale = lerp(0.2f, 0.4f, rand01(hash(seed ^ 90455u)));
        float t = rand01(hash(seed ^ 3845u));
        float3 pebble_pos = lerp(p0, p1, t) - offset;
        uint pebble_slot;
        InterlockedAdd(__alias_draw_commands[pebble_indirect_draw_idx].instanceCount, 1u, pebble_slot);
        float4x4 pm = mul(affine_translation(pebble_pos), mul(basis, affine_scale(float3(pebble_scale, pebble_scale, pebble_scale))));
        __alias_stairs_buffer[pebble_buffer_offset + pebble_slot].transform = pack_affine(pm);
    }
}

#endif // TINYGLADE_STAIRS_PLACE_STAIRS_INC_HLSL
