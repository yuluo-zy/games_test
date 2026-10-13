// ==RECONSTRUCTED== shaders/garden/spawn_flowers_buildings.cs.hlsl | evidence: spirv-cross of _garden_spawn_flowers_buildings.cs.hlsl_cs_main_0.bin (two jittered sites; exclusion; garden_mask.y>0.3 skip; buildings_mask.y band 0.55 (0.3/0.4 widths); buildings_non_blurred>0.99999 skip; perlin(p*0.5)>0.6 && buildings.x>0 gate; FlowerXform cmd 11, type 0/1) | status: O (algorithm from SPIR-V); I (thresholds)

#include "inc/common.hlsl"
#include "inc/utils.hlsl"
#include "inc/consts.hlsl"
#include "inc/hash.hlsl"
#include "inc/draw_indirect.hlsl"
#include "inc/perlin.hlsl"
Texture2D<float4> terrain_heightmap : register(t15, space0);
#include "inc/terrain_heightmap.inc.hlsl"
#include "inc/uv.hlsl"
#include "buildings/building_mask.inc.hlsl"
#include "garden/defs.inc.hlsl"
#include "garden/placement_utils.inc.hlsl"

cbuffer $Globals : register(b14, space0)
{
    int draw_indirect_buffer_offset;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 garden_mask_dims;
    uint2 buildings_mask_dims;
    uint2 buildings_mask_non_blurred_dims;
    int2 grid_dims;
    uint2 path_mask_dims;
    uint2 rocky_terrain_dims;
    uint2 water_mask_dims;
    uint2 foliage_exclusion_mask_dims;
};

RWStructuredBuffer<DrawIndexedIndirectCommand> __alias_draw_commands : register(u13, space0);
Texture2D<float4> garden_mask : register(t17, space0);
Texture2D<float4> buildings_mask : register(t18, space0);
Texture2D<float4> buildings_mask_non_blurred : register(t19, space0);
Texture2D<float4> path_mask : register(t21, space0);
Texture2D<float4> rocky_terrain : register(t22, space0);
Texture2D<float4> water_mask : register(t23, space0);
Texture2D<float4> foliage_exclusion_mask : register(t24, space0);
RWStructuredBuffer<FlowerXform> __alias_flower_buffer : register(u20, space0);

[numthreads(8, 8, 1)]
void cs_main(uint3 id : SV_DispatchThreadID)
{
    float2 base = ((float2(id.xy) + 0.5f) / float2(grid_dims) - 0.5f) * 130.0f;

    float rnd0 = rand01(hash((id.x * id.y / 2u) + (id.x * 20u) + 100u));
    float2 p0 = base + lerp(-0.75f, 0.75f, rnd0).xx;
    uint seed1 = uint((rnd0 * 100.0f) - 341.0f);
    float2 p1 = base + lerp(-0.75f, 0.75f, rand01(hash(seed1))).xx;

    int2 gid = int2(id.xy);

    // Primary site
    {
        float2 uv = placement_uv(p0);
        if (placement_excluded(terrain_heightmap, terrain_heightmap_dims, path_mask, path_mask_dims,
                               rocky_terrain, water_mask, foliage_exclusion_mask,
                               sampler_llb, sampler_default, uv))
            return;
        if (garden_mask.Load(int3(uint2(uv * float2(path_mask_dims)), 0u)).y > 0.3f)
            return;

        float4 bm = buildings_mask.SampleLevel(sampler_default, uv, 0.0f);
        float band = band_falloff_garden(abs(bm.y - 0.55f), 3.3333332538604736328125f);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
            return;

        if (perlin(p0 * 0.5f) > 0.6f && bm.x > 0.0f)
        {
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.2f, 1.0f, band);
            if (min(scale.x, 0.75f) < 0.4f)
                return;
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 2.0f;

            float4x4 m = placement_transform(float3(p0.x, 0.5f, p0.y), s3,
                                             3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            Affine3Packed packed = pack_affine(m);
            float2 pos_xz = mul(transpose(float4x3(
                float3(packed.data[0], packed.data[1], packed.data[2]),
                float3(packed.data[3], packed.data[4], packed.data[5]),
                float3(packed.data[6], packed.data[7], packed.data[8]),
                float3(packed.data[9], packed.data[10], packed.data[11]))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
            packed.data[11] += get_terrain_height(terrain_heightmap, sampler_default, pos_xz);

            FlowerXform f;
            f.transform = packed;
            f.color = float3(-1.0f, -1.0f, -1.0f);
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
        if (garden_mask.Load(int3(uint2(uv * float2(path_mask_dims)), 0u)).y > 0.3f)
            return;

        float4 bm = buildings_mask.SampleLevel(sampler_default, uv, 0.0f);
        float band = band_falloff_garden(abs(bm.y - 0.55f), 2.5f);
        if (buildings_mask_non_blurred.SampleLevel(sampler_default, uv, 0.0f).x > 0.999989986419677734375f)
            return;

        if (perlin((p1 * 0.5f) + 100.0f) > 0.6f && bm.x > 0.0f)
        {
            float fs = lerp(0.8f, 1.2f, rand01(hash(gid.x + gid.y)));
            float3 scale = fs.xxx * lerp(0.2f, 1.0f, band);
            if (min(scale.x, 0.75f) < 0.4f)
                return;
            float3 s3 = float3(min(scale.x, 0.75f), (scale.y * 1.2f) * fs, min(scale.z, 0.75f)) * 2.0f;

            float4x4 m = placement_transform(float3(p1.x, 0.5f, p1.y), s3,
                                             3.14f * rand01(hash(uint(gid.x * gid.y + gid.x))));

            uint base;
            InterlockedAdd(__alias_draw_commands[11u].instanceCount, 1u, base);
            Affine3Packed packed = pack_affine(m);
            float2 pos_xz = mul(transpose(float4x3(
                float3(packed.data[0], packed.data[1], packed.data[2]),
                float3(packed.data[3], packed.data[4], packed.data[5]),
                float3(packed.data[6], packed.data[7], packed.data[8]),
                float3(packed.data[9], packed.data[10], packed.data[11]))), float4(0.0f, 0.0f, 0.0f, 1.0f)).xz;
            packed.data[11] += get_terrain_height(terrain_heightmap, sampler_default, pos_xz);

            FlowerXform f;
            f.transform = packed;
            f.color = float3(-1.0f, -1.0f, -1.0f);
            f.flower_type = 1u;
            __alias_flower_buffer[base] = f;
        }
    }
}
