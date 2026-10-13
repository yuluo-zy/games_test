// ==RECONSTRUCTED== shaders/garden/spawn_bushes_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_bushes_garden.cs.hlsl_cs_main_0.bin (exclusion masks; buildings_non_blurred<0.5; garden_mask.y+0.005>0.5 -> BushXform (cmd 10, 48B) with T(terrain height)*S*R yaw; 30% -> FlowerXform (cmd 11, 64B, color=-1, type=2)) | status: O (algorithm from SPIR-V); I (smoothstep/threshold literals)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "garden/defs.inc.hlsl"
#include "garden/placement_utils.inc.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t18, space0);
Texture2D<float4> path_mask : register(t21, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> foliage_exclusion_mask : register(t24, space0);
RWStructuredBuffer<BushXform> __alias_bush_buffer : register(u19, space0);
RWStructuredBuffer<FlowerXform> __alias_bush_flowers : register(u20, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float rnd = rand01(hash(id.x * id.y + id.x));
    float2 world_pos = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f + lerp(-1.0f, 1.0f, rnd).xx;

    float2 uv = placement_uv(world_pos);
    if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                           rocky_terrain, water_mask, foliage_exclusion_mask,
                           sampler_llb, sampler_default, uv))
        return;

    if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
        return;

    float g = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y + 0.005f;
    if (g > 0.5f)
    {
        float height = get_terrain_height(terrain_heightmap, sampler_default, world_pos);
        float3 pos = float3(world_pos.x, height, world_pos.y);

        float bush_scale = lerp(0.8f, 1.2f, rand01(hash(id.x + id.y)));
        float3 scale = bush_scale.xxx * lerp(0.3f, 0.8f, sqrt(clamp(
            smoothstep(0.45f, 1.0f, g) + smoothstep(0.6f, 1.0f, g) * 0.2f, 0.0f, 1.0f)));
        float4x4 s = float4x4(
            float4(min(scale.x, 0.75f), 0.0f, 0.0f, 0.0f),
            float4(0.0f, min(scale.y * bush_scale, 1.2f), 0.0f, 0.0f),
            float4(0.0f, 0.0f, min(scale.z, 0.75f), 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 identity = float4x4(
            float4(1.0f, 0.0f, 0.0f, 0.0f),
            float4(0.0f, 1.0f, 0.0f, 0.0f),
            float4(0.0f, 0.0f, 1.0f, 0.0f),
            float4(0.0f, 0.0f, 0.0f, 1.0f));

        float4x4 m = mul(mul(
            affine_translation(pos),
            mul(identity, s)),
            rotate_y(3.14f * rnd));

        uint bush_base;
        InterlockedAdd(__alias_draw_commands[10u].instanceCount, 1u, bush_base);
        __alias_bush_buffer[bush_base] = BushXform(pack_affine(m));

        if (rand01(hash((id.x * 2u) + (id.y / 5u) + 1233u)) > 0.3f && g > 0.8f)
        {
            uint flower_base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, flower_base);
            FlowerXform ff;
            ff.transform = pack_affine(m);
            ff.color = float3(-1.0f, -1.0f, -1.0f);
            ff.flower_type = 2u;
            __alias_bush_flowers[flower_base] = ff;
        }
    }
}
