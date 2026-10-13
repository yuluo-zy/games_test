// ==RECONSTRUCTED== shaders/light_binning/bin_lights_to_cells.cs.hlsl | status: O (grid binning algorithm); O/I (reservoir replacement RNG carry-chain)
// MERGE (gi-deferred review 2026-10-13): A body + B feature flag. A's RNG matches the bin
// exactly (wang_hash((idx*1664525 + hash(frame_index) + 1013904223) * 1664525)) and A declares
// lantern_buf as StructuredBuffer<LanternPacked> (bin debug name LanternPacked). B's
// BIN_LIGHTS_FULL flag reproduces the 8fae reduced variant (count-only, no light_indices /
// frame_constants). Reservoir replacement uses the bin's carried-rng mantissa form
// (asfloat(rng & 0x7FFFFF | 0x3F800000) - 1) per the review fidelity note.
// Evidence: _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_14dca8ba9c11c197.bin (LocalSize 64, entry cs_main).
//   bindings: lantern_buf t13 (StructuredBuffer<LanternPacked>), $Globals b14 {lantern_count},
//   __alias_indirection_grid u15 (RWByteAddressBuffer), __alias_light_indices u16
//   (RWByteAddressBuffer), frame_constants set1/b0; caps {Shader, RuntimeDescriptorArray}.
//   For each lantern (dispatch thread id): influence radius r = 6*sqrt(intensity)
//   (asfloat bit-trick), grid-space AABB from (pos ± r), and for every cell in the
//   AABB that the radius sphere overlaps (point-to-AABB distance < r2) reserve a slot:
//   InterlockedAdd(indirection_grid[cell], 1<<20) bumps the count field (low20 = first
//   index), write lantern id into light_indices[start+count]; past 64 lights, a
//   reservoir sample replaces a hash-chosen slot with probability 64/(count+1).
//   Two shipped variants (14dca8ba full / 8faee3cc reduced): the reduced variant drops
//   frame_constants + light_indices writes (compile-time feature toggle BIN_LIGHTS_FULL).

#ifndef BIN_LIGHTS_FULL
#define BIN_LIGHTS_FULL 1
#endif

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/perlin.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/math.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/pack_unpack.hlsl"
#include "grid.inc.hlsl"

StructuredBuffer<LanternPacked> lantern_buf : register(t13, space0);

cbuffer $Globals : register(b14, space0)
{
    uint lantern_count;
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
#if BIN_LIGHTS_FULL
RWByteAddressBuffer __alias_light_indices : register(u16, space0);
#endif

// 2nd-stage integer hash (Wang) observed after the LCG mix of lantern index and
// hash(frame_index) in the full bin variant.
uint wang_hash(uint x)
{
    x ^= (x >> 11u);
    x ^= (x << 7u) & 2636928640u;
    x ^= (x << 15u) & 4022730752u;
    x ^= (x >> 18u);
    return x;
}

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    uint lantern_idx = gl_GlobalInvocationID.x;
    if (lantern_idx >= lantern_count) {
        return;
    }

    float3 lantern_pos = lantern_buf[lantern_idx].position;
    float intensity = f16tof32(lantern_buf[lantern_idx].intensity__color_id & 0xFFFFu);
    if (intensity <= 0.0f) {
        return;
    }

    // Lantern influence radius = 6 * sqrt(intensity) via the fast-sqrt bit trick
    // (magic constant from the original disassembly).
    float radius = 6.0f * asfloat(532487669 + (asint(intensity) >> 1));

    // Grid-space AABB of the influence sphere.
    uint3 cell_min = uint3(clamp(floor((lantern_pos - radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS) - 1.0f));
    uint3 cell_max = uint3(clamp(ceil((lantern_pos + radius) * GRID_WORLD_SCALE + GRID_WORLD_OFFSET),
                                 0.0f, float3(GRID_DIMS)));

#if BIN_LIGHTS_FULL
    // Reservoir RNG seed: wang_hash(((idx * 1664525) + hash(frame_index) + 1013904223) * 1664525).
    uint hash_state = wang_hash(((lantern_idx * 1664525u) + hash(frame_index) + 1013904223u) * 1664525u);
#endif

    for (uint z = cell_min.z; z < cell_max.z; z++) {
        for (uint y = cell_min.y; y < cell_max.y; y++) {
            for (uint x = cell_min.x; x < cell_max.x; x++) {
                uint3 cell = uint3(x, y, z);
                float3 aabb_min = grid_cell_min_world(cell);
                float3 aabb_max = grid_cell_max_world(cell);

                // Squared distance from the lantern to the cell AABB.
                float d2 = radius * radius;
                [unroll]
                for (uint i = 0u; i < 3u; ++i) {
                    if (lantern_pos[i] < aabb_min[i]) {
                        float e = lantern_pos[i] - aabb_min[i];
                        d2 -= e * e;
                    } else if (lantern_pos[i] > aabb_max[i]) {
                        float e = lantern_pos[i] - aabb_max[i];
                        d2 -= e * e;
                    }
                }

                if (d2 > 0.0f) {
                    uint cell_idx = grid_cell_index(cell);
#if BIN_LIGHTS_FULL
                    uint prev;
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u << 20u, prev);
                    uint start = prev & 0xFFFFFu;
                    uint count = prev >> 20u;
                    if (count < 64u) {
                        __alias_light_indices.Store<uint>((start + count) * 4u, lantern_idx);
                    } else {
                        // Reservoir: keep 64 samples, replace with probability 64/(count+1),
                        // using the carried-rng mantissa form the bin uses.
                        float u = asfloat((hash_state & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
                        hash_state = hash(hash_state);
                        if (u < (64.0f / float(count + 1u))) {
                            __alias_light_indices.Store<uint>((start + (hash_state % 64u)) * 4u, lantern_idx);
                            hash_state = hash(hash_state);
                        }
                    }
#else
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u, 1u);
#endif
                }
            }
        }
    }
}
