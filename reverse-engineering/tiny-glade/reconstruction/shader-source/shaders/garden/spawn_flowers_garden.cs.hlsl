// ==RECONSTRUCTED== shaders/garden/spawn_flowers_garden.cs.hlsl | evidence: spirv-cross of _garden_spawn_flowers_garden.cs.hlsl_cs_main_0.bin (two jittered candidate sites; exclusion; buildings<0.5; garden_mask.y band around 0.6; perlin(world*0.5) gate; FlowerXform (cmd 11, 64B, color=(-1|1), type 0/1)) | status: O (algorithm from SPIR-V); I (thresholds/scale literals)

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
Texture2D<float4> path_mask : register(t20, space0);
Texture2D<float4> rocky_terrain : register(t21, space0);
Texture2D<float4> water_mask : register(t22, space0);
Texture2D<float4> foliage_exclusion_mask : register(t23, space0);
RWStructuredBuffer<FlowerXform> __alias_flower_buffer : register(u19, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float2 base = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;
    float rnd0 = rand01(hash(id.x * id.y + id.x + 100u));
    float2 p0 = base + lerp(-0.75f, 0.75f, rnd0).xx;
    uint seed1 = uint((rnd0 * 100.0f) - 34.0f);
    float2 p1 = base + lerp(-0.75f, 0.75f, rand01(hash(seed1))).xx;

    int2 gid = int2(id.xy);

    // Primary site
    {
        float2 uv = placement_uv(p0);
        if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                               rocky_terrain, water_mask, foliage_exclusion_mask,
                               sampler_llb, sampler_default, uv))
            return;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
            return;

        float gy = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y;
        float band = band_falloff_garden(abs(gy - 0.6f), 5.0f);
        bool full = smoothstep(0.9f, 1.0f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y) > 0.0f;

        if (perlin(p0 * 0.5f) * 2.0f > 0.6f && gy > 0.425f)
        {
            float3 pos = float3(p0.x, full ? 1.1f : 0.5f, p0.y);
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.5f, 1.0f, band + (full ? 1.5f : 0.0f));
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 1.5f;

            float4x4 m = placement_transform(pos, s3, 3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            float h = get_terrain_height(terrain_heightmap, sampler_default, p0);
            Affine3Packed packed = pack_affine(m);
            packed.data[11] += h;
            FlowerXform f;
            f.transform = packed;
            f.color = full ? float3(1.0f, 1.0f, 1.0f) : float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 0u;
            __alias_flower_buffer[base] = f;
        }
    }

    // Secondary site
    {
        float2 uv = placement_uv(p1);
        if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                               rocky_terrain, water_mask, foliage_exclusion_mask,
                               sampler_llb, sampler_default, uv))
            return;
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.5f)
            return;

        float gy = garden_mask.SampleLevel(sampler_default, uv, 0.0f).y;
        float band = band_falloff_garden(abs(gy - 0.65f), 3.3333332538604736328125f);
        bool full = smoothstep(0.9f, 1.0f, garden_mask.SampleLevel(sampler_default, uv, 0.0f).y) > 0.0f;

        if (perlin((p1 * 0.5f) + 100.0f) * 2.0f > 0.6f && gy > 0.425f)
        {
            float3 pos = float3(p1.x, full ? 1.1f : 0.5f, p1.y);
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.5f, 1.0f, band + (full ? 1.5f : 0.0f));
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 1.5f;

            float4x4 m = placement_transform(pos, s3, 3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            float h = get_terrain_height(terrain_heightmap, sampler_default, p1);
            Affine3Packed packed = pack_affine(m);
            packed.data[11] += h;
            FlowerXform f;
            f.transform = packed;
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 1u;
            __alias_flower_buffer[base] = f;
        }
    }
}
