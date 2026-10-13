// ==RECONSTRUCTED== shaders/light_binning/bin_lights_to_cells.cs.hlsl (candidate gi-deferred-b)
// Evidence: _light_binning_bin_lights_to_cells.cs.hlsl_cs_main_{14dca8ba9c11c197,8faee3cc8c252c04}.bin.
//   bindings 14dc: lantern_buf t13 (ByteAddressBuffer), $Globals b14 {lantern_count},
//     __alias_indirection_grid u15, __alias_light_indices u16, frame_constants set1/b0;
//   bindings 8fae: lantern_buf t13, $Globals b14, __alias_indirection_grid u15 (no indices/frame).
//   LocalSize [64,1,1], entry cs_main.
//   spirv-cross: per lantern, skip when intensity<=0; radius = 6*sqrt(intensity)
//   (fast-sqrt bit trick), grid cell AABB [floor((pos-radius)*scale+offset),
//   ceil((pos+radius)*scale+offset)] clamped to 32x16x32, per intersecting cell an
//   InterlockedAdd (+1 count-only / +1<<20 full), full variant appends the lantern
//   index into the cell's slot range (64 max) or, when full, replaces a random
//   slot with probability 64/(count+1) via a per-thread hash rng (perlin hash2).
//   The two hex variants are compile-time feature variants: BIN_LIGHTS_FULL=1 -> 14dc.
// Status: O (structure from disassembly); I (flag name; fast-sqrt helper name).

#include "inc/common.hlsl"
#include "inc/consts.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/hash.hlsl"
#include "inc/lanterns.hlsl"
#include "inc/math.hlsl"
#include "inc/math_const.hlsl"
#include "inc/pack_unpack.hlsl"
#include "inc/perlin.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/color/srgb.hlsl"
#include "inc/uv.hlsl"
#include "grid.inc.hlsl"

#ifndef BIN_LIGHTS_FULL
#define BIN_LIGHTS_FULL 1
#endif

ByteAddressBuffer lantern_buf : register(t13, space0);

cbuffer $Globals : register(b14, space0)
{
    uint lantern_count;
};

RWByteAddressBuffer __alias_indirection_grid : register(u15, space0);
#if BIN_LIGHTS_FULL
RWByteAddressBuffer __alias_light_indices : register(u16, space0);
#endif

// Fast approximate sqrt via the classic bit trick (radius = 6*sqrt(intensity)).
float fast_sqrt(float x)
{
    return asfloat(532487669 + (asint(x) >> 1));
}

[numthreads(64, 1, 1)]
void cs_main(uint3 globalThreadID : SV_DispatchThreadID)
{
    uint lantern = globalThreadID.x;
    if (lantern >= lantern_count) {
        return;
    }

    LanternPacked lantern_data;
    lantern_data.position = lantern_buf.Load<float3>(lantern * 16u + 0u);
    lantern_data.intensity__color_id = lantern_buf.Load<uint>(lantern * 16u + 12u);

    float intensity = f16tof32(lantern_data.intensity__color_id & 0xffffu);
    if (intensity <= 0.0f) {
        return;
    }

    float radius = 6.0f * fast_sqrt(intensity);
    float3 pos = lantern_data.position;

    uint3 cell_min = uint3(clamp(floor((pos - radius) * BIN_LIGHT_GRID_WORLD_SCALE + BIN_LIGHT_GRID_OFFSET), float3(0.0f, 0.0f, 0.0f), float3(31.0f, 15.0f, 31.0f)));
    uint3 cell_max = uint3(clamp(ceil((pos + radius) * BIN_LIGHT_GRID_WORLD_SCALE + BIN_LIGHT_GRID_OFFSET), float3(0.0f, 0.0f, 0.0f), float3(32.0f, 16.0f, 32.0f)));

#if BIN_LIGHTS_FULL
    uint rng = hash(uint2(lantern, frame_index));
    rng ^= (rng >> 18u);
#endif

    for (uint z = cell_min.z; z < cell_max.z; ++z) {
        for (uint y = cell_min.y; y < cell_max.y; ++y) {
            for (uint x = cell_min.x; x < cell_max.x; ++x) {
                float3 cell_pos = float3(x, y, z);
                float3 cmin = (cell_pos - BIN_LIGHT_GRID_OFFSET) * BIN_LIGHT_GRID_CELL_SIZE;
                float3 cmax = (cell_pos + float3(-15.0f, -1.0f, -15.0f)) * BIN_LIGHT_GRID_CELL_SIZE;

                float r2 = radius * radius;
                if (pos.x < cmin.x) {
                    float d = pos.x - cmin.x;
                    r2 -= d * d;
                } else if (pos.x > cmax.x) {
                    float d = pos.x - cmax.x;
                    r2 -= d * d;
                }
                if (pos.y < cmin.y) {
                    float d = pos.y - cmin.y;
                    r2 -= d * d;
                } else if (pos.y > cmax.y) {
                    float d = pos.y - cmax.y;
                    r2 -= d * d;
                }
                if (pos.z < cmin.z) {
                    float d = pos.z - cmin.z;
                    r2 -= d * d;
                } else if (pos.z > cmax.z) {
                    float d = pos.z - cmax.z;
                    r2 -= d * d;
                }

                if (r2 > 0.0f) {
                    uint cell_idx = (32u * ((32u * y) + z)) + x;
#if BIN_LIGHTS_FULL
                    uint prev;
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u + 0u, 1048576u, prev);
                    uint slot0 = prev & 1048575u;
                    uint cnt = prev >> 20u;
                    if (cnt < BIN_LIGHT_GRID_MAX_LIGHTS) {
                        __alias_light_indices.Store<uint>((slot0 + cnt) * 4u + 0u, lantern);
                    } else {
                        float u = asfloat((rng & 0x7FFFFFu) | 0x3F800000u) - 1.0f;
                        rng = hash(rng);
                        if (u < (float(BIN_LIGHT_GRID_MAX_LIGHTS) / float(cnt + 1u))) {
                            __alias_light_indices.Store<uint>((slot0 + (rng % BIN_LIGHT_GRID_MAX_LIGHTS)) * 4u + 0u, lantern);
                            rng = hash(rng);
                        }
                    }
#else
                    __alias_indirection_grid.InterlockedAdd(cell_idx * 4u + 0u, 1u);
#endif
                }
            }
        }
    }
}
