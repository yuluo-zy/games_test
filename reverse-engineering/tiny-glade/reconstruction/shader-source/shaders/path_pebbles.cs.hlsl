// ==RECONSTRUCTED== shaders/path_pebbles.cs.hlsl | target: _path_pebbles.cs.hlsl_cs_main_0.bin | evidence: spirv-cross + SPIR-V disasm (grid-cell scatter over scatter_grid_dim, 104-unit cell, hash jitter; height -> path_mask rejection <0.9; rocky/water in-bounds rejection; per-vertex random scale/angle/tilt via hash(seed±k); atomic-increment draw_commands[draw_indirect_buffer_offset].instanceCount; compose scale*T(basis)*translation, pack_affine, store into pebble_placement_buffer[instance]) | bindings: sampler_llb s4, sampler_default s9, __alias_draw_commands u13, $Globals b14, terrain_heightmap t15, path_mask t17, rocky_terrain t18, water_mask t19, __alias_pebble_placement_buffer u20 | local_size [64,1,1] | status: O (algorithm); residual do-while/phi + int3-Load codegen = dxc-version artifacts

#include "inc/common.hlsl"
#include "inc/debug_utils.hlsl"
#include "inc/tracked_buffer.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/math_const.hlsl"
#include "inc/gaussian_rand.hlsl"
#include "inc/simplex_noise.hlsl"
#include "inc/draw_indirect_types.hlsl"
#include "inc/draw_indirect.hlsl"

// The original dxc build emits SPV_EXT_descriptor_indexing + RuntimeDescriptorArray
// for the structured-buffer SSBOs; forced here to match the target module.
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
RWStructuredBuffer<Affine3Packed> __alias_pebble_placement_buffer : register(u20, space0);

Texture2D<float4> terrain_heightmap : register(t15, space0);
Texture2D<float4> path_mask : register(t17, space0);
Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t19, space0);

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint pebble_buffer_size;
    int scatter_grid_dim;
};

// inc/scatter.hlsl references scatter_grid_dim as a global, so $Globals must be
// declared before its include.
#include "inc/scatter.hlsl"
#include "inc/frame_constants.hlsl"
#include "inc/uv.hlsl"
#include "inc/ray_cone.hlsl"
#include "inc/terrain_heightmap.inc.hlsl"
#include "paths/path_mask_sampler.inc.hlsl"
#include "inc/texture_utils.hlsl"
#include "inc/rocky_terrain.inc.hlsl"
#include "water/water_mask_sampler.inc.hlsl"

[numthreads(64, 1, 1)]
void cs_main(uint3 gl_GlobalInvocationID : SV_DispatchThreadID)
{
    int index = int(gl_GlobalInvocationID.x);
    int grid = scatter_grid_dim;
    int area = grid * grid;
    int cell = index - area * (index / area);
    float g = float(grid);
    float2 grid_pos = (float2(float(cell - grid * (cell / grid)) / g, float(cell / grid) / g) - 0.5f) * 104.0f;

    uint iu = uint(index);
    float2 jitter = float2(rand01(hash(iu)), rand01(hash(iu ^ 188053769u))) * 6.3f;

    float px = grid_pos.x + jitter.x;
    float pz = grid_pos.y + jitter.y;

    float h = lerp(-2.5f, 24.0f, terrain_heightmap.SampleLevel(sampler_default, (float3(px, 0.0f, pz).xz * 0.0076923076994717121124267578125f + 0.5f), 0.0f).x);
    float2 mask_uv = (float3(px, h, pz).xz * 0.0076923076994717121124267578125f) + 0.5f;

    if (path_mask.SampleLevel(sampler_llb, mask_uv, 0.0f).x < 0.9f)
        return;

    bool in_bounds = !(mask_uv.x < 0.0f) && !(mask_uv.x > 1.0f) && !(mask_uv.y < 0.0f) && !(mask_uv.y > 1.0f);

    float rocky = 0.0f;
    if (in_bounds)
        rocky = rocky_terrain.SampleLevel(sampler_default, mask_uv, 0.0f).x;

    float water = 0.0f;
    if (in_bounds)
        water = water_mask.SampleLevel(sampler_default, mask_uv, 0.0f).y;

    if (rocky > 0.0f || water > 0.5f)
        return;

    int seed = (int(px) ^ (345 + int(pz))) ^ 2378;

    float scale = lerp(0.4f, 1.2f, rand01(hash(uint(seed)))) * 1.2f;

    float angle = (1.0f - asfloat((hash(uint(seed)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir = normalize(float2(cos(angle), sin(angle)));

    float angle2 = (1.0f - asfloat((hash(uint(seed - 235)) & 0x7FFFFFu) | 0x3F800000u)) * 6.283185482025146484375f;
    float2 dir2 = normalize(float2(cos(angle2), sin(angle2)));

    float tilt_slope = rand01(hash(uint(seed + 2345))) * 0.5f;
    float3 tilt = normalize(float3(dir2.x * tilt_slope, 1.0f, dir2.y * tilt_slope));

    uint slot;
    InterlockedAdd(__alias_draw_commands[uint(draw_indirect_buffer_offset)].instanceCount, 1u, slot);

    float3 f = float3(dir.x, 0.0f, dir.y);
    float3 r = cross(f, tilt);
    float4x4 basis = float4x4(float4(r, 0.0f), float4(tilt, 0.0f), float4(f, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 scale_m = float4x4(float4(scale, 0.0f, 0.0f, 0.0f), float4(0.0f, scale, 0.0f, 0.0f), float4(0.0f, 0.0f, scale, 0.0f), float4(0.0f, 0.0f, 0.0f, 1.0f));
    float4x4 trans = float4x4(float4(1.0f, 0.0f, 0.0f, px), float4(0.0f, 1.0f, 0.0f, h), float4(0.0f, 0.0f, 1.0f, pz), float4(0.0f, 0.0f, 0.0f, 1.0f));

    float4x4 m = mul(mul(scale_m, transpose(basis)), trans);
    __alias_pebble_placement_buffer[slot] = pack_affine(m);
}
