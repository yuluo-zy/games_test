// ==RECONSTRUCTED== shaders/garden/spawn_small_flowers_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_small_flowers_garden.cs.hlsl_cs_main_0.bin (jittered world pos seed+100; exclusion masks; garden_mask.y in (0.35,0.8) band; Flower03Xform{float4(x,0,z,size)} -> draw cmd 16, stride 16) | status: O (algorithm from SPIR-V); I (band/size literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "inc/perlin.hlsl"
#include "garden/placement_utils.inc.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> path_mask : register(t19, space0);
Texture2D<float4> rocky_terrain : register(t20, space0);
Texture2D<float4> water_mask : register(t21, space0);
Texture2D<float4> foliage_exclusion_mask : register(t22, space0);
RWStructuredBuffer<Flower03Xform> __alias_flower03_buffer : register(u18, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float rnd = rand01(hash(id.x * id.y + id.x + 100u));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-0.75f, 0.75f, rnd).xx;

    float2 uv = placement_uv(world_pos);
    if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                           rocky_terrain, water_mask, foliage_exclusion_mask,
                           sampler_llb, sampler_default, uv))
        return;

    float g = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y - 0.05f;
    if (g > 0.35f && g < 0.8f)
    {
        float size = lerp(0.6f, 1.0f, rand01(hash(id.x + id.y)));

        uint base;
        InterlockedAdd(__alias_draw_commands[16u].instanceCount, 1u, base);
        __alias_flower03_buffer[base] = Flower03Xform(float4(world_pos.x, 0.0f, world_pos.y, size));
    }
}
