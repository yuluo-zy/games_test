// ==RECONSTRUCTED== shaders/rocky_terrain/stairs/stairs.cs.hlsl
// target: _rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0.bin
// evidence: spirv-cross (_rocky_terrain_stairs_stairs.cs.hlsl_cs_main_0_0.hlsl.cross) + SPIR-V disasm:
//   per-cell marching-squares contouring over the terrain height field. A contouring-grid cell is
//   kept only when at least one corner sits on rock (rocky_terrain.x clamp>0.2), near a path
//   (smoothstep(path_mask.x)>0.99) and off water (water_mask.y<0.5). For each stair iso-level
//   (lerp(0.8,10,level/16)) the 4-bit corner-state mask selects the crossing edges (marching_squares.inc),
//   find_root (inc/find_root.hlsl, via place_stairs.inc) bisects each edge to the target height, and
//   place_stairs.inc writes a Stair{Affine3Packed} into __alias_stairs_buffer and bumps
//   __alias_draw_commands[draw_indirect_buffer_offset].instanceCount; pebbles reuse the buffer at
//   pebble_buffer_offset and the pebble_indirect_draw_idx command. Binding/entry/LocalSize verbatim;
//   the two SSBOs are plain RWStructuredBuffer (target module emits only OpCapability Shader, no
//   SPV_EXT_descriptor_indexing, so NO [[vk::ext_capability(5302)]] here — verified from the bin).
// bindings: sampler_llb s4, sampler_default s9, __alias_draw_commands u13 (SSBO), $Globals b14 (UBO),
//   terrain_heightmap t15, rocky_terrain t17, path_mask t18, water_mask t19, __alias_stairs_buffer u20 (SSBO)
// entry: cs_main | local_size [8,8,1] | gid uint3 (SV_DispatchThreadID)
// status: O (algorithm); residual do-while/phi/switch + int3-Load codegen = dxc-version artifacts;
//   OpSource/OpString/OpLine count drift = reconstructed inc-layer extra include edges (inc/* read-only)

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/draw_indirect_types.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

struct Stair
{
    Affine3Packed transform;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
RWStructuredBuffer<Stair> __alias_stairs_buffer : register(u20, space0);

Texture2D<float4> rocky_terrain : register(t17, space0);
Texture2D<float4> path_mask : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 path_mask_dims;
    uint2 water_mask_dims;
    uint2 contouring_grid_dims;
    uint pebble_indirect_draw_idx;
    uint pebble_buffer_offset;
};

// terrain_height(float2) is now provided by inc/terrain_heightmap.inc.hlsl
// (adjudicated inc-fix #3); it samples the global terrain_heightmap texture
// declared below at register t15.

// Bounds-checked rocky-terrain sample; outside the [0,1]^2 uv domain it reads 0.
float4 sample_rocky(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
}

// Bounds-checked water-mask sample; outside the [0,1]^2 uv domain it reads 0.
float water_mask_value(float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f;
    if (uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f)
        return 0.0f;
    return water_mask.SampleLevel(sampler_default, uv, 0.0f).y;
}

// The three stair include files (this shader's contouring logic), dependency
// chain: isolines.inc -> marching_squares.inc -> place_stairs.inc -> inc/find_root.hlsl.
#include "isolines.inc.hlsl"

[numthreads(8, 8, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    if (gl_GlobalInvocationID.x > contouring_grid_dims.x ||
        gl_GlobalInvocationID.y > contouring_grid_dims.y)
        return;

    float2 grid_dims = float2(contouring_grid_dims);
    float2 center = (float2(gl_GlobalInvocationID.x + 0.5f, gl_GlobalInvocationID.y + 0.5f) / grid_dims - 0.5f) * 130.0f;
    float2 half = 65.0f / grid_dims;

    float2 corners[4];
    corners[0] = center + float2(-half.x, -half.y);
    corners[1] = center + float2(half.x, -half.y);
    corners[2] = center + float2(half.x, half.y);
    corners[3] = center + float2(-half.x, half.y);

    int eligible = 0;
    for (int i = 0; i < 4; i++)
    {
        float rocky = clamp(sample_rocky(corners[i]).x, 0.0f, 1.0f);
        bool near_path = smoothstep(0.075f, 0.125f, path_mask.SampleLevel(sampler_llb, corners[i] * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5f, 0.0f).x) > 0.99f;
        bool off_water = water_mask_value(corners[i]) < 0.5f;
        if (rocky > 0.2f && near_path && off_water)
            eligible++;
    }
    if (eligible == 0)
        return;

    float4 corner_heights;
    for (int i = 0; i < 4; i++)
        corner_heights[i] = terrain_height(corners[i]);

    place_stair_isolines(corners, corner_heights);
}
